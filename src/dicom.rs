use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

use crate::readers::*;
use crate::dicom_reader::DicomReader;
use crate::dicom_handlers::*;
use crate::dicom_tag::*;
use crate::vr_type::*;
use crate::tags::*;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};

const STANDARD_PREAMBLE: &str = "DICM";

fn next_tag(reader: &mut (impl Read + Seek), syntax: TransferSyntax) -> DicomTag {

    let endian_reader = match syntax.endian_encoding {
        EndianEncoding::LittleEndian => reader,
        EndianEncoding::BigEndian    => reader
    };

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

    let (tag_value, value_length) = match test_length < 0 {
        true => {
            let value = match vr {
                VrType::Delimiter | 
                VrType::SequenceOfItems => endian_reader.read_ignored(),
                VrType::Attribute       => endian_reader.read_attribute(),
                _                       => panic!("Tag ({}, {}) has invalid value length {}", group, element, test_length)
            };
            (value, None)
        },
        false => {
            let length = usize::try_from(test_length).unwrap(); // TODO: proper error propagation instead of unwrap.
            let value = match vr {
                VrType::Delimiter | 
                VrType::SequenceOfItems => endian_reader.read_ignored(),
        
                VrType::Attribute       => endian_reader.read_attribute(),
        
                VrType::UnsignedShort  => endian_reader.read_numeric(length, Numeric::U16),
                VrType::SignedShort    => endian_reader.read_numeric(length, Numeric::I16),
                VrType::UnsignedLong   => endian_reader.read_numeric(length, Numeric::U32),
                VrType::SignedLong     => endian_reader.read_numeric(length, Numeric::I32),
                VrType::Float          => endian_reader.read_numeric(length, Numeric::F32),
                VrType::Double         => endian_reader.read_numeric(length, Numeric::F64),
        
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
                VrType::Uid            => endian_reader.read_text(length),
        
                VrType::DecimalString | 
                VrType::IntegerString | 
                VrType::LongString     => endian_reader.read_numeric_string(length)
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

    let tag_syntax = reader.peek_syntax(syntax);

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
    // Dicom file header,
    // - Fixed preamble not to be used: 128 bytes.
    // - DICOM Prefix "DICM": 4 bytes.
    // - File Meta Information: sequence of FileMetaAttribute.
    //   FileMetaAttribute structure: (0002,xxxx), encoded with ExplicitVRLittleEndian Transfer Syntax.
    let (preamble_length, dicm_mark_length) = (128, 4);
    let _ = reader.seek(SeekFrom::Start(preamble_length)).unwrap();

    let dicm_mark = reader.read_str(dicm_mark_length);

    if dicm_mark != STANDARD_PREAMBLE {
        let _ = reader.seek(SeekFrom::Start(0)).unwrap();
    }

    let limit_pos = reader.len();

    parse_tags(reader, 0, TransferSyntax::default(), limit_pos, dicom_handler);
}