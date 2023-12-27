// use std::fs::File;
// use std::io::Read;
mod metadata;

fn main() {
    let _x = metadata::ID3::new("examples/spirit_plains.mp3".to_string());
}