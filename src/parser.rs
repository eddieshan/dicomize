use std::convert::TryFrom;

use crate::dicom_types::*;
use crate::tags::*;
use crate::transfer_syntax::*;
use crate::vr_types::*;
use crate::readers::BinaryBufferReader;

const STANDARD_PREAMBLE: &str = "DICM";
const UNKNOWN_VALUE: &str = "UNKNOWN";

fn parse_vr_type (reader: &mut BinaryBufferReader, group: u16, element: u16, vr_encoding: VrEncoding) -> VrType {
    let vr_type = tag_vr_type(group, element);
    let is_even_group = even(group);
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

fn parse_syntax(syntax: &String) -> TransferSyntax {
    if syntax.eq_ignore_ascii_case(EXPLICIT_LE) {
        return TransferSyntax { vr_encoding: VrEncoding::Explicit, endian_encoding: EndianEncoding::LittleEndian };
    }
    else if syntax.eq_ignore_ascii_case(EXPLICIT_BE) {
        return TransferSyntax { vr_encoding: VrEncoding::Explicit, endian_encoding: EndianEncoding::BigEndian };
    }
    else if syntax.eq_ignore_ascii_case(IMPLICIT_LE) {
        return TransferSyntax { vr_encoding: VrEncoding::Implicit, endian_encoding: EndianEncoding::LittleEndian };
    }
    else {
        return TransferSyntax { vr_encoding: VrEncoding::Explicit, endian_encoding: EndianEncoding::LittleEndian };
    }
}

fn read_vr_encoding_length(reader: &mut BinaryBufferReader, encoding: VrEncoding) -> TagMarker {
    let length = match encoding {
        VrEncoding::Explicit => i32::from(reader.read_i16()),
        VrEncoding::Implicit => reader.read_i32()
    };

    marker(reader, length)
}

fn marker(reader: &mut BinaryBufferReader, length: i32) -> TagMarker {
    TagMarker {
        value_length: length,
        stream_position: reader.pos()
    }
}

fn skip_reserved(reader: &mut BinaryBufferReader, encoding: VrEncoding) {
    match encoding {
        VrEncoding::Explicit => reader.jump(2),
        _                    => {}
    }        
}

fn basic_tag(id: (u16, u16), syntax: TransferSyntax, vr: VrType, marker: TagMarker, value: String) -> DicomTag {
    DicomTag {
        id: id,
        syntax: syntax,
        vr: vr,
        vm: None,
        marker: marker,
        value: value
    }
}

fn text_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType, marker: TagMarker) -> DicomTag {
    match usize::try_from(marker.value_length) {
        Ok(length) => {
            let value = reader.read_str(length);
            basic_tag(id, syntax, vr, marker, String::from(value))
        },        
        Err(_) => panic!("Tag marker has invalid value length") // TODO: use proper error propagation instead of panic.
    }
}

fn vm_tag(id: (u16, u16), syntax: TransferSyntax, vr: VrType, vm: i64, marker: TagMarker, value: String) -> DicomTag {
    DicomTag {
        id: id,
        syntax: syntax,
        vr: vr,
        vm: Some(vm),
        marker: marker,
        value: value
    }
}

fn ignored_tag(reader: &mut BinaryBufferReader, id: (u16, u16), syntax: TransferSyntax, vr: VrType, length: i32) -> DicomTag {
    let marker = marker(reader, length);
    basic_tag(id, syntax, vr, marker, String::from(""))
}

fn read_number_series(size: i64, length: i32) -> (String, i64) {
    let value = String::from("");

    let vm = (i64::from(length))/size;

    // TODO: calculate value as concatenation of multiple reads.
    (value, vm)
}

fn number_tag(id: (u16, u16), syntax: TransferSyntax, vr: VrType, marker: TagMarker, size: i64) -> DicomTag { 
    let (value, vm) = read_number_series(size, marker.value_length);

    DicomTag {
        id: id,
        syntax: syntax,
        vr: vr,
        vm: Some(vm),
        marker: marker,
        value: value
    }
}

fn next_tag(reader: &mut BinaryBufferReader, syntax: TransferSyntax) -> DicomTag {

    // First pass to get get transfer syntax based on lookup of group number.
    // Then rewind and start reading this time using the specified encoding.
    let group_peek = reader.read_rewind_u16();

    let next_syntax = match group_peek {
        0x0002_u16 => default_syntax(),
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
        VrType::OtherByte | VrType::OtherFloat | VrType::OtherWord | VrType::Unknown => {
            skip_reserved(endian_reader, next_syntax.vr_encoding);
            let length = endian_reader.read_i32();
            let marker = marker(endian_reader, length);

            text_tag(endian_reader, tag_id, next_syntax, vr, marker)

        },
        VrType::UnlimitedText => {
            skip_reserved(endian_reader, next_syntax.vr_encoding);
            let length = endian_reader.read_i32();
            let marker = marker(endian_reader, length);

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
                            vm_tag(tag_id, next_syntax, vr, vm, marker, String::from(value))
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
        VrType::UnsignedLong => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            number_tag(tag_id, next_syntax, vr, marker, 4)
        },
        VrType::UnsignedShort => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            number_tag(tag_id, next_syntax, vr, marker, 2)
        },
        VrType::SignedLong => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            number_tag(tag_id, next_syntax, vr, marker, 4)
        },
        VrType::SignedShort => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            number_tag(tag_id, next_syntax, vr, marker, 2)
        },
        VrType::Float => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            number_tag(tag_id, next_syntax, vr, marker, 4)
        },
        VrType::Double => {
            let marker = read_vr_encoding_length(endian_reader, next_syntax.vr_encoding);
            number_tag(tag_id, next_syntax, vr, marker, 8)
        },
        VrType::Delimiter => {
            let length = endian_reader.read_i32();
            ignored_tag(endian_reader, tag_id, syntax, vr, length)
        }
    };

    DicomTag {
        id: (0_u16, 0_u16),
        syntax: next_syntax,
        vr: VrType::Unknown,
        vm: None,
        marker: TagMarker {
            value_length: 0,
            stream_position: 0
        },
        value: String::from(UNKNOWN_VALUE)
    }
}

