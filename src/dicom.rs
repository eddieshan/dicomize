use std::convert::TryFrom;

use crate::utils;
use crate::dicom_tree::*;
use crate::vr_type::*;
use crate::tags::*;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};
use crate::readers::BinaryBufferReader;

const STANDARD_PREAMBLE: &str = "DICM";

type NumericRead = (usize, Numeric, fn(&mut BinaryBufferReader) -> TagValue);

const U16_SIZE: usize = 2;
const I16_SIZE: usize = 2;
const U32_SIZE: usize = 4;
const I32_SIZE: usize = 4;
const F32_SIZE: usize = 4;
const F64_SIZE: usize = 8;

fn u16_tag(reader: &mut BinaryBufferReader) -> TagValue {
    TagValue::U16(reader.read_u16())
}

fn i16_tag(reader: &mut BinaryBufferReader) -> TagValue {
    TagValue::I16(reader.read_i16())
}

fn u32_tag(reader: &mut BinaryBufferReader) -> TagValue {
    TagValue::U32(reader.read_u32())
}

fn i32_tag(reader: &mut BinaryBufferReader) -> TagValue {
    TagValue::I32(reader.read_i32())
}

fn f32_tag(reader: &mut BinaryBufferReader) -> TagValue {
    TagValue::F32(reader.read_f32())
}

fn f64_tag(reader: &mut BinaryBufferReader) -> TagValue {
    TagValue::F64(reader.read_f64())
}

// Numeric parsing requieres a trio of params per type: numeric type, type size and parsing function.
// Bundling them in const tuples helps keep them consistent and avoid errors caused by mixing them up.
// Triplet type is aliased as NumericRead for convenience.
const READ_U16: NumericRead = (U16_SIZE, Numeric::U16, u16_tag);
const READ_I16: NumericRead = (I16_SIZE, Numeric::I16, i16_tag);
const READ_U32: NumericRead = (U32_SIZE, Numeric::U32, u32_tag);
const READ_I32: NumericRead = (I32_SIZE, Numeric::I32, i32_tag);
const READ_F32: NumericRead = (F32_SIZE, Numeric::F32, f32_tag);
const READ_F64: NumericRead = (F64_SIZE, Numeric::F64, f64_tag);

fn parse_vr_type (reader: &mut BinaryBufferReader, group: u16, element: u16, vr_encoding: VrEncoding) -> VrType {
    let vr_type = tag_vr_type(group, element);
    let is_even_group = utils::even(group);
    let is_private_code = element <= 0xFFu16;

    match (vr_type, vr_encoding, is_even_group, is_private_code) {
        (VrType::Delimiter, _, _, _)    => vr_type,
        (_, VrEncoding::Explicit, _, _) => {
            let code = reader.read_2();
            get_vr_type(&code)
        },
        (_, _, true, _)                 => vr_type,
        (_, _, false, true)             => VrType::LongString,
        (_, _, false, false)            => VrType::Unknown
    }
}

fn skip_reserved(reader: &mut BinaryBufferReader) {
    println!("RESERVED TAG");
    reader.jump(2);
}

fn ignored_tag() -> TagValue {
    TagValue::Ignored
}

fn text_tag(reader: &mut BinaryBufferReader, value_length: usize) -> TagValue {
    let value = reader.read_str(value_length);
    println!("TEXT TAG | LENGTH: {} | VALUE: {}", value_length, value);
    TagValue::String(String::from(value))
}

fn attribute_tag(reader: &mut BinaryBufferReader) -> TagValue {
    let group = reader.read_u16();
    let element = reader.read_u16();

    println!("ATTRIBUTE TAG ({}, {})", group, element);

    TagValue::Attribute(group, element)
}

fn numeric_tag(reader: &mut BinaryBufferReader, value_length: usize, numeric_read: NumericRead) -> TagValue {
    let (size, number_type, read_value) = numeric_read;
    let vm = value_length/size;

    let value = match vm {
        1 => read_value(reader),
        _ => {
            let buf = reader.read_bytes(value_length);
            TagValue::MultiNumeric(number_type, buf)
        }
    };

    println!("NUMBER TAG | VM: {} | SIZE: {} | LENGTH: {}", vm, size, value_length);
    value
}

fn numeric_string_tag(reader: &mut BinaryBufferReader, value_length: usize) -> TagValue {
    let value = reader.read_str(value_length);
    let vm = value.split('\\').count();
    match vm {
        1 => TagValue::String(String::from(value)),
        _ => TagValue::MultiString(String::from(value))
    }    
}

