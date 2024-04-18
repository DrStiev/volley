use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    traits::{DeviceTrait, ProcessResult, StreamTrait},
    Sample,
};
use cpal::{Data, FromSample, Sample, SampleFormat};

pub fn set_system_volume(volume: f64) -> Result<f64, &'static str> {
    // implement platform specific volume  control logic using FFI
    // window: winapi
    // macos objc
    // linux alsa
    if volume > 100.0 || volume < 0.0 {
        return Err("Volume percentage is incorrect!");
    }
    Ok(volume)
}

pub fn is_microphone_capturing_sound(stream: &mut dyn StreamTrait) -> bool {
    for sample in stream.next_sample() {
        // Check if the sample magnitude is non-zero
        if sample.magnitude() > 0.001 {
            return true;
        }
    }
    false
}
