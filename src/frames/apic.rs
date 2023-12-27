use image::{io::Reader as ImageReader, DynamicImage};
use std::io::Cursor;
use crate::utils::*;
use crate::frames::frame::Frame;

#[derive(Clone)]
pub struct Apic {
    pub src: DynamicImage,
    pub extension: String
}

impl Apic {
    pub fn save_img(&self, filename: String) {
        let ext = String::from(&self.extension);
        self.src.save(format!("assets/{filename}.{ext}")).expect("Failed to save image");
    }
}

impl Frame<Apic> for Apic {
    fn read(data: &[u8], _frame_size: usize) -> Self {
        let (mime, idx) = read_text(&data[1..], data[0] == 1);
        let img = ImageReader::new(Cursor::new(&data[idx+2..]))
            .with_guessed_format().expect("Format not guessed")
            .decode()
            .expect("Image could not be decoded");
        let _pic_type = data[idx];

        Self {
            src: img,
            extension: String::from(&mime[6..])
        }
    }
}