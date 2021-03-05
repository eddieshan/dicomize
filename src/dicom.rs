use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

use crate::binary_reader::*;
use crate::dicom_reader::DicomReader;
use crate::dicom_handlers::*;
use crate::dicom_tag::*;
use crate::vr_type;
use crate::tags;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};

const STANDARD_PREAMBLE: &str = "DICM";

fn endian_reader<T: Read+Seek>(reader: &mut T, syntax: TransferSyntax) -> &mut T {
    // TODO: handle big endian reading, for now assuming little endian by default.
    match syntax.endian_encoding {
        EndianEncoding::LittleEndian => reader,
        EndianEncoding::BigEndian    => reader
    }
}

fn next_tag(reader: &mut (impl Read + Seek), syntax: TransferSyntax) -> DicomTag {

    let endian_reader = endian_reader(reader, syntax);

    let group = endian_reader.read_u16();
    let element = endian_reader.read_u16();

    let vr_code = endian_reader.read_vr_code(group, element, syntax.vr_encoding);

    let test_length = match syntax.vr_encoding {
        VrEncoding::Implicit => endian_reader.read_i32(),
        VrEncoding::Explicit => vr_type::get_explicit_vr(vr_code, endian_reader)
    };

    let value_length = match test_length > 0 {
        true  => usize::try_from(test_length).unwrap(),
        false => 0
    };

    let tag_value = match vr_code {
        vr_type::DELIMITER           => VrValue::Delimiter,
        vr_type::SEQUENCE_OF_ITEMS   => VrValue::SequenceOfItems,
        vr_type::ATTRIBUTE           => VrValue::Attribute(endian_reader.read_u16(), endian_reader.read_u16()),

        vr_type::UNSIGNED_SHORT      => VrValue::UnsignedShort(endian_reader.read_vm_16(value_length, u16::from_ne_bytes)),
        vr_type::SIGNED_SHORT        => VrValue::SignedShort(endian_reader.read_vm_16(value_length, i16::from_ne_bytes)),
        vr_type::UNSIGNED_LONG       => VrValue::UnsignedLong(endian_reader.read_vm_32(value_length, u32::from_ne_bytes)),
        vr_type::SIGNED_LONG         => VrValue::SignedLong(endian_reader.read_vm_32(value_length, i32::from_ne_bytes)),
        vr_type::FLOAT               => VrValue::Float(endian_reader.read_vm_32(value_length, f32::from_ne_bytes)),
        vr_type::DOUBLE              => VrValue::Double(endian_reader.read_vm_64(value_length, f64::from_ne_bytes)),
       
        vr_type::APPLICATION_ENTITY  => VrValue::ApplicationEntity(endian_reader.read_string(value_length)),
        vr_type::AGE_STRING          => VrValue::AgeString(endian_reader.read_string(value_length)),
        vr_type::CODE_STRING         => VrValue::CodeString(endian_reader.read_string(value_length)),
        vr_type::LONG_TEXT           => VrValue::LongText(endian_reader.read_string(value_length)),
        vr_type::PERSON_NAME         => VrValue::PersonName(endian_reader.read_string(value_length)),
        vr_type::SHORT_STRING        => VrValue::ShortString(endian_reader.read_string(value_length)),
        vr_type::SHORT_TEXT          => VrValue::ShortText(endian_reader.read_string(value_length)),
        vr_type::UNLIMITED_TEXT      => VrValue::UnlimitedText(endian_reader.read_string(value_length)),

        vr_type::DATE                => VrValue::Date(endian_reader.read_string(value_length)),
        vr_type::DATE_TIME           => VrValue::DateTime(endian_reader.read_string(value_length)),
        vr_type::TIME                => VrValue::Time(endian_reader.read_string(value_length)),
        vr_type::DECIMAL_STRING      => VrValue::DecimalString(endian_reader.read_string(value_length)),
        vr_type::INTEGER_STRING      => VrValue::IntegerString(endian_reader.read_string(value_length)),
        vr_type::LONG_STRING         => VrValue::LongString(endian_reader.read_string(value_length)),
        vr_type::UID                 => VrValue::Uid(endian_reader.read_string(value_length)),

        vr_type::OTHER_BYTE          => VrValue::OtherByte(endian_reader.read_bytes(value_length)),
        vr_type::OTHER_FLOAT         => VrValue::OtherFloat(endian_reader.read_bytes(value_length)),
        vr_type::OTHER_WORD          => VrValue::OtherWord(endian_reader.read_bytes(value_length)),
        vr_type::UNKNOWN             => VrValue::Unknown(endian_reader.read_bytes(value_length)),
        _                            => VrValue::Unknown(endian_reader.read_bytes(value_length))
    };

    DicomTag {
        group: group,
        element: element,
        syntax: syntax,
        value: tag_value,
        value_length: value_length
    }
}

fn parse_tags(reader: &mut (impl Read + Seek), parent_index: usize, syntax: TransferSyntax, limit_pos: u64, dicom_handler: &mut impl DicomHandler) {

    let tag_syntax = reader.peek_syntax(syntax);

    let tag = next_tag(reader, tag_syntax);
    let value_length = tag.value_length;

    let is_sequence = match tag.value {
        VrValue::SequenceOfItems => true,
        _                        => false
    };
    
    let not_a_sequence_delimiter = match (tag.group, tag.element) {
        tags::SEQUENCE_DELIMITER => false,
        _                        => true
    };

    let child_syntax = match tag.try_transfer_syntax() {
        Some(s) => s,
        None    => syntax
    };

    let child_index = dicom_handler.handle_tag(parent_index, tag);
    let stream_pos = reader.pos();

    if stream_pos < limit_pos && not_a_sequence_delimiter {
        let next_limit = match (is_sequence, value_length > 0) {
            (true, false) => Some(reader.len()),
            (true, true)  => Some(stream_pos + u64::try_from(value_length).unwrap()),
            (_, _)        => None
        };
    
        if let Some(l) = next_limit {            
            parse_tags(reader, child_index, child_syntax, l, dicom_handler);
        };

        parse_tags(reader, parent_index, child_syntax, limit_pos, dicom_handler);
    }
}

pub fn parse(reader: &mut (impl Read + Seek), dicom_handler: &mut impl DicomHandler) {
    let (preamble_length, dicm_mark_length) = (128, 4);
    let _ = reader.seek(SeekFrom::Start(preamble_length)).unwrap();

    let dicm_mark = reader.read_string(dicm_mark_length);

    if dicm_mark != STANDARD_PREAMBLE {
        let _ = reader.seek(SeekFrom::Start(0)).unwrap();
    }

    let limit_pos = reader.len();
    let initial_syntax = TransferSyntax::default();
 
    parse_tags(reader, 0, initial_syntax, limit_pos, dicom_handler);
}