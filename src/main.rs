use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    traits::{DeviceTrait, ProcessResult, StreamTrait},
    Sample,
};
use cpal::{Data, FromSample, Sample, SampleFormat};
use volley::volume::is_microphone_capturing_sound;

fn main() {
    println!("Hello, world!");

    let input_devices = cpal::default_host()
        .input_devices()
        .expect("No Audio Devices Available");
    for device in input_devices {
        println!(
            "Input Device: {}",
            device.name().expect("Input Device Not Available")
        );
    }

    let output_devices = cpal::default_host()
        .output_devices()
        .expect("No Audio Devices Available");
    for device in output_devices {
        println!(
            "Output Device: {}",
            device.name().expect("Output Device Not Available")
        );
    }

    let device = cpal::default_input_device()?;
    let config = device.default_input_config()?;

    let mut stream = device.build_input_stream(config, is_microphone_capturing_sound, |_, _| {})?;

    stream.play()?;

    println!("Press Enter to stop...");
    std::io::read_line(&mut std::io::stdin())?;

    stream.stop()?;
}
