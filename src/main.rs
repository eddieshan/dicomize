mod utils;
mod errors;
mod readers;

mod vr_type;
mod sop_class;
mod transfer_syntax;
mod tags;
mod dicom_tree;
mod dicom;

use std::env;
use std::fs;
use std::time::Instant;

const MIN_ARGUMENTS: usize = 2;

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