fn next_tag(reader: &mut BinaryBufferReader, syntax: TransferSyntax) -> DicomTag {

    // First pass to get get transfer syntax based on lookup of group number.
    // Then rewind and start reading this time using the specified encoding.
    let group_peek = reader.read_rewind_u16();

    let next_syntax = match group_peek {
        0x0002_u16 => TransferSyntax::default(),
        _          => syntax
    };

    let endian_reader = match next_syntax.endian_encoding {
        EndianEncoding::LittleEndian => reader,
        EndianEncoding::BigEndian    => reader
    };

    let group = endian_reader.read_u16();
    let element = endian_reader.read_u16();

    let vr = parse_vr_type(endian_reader, group, element, next_syntax.vr_encoding);

    let test_length = match syntax.vr_encoding {
        VrEncoding::Implicit => endian_reader.read_i32(),
        VrEncoding::Explicit => match vr {
            VrType::SequenceOfItems | VrType::OtherByte | VrType::OtherFloat | VrType::OtherWord | VrType::UnlimitedText | VrType::Unknown => {
                skip_reserved(endian_reader);
                endian_reader.read_i32()
            },
            VrType::Delimiter => endian_reader.read_i32(),
            _                 => i32::from(endian_reader.read_i16())
        }
    };

    let stream_pos = endian_reader.pos();

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
            let length = usize::try_from(test_length).unwrap();
            let value = match vr {
                VrType::Delimiter | 
                VrType::SequenceOfItems => ignored_tag(),
        
                VrType::Attribute       => attribute_tag(endian_reader),
        
                VrType::UnsignedShort  => numeric_tag(endian_reader, length, READ_U16),
                VrType::SignedShort    => numeric_tag(endian_reader, length, READ_I16),
                VrType::UnsignedLong   => numeric_tag(endian_reader, length, READ_U32),
                VrType::SignedLong     => numeric_tag(endian_reader, length, READ_I32),
                VrType::Float          => numeric_tag(endian_reader, length, READ_F32),
                VrType::Double         => numeric_tag(endian_reader, length, READ_F64),
        
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
        syntax: next_syntax,
        vr: vr,
        stream_position: stream_pos,
        value_length: value_length,
        value: tag_value
    }    
}

fn parse_tags<'a> (reader: &mut BinaryBufferReader, nodes: &mut Vec<Node>, parent_index: usize, syntax: TransferSyntax, limit_pos: usize) {
    let tag = next_tag(reader, syntax);
    let value_length = tag.value_length;
    let tag_id = tag.id;
    let vr = tag.vr;

    let child_syntax = match (tag_id, &tag.value) {
        (TRANSFER_SYNTAX_UID, TagValue::String(s)) => TransferSyntax::parse(&s), //  TODO: tags representing child syntax should have their own type.
        (TRANSFER_SYNTAX_UID, _)                   => panic!("Transfer syntax cannot be encoded in a numeric value"),
        (_, _)                                     => syntax
    };

    let child = Node { tag: tag, children: Vec::new() };
    nodes.push(child);

    let child_index = nodes.len() - 1;
    nodes[parent_index].children.push(child_index);     

    if reader.pos() < limit_pos && tag_id != SEQUENCE_DELIMITER {
        let next_limit = match (vr, value_length) {
            (VrType::SequenceOfItems, None)    => Some(reader.len()),
            (VrType::SequenceOfItems, Some(l)) => Some(reader.pos() + l),
            (_, _)                             => None
        };

        if let Some(l) = next_limit {            
            parse_tags(reader, nodes, child_index, child_syntax, l);
        };

        parse_tags(reader, nodes, parent_index, child_syntax, limit_pos);        
    }
}

pub fn parse(buffer: Vec<u8>) -> Vec<Node> {
    // Dicom file header,
    // - Fixed preamble not to be used: 128 bytes.
    // - DICOM Prefix "DICM": 4 bytes.
    // - File Meta Information: sequence of FileMetaAttribute.
    //   FileMetaAttribute structure: (0002,xxxx), encoded with ExplicitVRLittleEndian Transfer Syntax.
    let (preamble_length, dicm_mark_length) = (128, 4);
    let reader = &mut BinaryBufferReader::new(buffer);
    reader.seek(preamble_length);

    let dicm_mark = reader.read_str(dicm_mark_length);

    if dicm_mark != STANDARD_PREAMBLE {
        reader.seek(0);
    }

    let root = Node { tag: DicomTag::empty(), children: Vec::new() };
    let mut nodes = vec![root];
    parse_tags(reader, &mut nodes, 0, TransferSyntax::default(), reader.len());
    
    nodes
}