use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

pub trait RewindExtensions {

    fn read_rewind_16<T>(&mut self, convert: fn([u8; 2]) -> T) -> T;
    
    fn read_rewind_u16(&mut self) -> u16;
}

pub trait SeekExtensions {

    fn len(&mut self) -> u64;

    fn pos(&mut self) -> u64;
}

pub trait BinaryReader {

    fn read_64<T>(&mut self, convert: fn([u8; 8]) -> T) -> T;
    
    fn read_32<T>(&mut self, convert: fn([u8; 4]) -> T) -> T;
    
    fn read_16<T>(&mut self, convert: fn([u8; 2]) -> T) -> T;
    
    fn read_2(&mut self) -> [u8; 2];
    
    fn read_bytes(&mut self, length: usize) -> Vec<u8>;
    
    fn read_i32(&mut self) -> i32;
    
    fn read_i16(&mut self) -> i16;
    
    fn read_u16(&mut self) -> u16;
    
    fn read_u32(&mut self) -> u32;
    
    fn read_f32(&mut self) -> f32;
    
    fn read_f64(&mut self) -> f64;

    fn read_string(&mut self, length: usize) -> String;
}

impl <T: Seek> SeekExtensions for T {

    fn len(&mut self) -> u64 {
        let current = self.pos();
        // Safe to unwrap, guaranteed by the stream impl.
        let len = self.seek(SeekFrom::End(0)).unwrap();
        self.seek(SeekFrom::Start(current)).unwrap();
        len
    }

    fn pos(&mut self) -> u64 {
        // Safe to unwrap, guaranteed by the stream impl.
        self.seek(SeekFrom::Current(0)).unwrap()
    }        
}

impl <T: Read+Seek> RewindExtensions for T {

    fn read_rewind_16<T1>(&mut self, convert: fn([u8; 2]) -> T1) -> T1 {
        let mut buffer = [0; 2];
        let _ = self.read(&mut buffer[..]).unwrap();
        let value = convert(buffer);
        let _ = self.seek(SeekFrom::Current(-2)).unwrap(); // TODO: proper error propagation instead of unwrap.
        value
    }
    
    fn read_rewind_u16(&mut self) -> u16 {
        self.read_rewind_16(u16::from_ne_bytes)
    }
}

impl <T: Read> BinaryReader for T {

    fn read_64<T1>(&mut self, convert: fn([u8; 8]) -> T1) -> T1 {
        let mut buffer = [0; 8];
        let _ = self.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
        let value = convert(buffer);
        value
    }
    
    fn read_32<T1>(&mut self, convert: fn([u8; 4]) -> T1) -> T1 {
        let mut buffer = [0; 4];
        let _ = self.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
        let value = convert(buffer);
        value
    }
    
    fn read_16<T1>(&mut self, convert: fn([u8; 2]) -> T1) -> T1 {
        let mut buffer = [0; 2];
        let _ = self.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
        let value = convert(buffer);
        value
    }
    
    fn read_2(&mut self) -> [u8; 2] {
        let mut buffer = [0; 2];
        let _ = self.read(&mut buffer[..]).unwrap(); // TODO: proper error propagation instead of unwrap.
        buffer
    }
    
    fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; length];
        let _ = self.read(buffer.as_mut_slice()).unwrap(); // TODO: proper error propagation instead of unwrap.
        buffer
    }
    
    fn read_i32(&mut self) -> i32 {
        self.read_32(i32::from_ne_bytes)
    }
    
    fn read_i16(&mut self) -> i16 {
        self.read_16(i16::from_ne_bytes)
    }
    
    fn read_u16(&mut self) -> u16 {
        self.read_16(u16::from_ne_bytes)
    }
    
    fn read_u32(&mut self) -> u32 {
        self.read_32(u32::from_ne_bytes)
    }
    
    fn read_f32(&mut self) -> f32 {
        self.read_32(f32::from_ne_bytes)
    }
    
    fn read_f64(&mut self) -> f64 {
        self.read_64(f64::from_ne_bytes)
    }

    fn read_string(&mut self, length: usize) -> String {
        let mut buffer = String::new();
        let safe_length = u64::try_from(length).unwrap();
        let _ = self.take(safe_length).read_to_string(&mut buffer).unwrap();
        buffer
    }    
}