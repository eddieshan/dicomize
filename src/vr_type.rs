use std::fmt;
use std::io::{Read, Seek};

use crate::binary_reader::*;
use crate::dicom_reader::DicomReader;

// Structural types.
pub const DELIMITER:u16 = 19524;          // Code: "DL".
pub const SEQUENCE_OF_ITEMS:u16 = 20819;  // Code: "SQ".
pub const ATTRIBUTE:u16 = 21569;          // Code: "AT".
pub const UID:u16 = 18773;                // Code: "UI".
pub const APPLICATION_ENTITY:u16 = 17729; // Code: "AE".

// Numeric types.
pub const UNSIGNED_LONG:u16 = 19541;      // Code: "UL".   
pub const UNSIGNED_SHORT:u16 = 21333;     // Code: "US".    
pub const SIGNED_LONG:u16 = 19539;        // Code: "SL". 
pub const SIGNED_SHORT:u16 = 21331;       // Code: "SS".  
pub const FLOAT:u16 = 19526;              // Code: "FL".
pub const DOUBLE:u16 = 17478;             // Code: "FD".

// String types.
pub const AGE_STRING:u16 = 21313;         // Code: "AS".
pub const CODE_STRING:u16 = 21315;        // Code: "CS". 
pub const LONG_TEXT:u16 = 21580;          // Code: "LT".
pub const PERSON_NAME:u16 = 20048;        // Code: "PN". 
pub const SHORT_STRING:u16 = 18515;       // Code: "SH".  
pub const SHORT_TEXT:u16 = 21587;         // Code: "ST".
pub const UNLIMITED_TEXT:u16 = 21589;     // Code: "UT".

// Time types.
pub const TIME:u16 = 19796;               // Code: "TM".
pub const DATE:u16 = 16708;               // Code: "DA".
pub const DATE_TIME :u16 = 21572;         // Code: "DT".

// Numeric string types.
pub const DECIMAL_STRING:u16 = 21316;     // Code: "DS".
pub const INTEGER_STRING:u16 = 21321;     // Code: "IS".
pub const LONG_STRING:u16 = 20300;        // Code: "LO".

// "Other" group.
pub const OTHER_BYTE:u16 = 16975;         // Code: "OB".
pub const OTHER_FLOAT:u16 = 17999;        // Code: "OF".
pub const OTHER_WORD:u16 = 22351;         // Code: "OW".

// Unknown type.
pub const UNKNOWN:u16 = 20053;            // Code: "UN".


#[derive(Copy, Clone)]
pub enum VrType {
    Delimiter,
    SequenceOfItems,
    ApplicationEntity,
    Uid,
    Attribute,
    UnsignedLong,
    UnsignedShort,
    SignedLong,
    SignedShort,
    Float,
    Double,
    AgeString,
    CodeString,
    LongText,
    PersonName,
    ShortString,
    ShortText,
    UnlimitedText,
    Date,
    DateTime,
    Time,
    DecimalString,
    IntegerString,
    LongString,
    OtherByte,
    OtherFloat,
    OtherWord,
    Unknown
}

impl fmt::Display for VrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VrType::Delimiter           => write!(f, "Delimiter"),
            VrType::SequenceOfItems     => write!(f, "SequenceOfItems"),
            VrType::Attribute           => write!(f, "Attribute"),
            VrType::UnsignedLong        => write!(f, "UnsignedLong"),
            VrType::UnsignedShort       => write!(f, "UnsignedShort"),
            VrType::SignedLong          => write!(f, "SignedLong"),
            VrType::SignedShort         => write!(f, "SignedShort"),
            VrType::Float               => write!(f, "Float"),
            VrType::Double              => write!(f, "Double"),
           
            VrType::ApplicationEntity   => write!(f, "ApplicationEntity"),
            VrType::AgeString           => write!(f, "AgeString"),
            VrType::CodeString          => write!(f, "CodeString"),
            VrType::LongText            => write!(f, "LongText"),
            VrType::PersonName          => write!(f, "PersonName"),
            VrType::ShortString         => write!(f, "ShortString"),
            VrType::ShortText           => write!(f, "ShortText"),
            VrType::UnlimitedText       => write!(f, "UnlimitedText"),
            VrType::Date                => write!(f, "Date"),
            VrType::DateTime            => write!(f, "DateTime"),
            VrType::Time                => write!(f, "Time"),
            VrType::DecimalString       => write!(f, "DecimalString"),
            VrType::IntegerString       => write!(f, "IntegerString"),
            VrType::LongString          => write!(f, "LongString"),
            VrType::Uid                 => write!(f, "Uid"),
            VrType::OtherByte           => write!(f, "OtherByte"),
            VrType::OtherFloat          => write!(f, "OtherFloat"),
            VrType::OtherWord           => write!(f, "OtherWord"),
            VrType::Unknown             => write!(f, "Unknown"),

        }
    }
}

