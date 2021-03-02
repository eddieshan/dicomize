use std::{str, string};
use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

use crate::binary_reader::*;
use crate::dicom_reader::DicomReader;
use crate::dicom_handlers::*;
use crate::dicom_tag::*;
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

fn convert_64<T1>(bytes: &Vec<u8>, convert: fn([u8; 8]) -> T1) -> T1 {
    convert([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

fn convert_32<T1>(bytes: &Vec<u8>, convert: fn([u8; 4]) -> T1) -> T1 {
    convert([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn convert_16<T1>(bytes: &Vec<u8>, convert: fn([u8; 2]) -> T1) -> T1 {
    convert([bytes[0], bytes[1]])
}

fn convert_id(bytes: &Vec<u8>) -> VrValue {
    VrValue::Id(u16::from_ne_bytes([bytes[0], bytes[1]]), u16::from_ne_bytes([bytes[2], bytes[3]]))
}

fn next_tag(reader: &mut (impl Read + Seek), syntax: TransferSyntax) -> DicomTag {

    let endian_reader = endian_reader(reader, syntax);

    let group = endian_reader.read_u16();
    let element = endian_reader.read_u16();

    let vr = endian_reader.read_vr(group, element, syntax.vr_encoding);

    let test_length = match syntax.vr_encoding {
        VrEncoding::Implicit => endian_reader.read_i32(),
        VrEncoding::Explicit => match vr {
            VrType::SequenceOfItems | VrType::OtherByte | VrType::OtherFloat | VrType::OtherWord | VrType::UnlimitedText | VrType::Unknown => {
                endian_reader.skip_reserved();
                endian_reader.read_i32()
            },
            VrType::Delimiter => endian_reader.read_i32(),
            _                 => i32::from(endian_reader.read_i16())
        }
    };

    let bytes = match test_length > 0 {
        true  => endian_reader.read_bytes(usize::try_from(test_length).unwrap()),
        false => Vec::new()
    };

    let tag_value = match vr {
        VrType::Delimiter           => VrValue::Empty,
        VrType::SequenceOfItems     => VrValue::Empty,
        VrType::Attribute           => convert_id(&bytes),

        VrType::UnsignedShort       => VrValue::U16(convert_16(&bytes, u16::from_ne_bytes)),
        VrType::SignedShort         => VrValue::I16(convert_16(&bytes, i16::from_ne_bytes)),
        VrType::UnsignedLong        => VrValue::U32(convert_32(&bytes, u32::from_ne_bytes)),
        VrType::SignedLong          => VrValue::I32(convert_32(&bytes, i32::from_ne_bytes)),
        VrType::Float               => VrValue::F32(convert_32(&bytes, f32::from_ne_bytes)),
        VrType::Double              => VrValue::F64(convert_64(&bytes, f64::from_ne_bytes)),

       
        VrType::ApplicationEntity   => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::AgeString           => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::CodeString          => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::LongText            => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::PersonName          => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::ShortString         => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::ShortText           => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::UnlimitedText       => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),

        VrType::Date                => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::DateTime            => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::Time                => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::DecimalString       => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::IntegerString       => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::LongString          => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),
        VrType::Uid                 => VrValue::Text(String::from(str::from_utf8(&bytes).unwrap())),

        VrType::OtherByte           => VrValue::Binary(bytes.len()),
        VrType::OtherFloat          => VrValue::Binary(bytes.len()),
        VrType::OtherWord           => VrValue::Binary(bytes.len()),
        VrType::Unknown             => VrValue::Binary(bytes.len())
    };

    DicomTag {
        group: group,
        element: element,
        syntax: syntax,
        vr: vr,
        value: tag_value,
        value_length: bytes.len()
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

    let dicm_mark = reader.read_str(dicm_mark_length);

    if dicm_mark != STANDARD_PREAMBLE {
        let _ = reader.seek(SeekFrom::Start(0)).unwrap();
    }

    let limit_pos = reader.len();
    let initial_syntax = TransferSyntax::default();
 
    parse_tags(reader, 0, initial_syntax, limit_pos, dicom_handler);
}