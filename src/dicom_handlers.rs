use crate::tags;
use crate::dicom_tag::DicomTag;

pub trait DicomHandler {
    fn handle_tag(&mut self, parent_index: usize, tag: DicomTag) -> usize;
}

pub struct DicomNode {
    tag: Option<DicomTag>, 
    children: Vec<usize>
}

pub struct DicomContainer {
    pub nodes: Vec<DicomNode>
}

impl DicomContainer {
    pub fn new() -> DicomContainer {
        DicomContainer { nodes: vec! [ DicomNode { tag: None, children: Vec::new() } ] }
    }
}

impl DicomHandler for DicomContainer {
    fn handle_tag(&mut self, parent_index: usize, tag: DicomTag) -> usize {
        let tag_name = match tags::try_tag_name(tag.group, tag.element) {
            Some(name) => name,
            None       => "UNKNOWN"
        };

        let vr = format!("{}", tag.vr);
        let id = format!("({}, {})", tag.group, tag.element);

        match tag.try_value() {
            Some(s) => println!("TAG | {:<20} | {:<14} | {:<38} | {}", vr, id, tag_name, s),
            None    => println!("TAG | {:<20} | {:<14} | {:<38} | [ NO VALUE ]", vr, id, tag_name)
        }

        let child = DicomNode { tag: Some(tag), children: Vec::new() };
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


impl DicomHandler for DicomDumper {
    fn handle_tag(&mut self, _: usize, tag: DicomTag) -> usize {
        let tag_name = match tags::try_tag_name(tag.group, tag.element) {
            Some(name) => name, 
            None       => "UNKNOWN"
        };

        let vr = format!("{}", tag.vr);
        let id = format!("({}, {})", tag.group, tag.element);

        match tag.try_value() {
            Some(s) => println!("TAG | {:<20} | {:<14} | {:<38} | {}", vr, id, tag_name, s),
            None    => println!("TAG | {:<20} | {:<14} | {:<38} | [ NO VALUE ]", vr, id, tag_name)
        }

        self.tags_count += 1;

        self.tags_count
    }
}