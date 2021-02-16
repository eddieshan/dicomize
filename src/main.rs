mod dicom_types;
mod vr_types;
mod sop_class;
mod transfer_syntax;
mod tags;
mod errors;
mod readers;
mod parser;

use std::env;
use std::fs;

fn main() {
    println!("DICOM COMMAND LINE PARSER");

    let args: Vec<String> = env::args().collect();

    let dicm_file = &args[1];

    println!("PROCESSING {}", dicm_file);

    let dicom_tree = match fs::read(dicm_file) {
        Ok(buffer) => {
            let _ = parser::parse_dicom(buffer);
        },
        Err(_) => { }
    };
}
