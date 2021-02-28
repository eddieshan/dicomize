use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

use crate::binary_reader::*;
use crate::dicom_reader::DicomReader;
use crate::dicom_handlers::*;
use crate::dicom_tag::*;
use crate::vr_type::*;
use crate::tags::*;
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

    let stream_pos = endian_reader.pos();

    let tag_value = match vr {
        VrType::Delimiter | VrType::SequenceOfItems => None,
        VrType::Attribute                           => Some(endian_reader.read_bytes(4)),
        _                                           => match test_length > 0 {
            true => {
                let length = usize::try_from(test_length).unwrap();
                Some(endian_reader.read_bytes(length))
            },
            false => None
        }
    };

    DicomTag {
        id: (group, element),
        syntax: syntax,
        vr: vr,
        stream_position: stream_pos,
        value: tag_value
    }    
}

fn parse_tags(reader: &mut (impl Read + Seek), parent_index: usize, syntax: TransferSyntax, limit_pos: u64, dicom_handler: &mut impl DicomHandler) {

    let tag_syntax = reader.peek_syntax(syntax);

    let tag = next_tag(reader, tag_syntax);
    let value_length = tag.try_value_len();

    let tag_id = tag.id;
    let vr = tag.vr;

    let child_syntax = match tag.try_transfer_syntax() {
        Some(s) => s,
        None    => syntax
    };

    let child_index = dicom_handler.handle_tag(parent_index, tag);
    let stream_pos = reader.pos();

    if stream_pos < limit_pos && tag_id != SEQUENCE_DELIMITER {
        let next_limit = match (vr, value_length) {
            (VrType::SequenceOfItems, None)    => Some(reader.len()),
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