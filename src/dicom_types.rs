use crate::vr_type::VrType;
use crate::transfer_syntax::TransferSyntax;

pub struct DicomTag {
    pub id: (u16, u16),
    pub syntax: TransferSyntax,
    pub vr: VrType,
    pub vm: Option<i64>,
    pub marker: TagMarker,
    pub value: String
}

#[derive(Copy, Clone)]
pub struct TagMarker {
    pub value_length: i32,
    pub stream_position: usize
}

pub struct Node {
    pub tag: DicomTag,
    pub children: Vec<usize>
}