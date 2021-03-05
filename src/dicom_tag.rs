use std::{fmt};

use crate::tags;
use crate::transfer_syntax::TransferSyntax;

pub enum VrValue {
    Delimiter,
    SequenceOfItems,
    Attribute(u16, u16),

    UnsignedShort(u16),
    SignedShort(i16),
    UnsignedLong(u32),
    SignedLong(i32),
    Float(f32),
    Double(f64),
   
    ApplicationEntity(String),
    AgeString(String),
    CodeString(String),
    LongText(String),
    PersonName(String),
    ShortString(String),
    ShortText(String),
    UnlimitedText(String),

    Date(String),
    DateTime(String),
    Time(String),
    DecimalString(String),
    IntegerString(String),
    LongString(String),
    Uid(String),

    OtherByte(Vec<u8>),
    OtherFloat(Vec<u8>),
    OtherWord(Vec<u8>),
    Unknown(Vec<u8>)
}

impl fmt::Display for VrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VrValue::Delimiter               => write!(f, "Delimiter"),
            VrValue::SequenceOfItems         => write!(f, "SequenceOfItems"),
            VrValue::Attribute(group, value) => write!(f, "Attribute: ({}, {})", group, value),

            VrValue::UnsignedShort(v)        => write!(f, "UnsignedShort: {}", v),
            VrValue::SignedShort(v)          => write!(f, "SignedShort: {}", v),
            VrValue::UnsignedLong(v)         => write!(f, "UnsignedLong: {}", v),
            VrValue::SignedLong(v)           => write!(f, "SignedLong: {}", v),            
            VrValue::Float(v)                => write!(f, "Float: {}", v),
            VrValue::Double(v)               => write!(f, "Double: {}", v),
           
            VrValue::ApplicationEntity(s)    => write!(f, "ApplicationEntity: {}", s),
            VrValue::AgeString(s)            => write!(f, "AgeString: {}", s),
            VrValue::CodeString(s)           => write!(f, "CodeString: {}", s),
            VrValue::LongText(s)             => write!(f, "LongText: {}", s),
            VrValue::PersonName(s)           => write!(f, "PersonName: {}", s),
            VrValue::ShortString(s)          => write!(f, "ShortString: {}", s),
            VrValue::ShortText(s)            => write!(f, "ShortText: {}", s),
            VrValue::UnlimitedText(s)        => write!(f, "UnlimitedText: {}", s),
 
            VrValue::Date(s)                 => write!(f, "Date: {}", s),
            VrValue::DateTime(s)             => write!(f, "DateTime: {}", s),
            VrValue::Time(s)                 => write!(f, "Time: {}", s),
            VrValue::DecimalString(s)        => write!(f, "DecimalString: {}", s),
            VrValue::IntegerString(s)        => write!(f, "IntegerString: {}", s),
            VrValue::LongString(s)           => write!(f, "LongString: {}", s),
            VrValue::Uid(s)                  => write!(f, "Uid: {}", s),
 
            VrValue::OtherByte(bytes)        => write!(f, "OtherByte: {} bytes", bytes.len()),
            VrValue::OtherFloat(bytes)       => write!(f, "OtherFloat: {} bytes", bytes.len()),
            VrValue::OtherWord(bytes)        => write!(f, "OtherWord: {} bytes", bytes.len()),
            VrValue::Unknown(bytes)          => write!(f, "Unknown: {} bytes", bytes.len())
        }
    }
}

pub struct DicomTag {
    pub group: u16,
    pub element: u16,
    pub syntax: TransferSyntax,
    pub value: VrValue,
    pub value_length: usize
}

impl DicomTag {
    pub fn try_transfer_syntax(&self) -> Option<TransferSyntax> {
        match ((self.group, self.element), &self.value) {
            (tags::TRANSFER_SYNTAX_UID, VrValue::Uid(syntax)) => {
                //  TODO: tags representing child syntax should have their own type.
                //let syntax = str::from_utf8(value).unwrap();
                Some(TransferSyntax::parse_str(&syntax))
            },
            (tags::TRANSFER_SYNTAX_UID, _)           => panic!("Transfer syntax cannot be encoded in a numeric value"),
            (_, _)                                   => None
        }
    }
}