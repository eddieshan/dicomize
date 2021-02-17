use crate::vr_type::VrType;

pub struct DicomTag {
    pub id: (u16, u16),
    pub syntax: TransferSyntax,
    pub vr: VrType,
    pub vm: Option<i64>,
    pub marker: TagMarker,
    pub value: String
}

#[derive(Copy, Clone)]
pub enum VrEncoding {
    Explicit,
    Implicit
}

#[derive(Copy, Clone)]
pub enum EndianEncoding {
    LittleEndian,
    BigEndian
}

#[derive(Copy, Clone)]
pub struct TransferSyntax {
    pub vr_encoding: VrEncoding,
    pub endian_encoding: EndianEncoding,
}

#[derive(Copy, Clone)]
pub struct TagMarker {
    pub value_length: i32,
    pub stream_position: usize
}

pub struct PixelSpacing {
    x: f64,
    y: f64
}

pub struct SliceDimensions {
    columns: i32,
    rows: i32
}

pub struct Node {
    pub tag: DicomTag,
    pub children: Vec<usize>
}