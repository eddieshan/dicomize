mod utils;
mod errors;
mod readers;

mod vr_type;
mod sop_class;
mod transfer_syntax;
mod tags;
mod dicom_tree;
mod dicom;

use std::fmt;
use std::env;
use std::fs;
use std::time::Instant;

use crate::dicom_tree::*;
use crate::vr_type::*;

const MIN_ARGUMENTS: usize = 2;

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

impl fmt::Display for TagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagValue::Ignored                => write!(f, "IGNORED"),
            TagValue::Attribute(group, name) => write!(f, "ATTRIBUTE ({}, {})", group, name),
            TagValue::String(s)              => write!(f, "STRING {}", s),
            TagValue::U32(v)                 => write!(f, "U32 {}", v),
            TagValue::I32(v)                 => write!(f, "I32 {}", v),
            TagValue::U16(v)                 => write!(f, "U16 {}", v),
            TagValue::I16(v)                 => write!(f, "I16 {}", v),
            TagValue::F32(v)                 => write!(f, "F32 {}", v),
            TagValue::F64(v)                 => write!(f, "F64 {}", v),
            TagValue::MultiNumeric(_, buf)   => write!(f, "MULTIPLE NUMERIC {}", buf.len()),
            TagValue::MultiString(s)         => write!(f, "MULTIPLE STRING {}", s),  
        }
    }
}

fn handle_tag(tag: &dicom_tree::DicomTag) {
    let tag_name = match tags::try_tag_name(tag.id.0, tag.id.1) {
        Some(name) => name, 
        None       => "UNKNOWN"
    };
    println!("TAG | {} | ({}, {}) | {} | {}", tag.vr, tag.id.0, tag.id.1, tag_name, tag.value);
}

fn process_dcim(dcim_file_path: &str) {

    println!("PROCESSING {} ...", dcim_file_path);

    match fs::read(dcim_file_path) {
        Ok(buffer) => {
            let _ = dicom::parse(buffer, handle_tag);
        },
        Err(err) => println!("ERROR: COULD NOT LOAD {}. {}", dcim_file_path, err)
    };
}

fn main() {
    println!("DICOM COMMAND LINE PARSER");

    let args: Vec<String> = env::args().collect();

    let now = Instant::now();

    match args.len() {
        MIN_ARGUMENTS => process_dcim(&args[1]),
        _             => println!("ERROR: UNEXPECTED NUMBER OF ARGUMENTS")
    }

    println!("FINISHED IN {}ms", now.elapsed().as_millis());    
}