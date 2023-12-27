use std::{fs::File, io::Read};
use crate::frames::{apic::Apic, frame::Frame};

pub struct ID3 {
    _title: String, 
    _artist: String,
    _year: String,
    _duration: String,
    _cover: Apic,
}

fn get_tag_size(sizes: &[u8]) -> u32 {
    let mut total: u32 = 0;
    for i in 0..4 {
        let value: u32 = (sizes[i] as u32) << 7 * (3-i);
        total += value;
    }

    total
}

fn _get_frame_size(sizes: &[u8]) -> usize {
    let mut total: usize = 0;
    for i in 0..4 {
        let value: usize = (sizes[i] as usize) << 8 * (3-i);
        total += value;
    }

    total
}

fn _get_frame(buffer: &[u8]) -> (String, usize, Vec<u8>) {
    let frame_id = String::from_utf8_lossy(&buffer[0..4]);
    let size = _get_frame_size(&buffer[4..8]);
    
    let _is_compressed = if size > 100 { true } else { false };
    let _is_encoded = if [64, 96, 192, 224].contains(&size) { true } else { false };
    
    let _flags = &buffer[8..10];
    let mut data: Vec<u8> = vec![];
    for (i, value) in buffer[10..].iter().enumerate() {
        if i as usize == size { break; }
        data.push(value.clone());
    }

    (frame_id.to_string(), size + 10, data)
}

fn _get_next_text(buffer: &[u8]) -> (String, usize) {
    let _is_unicode = buffer[0] == 0;


    ("teste".to_string(), 12)
}

impl ID3 {
    pub fn new(filepath: String) -> Result<Self, &'static str> {
        let mut f: File = File::open(filepath).expect("File not found!");
        let mut buffer: Vec<u8> = Vec::new();
        
        f.read_to_end(&mut buffer).expect("Could not read the file!");
        
        // First 10 bytes from ID3 files:
        //    - [0..2] -> "I D 3"
        //    - [3..4] -> "V R", where V is the version, and R the revision number
        //    - [4..4] -> "F", an ignored flag
        //    - [5..9] -> "W X Y Z", which are size bytes (and each 8th bit is ignored, so a 28 bit total size) 
        let id3 = String::from_utf8_lossy(&buffer[0..3]);
        if !id3.eq("ID3") {
            return Err("Not an ID3 file!");
        }

        let version = &buffer[3..4][0];
        if version != &3 {
            return Err("Expected ID3v2, got ID3v{version}");
        }

        let revision_number = &buffer[4..5][0];
        let flag = &buffer[5..6][0]; // abc0 0000 -> a=Unsynchronisation, b=Extended header, c=Experimental indicator
        let size = get_tag_size(&buffer[6..10]); 
        println!("v2.{:?}{:?}, size={size}, flag={:?}", version, revision_number, flag);

        let mut _has_extended_header = flag >= &64;

        /* Frame */
        let mut fid = String::from("");
        let mut idx: usize = 10;
        let mut data = Vec::<u8>::new();
        while !fid.eq("APIC") {
            let size;
            (fid, size, data) = _get_frame(&buffer[idx as usize..]); 
            idx += size;
        } 

        let apic = Apic::read(&data, size as usize);
        apic.save_img("TTPAudio".to_string());

        Ok(Self {
            _title: "String".to_string(), 
            _artist: "String".to_string(),
            _year: "String".to_string(),
            _duration: "String".to_string(),
            _cover: apic,
        })
    }
}