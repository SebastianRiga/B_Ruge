//! Module containing the internal traits used by the audio data structs. Not for outside use!

use std::fs::File;
use std::io::BufReader;

use rodio::decoder::LoopedDecoder;
use rodio::Decoder;

pub trait AudioChannel {
    fn decoder_once(&self, resource: &str) -> Decoder<BufReader<File>> {
        let buffer = self.read_audio_resource(resource);
        Decoder::new(buffer).unwrap()
    }

    fn decoder_loop(&self, resource: &str) -> LoopedDecoder<BufReader<File>> {
        let buffer = self.read_audio_resource(resource);
        Decoder::new_looped(buffer).unwrap()
    }

    fn read_audio_resource(&self, resource: &str) -> BufReader<File> {
        let file = File::open(resource).unwrap();
        BufReader::new(file)
    }
}
