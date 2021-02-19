use crate::vr_type::VrType;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};

pub const UNKNOWN_VALUE: &str = "UNKNOWN";

#[derive(Copy, Clone)]
pub struct TagMarker {
    pub value_length: i32,
    pub stream_position: usize
}

impl TagMarker {
    pub fn new(pos: usize, length: i32) -> TagMarker {
        TagMarker {
            value_length: length,
            stream_position: pos
        }
    }
}

pub enum TagValue {
    String(String),
    U32(u32),
    I32(i32),
    U16(u16),
    I16(i16),
    F32(f32),
    F64(f64)
}

pub struct DicomTag {
    pub id: (u16, u16),
    pub syntax: TransferSyntax,
    pub vr: VrType,
    pub vm: Option<i64>,
    pub marker: TagMarker,
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
            vm: None,
            marker: TagMarker {
                value_length: 0,
                stream_position: 0
            },
            value: TagValue::String(String::from(UNKNOWN_VALUE))
        }        
    }

    pub fn simple(id: (u16, u16), syntax: TransferSyntax, vr: VrType, marker: TagMarker, value: String) -> DicomTag {
        DicomTag {
            id: id,
            syntax: syntax,
            vr: vr,
            vm: None,
            marker: marker,
            value: TagValue::String(value)
        }
    }

    pub fn multiple(id: (u16, u16), syntax: TransferSyntax, vr: VrType, vm: i64, marker: TagMarker, value: TagValue) -> DicomTag {
        DicomTag {
            id: id,
            syntax: syntax,
            vr: vr,
            vm: Some(vm),
            marker: marker,
            value: value
        }
    }    
}

pub struct Node {
    pub tag: DicomTag,
    pub children: Vec<usize>
}