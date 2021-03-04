use std::{str, string};
use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

use crate::binary_reader::*;
use crate::dicom_reader::DicomReader;
use crate::dicom_handlers::*;
use crate::dicom_tag::*;
use crate::vr_type;
use crate::vr_type::*;
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

    let (vr, length_size) = vr_type::get_vr_type(vr_code);

    let test_length = match (syntax.vr_encoding, length_size) {
        (VrEncoding::Implicit, _)                            => endian_reader.read_i32(),
        (VrEncoding::Explicit, ValueLengthSize::ReservedI32) => endian_reader.read_reserved_i32(),
        (VrEncoding::Explicit, ValueLengthSize::I32)         => endian_reader.read_i32(),
        (VrEncoding::Explicit, ValueLengthSize::I16)         => i32::from(endian_reader.read_i16())
    };

    let value_length = match test_length > 0 {
        true  => usize::try_from(test_length).unwrap(),
        false => 0
    };

    let tag_value = match vr {
        VrType::Delimiter           => VrValue::Empty,
        VrType::SequenceOfItems     => VrValue::Empty,
        VrType::Attribute           => VrValue::Id(endian_reader.read_u16(), endian_reader.read_u16()),

        VrType::UnsignedShort       => VrValue::U16(endian_reader.read_vm_16(value_length, u16::from_ne_bytes)),
        VrType::SignedShort         => VrValue::I16(endian_reader.read_vm_16(value_length, i16::from_ne_bytes)),
        VrType::UnsignedLong        => VrValue::U32(endian_reader.read_vm_32(value_length, u32::from_ne_bytes)),
        VrType::SignedLong          => VrValue::I32(endian_reader.read_vm_32(value_length, i32::from_ne_bytes)),
        VrType::Float               => VrValue::F32(endian_reader.read_vm_32(value_length, f32::from_ne_bytes)),
        VrType::Double              => VrValue::F64(endian_reader.read_vm_64(value_length, f64::from_ne_bytes)),
       
        VrType::ApplicationEntity   => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::AgeString           => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::CodeString          => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::LongText            => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::PersonName          => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::ShortString         => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::ShortText           => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::UnlimitedText       => VrValue::Text(endian_reader.read_string(value_length)),

        VrType::Date                => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::DateTime            => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::Time                => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::DecimalString       => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::IntegerString       => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::LongString          => VrValue::Text(endian_reader.read_string(value_length)),
        VrType::Uid                 => VrValue::Text(endian_reader.read_string(value_length)),

        VrType::OtherByte           => VrValue::Binary(endian_reader.read_bytes(value_length)),
        VrType::OtherFloat          => VrValue::Binary(endian_reader.read_bytes(value_length)),
        VrType::OtherWord           => VrValue::Binary(endian_reader.read_bytes(value_length)),
        VrType::Unknown             => VrValue::Binary(endian_reader.read_bytes(value_length))
    };

    DicomTag {
        group: group,
        element: element,
        syntax: syntax,
        vr: vr,
        value: tag_value,
        value_length: value_length
    }
}

fn parse_tags(reader: &mut (impl Read + Seek), parent_index: usize, syntax: TransferSyntax, limit_pos: u64, dicom_handler: &mut impl DicomHandler) {

    let tag_syntax = reader.peek_syntax(syntax);

    let tag = next_tag(reader, tag_syntax);
    let value_length = tag.value_length;

    let vr = tag.vr;
    
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
        let next_limit = match (vr, value_length > 0) {
            (VrType::SequenceOfItems, false)   => Some(reader.len()),
            (VrType::SequenceOfItems, true)    => Some(stream_pos + u64::try_from(value_length).unwrap()),
            (_, _)                             => None
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