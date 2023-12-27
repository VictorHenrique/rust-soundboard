// use std::fs::File;
// use std::io::Read;
mod metadata;
mod utils;
mod frames;

// mod apic;

fn main() {
    let _x = match metadata::ID3::new("examples/spirit_plains.mp3".to_string()) {
        Ok(t) => t, 
        Err(e) => panic!("Problem opening the file: {:?}", e)
    };
}