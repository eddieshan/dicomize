use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

use crate::utils;
use crate::readers;
use crate::abstractions::*;
use crate::dicom_tag::*;
use crate::vr_type::*;
use crate::tags::*;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};

const STANDARD_PREAMBLE: &str = "DICM";

fn read_vr(reader: &mut impl Read, group: u16, element: u16, vr_encoding: VrEncoding) -> VrType {
    let vr_type = tag_vr_type(group, element);
    let is_even_group = utils::even(group);
    let is_private_code = element <= 0xFFu16;

    match (vr_type, vr_encoding, is_even_group, is_private_code) {
        (VrType::Delimiter, _, _, _)    => vr_type,
        (_, VrEncoding::Explicit, _, _) => {
            let code = readers::read_2(reader);
            get_vr_type(&code)
        },
        (_, _, true, _)                 => vr_type,
        (_, _, false, true)             => VrType::LongString,
        (_, _, false, false)            => VrType::Unknown
    }
}

fn skip_reserved(reader: &mut impl Seek) {
    let _ = reader.seek(SeekFrom::Current(2)).unwrap(); // TODO: proper error propagation instead of unwrap.
}

fn ignored_tag() -> TagValue {
    TagValue::Ignored
}

fn text_tag(reader: &mut impl Read, value_length: usize) -> TagValue {
    let value = readers::read_str(reader, value_length);
    TagValue::String(value)
}

fn attribute_tag(reader: &mut impl Read) -> TagValue {
    let group = readers::read_u16(reader);
    let element = readers::read_u16(reader);
    TagValue::Attribute(group, element)
}

fn numeric_tag(reader: &mut impl Read, value_length: usize, number_type: Numeric) -> TagValue {
    let buf = readers::read_bytes(reader, value_length);
    TagValue::Numeric(number_type, buf)
}

fn numeric_string_tag(reader: &mut impl Read, value_length: usize) -> TagValue {
    let value = readers::read_str(reader, value_length);
    let vm = value.split('\\').count();
    match vm {
        1 => TagValue::String(value),
        _ => TagValue::MultiString(value)
    }    
}

fn peek_syntax(reader: &mut (impl Read + Seek), syntax: TransferSyntax) -> TransferSyntax {
    // First pass to get get transfer syntax based on lookup of group number.
    // Then rewind and start reading this time using the specified encoding.
    let group_peek = readers::read_rewind_u16(reader);

    match group_peek {
        0x0002_u16 => TransferSyntax::default(),
        _          => syntax
    }
}

