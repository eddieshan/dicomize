use std::fmt;

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

pub enum ValueLengthSize {
    I32,
    ReservedI32,
    I16
}

pub fn get_vr_type(vr_code: u16) -> (VrType, ValueLengthSize) {
    //(VrType::Delimiter, ValueLengthSize::I32)
    match vr_code {
        DELIMITER           => (VrType::Delimiter, ValueLengthSize::I32),
        SEQUENCE_OF_ITEMS   => (VrType::SequenceOfItems, ValueLengthSize::ReservedI32),
        UID                 => (VrType::Uid, ValueLengthSize::I16),
        ATTRIBUTE           => (VrType::Attribute, ValueLengthSize::I16),
        APPLICATION_ENTITY  => (VrType::ApplicationEntity, ValueLengthSize::I16),
        UNSIGNED_LONG       => (VrType::UnsignedLong, ValueLengthSize::I16),
        UNSIGNED_SHORT      => (VrType::UnsignedShort, ValueLengthSize::I16),
        SIGNED_LONG         => (VrType::SignedLong, ValueLengthSize::I16),
        SIGNED_SHORT        => (VrType::SignedShort, ValueLengthSize::I16),
        FLOAT               => (VrType::Float, ValueLengthSize::I16),
        DOUBLE              => (VrType::Double, ValueLengthSize::I16),
        AGE_STRING          => (VrType::AgeString, ValueLengthSize::I16),
        CODE_STRING         => (VrType::CodeString, ValueLengthSize::I16),
        LONG_TEXT           => (VrType::LongText, ValueLengthSize::I16),
        PERSON_NAME         => (VrType::PersonName, ValueLengthSize::I16),
        SHORT_STRING        => (VrType::ShortString, ValueLengthSize::I16),
        SHORT_TEXT          => (VrType::ShortText, ValueLengthSize::I16),
        UNLIMITED_TEXT      => (VrType::UnlimitedText, ValueLengthSize::ReservedI32),
        DATE                => (VrType::Date, ValueLengthSize::I16),
        DATE_TIME           => (VrType::DateTime, ValueLengthSize::I16),
        TIME                => (VrType::Time, ValueLengthSize::I16),
        DECIMAL_STRING      => (VrType::DecimalString, ValueLengthSize::I16),
        INTEGER_STRING      => (VrType::IntegerString, ValueLengthSize::I16),
        LONG_STRING         => (VrType::LongText, ValueLengthSize::I16),
        OTHER_BYTE          => (VrType::OtherByte, ValueLengthSize::ReservedI32),
        OTHER_FLOAT         => (VrType::OtherFloat, ValueLengthSize::ReservedI32),
        OTHER_WORD          => (VrType::OtherWord, ValueLengthSize::ReservedI32),
        UNKNOWN             => (VrType::Unknown, ValueLengthSize::ReservedI32),
        _                   => (VrType::Unknown, ValueLengthSize::ReservedI32)
    }
}