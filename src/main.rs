mod utils;
mod errors;
mod readers;

mod vr_type;
mod sop_class;
mod transfer_syntax;
mod tags;
mod dicom_tag;
mod dicom_container;
mod dicom;
mod abstractions;

use std::env;
use std::time::Instant;
use std::fs::File;

use crate::dicom_tag::*;
use crate::dicom_container::*;

const MIN_ARGUMENTS: usize = 2;

fn process_dcim(dcim_file_path: &str) {

    println!("PROCESSING {} ...", dcim_file_path);

    let root = Node { tag: DicomTag::empty(), children: Vec::new() };

    let mut container = DicomContainer { nodes: vec![root] };

    match File::open(dcim_file_path) {
        Ok(mut reader) => dicom::parse(&mut reader, &mut container),
        Err(err)   => println!("ERROR: COULD NOT LOAD {}. {}", dcim_file_path, err)
    }

    println!("Found {} dicom nodes", container.nodes.len());
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