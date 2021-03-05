// Transfer syntax types and catalogue.
pub const EXPLICIT_LE: &str = "1.2.840.10008.1.2.1";
pub const IMPLICIT_LE: &str = "1.2.840.10008.1.2";
pub const EXPLICIT_BE: &str = "1.2.840.10008.1.2.2";

const TRANSFER_SYNTAXES:[(&str, &str); 140] = [
    ("1.2.840.10008.5.1.4.1.1.9.1.3", "Ambulatory Electrocardiogram Waveform"),
    ("1.2.840.10008.5.1.1.15", "Basic Annotation Box"),
    ("1.2.840.10008.5.1.1.4.1", "Basic Color Image Box"),
    ("1.2.840.10008.5.1.1.2", "Basic Film Box"),
    ("1.2.840.10008.5.1.1.1", "Basic Film Session"),
    ("1.2.840.10008.5.1.1.4", "Basic Grayscale Image Box"),
    ("1.2.840.10008.5.1.1.24.1", "Basic Print Image Overlay Box (Retired)"),
    ("1.2.840.10008.1.9", "Basic Study Content Notification (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.88.11", "Basic Text Structure Report"),
    ("1.2.840.10008.5.1.4.1.1.9.4.1", "Basic Voice Audio Waveform"),
    ("1.2.840.10008.5.1.4.1.1.11.4", "Blending Softcopy Presentation State"),
    ("1.2.840.10008.5.1.4.37.2", "Breast Imaging Relevant Patient"),
    ("1.2.840.10008.5.1.4.1.1.9.3.1", "Cardiac Electrophysiology Waveform"),
    ("1.2.840.10008.5.1.4.37.3", "Cardiac Relevant Patient"),
    ("1.2.840.10008.5.1.4.1.1.88.65", "Chest Computer-Aid Diagnosis Structured Report"),
    ("1.2.840.10008.5.1.4.1.1.11.2", "Color Softcopy Presentation State"),
    ("1.2.840.10008.5.1.4.1.1.88.33", "Comprehensive Structured Report"),
    ("1.2.840.10008.5.1.4.1.1.1", "Computed Radiography Image"),
    ("1.2.840.10008.5.1.4.1.1.2", "Computed Tomography Image"),
    ("1.2.840.10008.5.1.4.1.1.66.3", "Deformable Spatial Registration"),
    ("1.2.840.10008.3.1.2.6.1", "Detached Interpretation Management (Retired)"),
    ("1.2.840.10008.3.1.2.1.1", "Detached Patient Management (Retired)"),
    ("1.2.840.10008.3.1.2.5.1", "Detached Results Management (Retired)"),
    ("1.2.840.10008.3.1.2.3.1", "Detached Study Management (Retired)"),
    ("1.2.840.10008.3.1.2.2.1", "Detached Visit Management (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.1.3", "Digital Intra-Oral X-Ray Image - for Presentation"),
    ("1.2.840.10008.5.1.4.1.1.1.3.1", "Digital Intra-Oral XRay Image - for Processing"),
    ("1.2.840.10008.5.1.4.1.1.1.2", "Digital Mammography X-Ray Image - for Presentation"),
    ("1.2.840.10008.5.1.4.1.1.1.2.1", "Digital Mammograph X-Ray Image - for Processing"),
    ("1.2.840.10008.5.1.4.1.1.1.1", "Digital X-Ray Image - Presentation"),
    ("1.2.840.10008.5.1.4.1.1.1.1.1", "Digital X-Ray Image for Processing"),
    ("1.2.840.10008.5.1.4.1.1.104.1", "Encapsulated Portable Document Format"),
    ("1.2.840.10008.5.1.4.1.1.2.1", "Enhanced Computed Tomography Image"),
    ("1.2.840.10008.5.1.4.1.1.4.1", "Enhanced Magnetic Resonance Image"),
    ("1.2.840.10008.5.1.4.1.1.88.22", "Enhanced Structure Report"),
    ("1.2.840.10008.5.1.4.1.1.12.1.1", "Enhanced X-Ray Angiographic Image"),
    ("1.2.840.10008.5.1.4.1.1.12.2.1", "Enhanced X-Ray Radiofluoroscopic Image"),
    ("1.2.840.10008.5.1.4.1.1.9.1.2", "General Electrocardiogram Waveform"),
    ("1.2.840.10008.5.1.4.32.3", "General Purpose Performed Procedure Step"),
    ("1.2.840.10008.5.1.4.32.2", "General Purpose Scheduled Procedure Step"),
    ("1.2.840.10008.5.1.4.32.1", "General Purpose Worklist"),
    ("1.2.840.10008.5.1.4.37.1", "General Relevant Patient"),
    ("1.2.840.10008.5.1.4.1.1.11.1", "Grayscale Softcopy Presentation State"),
    ("1.2.840.10008.5.1.4.38.1", "Hanging Protocol"),
    ("1.2.840.10008.5.1.4.38.3", "Hanging Protocol - Move"),
    ("1.2.840.10008.5.1.4.38.2", "Hanging Protocol - Query"),
    ("1.2.840.10008.5.1.1.30", "Hardcopy Color Image (Retired)"),
    ("1.2.840.10008.5.1.1.29", "Hardcopy Grayscale Image (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.9.2.1", "Hemodynamic Waveform"),
    ("1.2.840.10008.5.1.1.24", "Image Overlay Box (Retired)"),
    ("1.2.840.10008.5.1.4.33", "Instance Availability"),
    ("1.2.840.10008.5.1.4.1.1.88.59", "Key Object Selection Document"),
    ("1.2.840.10008.5.1.4.1.1.4", "Magnetic Resonance Image"),
    ("1.2.840.10008.5.1.4.1.1.4.2", "Magnetic Resonance Spectroscopy"),
    ("1.2.840.10008.5.1.4.1.1.88.50", "Mammography Computer-Aided Diagnosis Structured Report"),
    ("1.2.840.10008.5.1.1.33", "Media Creation Management"),
    ("1.2.840.10008.1.3.10", "Media Storage Directory"),
    ("1.2.840.10008.3.1.2.3.3", "Modality Performed Procedure Step"),
    ("1.2.840.10008.3.1.2.3.5", "Modality Performed Procedure Step - Notification"),
    ("1.2.840.10008.3.1.2.3.4", "Modality Performed Procedure Step - Retrieve"),
    ("1.2.840.10008.5.1.4.31", "Modality Worklist"),
    ("1.2.840.10008.5.1.4.1.1.7.2", "Multi-Frame Grayscale Byte Secondary Capture Image"),
    ("1.2.840.10008.5.1.4.1.1.7.3", "Multi-Frame Grayscale Word Secondary Capture Image"),
    ("1.2.840.10008.5.1.4.1.1.7.1", "Multi-Frame Single Bit Secondary Capture Image"),
    ("1.2.840.10008.5.1.4.1.1.7.4", "Multi-Frame True Color Secondary Capture Image"),
    ("1.2.840.10008.5.1.4.1.1.20", "Nuclear Medicine Image"),
    ("1.2.840.10008.5.1.4.1.1.5", "Nuclear Medicine Image (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.77.1.5.2", "Ophthalmic Photography 16 Bit Image"),
    ("1.2.840.10008.5.1.4.1.1.77.1.5.1", "Ophthalmic Photography 8 Bit Image"),
    ("1.2.840.10008.5.1.4.1.2.1.2", "Patient Root - Move"),
    ("1.2.840.10008.5.1.4.1.2.1.1", "Patient Root - Query"),
    ("1.2.840.10008.5.1.4.1.2.1.3", "Patient Root - Retrieve"),
    ("1.2.840.10008.5.1.4.1.2.3.2", "Patient/Study Only - Move (Retired)"),
    ("1.2.840.10008.5.1.4.1.2.3.1", "Patient/Study Only - Query (Retired)"),
    ("1.2.840.10008.5.1.4.1.2.3.3", "Patient/Study Only - Retrieve (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.128", "Positron Emission Tomography Image"),
    ("1.2.840.10008.5.1.1.23", "Presentation Lookup Table"),
    ("1.2.840.10008.5.1.1.16", "Printer"),
    ("1.2.840.10008.5.1.1.16.376", "Printer Configuration"),
    ("1.2.840.10008.5.1.1.14", "Print Job"),
    ("1.2.840.10008.5.1.1.26", "Print Queue Management (Retired)"),
    ("1.2.840.10008.1.40", "Procedural Event Logging"),
    ("1.2.840.10008.5.1.4.1.1.88.40", "Procedure Log"),
    ("1.2.840.10008.5.1.4.1.1.11.3", "Pseudo-Color Softcopy Presentation State"),
    ("1.2.840.10008.5.1.1.31", "Pull Print Request (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.481.4", "Radio Therapy Beams Treatment Record"),
    ("1.2.840.10008.5.1.4.1.1.481.6", "Radio Therapy Brachy Treatment Record"),
    ("1.2.840.10008.5.1.4.1.1.481.2", "Radio Therapy Dose"),
    ("1.2.840.10008.5.1.4.1.1.481.1", "Radio Therapy Image"),
    ("1.2.840.10008.5.1.4.1.1.481.9", "Radio Therapy Ion Beams Treatment Record"),
    ("1.2.840.10008.5.1.4.1.1.481.8", "Radio Therapy Ion Plan"),
    ("1.2.840.10008.5.1.4.1.1.481.5", "Radio Therapy Plan"),
    ("1.2.840.10008.5.1.4.1.1.481.3", "Radio Therapy Structure Set"),
    ("1.2.840.10008.5.1.4.1.1.481.7", "Radio Therapy Treatment Summary Record"),
    ("1.2.840.10008.5.1.4.1.1.66", "Raw Data"),
    ("1.2.840.10008.5.1.4.1.1.67", "Real World Value Mapping"),
    ("1.2.840.10008.5.1.1.4.2", "Referenced Image Box (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.7", "Secondary Capture Image"),
    ("1.2.840.10008.5.1.4.1.1.66.4", "Segmentation"),
    ("1.3.12.2.1107.5.9.1", "Siemens CSA Non-Image"),
    ("1.3.12.2.1107.5.99.3.11", "Siemens syngo Frame Set"),
    ("1.3.12.2.1107.5.99.3.10", "Siemens syngo Volume Set"),
    ("1.2.840.10008.5.1.4.1.1.66.2", "Spatial Fiducials"),
    ("1.2.840.10008.5.1.4.1.1.66.1", "Spatial Registration"),
    ("1.2.840.10008.5.1.4.1.1.9", "Standalone Curve (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.10", "Standalone Modality Lookup Table (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.8", "Standalone Overlay (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.129", "Standalone Positron Emission Tomography Curve (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.11", "Standalone Volume of Interest Lookup Table (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.77.1.5.3", "Stereometric Relationship"),
    ("1.2.840.10008.1.20.2", "Storage Commitment Pull Model (Retired)"),
    ("1.2.840.10008.1.20.1", "Storage Commitment Push Model"),
    ("1.2.840.10008.5.1.1.27", "Stored Print (Retired)"),
    ("1.2.840.10008.3.1.2.3.2", "Study Component (Retired)"),
    ("1.2.840.10008.5.1.4.1.2.2.2", "Study Root - Move"),
    ("1.2.840.10008.5.1.4.1.2.2.1", "Study Root - Query"),
    ("1.2.840.10008.5.1.4.1.2.2.3", "Study Root - Retrieve"),
    ("1.2.392.200036.9116.7.8.1.1.1", "Toshiba MDW Non- Image"),
    ("1.2.840.10008.5.1.4.1.1.6.1", "Ultrasound Image"),
    ("1.2.840.10008.5.1.4.1.1.6", "Ultrasound Image (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.3.1", "Ultrasound Multi-Frame Image"),
    ("1.2.840.10008.5.1.4.1.1.3", "Ultrasound Multi-Frame Image (Retired)"),
    ("1.2.840.10008.1.1", "Verification"),
    ("1.2.840.10008.5.1.4.1.1.77.1.1.1", "Video Endoscopic Image"),
    ("1.2.840.10008.5.1.4.1.1.77.1.2.1", "Video Microscopic Image"),
    ("1.2.840.10008.5.1.4.1.1.77.1.4.1", "Video Photograph Image"),
    ("1.2.840.10008.5.1.4.1.1.77.1.1", "Visible Light Endoscopic Image"),
    ("1.2.840.10008.5.1.4.1.1.77.1", "Visible Light Image (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.77.1.2", "Visible Light Microscopic Image"),
    ("1.2.840.10008.5.1.4.1.1.77.2", "Visible Light Multi-Frame Image (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.77.1.4", "Visible Light Photographic Image"),
    ("1.2.840.10008.5.1.4.1.1.77.1.3", "Visible Light Slide-Coordinates Microscopic Image"),
    ("1.2.840.10008.5.1.1.22", "Volume of Interest Lookup Table Box (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.9.1.1", "X12-Lead Electrocardiogram Waveform"),
    ("1.2.840.10008.5.1.4.1.1.13.1.1", "X-Ray 3D Angiographic Image Storage"),
    ("1.2.840.10008.5.1.4.1.1.13.1.2", "X-Ray 3D Craniofacial Image Storage"),
    ("1.2.840.10008.5.1.4.1.1.12.3", "X-Ray Angiographic BiPlane Image (Retired)"),
    ("1.2.840.10008.5.1.4.1.1.12.1", "X-Ray Angiographic Image"),
    ("1.2.840.10008.5.1.4.1.1.88.67", "X-Ray Radiation Dose Structured Report"),
    ("1.2.840.10008.5.1.4.1.1.12.2", "X-Ray Radiofluoroscopic")
];

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

