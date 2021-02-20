use std::convert::TryFrom;

use crate::vr_type::VrType;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};

pub const UNKNOWN_VALUE: &str = "UNKNOWN";

pub enum TagValue {    
    Ignored,
    Attribute(u16, u16),
    String(String),
    U32(u32),
    I32(i32),
    U16(u16),
    I16(i16),
    F32(f32),
    F64(f64),
    Multiple(usize, String)
}

pub struct DicomTag {
    pub id: (u16, u16),
    pub syntax: TransferSyntax,
    pub vr: VrType,
    pub stream_position: usize,
    pub value_length: Option<usize>,
    pub value: TagValue
}

impl DicomTag {
    pub fn empty() -> DicomTag {
        DicomTag {
            id: (0_u16, 0_u16),
            syntax: TransferSyntax { 
                vr_encoding: VrEncoding::Explicit, 
                endian_encoding: EndianEncoding::LittleEndian
            },
            vr: VrType::Unknown,
            stream_position: 0,
            value_length: None,
            value: TagValue::String(String::from(UNKNOWN_VALUE))
        }
    }    
}

pub struct Node {
    pub tag: DicomTag,
    pub children: Vec<usize>
}