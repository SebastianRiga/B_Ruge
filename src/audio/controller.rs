//! Singleton controller for interaction with the current systems audio devices to play music, ambiance and sound effects.

use std::error::Error;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::audio::structs::{AudioController, AudioStream};

lazy_static! {
    static ref AUDIO_CONTROLLER: Mutex<AudioController> = {
        let controller = AudioController::new_idle();
        Mutex::new(controller)
    };
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let try_block = || -> Result<(), Box<dyn Error>> {
        let mut controller = AUDIO_CONTROLLER.lock()?;

        let audio_stream = AudioStream::new()?;
        controller.init(audio_stream)?;

        Ok(())
    };

    try_block()
}

pub fn play_ambiance(resource: &str) {
    let try_block = || -> Result<(), Box<dyn Error>> {
        let mut controller = AUDIO_CONTROLLER.lock()?;
        controller.play_ambiance(resource)?;
        Ok(())
    };

    if let Err(error) = try_block() {
        panic!("{}", error.to_string());
    }
}

pub fn play_background(resource: &str) {
    let try_block = || -> Result<(), Box<dyn Error>> {
        let mut controller = AUDIO_CONTROLLER.lock()?;
        controller.play_background_music(resource)?;
        Ok(())
    };

    if let Err(error) = try_block() {
        panic!("{}", error.to_string());
    }
}

pub fn play_sound_effect(resource: &str) {
    let try_block = || -> Result<(), Box<dyn Error>> {
        let mut controller = AUDIO_CONTROLLER.lock()?;
        controller.play_sound_effect(resource)?;
        Ok(())
    };

    if let Err(error) = try_block() {
        panic!("{}", error.to_string());
    }
}
