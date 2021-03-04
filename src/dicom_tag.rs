use std::{fmt};

use crate::tags;
use crate::vr_type::VrType;
use crate::transfer_syntax::TransferSyntax;

pub enum VrValue {
    Empty,
    U32(u32),
    I32(i32),
    U16(u16),
    I16(i16),
    F32(f32),
    F64(f64),
    Text(String),
    Multi,
    Id(u16, u16),
    Binary(Vec<u8>)
}

impl fmt::Display for VrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VrValue::Empty           => write!(f, "[ NO VALUE ]"),
            VrValue::U32(v)          => write!(f, "{}", v),
            VrValue::I32(v)          => write!(f, "{}", v),
            VrValue::U16(v)          => write!(f, "{}", v),
            VrValue::I16(v)          => write!(f, "{}", v),
            VrValue::F32(v)          => write!(f, "{}", v),
            VrValue::F64(v)          => write!(f, "{}", v),
            VrValue::Id(group, name) => write!(f, "({}, {})", group, name),
            VrValue::Text(s)         => write!(f, "{}", s),
            VrValue::Multi           => write!(f, "MULTIPLE VALUE"),
            VrValue::Binary(vec)    => write!(f, "{} bytes", vec.len())
        }
    }
}

pub struct DicomTag {
    pub group: u16, 
    pub element: u16,
    pub syntax: TransferSyntax,
    pub vr: VrType,
    pub value: VrValue,
    pub value_length: usize
}

impl DicomTag {
    pub fn try_transfer_syntax(&self) -> Option<TransferSyntax> {
        match ((self.group, self.element), &self.value) {
            (tags::TRANSFER_SYNTAX_UID, VrValue::Text(syntax)) => {
                //  TODO: tags representing child syntax should have their own type.
                //let syntax = str::from_utf8(value).unwrap();
                Some(TransferSyntax::parse_str(&syntax))
            },
            (tags::TRANSFER_SYNTAX_UID, _)           => panic!("Transfer syntax cannot be encoded in a numeric value"),
            (_, _)                                   => None
        }
    }
}