fn next_tag(reader: &mut (impl Read + Seek), syntax: TransferSyntax) -> DicomTag {

    let endian_reader = match syntax.endian_encoding {
        EndianEncoding::LittleEndian => reader,
        EndianEncoding::BigEndian    => reader
    };

    let group = readers::read_u16(endian_reader);
    let element = readers::read_u16(endian_reader);

    let vr = read_vr(endian_reader, group, element, syntax.vr_encoding);

    let test_length = match syntax.vr_encoding {
        VrEncoding::Implicit => readers::read_i32(endian_reader),
        VrEncoding::Explicit => match vr {
            VrType::SequenceOfItems | VrType::OtherByte | VrType::OtherFloat | VrType::OtherWord | VrType::UnlimitedText | VrType::Unknown => {
                skip_reserved(endian_reader);
                readers::read_i32(endian_reader)
            },
            VrType::Delimiter => readers::read_i32(endian_reader),
            _                 => i32::from(readers::read_i16(endian_reader))
        }
    };

    let stream_pos = readers::stream_pos(endian_reader);

    let (tag_value, value_length) = match test_length < 0 {
        true => {
            let value = match vr {
                VrType::Delimiter | 
                VrType::SequenceOfItems => ignored_tag(),
                VrType::Attribute       => attribute_tag(endian_reader),
                _                       => panic!("Tag ({}, {}) has invalid value length {}", group, element, test_length)
            };
            (value, None)
        },
        false => {
            let length = usize::try_from(test_length).unwrap(); // TODO: proper error propagation instead of unwrap.
            let value = match vr {
                VrType::Delimiter | 
                VrType::SequenceOfItems => ignored_tag(),
        
                VrType::Attribute       => attribute_tag(endian_reader),               
        
                VrType::UnsignedShort  => numeric_tag(endian_reader, length, Numeric::U16),
                VrType::SignedShort    => numeric_tag(endian_reader, length, Numeric::I16),
                VrType::UnsignedLong   => numeric_tag(endian_reader, length, Numeric::U32),
                VrType::SignedLong     => numeric_tag(endian_reader, length, Numeric::I32),
                VrType::Float          => numeric_tag(endian_reader, length, Numeric::F32),
                VrType::Double         => numeric_tag(endian_reader, length, Numeric::F64),
        
                VrType::OtherByte | 
                VrType::OtherFloat | 
                VrType::OtherWord | 
                VrType::UnlimitedText | 
                VrType::Unknown | 
                VrType::ApplicationEntity | 
                VrType::AgeString | 
                VrType::CodeString | 
                VrType::Date | 
                VrType::DateTime | 
                VrType::LongText | 
                VrType::PersonName | 
                VrType::ShortString | 
                VrType::ShortText | 
                VrType::Time | 
                VrType::Uid            => text_tag(endian_reader, length),
        
                VrType::DecimalString | 
                VrType::IntegerString | 
                VrType::LongString     => numeric_string_tag(endian_reader, length)
            };
            (value, Some(length))    
        }
    };

    DicomTag {
        id: (group, element),
        syntax: syntax,
        vr: vr,
        stream_position: stream_pos,
        value_length: value_length,
        value: tag_value
    }    
}

fn parse_tags(reader: &mut (impl Read + Seek), parent_index: usize, syntax: TransferSyntax, limit_pos: u64, dicom_handler: &mut impl DicomHandler) {

    let tag_syntax = peek_syntax(reader, syntax);

    let tag = next_tag(reader, tag_syntax);
    let value_length = tag.value_length;
    let tag_id = tag.id;
    let vr = tag.vr;

    let child_syntax = match (tag_id, &tag.value) {
        (TRANSFER_SYNTAX_UID, TagValue::String(s)) => TransferSyntax::parse(&s), //  TODO: tags representing child syntax should have their own type.
        (TRANSFER_SYNTAX_UID, _)                   => panic!("Transfer syntax cannot be encoded in a numeric value"),
        (_, _)                                     => syntax
    };

    let child_index = dicom_handler.handle_tag(parent_index, tag);
    let stream_pos = readers::stream_pos(reader);

    if stream_pos < limit_pos && tag_id != SEQUENCE_DELIMITER {
        let next_limit = match (vr, value_length) {
            (VrType::SequenceOfItems, None)    => Some(readers::stream_len(reader)),
            (VrType::SequenceOfItems, Some(l)) => Some(stream_pos + u64::try_from(l).unwrap()),
            (_, _)                             => None
        };

        if let Some(l) = next_limit {            
            parse_tags(reader, child_index, child_syntax, l, dicom_handler);
        };

        parse_tags(reader, parent_index, child_syntax, limit_pos, dicom_handler);
    }
}

pub fn parse(reader: &mut (impl Read + Seek), dicom_handler: &mut impl DicomHandler) {
    // Dicom file header,
    // - Fixed preamble not to be used: 128 bytes.
    // - DICOM Prefix "DICM": 4 bytes.
    // - File Meta Information: sequence of FileMetaAttribute.
    //   FileMetaAttribute structure: (0002,xxxx), encoded with ExplicitVRLittleEndian Transfer Syntax.
    let (preamble_length, dicm_mark_length) = (128, 4);
    let _ = reader.seek(SeekFrom::Start(preamble_length)).unwrap();

    let dicm_mark = readers::read_str(reader, dicm_mark_length);

    if dicm_mark != STANDARD_PREAMBLE {
        let _ = reader.seek(SeekFrom::Start(0)).unwrap();
    }

    let limit_pos = readers::stream_len(reader);

    parse_tags(reader, 0, TransferSyntax::default(), limit_pos, dicom_handler);
}