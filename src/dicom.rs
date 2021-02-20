use std::convert::TryFrom;

use crate::utils;
use crate::dicom_tree::*;
use crate::vr_type::*;
use crate::tags::*;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};
use crate::readers::BinaryBufferReader;

const STANDARD_PREAMBLE: &str = "DICM";

const U16_SIZE: usize = 2;
const I16_SIZE: usize = 2;
const U32_SIZE: usize = 4;
const I32_SIZE: usize = 4;
const F32_SIZE: usize = 4;
const F64_SIZE: usize = 8;

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
    //println!("IGNORED TAG ({}, {})", id.0, id.1);
    TagValue::Ignored
}

fn text_tag(reader: &mut BinaryBufferReader, value_length: usize) -> TagValue {
    let value = reader.read_str(value_length);
    println!("TEXT TAG | LENGTH: {} | VALUE: {}", value_length, value);
    TagValue::String(String::from(value))
}

fn attribute_tag() -> TagValue {
    TagValue::String(String::from(""))
}

fn attribute_id(reader: &mut BinaryBufferReader) -> (u16, u16) {
    let next_group = reader.read_u16();
    let next_element = reader.read_u16();

    println!("ATTRIBUTE TAG ({}, {})", next_group, next_element);

    (next_group, next_element)
}


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

fn numeric_tag(reader: &mut BinaryBufferReader, value_length: usize, size: usize, read_value: fn(&mut BinaryBufferReader) -> TagValue) -> TagValue {
    let vm = value_length/size;

    let value = match vm {
        1 => read_value(reader),
        _ => {
            reader.jump(value_length);
            TagValue::Multiple(vm, String::from(""))
        }
    };

    println!("NUMBER TAG | VM: {} | SIZE: {} | LENGTH: {}", vm, size, value_length);
    value
}

fn numeric_string_tag(reader: &mut BinaryBufferReader, value_length: usize) -> TagValue {
    let value = reader.read_str(value_length);
    let vm = value.split('\\').count();
    TagValue::Multiple(vm, String::from(value))
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

    let tag_id = match vr {
        VrType::Attribute => attribute_id(endian_reader),
        _                 => (group, element)
    };

    let (tag_value, value_length) = match test_length < 0 {
        true => {
            let value = match vr {
                VrType::Delimiter | 
                VrType::SequenceOfItems => ignored_tag(),    
                VrType::Attribute       => attribute_tag(),
                _                       => panic!("Tag ({}, {}) has invalid value length")
            };
            (value, None)
        },
        false => {
            let length = usize::try_from(test_length).unwrap();
            let value = match vr {
                VrType::Delimiter | 
                VrType::SequenceOfItems => ignored_tag(),
        
                VrType::Attribute      => attribute_tag(),
        
                VrType::UnsignedShort  => numeric_tag(endian_reader, length, U16_SIZE, u16_tag),
                VrType::SignedShort    => numeric_tag(endian_reader, length, I16_SIZE, i16_tag),
                VrType::UnsignedLong   => numeric_tag(endian_reader, length, U32_SIZE, u32_tag),
                VrType::SignedLong     => numeric_tag(endian_reader, length, I32_SIZE, i32_tag),
                VrType::Float          => numeric_tag(endian_reader, length, F32_SIZE, f32_tag),
                VrType::Double         => numeric_tag(endian_reader, length, F64_SIZE, f64_tag),
        
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
        id: tag_id,
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