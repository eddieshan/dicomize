use std::{fmt, str};

use crate::tags;
use crate::vr_type::VrType;
use crate::transfer_syntax::{VrEncoding, EndianEncoding, TransferSyntax};

pub const UNKNOWN_VALUE: &str = "UNKNOWN";

pub enum VrValue {
    U32(u32),
    I32(i32),
    U16(u16),
    I16(i16),
    F32(f32),
    F64(f64),
    Text(String),
    Multi,
    Attribute(u16, u16),
    PixelData
}

impl fmt::Display for VrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VrValue::U32(v)                 => write!(f, "{}", v),
            VrValue::I32(v)                 => write!(f, "{}", v),
            VrValue::U16(v)                 => write!(f, "{}", v),
            VrValue::I16(v)                 => write!(f, "{}", v),
            VrValue::F32(v)                 => write!(f, "{}", v),
            VrValue::F64(v)                 => write!(f, "{}", v),
            VrValue::Attribute(group, name) => write!(f, "({}, {})", group, name),
            VrValue::Text(s)                => write!(f, "{}", s),
            VrValue::Multi                  => write!(f, "MULTIPLE VALUE"),
            VrValue::PixelData              => write!(f, "PIXEL DATA")
        }
    }
}

pub struct DicomTag {
    pub group: u16, 
    pub element: u16,
    pub syntax: TransferSyntax,
    pub vr: VrType,
    pub stream_position: u64,
    pub value: Option<Vec<u8>>
}

impl DicomTag {
    pub fn empty() -> DicomTag {
        DicomTag {
            group: 0, 
            element: 0,
            syntax: TransferSyntax { 
                vr_encoding: VrEncoding::Explicit, 
                endian_encoding: EndianEncoding::LittleEndian
            },
            vr: VrType::Unknown,
            stream_position: 0,
            value: None
        }
    }

    fn convert_64<T1>(bytes: &Vec<u8>, convert: fn([u8; 8]) -> T1) -> T1 {
        convert([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }

    fn convert_32<T1>(bytes: &Vec<u8>, convert: fn([u8; 4]) -> T1) -> T1 {
        convert([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn convert_16<T1>(bytes: &Vec<u8>, convert: fn([u8; 2]) -> T1) -> T1 {
        convert([bytes[0], bytes[1]])
    }

    fn convert_attribute(bytes: &Vec<u8>) -> VrValue {
        VrValue::Attribute(u16::from_ne_bytes([bytes[0], bytes[1]]), u16::from_ne_bytes([bytes[2], bytes[3]]))
    }

    fn vr_value(vr: VrType, bytes: &Vec<u8>) -> VrValue {
        match vr {    
            VrType::Attribute      => DicomTag::convert_attribute(bytes),
            VrType::UnsignedShort  => VrValue::U16(DicomTag::convert_16(bytes, u16::from_ne_bytes)),
            VrType::SignedShort    => VrValue::I16(DicomTag::convert_16(bytes, i16::from_ne_bytes)),
            VrType::UnsignedLong   => VrValue::U32(DicomTag::convert_32(bytes, u32::from_ne_bytes)),
            VrType::SignedLong     => VrValue::I32(DicomTag::convert_32(bytes, i32::from_ne_bytes)),
            VrType::Float          => VrValue::F32(DicomTag::convert_32(bytes, f32::from_ne_bytes)),
            VrType::Double         => VrValue::F64(DicomTag::convert_64(bytes, f64::from_ne_bytes)),
            VrType::OtherByte | 
            VrType::OtherFloat | 
            VrType::OtherWord | 
            VrType::UnlimitedText | 
            VrType::Unknown | 
            VrType::ApplicationEntity | 
            VrType::AgeString | 
            VrType::CodeString | 
            VrType::Date | 
            VrType::DateTime | 
            VrType::LongText | 
            VrType::PersonName | 
            VrType::ShortString | 
            VrType::ShortText | 
            VrType::Time | 
            VrType::Uid            => match str::from_utf8(bytes) {
                Ok(v)  => VrValue::Text(String::from(v)),
                Err(e) => VrValue::Text(format!("ERROR CONVERTING VR {}", e)) // TODO: convert return value to Result<VrValue, String>.
            },    
            VrType::DecimalString | 
            VrType::IntegerString | 
            VrType::LongString     => VrValue::Text(String::from(str::from_utf8(bytes).unwrap())),
            _                      => panic!("Unsupported value conversion for VR {}", vr)
        }
    }

    pub fn try_value(&self) -> Option<VrValue> {
        self.value.as_ref().map(|v| {
            match (self.group, self.element) {
                tags::PIXEL_DATA => VrValue::PixelData,
                _                => DicomTag::vr_value(self.vr, v)
            }
        })
    }

    pub fn try_value_len(&self) -> Option<usize> {
        self.value.as_ref().map(|value| value.len())
    }

    pub fn try_transfer_syntax(&self) -> Option<TransferSyntax> {
        match ((self.group, self.element), self.value.as_ref()) {
            (tags::TRANSFER_SYNTAX_UID, Some(value)) => {
                //  TODO: tags representing child syntax should have their own type.
                let syntax = str::from_utf8(value).unwrap();
                Some(TransferSyntax::parse_str(&syntax))
            },
            (tags::TRANSFER_SYNTAX_UID, _)           => panic!("Transfer syntax cannot be encoded in a numeric value"),
            (_, _)                                   => None
        }
    }
}