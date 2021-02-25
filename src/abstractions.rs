use crate::dicom_tag::DicomTag;

pub trait DicomHandler {
    fn handle_tag(&mut self, parent_index: usize, tag: DicomTag) -> usize;
}