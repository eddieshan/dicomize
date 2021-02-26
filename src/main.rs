mod utils;
mod errors;
mod readers;

mod vr_type;
mod sop_class;
mod transfer_syntax;
mod tags;
mod dicom_tag;
mod dicom_handlers;
mod dicom;

use std::env;
use std::time::Instant;
use std::fs::File;

use crate::dicom_tag::*;
use crate::dicom_handlers::*;

const MIN_ARGUMENTS: usize = 2;

fn load_dcim(dcim_file_path: &str) {

    println!("LOADING DICOM TAGS IN {} ...", dcim_file_path);

    let root = Node { tag: DicomTag::empty(), children: Vec::new() };

    let mut container = DicomContainer { nodes: vec![root] };

    match File::open(dcim_file_path) {
        Ok(mut reader) => dicom::parse(&mut reader, &mut container),
        Err(err)   => println!("ERROR: COULD NOT LOAD {}. {}", dcim_file_path, err)
    }

    println!("Found {} dicom nodes", container.nodes.len());
}

fn dump_dcim(dcim_file_path: &str) {

    println!("DUMPING DICOM TAGS IN {} ...", dcim_file_path);

    let mut container = DicomDumper::new();

    match File::open(dcim_file_path) {
        Ok(mut reader) => dicom::parse(&mut reader, &mut container),
        Err(err)   => println!("ERROR: COULD NOT LOAD {}. {}", dcim_file_path, err)
    }

    println!("Found {} dicom nodes", container.len());
}

fn main() {
    println!("DICOM COMMAND LINE PARSER");

    let args: Vec<String> = env::args().collect();

    let now = Instant::now();    

    match args.len() {
        MIN_ARGUMENTS => dump_dcim(&args[1]),
        _             => println!("ERROR: UNEXPECTED NUMBER OF ARGUMENTS")
    }

    println!("FINISHED IN {}ms", now.elapsed().as_millis());    
}