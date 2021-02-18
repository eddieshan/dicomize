use std::convert::{TryFrom, TryInto};

use crate::utils;
use crate::dicom_tree::*;
use crate::vr_type::*;
use crate::tags::*;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};
use crate::readers::BinaryBufferReader;

const STANDARD_PREAMBLE: &str = "DICM";

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
    match encoding {
        VrEncoding::Explicit => reader.jump(2),
        _                    => {}
    }
}

fn text_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType, marker: TagMarker) -> DicomTag {
    match usize::try_from(marker.value_length) {
        Ok(length) => {
            let value = reader.read_str(length);
            println!("TEXT TAG | LENGTH: {} | VALUE: {}", length, value);
            DicomTag::simple(id, syntax, vr, marker, String::from(value))
        },        
        Err(_) => panic!("Tag marker has invalid value length") // TODO: use proper error propagation instead of panic.
    }
}

fn ignored_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType, length: i32) -> DicomTag {
    let marker = TagMarker::new(reader.pos(), length);
    DicomTag::simple(id, syntax, vr, marker, String::from(""))
}

fn number_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType) -> DicomTag {
    let marker = read_vr_encoding_length(reader, syntax.vr_encoding);

    let (size, value) = match vr {
        VrType::UnsignedLong  => (4, reader.read_u32().to_string()),
        VrType::UnsignedShort => (2, reader.read_u16().to_string()),
        VrType::SignedLong    => (4, reader.read_i32().to_string()),
        VrType::SignedShort   => (2, reader.read_i16().to_string()),
        VrType::Float         => (4, reader.read_f32().to_string()),
        VrType::Double        => (8, reader.read_f64().to_string()),
        _                     => panic!("Tag value is not a supported numeric VR type")
    };    

    let vm = i64::from(marker.value_length)/size;

    println!("NUMBER TAG | VALUE: {} | VM: {} | SIZE: {} | LENGTH: {}", value, vm, size, marker.value_length);

    DicomTag::multiple(id, syntax, vr, vm, marker, value)
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

    println!("TAG | ({}, {})", group, element);

    let vr = parse_vr_type(endian_reader, group, element, syntax.vr_encoding);

    match vr {
        VrType::OtherByte | VrType::OtherFloat | VrType::OtherWord | VrType::Unknown => {
            skip_reserved(endian_reader, next_syntax.vr_encoding);
            let length = endian_reader.read_i32();
            let marker = TagMarker::new(endian_reader.pos(), length);

            text_tag(endian_reader, tag_id, next_syntax, vr, marker)
        },
        VrType::UnlimitedText => {
            skip_reserved(endian_reader, next_syntax.vr_encoding);
            let length = endian_reader.read_i32();
            let marker = TagMarker::new(endian_reader.pos(), length);

            text_tag(endian_reader, tag_id, next_syntax, vr, marker)
        },
        VrType::SequenceOfItems => {
            skip_reserved(endian_reader, next_syntax.vr_encoding);
            let length = endian_reader.read_i32();

            ignored_tag(endian_reader, tag_id, syntax, vr, length)
        },
        VrType::ApplicationEntity | VrType::AgeString | VrType::CodeString | VrType::Date | 
        VrType::DateTime | VrType::LongText | VrType::PersonName | VrType::ShortString | 
        VrType::ShortText | VrType::Time | VrType::Uid => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);

            text_tag(endian_reader, tag_id, next_syntax, vr, marker)
        },
        VrType::DecimalString | VrType::IntegerString | VrType::LongString => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);

            match usize::try_from(marker.value_length) {
                Ok(length) => {
                    let value = endian_reader.read_str(length);
                    match i64::try_from(value.split('\\').count()) {
                        Ok(vm) => {
                            DicomTag::multiple(tag_id, next_syntax, vr, vm, marker, String::from(value))
                        },
                        Err(_) => panic!("Tag marker has invalid value length")
                    }                    
                },
                Err(_) => panic!("Tag marker has invalid value length")
            }
        },
        VrType::Attribute => {
            // 2 bytes value length, 4 bytes value. {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            let next_group = endian_reader.read_u16();
            let next_element = endian_reader.read_u16();

            DicomTag {
                id: (next_group, next_element),
                syntax: syntax,
                vr: vr,
                vm: None,
                marker: marker,
                value: String::from("")
            }
        },
        VrType::UnsignedLong => number_tag(endian_reader, tag_id, next_syntax, vr),
        VrType::UnsignedShort => number_tag(endian_reader, tag_id, next_syntax, vr),
        VrType::SignedLong => number_tag(endian_reader, tag_id, next_syntax, vr),
        VrType::SignedShort => number_tag(endian_reader, tag_id, next_syntax, vr),
        VrType::Float => number_tag(endian_reader, tag_id, next_syntax, vr),
        VrType::Double => number_tag(endian_reader, tag_id, next_syntax, vr),
        VrType::Delimiter => {
            let length = endian_reader.read_i32();
            ignored_tag(endian_reader, tag_id, syntax, vr, length)
        }
    }
}

fn parse_tags<'a> (reader: &mut BinaryBufferReader, nodes: &mut Vec<Node>, parent_index: usize, syntax: TransferSyntax, limit_pos: usize) {
    let tag = next_tag(reader, syntax);
    let value_length = tag.marker.value_length;
    let tag_id = tag.id;
    let vr = tag.vr;

    let child_syntax = match tag_id {
        TRANSFER_SYNTAX_UID => TransferSyntax::parse(&tag.value),
        _                   => syntax
    };    
    
    let child = Node { tag: tag, children: Vec::new() };
    nodes.push(child);

    let child_index = nodes.len() - 1;
    nodes[parent_index].children.push(child_index);     

    if reader.pos() < limit_pos && tag_id != SEQUENCE_DELIMITER {               

        let next_limit = match (vr, value_length) {
            (VrType::SequenceOfItems, -1) => Some(reader.len()),
            (VrType::SequenceOfItems, _)   => {
                match usize::try_from(value_length) {
                    Ok(v) => Some(reader.pos() + v),
                    Err(_) => None
                }
            },
            (_, _)                          => None
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