use std::io::{Read, Seek, SeekFrom};

use crate::utils;
use crate::vr_type;
use crate::vr_type::VrType;
use crate::tags;
use crate::transfer_syntax::{VrEncoding, TransferSyntax};
use crate::binary_reader::*;

pub trait DicomReader {
    fn read_vr_code(&mut self, group: u16, element: u16, vr_encoding: VrEncoding) -> u16;

    fn read_reserved_i32(&mut self) -> i32;

    fn peek_syntax(&mut self, syntax: TransferSyntax) -> TransferSyntax;

    fn read_vm_16<T1>(&mut self, length: usize, convert: fn([u8; 2]) -> T1) -> T1;

    fn read_vm_32<T1>(&mut self, length: usize, convert: fn([u8; 4]) -> T1) -> T1;

    fn read_vm_64<T1>(&mut self, length: usize, convert: fn([u8; 8]) -> T1) -> T1;
}

impl <T: Read + Seek> DicomReader for T {
    fn read_vr_code(&mut self, group: u16, element: u16, vr_encoding: VrEncoding) -> u16 {
        let vr_code = tags::tag_vr_type(group, element);
        let is_even_group = utils::even(group);
        let is_private_code = element <= 0xFFu16;
    
        match (vr_code, vr_encoding, is_even_group, is_private_code) {
            (vr_type::DELIMITER, _, _, _)   => vr_code,
            (_, VrEncoding::Explicit, _, _) => self.read_u16(),
            (_, _, true, _)                 => vr_code,
            (_, _, false, true)             => vr_type::LONG_STRING,
            (_, _, false, false)            => vr_type::UNKNOWN
        }
    }
    
    fn read_reserved_i32(&mut self) -> i32 {
        let _ = self.seek(SeekFrom::Current(2)).unwrap(); // TODO: proper error propagation instead of unwrap.
        self.read_i32()
    }

    fn peek_syntax(&mut self, syntax: TransferSyntax) -> TransferSyntax {
        // First pass to get get transfer syntax based on lookup of group number.
        // Then rewind and restart read using the specific encoding.
        let group_peek = self.read_rewind_u16();
    
        match group_peek {
            0x0002_u16 => TransferSyntax::default(),
            _          => syntax
        }
    }

    fn read_vm_16<T1>(&mut self, length: usize, convert: fn([u8; 2]) -> T1) -> T1 {
        let mut buffer = vec![0u8; length];
        let _ = self.read(buffer.as_mut_slice()).unwrap(); // TODO: proper error propagation instead of unwrap.
        convert([buffer[0], buffer[1]])
    }

    fn read_vm_32<T1>(&mut self, length: usize, convert: fn([u8; 4]) -> T1) -> T1 {
        let mut buffer = vec![0u8; length];
        let _ = self.read(buffer.as_mut_slice()).unwrap(); // TODO: proper error propagation instead of unwrap.
        convert([buffer[0], buffer[1], buffer[2], buffer[3]])
    }

    fn read_vm_64<T1>(&mut self, length: usize, convert: fn([u8; 8]) -> T1) -> T1 {
        let mut buffer = vec![0u8; length];
        let _ = self.read(buffer.as_mut_slice()).unwrap(); // TODO: proper error propagation instead of unwrap.
        convert([buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]])
    }    
}