pub fn get_implicit_vr(vr_code: u16) -> VrType {
    match vr_code {
        DELIMITER           => VrType::Delimiter,
        SEQUENCE_OF_ITEMS   => VrType::SequenceOfItems,
        UID                 => VrType::Uid,
        ATTRIBUTE           => VrType::Attribute,
        APPLICATION_ENTITY  => VrType::ApplicationEntity,
        UNSIGNED_LONG       => VrType::UnsignedLong,
        UNSIGNED_SHORT      => VrType::UnsignedShort,
        SIGNED_LONG         => VrType::SignedLong,
        SIGNED_SHORT        => VrType::SignedShort,
        FLOAT               => VrType::Float,
        DOUBLE              => VrType::Double,
        AGE_STRING          => VrType::AgeString,
        CODE_STRING         => VrType::CodeString,
        LONG_TEXT           => VrType::LongText,
        PERSON_NAME         => VrType::PersonName,
        SHORT_STRING        => VrType::ShortString,
        SHORT_TEXT          => VrType::ShortText,
        UNLIMITED_TEXT      => VrType::UnlimitedText,
        DATE                => VrType::Date,
        DATE_TIME           => VrType::DateTime,
        TIME                => VrType::Time,
        DECIMAL_STRING      => VrType::DecimalString,
        INTEGER_STRING      => VrType::IntegerString,
        LONG_STRING         => VrType::LongText,
        OTHER_BYTE          => VrType::OtherByte,
        OTHER_FLOAT         => VrType::OtherFloat,
        OTHER_WORD          => VrType::OtherWord,
        UNKNOWN             => VrType::Unknown,
        _                   => VrType::Unknown
    }
}

pub fn get_explicit_vr<T: Read+Seek>(vr_code: u16, reader: &mut T) -> (VrType, i32) {
    match vr_code {
        DELIMITER           => (VrType::Delimiter, reader.read_i32()),
        SEQUENCE_OF_ITEMS   => (VrType::SequenceOfItems, reader.read_reserved_i32()),
        UID                 => (VrType::Uid, i32::from(reader.read_i16())),
        ATTRIBUTE           => (VrType::Attribute, i32::from(reader.read_i16())),
        APPLICATION_ENTITY  => (VrType::ApplicationEntity, i32::from(reader.read_i16())),
        UNSIGNED_LONG       => (VrType::UnsignedLong, i32::from(reader.read_i16())),
        UNSIGNED_SHORT      => (VrType::UnsignedShort, i32::from(reader.read_i16())),
        SIGNED_LONG         => (VrType::SignedLong, i32::from(reader.read_i16())),
        SIGNED_SHORT        => (VrType::SignedShort, i32::from(reader.read_i16())),
        FLOAT               => (VrType::Float, i32::from(reader.read_i16())),
        DOUBLE              => (VrType::Double, i32::from(reader.read_i16())),
        AGE_STRING          => (VrType::AgeString, i32::from(reader.read_i16())),
        CODE_STRING         => (VrType::CodeString, i32::from(reader.read_i16())),
        LONG_TEXT           => (VrType::LongText, i32::from(reader.read_i16())),
        PERSON_NAME         => (VrType::PersonName, i32::from(reader.read_i16())),
        SHORT_STRING        => (VrType::ShortString, i32::from(reader.read_i16())),
        SHORT_TEXT          => (VrType::ShortText, i32::from(reader.read_i16())),
        UNLIMITED_TEXT      => (VrType::UnlimitedText, reader.read_reserved_i32()),
        DATE                => (VrType::Date, i32::from(reader.read_i16())),
        DATE_TIME           => (VrType::DateTime, i32::from(reader.read_i16())),
        TIME                => (VrType::Time, i32::from(reader.read_i16())),
        DECIMAL_STRING      => (VrType::DecimalString, i32::from(reader.read_i16())),
        INTEGER_STRING      => (VrType::IntegerString, i32::from(reader.read_i16())),
        LONG_STRING         => (VrType::LongText, i32::from(reader.read_i16())),
        OTHER_BYTE          => (VrType::OtherByte, reader.read_reserved_i32()),
        OTHER_FLOAT         => (VrType::OtherFloat, reader.read_reserved_i32()),
        OTHER_WORD          => (VrType::OtherWord, reader.read_reserved_i32()),
        UNKNOWN             => (VrType::Unknown, reader.read_reserved_i32()),
        _                   => (VrType::Unknown, reader.read_reserved_i32())
    }
}