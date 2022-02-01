# dicomize

A Dicom file parsing library with a simple CLI in Rust.

## Description

The Dicom file format represents a taxonomy of clinical data including embedded binary image data.  
Essentially, it is an n-ary tree with each node containing a tag id, a value and optionally some metadata.


## Design

The library provides a SAX style Dicom parser, that is, an event oriented parser that decouples parsing  
from  what you want to do with the parsed data. It only requires that you pass a DicomHandler trait where  
you can use  the parsed data from each Dicom tag however you like.  

Two reference DicomHandler implementations are provided,

- A simple DicomDumper that prints each Dicom tag data in the console.
- A DicomContainer that stores the Dicom taxonomy in an n-ary tree embedded in a Vec, where each  
  node has a Vec of children indices. This data structure is a compromise solution to get a tree container  
  in idiomatic Rust without pointers nor unsafe code, while keeping reasonable memory usage with a  
  semi-contiguous memory layout (only children indices are stored outside the main Vec containing nodes).  

It is perfectly viable to write a more memory efficient container using a fully contiguous block of memory,  
the downside is it would require 2 passes on the Dicom file. A first to count the total number of nodes and   
allocate a fixed size block of memory, and a second to parse and store the nodes.
