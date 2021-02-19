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

fn read_vr_encoding_length(reader: &mut BinaryBufferReader, encoding: VrEncoding) -> TagMarker {
    let length = match encoding {
        VrEncoding::Explicit => i32::from(reader.read_i16()),
        VrEncoding::Implicit => reader.read_i32()
    };
    TagMarker::new(reader.pos(), length)
}

fn skip_reserved(reader: &mut BinaryBufferReader, encoding: VrEncoding) {
    println!("RESERVED TAG");
    match encoding {
        VrEncoding::Explicit => reader.jump(2),
        _                    => {}
    }
}

fn text_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType) -> DicomTag {
    let marker = read_marker(reader);
    custom_text_tag(reader, id, syntax, vr, marker)
}

fn custom_text_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType, marker: TagMarker) -> DicomTag {
    match marker.value_length {
        Some(length) => {
            let value = reader.read_str(length);
            println!("TEXT TAG ({}, {}) | LENGTH: {} | VALUE: {}", id.0, id.1, length, value);
            DicomTag::simple(id, syntax, vr, marker, String::from(value))
        },        
        None => panic!("Tag marker has invalid value length") // TODO: marker value length should not be Option at this point.
    }
}

fn read_marker(reader: &mut BinaryBufferReader) -> TagMarker {
    let length = reader.read_i32();
    let pos = reader.pos();
    TagMarker::new(pos, length)
}

fn ignored_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType) -> DicomTag {
    println!("IGNORED TAG ({}, {})", id.0, id.1);
    let marker = read_marker(reader);
    DicomTag::simple(id, syntax, vr, marker, String::from(""))
}

fn attribute_tag(reader: &mut BinaryBufferReader, syntax: TransferSyntax, vr: VrType) -> DicomTag {
    let marker = read_vr_encoding_length(reader, syntax.vr_encoding);
    let next_group = reader.read_u16();
    let next_element = reader.read_u16();

    println!("ATTRIBUTE TAG ({}, {})", next_group, next_element);

    DicomTag {
        id: (next_group, next_element),
        syntax: syntax,
        vr: vr,
        vm: None,
        marker: marker,
        value: TagValue::String(String::from(""))
    }
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

fn numeric_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType, size: usize, read_value: fn(&mut BinaryBufferReader) -> TagValue) -> DicomTag {
    let marker = read_vr_encoding_length(reader, syntax.vr_encoding);

    match marker.value_length {
        Some(length) => {
            match i64::try_from(length/size) {
                Ok(vm) => {
                    let value = match vm {
                        1 => read_value(reader),
                        _ => {
                            reader.jump(length);
                            TagValue::String(String::from(""))
                        }
                    };

                    println!("NUMBER TAG ({}, {}) | VM: {} | SIZE: {} | LENGTH: {}", id.0, id.1, vm, size, length);
                    DicomTag::multiple(id, syntax, vr, vm, marker, value)        
                },
                Err(_) => panic!("VM has invalid value")  // TODO: propagate Error upwards instead of instant panic here.
            }
        },
        None => panic!("Tag marker has invalid value length") // TODO: marker value length should not be Option at this point.
    }    
}

fn numeric_string_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType) -> DicomTag {
    let marker = read_vr_encoding_length(reader, syntax.vr_encoding);

    match marker.value_length {
        Some(length) => {
            let value = reader.read_str(length);
            match i64::try_from(value.split('\\').count()) {
                Ok(vm) => DicomTag::multiple(id, syntax, vr, vm, marker, TagValue::String(String::from(value))),
                Err(_) => panic!("VM has invalid value")
            }                    
        },                
        None => panic!("Tag marker has invalid value length") // TODO: marker value length should not be Option at this point.
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
    let tag_id = (group, element);

    let vr = parse_vr_type(endian_reader, group, element, syntax.vr_encoding);

    match vr {
        VrType::Delimiter      => ignored_tag(endian_reader, tag_id, syntax, vr),
        VrType::Attribute      => attribute_tag(endian_reader, next_syntax, vr),
        VrType::UnsignedShort  => numeric_tag(endian_reader, tag_id, next_syntax, vr, U16_SIZE, u16_tag),
        VrType::SignedShort    => numeric_tag(endian_reader, tag_id, next_syntax, vr, I16_SIZE, i16_tag),
        VrType::UnsignedLong   => numeric_tag(endian_reader, tag_id, next_syntax, vr, U32_SIZE, u32_tag),
        VrType::SignedLong     => numeric_tag(endian_reader, tag_id, next_syntax, vr, I32_SIZE, i32_tag),
        VrType::Float          => numeric_tag(endian_reader, tag_id, next_syntax, vr, F32_SIZE, f32_tag),
        VrType::Double         => numeric_tag(endian_reader, tag_id, next_syntax, vr, F64_SIZE, f64_tag),
        VrType::OtherByte | VrType::OtherFloat | VrType::OtherWord | VrType::UnlimitedText | VrType::Unknown => {
            skip_reserved(endian_reader, next_syntax.vr_encoding);
            text_tag(endian_reader, tag_id, next_syntax, vr)
        },
        VrType::SequenceOfItems => {
            skip_reserved(endian_reader, next_syntax.vr_encoding);
            ignored_tag(endian_reader, tag_id, syntax, vr)
        },
        VrType::ApplicationEntity | VrType::AgeString | VrType::CodeString | VrType::Date | 
        VrType::DateTime | VrType::LongText | VrType::PersonName | VrType::ShortString | 
        VrType::ShortText | VrType::Time | VrType::Uid => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            custom_text_tag(endian_reader, tag_id, next_syntax, vr, marker)
        },
        VrType::DecimalString | VrType::IntegerString | VrType::LongString => numeric_string_tag(endian_reader, tag_id, next_syntax, vr)
    }
}

fn parse_tags<'a> (reader: &mut BinaryBufferReader, nodes: &mut Vec<Node>, parent_index: usize, syntax: TransferSyntax, limit_pos: usize) {
    let tag = next_tag(reader, syntax);
    let value_length = tag.marker.value_length;
    let tag_id = tag.id;
    let vr = tag.vr;

    let child_syntax = match (tag_id, &tag.value) {
        (TRANSFER_SYNTAX_UID, TagValue::String(s))  => TransferSyntax::parse(&s),
        (TRANSFER_SYNTAX_UID, _)                    => panic!("Transfer syntax cannot be encoded in a numeric value"),
        (_, _)                                      => syntax
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