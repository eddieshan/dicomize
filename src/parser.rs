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

