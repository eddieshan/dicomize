mod utils;
mod dicom_types;
mod vr_type;
mod sop_class;
mod transfer_syntax;
mod tags;
mod errors;
mod readers;
mod parser;

use std::env;
use std::fs;
use std::time::Instant;

const MIN_ARGUMENTS: usize = 2;

fn process_dcim(dcim_file_path: &str) {

    println!("PROCESSING {} ...", dcim_file_path);

    match fs::read(dcim_file_path) {
        Ok(buffer) => {
            let _ = parser::parse_dicom(buffer);
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