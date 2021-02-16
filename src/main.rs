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
use std::time::Instant;

fn main() {
    println!("DICOM COMMAND LINE PARSER");

    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let dicm_file = &args[1];

            println!("PROCESSING {} ...", dicm_file);
        
            let now = Instant::now();
        
            let dicom_tree = match fs::read(dicm_file) {
                Ok(buffer) => {
                    let _ = parser::parse_dicom(buffer);
                },
                Err(err) => println!("ERROR LOADING {}: {}", dicm_file, err)
            };
        
            println!("FINISHED IN {}ms", now.elapsed().as_millis());
        },
        _ => println!("ERROR: UNEXPECTED NUMBER OF ARGUMENTS")
    }
}