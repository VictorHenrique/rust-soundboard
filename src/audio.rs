use std::{io::BufReader, fs::File};
use mp4parse;

pub struct Audio(pub String);

impl Audio {
    pub fn load(self) -> rodio::Decoder<BufReader<File>> {
        let file = std::fs::File::open(self.0).unwrap();
        let x = mp4parse::read_mp4(file);
        println!(x.artist());
        rodio::Decoder::new(BufReader::new(file)).unwrap()
    }

    pub fn play(self) {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        
        let src = self.load();
        
        sink.append(src);
        sink.play();

        sink.sleep_until_end();
    }
}