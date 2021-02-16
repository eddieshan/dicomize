use crate::dicom_types::VrType;

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