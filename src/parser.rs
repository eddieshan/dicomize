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