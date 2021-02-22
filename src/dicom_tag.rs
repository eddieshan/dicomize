use std::fmt;

use crate::vr_type::VrType;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};

pub const UNKNOWN_VALUE: &str = "UNKNOWN";

pub enum Numeric {
    U32,
    I32,
    U16,
    I16,
    F32,
    F64
}

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
    MultiNumeric(Numeric, Vec<u8>), // TODO: pending revision of non-typed buffer implementation.
    MultiString(String)
}

impl fmt::Display for TagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagValue::Ignored                => write!(f, "IGNORED"),
            TagValue::Attribute(group, name) => write!(f, "ATTRIBUTE ({}, {})", group, name),
            TagValue::String(s)              => write!(f, "STRING {}", s),
            TagValue::U32(v)                 => write!(f, "U32 {}", v),
            TagValue::I32(v)                 => write!(f, "I32 {}", v),
            TagValue::U16(v)                 => write!(f, "U16 {}", v),
            TagValue::I16(v)                 => write!(f, "I16 {}", v),
            TagValue::F32(v)                 => write!(f, "F32 {}", v),
            TagValue::F64(v)                 => write!(f, "F64 {}", v),
            TagValue::MultiNumeric(_, buf)   => write!(f, "MULTIPLE NUMERIC {}", buf.len()),
            TagValue::MultiString(s)         => write!(f, "MULTIPLE STRING {}", s),  
        }
    }
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