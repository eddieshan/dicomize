use std::str;

use crate::errors;

pub struct BinaryBufferReader {
    pos: usize,
    buffer: Vec<u8>
}

// BinaryBufferReader is used to read Dicom files loaded into memory.
// Why not just read & parse directly from files?
// Rationale: 
// - A Dicom stack can contain hundreds/thousands of slices.
//   Load time is an issue if they are processed sequentially.
// - Read & parse directly from files hinders parallelization due to disk contention.
// - Mem buffering allows parallel processing of Dicom slices.
impl BinaryBufferReader {
    pub fn new(buffer: Vec<u8>) -> BinaryBufferReader {        
        BinaryBufferReader { pos: 0, buffer: buffer }    
    }

    fn read_32<T>(&mut self, convert: fn([u8; 4]) -> T) -> T {
        let value = convert([
            self.buffer[self.pos], 
            self.buffer[self.pos + 1], 
            self.buffer[self.pos + 2], 
            self.buffer[self.pos + 3]
        ]);
        self.pos = self.pos + 4;

        value
    }

    fn read_16<T>(&mut self, convert: fn([u8; 2]) -> T) -> T {
        let value = convert([
            self.buffer[self.pos], 
            self.buffer[self.pos + 1]
        ]);
        self.pos = self.pos + 2;

        value
    }

    fn read_rewind_16<T>(&mut self, convert: fn([u8; 2]) -> T) -> T {
        let value = convert([
            self.buffer[self.pos], 
            self.buffer[self.pos + 1]
        ]);

        value
    }

    pub fn read_2(&mut self) -> [u8; 2] {
        let value = [ self.buffer[self.pos], self.buffer[self.pos + 1] ];
        self.pos = self.pos + 2;
        value
    }

    pub fn read_str(&mut self, length: usize) -> &str {
        let end = self.pos + length - 1;
        let value = &self.buffer[ self.pos .. end ];
        self.pos = end;
        match str::from_utf8(value) {
            Ok(v) => v,
            Err(_) => panic!(errors::NON_UTF8_STRING) // TODO: propagate Error upwards instead of instant panic here.
        }
    }

    pub fn read_i32(&mut self) -> i32 {
        self.read_32(i32::from_ne_bytes)
    }

    pub fn read_i16(&mut self) -> i16 {
        self.read_16(i16::from_ne_bytes)
    }

    pub fn read_u16(&mut self) -> u16 {
        self.read_16(u16::from_ne_bytes)
    }

    pub fn read_rewind_u16(&mut self) -> u16 {
        self.read_rewind_16(u16::from_ne_bytes)
    }    

    pub fn read_f32(&mut self) -> f32 {
        self.read_32(f32::from_ne_bytes)
    }

    pub fn seek(&mut self, position: usize) {
        self.pos = position;
    }

    pub fn jump(&mut self, jump: usize) {
        self.pos += jump;
    }    

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}