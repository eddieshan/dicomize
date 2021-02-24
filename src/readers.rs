use std::str;
use std::io::{Read, Seek, SeekFrom};

use crate::errors;

fn read_64<T>(reader: &mut impl Read, convert: fn([u8; 8]) -> T) -> T {
    let mut buffer = [0; 8];
    let _ = reader.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
    let value = convert(buffer);
    value
}

fn read_32<T>(reader: &mut impl Read, convert: fn([u8; 4]) -> T) -> T {
    let mut buffer = [0; 4];
    let _ = reader.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
    let value = convert(buffer);
    value
}    

fn read_16<T>(reader: &mut impl Read, convert: fn([u8; 2]) -> T) -> T {
    let mut buffer = [0; 2];
    let _ = reader.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
    let value = convert(buffer);
    value
}

fn read_rewind_16<T>(reader: &mut (impl Read + Seek), convert: fn([u8; 2]) -> T) -> T {
    let mut buffer = [0; 2];
    let _ = reader.read(&mut buffer[..]).unwrap();
    let value = convert(buffer);
    let _ = reader.seek(SeekFrom::Current(-2)).unwrap(); // TODO: proper error propagation instead of unwrap.
    value
}

pub fn read_2(reader: &mut impl Read) -> [u8; 2] {
    let mut buffer = [0; 2];
    let _ = reader.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
    buffer
}

pub fn read_str(reader: &mut impl Read, length: usize) -> String {
    let buffer = read_bytes(reader, length);
    String::from_utf8(buffer).unwrap() // TODO: proper error propagation instead of unwrap.
}

pub fn read_bytes(reader: &mut impl Read, length: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; length];
    let _ = reader.read(buffer.as_mut_slice()).unwrap(); // TODO: proper error propagation instead of unwrap.
    buffer
}

pub fn read_i32(reader: &mut impl Read) -> i32 {
    read_32(reader, i32::from_ne_bytes)
}

pub fn read_i16(reader: &mut impl Read) -> i16 {
    read_16(reader, i16::from_ne_bytes)
}

pub fn read_u16(reader: &mut impl Read) -> u16 {
    read_16(reader, u16::from_ne_bytes)
}

pub fn read_u32(reader: &mut impl Read) -> u32 {
    read_32(reader, u32::from_ne_bytes)
}

pub fn read_rewind_u16(reader: &mut (impl Read + Seek)) -> u16 {
    read_rewind_16(reader, u16::from_ne_bytes)
}    

pub fn read_f32(reader: &mut impl Read) -> f32 {
    read_32(reader, f32::from_ne_bytes)
}

pub fn read_f64(reader: &mut impl Read) -> f64 {
    read_64(reader, f64::from_ne_bytes)
}

pub fn stream_len(reader: &mut impl Seek) -> u64 {
    let current = stream_pos(reader);
    // Safe to unwrap, guaranteed by the stream impl.
    let len = reader.seek(SeekFrom::End(0)).unwrap();
    reader.seek(SeekFrom::Start(current)).unwrap();
    len
}

pub fn stream_pos(reader: &mut impl Seek) -> u64 {
    // Safe to unwrap, value guaranteed by the stream impl.
    reader.seek(SeekFrom::Current(0)).unwrap()
}