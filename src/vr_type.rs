use std::fmt;

#[derive(Copy, Clone)]
pub enum VrType {
    OtherByte,
    OtherFloat,
    OtherWord,
    Unknown,
    UnlimitedText,
    SequenceOfItems,
    ApplicationEntity,
    AgeString,
    CodeString,
    Date,
    DateTime,
    LongText,
    PersonName,
    ShortString,
    ShortText,
    Time,
    DecimalString,
    IntegerString,
    LongString,
    Uid,
    Attribute,
    UnsignedLong,
    UnsignedShort,
    SignedLong,
    SignedShort,
    Float,
    Double,
    Delimiter
}

impl fmt::Display for VrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VrType::OtherByte           => write!(f, "OtherByte"),
            VrType::OtherFloat          => write!(f, "OtherFloat"),
            VrType::OtherWord           => write!(f, "OtherWord"),
            VrType::Unknown             => write!(f, "Unknown"),
            VrType::UnlimitedText       => write!(f, "UnlimitedText"),
            VrType::SequenceOfItems     => write!(f, "SequenceOfItems"),
            VrType::ApplicationEntity   => write!(f, "ApplicationEntity"),
            VrType::AgeString           => write!(f, "AgeString"),
            VrType::CodeString          => write!(f, "CodeString"),
            VrType::Date                => write!(f, "Date"),
            VrType::DateTime            => write!(f, "DateTime"),
            VrType::LongText            => write!(f, "LongText"),
            VrType::PersonName          => write!(f, "PersonName"),
            VrType::ShortString         => write!(f, "ShortString"),
            VrType::ShortText           => write!(f, "ShortText"),
            VrType::Time                => write!(f, "Time"),
            VrType::DecimalString       => write!(f, "DecimalString"),
            VrType::IntegerString       => write!(f, "IntegerString"),
            VrType::LongString          => write!(f, "LongString"),
            VrType::Uid                 => write!(f, "Uid"),
            VrType::Attribute           => write!(f, "Attribute"),
            VrType::UnsignedLong        => write!(f, "UnsignedLong"),
            VrType::UnsignedShort       => write!(f, "UnsignedShort"),
            VrType::SignedLong          => write!(f, "SignedLong"),
            VrType::SignedShort         => write!(f, "SignedShort"),
            VrType::Float               => write!(f, "Float"),
            VrType::Double              => write!(f, "Double"),
            VrType::Delimiter           => write!(f, "Delimiter")
        }
    }
}

const CATALOGUE: [(char, char, VrType); 28] = [
    ('O', 'B', VrType::OtherByte), //Other Byte String
    ('O', 'F', VrType::OtherFloat), //Other Float String
    ('O', 'W', VrType::OtherWord), //Other Word String
    ('U', 'N', VrType::Unknown), //Unknown content
    ('U', 'T', VrType::UnlimitedText), //Unlimited Text
    ('S', 'Q', VrType::SequenceOfItems), //Sequence of Items 
    ('A', 'E', VrType::ApplicationEntity), //Application Entity
    ('A', 'S', VrType::AgeString), //Age String
    ('C', 'S', VrType::CodeString), //Code String
    ('D', 'A', VrType::Date), //Date
    ('D', 'T', VrType::DateTime), //Date Time
    ('L', 'T', VrType::LongText), //Long Text
    ('P', 'N', VrType::PersonName), //Person Name
    ('S', 'H', VrType::ShortString), //Short String
    ('S', 'T', VrType::ShortText), //Short Text
    ('T', 'M', VrType::Time), //Time
    ('D', 'S', VrType::DecimalString), //Decimal String
    ('I', 'S', VrType::IntegerString), //Integer String
    ('L', 'O', VrType::LongText), // Long String
    ('U', 'I', VrType::Uid), // Unique Identifier (UID)
    ('A', 'T', VrType::Attribute), // Attribute Tag
    ('U', 'L', VrType::UnsignedLong), // Unsigned Long (32 Bit, 4 Bytes)
    ('U', 'S', VrType::UnsignedShort), // Unsigned Short
    ('S', 'L', VrType::SignedLong), // Signed long (32 Bit, 4 Bytes)
    ('S', 'S', VrType::SignedShort), // Signed short (16 Bit, 2 Bytes)
    ('F', 'L', VrType::Float), // Floating Point Single (32 Bit, 4 Byte)
    ('F', 'D', VrType::Double), // Floating Point Double (64 Bit, 8 Byte)
    ('D', 'L', VrType::Delimiter), // Special SQ related Data Elements Items:
];

pub fn get_vr_type(vr_code: &[u8; 2]) -> VrType {
    let c0 = char::from(vr_code[0]);
    let c1 = char::from(vr_code[1]);

    for item in CATALOGUE.iter() {
        if item.0 == c0 && item.1 == c1 {
            return item.2;
        }
    }

    return VrType::Unknown;
}