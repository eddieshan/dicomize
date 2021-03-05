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

pub fn get_explicit_vr<T: Read+Seek>(vr_code: u16, reader: &mut T) -> i32 {
    match vr_code {
        DELIMITER           => reader.read_i32(),
        SEQUENCE_OF_ITEMS   => reader.read_reserved_i32(),
        UID                 => i32::from(reader.read_i16()),
        ATTRIBUTE           => i32::from(reader.read_i16()),
        APPLICATION_ENTITY  => i32::from(reader.read_i16()),
        UNSIGNED_LONG       => i32::from(reader.read_i16()),
        UNSIGNED_SHORT      => i32::from(reader.read_i16()),
        SIGNED_LONG         => i32::from(reader.read_i16()),
        SIGNED_SHORT        => i32::from(reader.read_i16()),
        FLOAT               => i32::from(reader.read_i16()),
        DOUBLE              => i32::from(reader.read_i16()),
        AGE_STRING          => i32::from(reader.read_i16()),
        CODE_STRING         => i32::from(reader.read_i16()),
        LONG_TEXT           => i32::from(reader.read_i16()),
        PERSON_NAME         => i32::from(reader.read_i16()),
        SHORT_STRING        => i32::from(reader.read_i16()),
        SHORT_TEXT          => i32::from(reader.read_i16()),
        UNLIMITED_TEXT      => reader.read_reserved_i32(),
        DATE                => i32::from(reader.read_i16()),
        DATE_TIME           => i32::from(reader.read_i16()),
        TIME                => i32::from(reader.read_i16()),
        DECIMAL_STRING      => i32::from(reader.read_i16()),
        INTEGER_STRING      => i32::from(reader.read_i16()),
        LONG_STRING         => i32::from(reader.read_i16()),
        OTHER_BYTE          => reader.read_reserved_i32(),
        OTHER_FLOAT         => reader.read_reserved_i32(),
        OTHER_WORD          => reader.read_reserved_i32(),
        UNKNOWN             => reader.read_reserved_i32(),
        _                   => reader.read_reserved_i32()
    }
}