impl TransferSyntax {
    pub fn default() -> TransferSyntax {
        TransferSyntax { 
            vr_encoding: VrEncoding::Explicit, 
            endian_encoding: EndianEncoding::LittleEndian 
        }
    }    

    pub fn parse_str(syntax: &str) -> TransferSyntax {
        if syntax.eq_ignore_ascii_case(EXPLICIT_LE) {
            TransferSyntax { vr_encoding: VrEncoding::Explicit, endian_encoding: EndianEncoding::LittleEndian }
        }
        else if syntax.eq_ignore_ascii_case(EXPLICIT_BE) {
            TransferSyntax { vr_encoding: VrEncoding::Explicit, endian_encoding: EndianEncoding::BigEndian }
        }
        else if syntax.eq_ignore_ascii_case(IMPLICIT_LE) {
            TransferSyntax { vr_encoding: VrEncoding::Implicit, endian_encoding: EndianEncoding::LittleEndian }
        }
        else {
            TransferSyntax { vr_encoding: VrEncoding::Explicit, endian_encoding: EndianEncoding::LittleEndian }
        }
    }    
}    

pub fn not_compressed(syntax: Option<&str>) -> bool {
    match syntax {
        Some(EXPLICIT_LE) | Some(EXPLICIT_BE) | Some(IMPLICIT_LE) => true,
        _ => false
    }
}    

pub fn try_name(transfer_syntax_id: &str) -> Option<&str> {
    for item in TRANSFER_SYNTAXES.iter() {
        if item.0 == transfer_syntax_id {
            return Some(item.1);
        }
    }

    None
}

