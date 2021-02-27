use std::io::{Read, Seek, SeekFrom};

use crate::utils;
use crate::vr_type;
use crate::vr_type::VrType;
use crate::tags;
use crate::transfer_syntax::{VrEncoding, TransferSyntax};
use crate::dicom_tag::*;
use crate::binary_reader::*;

pub trait DicomReader {

    fn read_vr(&mut self, group: u16, element: u16, vr_encoding: VrEncoding) -> VrType;

    fn skip_reserved(&mut self);

    fn peek_syntax(&mut self, syntax: TransferSyntax) -> TransferSyntax;        
    
    // fn read_ignored(&self) -> TagValue;
    
    // fn read_text(&mut self, value_length: usize) -> TagValue;
    
    // fn read_attribute(&mut self) -> TagValue;
    
    // fn read_numeric(&mut self, value_length: usize, number_type: Numeric) -> TagValue;
    
    // fn read_numeric_string(&mut self, value_length: usize) -> TagValue;
}

impl <T: Read + Seek> DicomReader for T {

    fn read_vr(&mut self, group: u16, element: u16, vr_encoding: VrEncoding) -> VrType {
        let vr_type = tags::tag_vr_type(group, element);
        let is_even_group = utils::even(group);
        let is_private_code = element <= 0xFFu16;
    
        match (vr_type, vr_encoding, is_even_group, is_private_code) {
            (VrType::Delimiter, _, _, _)    => vr_type,
            (_, VrEncoding::Explicit, _, _) => {
                let code = self.read_2();
                vr_type::get_vr_type(&code)
            },
            (_, _, true, _)                 => vr_type,
            (_, _, false, true)             => VrType::LongString,
            (_, _, false, false)            => VrType::Unknown
        }
    }
    
    fn skip_reserved(&mut self) {
        let _ = self.seek(SeekFrom::Current(2)).unwrap(); // TODO: proper error propagation instead of unwrap.
    }

    fn peek_syntax(&mut self, syntax: TransferSyntax) -> TransferSyntax {
        // First pass to get get transfer syntax based on lookup of group number.
        // Then rewind and start reading this time using the specified encoding.
        let group_peek = self.read_rewind_u16();
    
        match group_peek {
            0x0002_u16 => TransferSyntax::default(),
            _          => syntax
        }
    }
    
    // fn read_ignored(&self) -> TagValue {
    //     TagValue::Ignored
    // }
    
    // fn read_text(&mut self, value_length: usize) -> TagValue {
    //     let value = self.read_str(value_length);
    //     TagValue::String(value)
    // }
    
    // fn read_attribute(&mut self) -> TagValue {
    //     let group = self.read_u16();
    //     let element = self.read_u16();
    //     TagValue::Attribute(group, element)
    // }
    
    // fn read_numeric(&mut self, value_length: usize, number_type: Numeric) -> TagValue {
    //     let buf = self.read_bytes(value_length);
    //     TagValue::Numeric(number_type, buf)
    // }
    
    // fn read_numeric_string(&mut self, value_length: usize) -> TagValue {
    //     let value = self.read_str(value_length);
    //     let vm = value.split('\\').count();
    //     match vm {
    //         1 => TagValue::String(value),
    //         _ => TagValue::MultiString(value)
    //     }    
    // }   
 
}