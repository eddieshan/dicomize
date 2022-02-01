# dicomize

A Dicom file parsing library with a simple CLI in Rust.

## Description

The Dicom file format represents a taxonomy of clinical data including embedded binary image data.  
Essentially, it is an n-ary tree where each node, or tag in Dicom lingo, contains an id and a value.  
Tag ids are classified in the taxonomy and describe the type of the tag value and its allowed uses. 


## API

The library provides a SAX style Dicom parser, that is, an event oriented parser that decouples parsing  
from what you want to do with the parsed data. There is only one function, dicom::parse.

TLDR: call dicom::parse passing a Read + Seek and a strategy pattern to handle each Dicom tag.

In detail:

```Rust
pub fn parse(reader: &mut (impl Read + Seek), dicom_handler: &mut impl DicomHandler)
```

It requires to be passed,

- reader: anything implementing a Read + Seek so you could parse a file but also a memory or a network stream.  
- dicom_handler: an implementation of the DicomHandler trait,

  ```Rust
  pub trait DicomHandler {
      fn handle_tag(&mut self, parent_index: usize, tag: DicomTag) -> usize;
  }
  ```

  where handle_tag allows you to decide how you want to handle each Dicom tag. parent_index is a unique,  sequentially generated id assigned to the parent tag.

Two reference DicomHandler implementations are provided,

- A simple DicomDumper that prints each Dicom tag data in the console.
- A DicomContainer that stores the Dicom taxonomy in an n-ary tree embedded in a Vec, where each  
  node has a Vec of children indices. This data structure is a compromise solution to get a tree container  
  in idiomatic Rust without pointers nor unsafe code, while keeping reasonable memory usage with a  
  semi-contiguous memory layout (only children indices are stored outside the main Vec containing nodes).  

It is perfectly viable to write a more memory efficient container using a fully contiguous block of memory,  
the downside is it would require 2 passes on the Dicom file. A first to count the total number of nodes and   
allocate a fixed size block of memory, and a second to parse and store the nodes.
