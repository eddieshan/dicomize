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

