use crate::abstractions;
use crate::tags;
use crate::dicom_tag::DicomTag;

pub struct Node {
    pub tag: DicomTag,
    pub children: Vec<usize>
}

pub struct DicomContainer {
    pub nodes: Vec<Node>
}

impl abstractions::DicomHandler for DicomContainer {
    fn handle_tag(&mut self, parent_index: usize, tag: DicomTag) -> usize {
        let tag_name = match tags::try_tag_name(tag.id.0, tag.id.1) {
            Some(name) => name, 
            None       => "UNKNOWN"
        };

        println!("TAG | {} | ({}, {}) | {} | {}", tag.vr, tag.id.0, tag.id.1, tag_name, tag.value);

        let child = Node { tag: tag, children: Vec::new() };
        self.nodes.push(child);

        let child_index = self.nodes.len() - 1;
        self.nodes[parent_index].children.push(child_index);

        child_index
    }
}

pub struct DicomDumper {
    tags_count: usize
}

impl DicomDumper {
    pub fn new() -> DicomDumper {
        DicomDumper { 
            tags_count: 0
        }
    }

    pub fn len(&self) -> usize {
        self.tags_count
    }
}


impl abstractions::DicomHandler for DicomDumper {
    fn handle_tag(&mut self, _: usize, tag: DicomTag) -> usize {
        let tag_name = match tags::try_tag_name(tag.id.0, tag.id.1) {
            Some(name) => name, 
            None       => "UNKNOWN"
        };

        println!("TAG | {} | ({}, {}) | {} | {}", tag.vr, tag.id.0, tag.id.1, tag_name, tag.value);

        self.tags_count += 1;

        self.tags_count
    }
}