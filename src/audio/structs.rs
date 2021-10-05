//! Module containing the internal data structs used by the audio controller. Not for outside use!

use rodio::{OutputStream, OutputStreamHandle, PlayError, Sink, StreamError};
use std::error::Error;

use crate::audio::traits::AudioChannel;

pub struct SingleChannel {
    sink: Sink,
}

impl SingleChannel {
    pub fn create() -> SingleChannel {
        SingleChannel {
            sink: Sink::new_idle().0,
        }
    }

    pub fn play(
        &mut self,
        resource: &str,
        audio_stream: &AudioStream,
    ) -> Result<(), Box<dyn Error>> {
        let mut try_block = || -> Result<(), Box<dyn Error>> {
            self.sink.stop();
            self.sink = audio_stream.create_sink()?;

            let decoder = self.decoder_loop(resource);
            self.sink.append(decoder);

            Ok(())
        };

        try_block()
    }
}

impl AudioChannel for SingleChannel {}
unsafe impl Send for SingleChannel {}
unsafe impl Sync for SingleChannel {}

pub struct MultiChannel {
    sink: Option<Sink>,
}

impl MultiChannel {
    pub fn create() -> MultiChannel {
        MultiChannel { sink: None }
    }

    pub fn init(&mut self, sink: Sink) {
        self.sink = Some(sink)
    }

    pub fn play(&mut self, resource: &str) -> Result<(), Box<dyn Error>> {
        let try_block = || -> Result<(), Box<dyn Error>> {
            return if let Some(sink) = &self.sink {
                let decoder = self.decoder_once(resource);
                sink.append(decoder);
                Ok(())
            } else {
                Err(Box::new(PlayError::NoDevice))
            };
        };

        try_block()
    }
}

impl AudioChannel for MultiChannel {}
unsafe impl Send for MultiChannel {}
unsafe impl Sync for MultiChannel {}

pub struct AudioStream {
    #[allow(dead_code)]
    output: OutputStream,
    handle: OutputStreamHandle,
}

impl AudioStream {
    pub fn new() -> Result<AudioStream, StreamError> {
        let try_block = || -> Result<AudioStream, StreamError> {
            let (output, handle) = OutputStream::try_default()?;
            let audio_stream = AudioStream { output, handle };
            Ok(audio_stream)
        };

        try_block()
    }

    pub fn create_sink(&self) -> Result<Sink, PlayError> {
        Sink::try_new(&self.handle)
    }
}

unsafe impl Send for AudioStream {}
unsafe impl Sync for AudioStream {}

pub struct AudioController {
    stream: Option<AudioStream>,
    channel_ambiance: SingleChannel,
    channel_background: SingleChannel,
    channel_sound_effect: MultiChannel,
}

impl AudioController {
    pub fn new_idle() -> AudioController {
        AudioController {
            stream: None,
            channel_ambiance: SingleChannel::create(),
            channel_background: SingleChannel::create(),
            channel_sound_effect: MultiChannel::create(),
        }
    }

    pub fn init(&mut self, stream: AudioStream) -> Result<(), PlayError> {
        let try_block = || -> Result<(), PlayError> {
            let sound_effect_sink = stream.create_sink()?;
            self.stream = Some(stream);
            self.channel_sound_effect.init(sound_effect_sink);
            Ok(())
        };

        try_block()
    }

    pub fn play_ambiance(&mut self, resource: &str) -> Result<(), Box<dyn Error>> {
        let mut try_block = || -> Result<(), Box<dyn Error>> {
            let stream = self.stream.as_ref().ok_or(StreamError::NoDevice)?;
            self.channel_ambiance.play(resource, stream)
        };

        try_block()
    }

    pub fn play_background_music(&mut self, resource: &str) -> Result<(), Box<dyn Error>> {
        let mut try_block = || -> Result<(), Box<dyn Error>> {
            let stream = self.stream.as_ref().ok_or(StreamError::NoDevice)?;
            self.channel_background.play(resource, stream)
        };

        try_block()
    }

    pub fn play_sound_effect(&mut self, resource: &str) -> Result<(), Box<dyn Error>> {
        self.channel_sound_effect.play(resource)
    }
}

unsafe impl Send for AudioController {}
unsafe impl Sync for AudioController {}
