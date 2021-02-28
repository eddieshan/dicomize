use crate::tags;
use crate::dicom_tag::DicomTag;

pub trait DicomHandler {
    fn handle_tag(&mut self, parent_index: usize, tag: DicomTag) -> usize;
}

pub struct Node {
    pub tag: DicomTag,
    pub children: Vec<usize>
}

pub struct DicomContainer {
    pub nodes: Vec<Node>
}

impl DicomHandler for DicomContainer {
    fn handle_tag(&mut self, parent_index: usize, tag: DicomTag) -> usize {
        let tag_name = match tags::try_tag_name(tag.id.0, tag.id.1) {
            Some(name) => name,
            None       => "UNKNOWN"
        };

        let vr = format!("{}", tag.vr);
        let id = format!("({}, {})", tag.id.0, tag.id.1);

        match tag.try_value() {
            Some(s) => println!("TAG | {:<20} | {:<14} | {:<38} | {}", vr, id, tag_name, s),
            None    => println!("TAG | {:<20} | {:<14} | {:<38} | [ NO VALUE ]", vr, id, tag_name)
        }

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


impl DicomHandler for DicomDumper {
    fn handle_tag(&mut self, _: usize, tag: DicomTag) -> usize {
        let tag_name = match tags::try_tag_name(tag.id.0, tag.id.1) {
            Some(name) => name, 
            None       => "UNKNOWN"
        };

        let vr = format!("{}", tag.vr);
        let id = format!("({}, {})", tag.id.0, tag.id.1);

        match tag.try_value() {
            Some(s) => println!("TAG | {:<20} | {:<14} | {:<38} | {}", vr, id, tag_name, s),
            None    => println!("TAG | {:<20} | {:<14} | {:<38} | [ NO VALUE ]", vr, id, tag_name)
        }

        self.tags_count += 1;

        self.tags_count
    }
}