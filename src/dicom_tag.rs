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
    MultiNumeric(Numeric, Vec<u8>), // TODO: pending revision of non-typed buffer implementation.
    MultiString(String)
}

impl fmt::Display for TagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagValue::Ignored                => write!(f, "IGNORED"),
            TagValue::Attribute(group, name) => write!(f, "ATTRIBUTE ({}, {})", group, name),
            TagValue::String(s)              => write!(f, "STRING {}", s),
            TagValue::MultiNumeric(_, buf)   => write!(f, "MULTIPLE NUMERIC {}", buf.len()),
            TagValue::MultiString(s)         => write!(f, "MULTIPLE STRING {}", s),  
        }
    }
}

pub struct DicomTag {
    pub id: (u16, u16),
    pub syntax: TransferSyntax,
    pub vr: VrType,
    pub stream_position: u64,
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