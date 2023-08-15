use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use ringbuffer::{AllocRingBuffer, RingBuffer};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn get_input_devices() -> HashMap<String, cpal::Device> {
    let host = cpal::default_host();

    // Gather the current input devices as found by cpal
    host.input_devices()
        .expect("Failed to get input devices")
        .map(|device| {
            (
                device
                    .name()
                    .unwrap_or_else(|_| String::from("<unknown device>")),
                device,
            )
        })
        .collect()
}

fn select_default_input_device() -> Option<cpal::Device> {
    get_input_devices().remove("default")
}

/// Create a buffer
fn init_ringbuffer(size: usize) -> Arc<Mutex<AllocRingBuffer<f32>>> {
    let size = (size * 5).next_power_of_two();
    let mut buffer = AllocRingBuffer::with_capacity(size);
    buffer.fill(f32::default());
    Arc::new(Mutex::new(buffer))
}

fn lib() {
    let audio_device = select_default_input_device().expect("No 'default' audio input found");
    let config = audio_device.default_input_config().unwrap();

    assert!(
        config.sample_format() == cpal::SampleFormat::F32,
        "Only works on audio devices with format of f32 currently."
    );

    println!(
        "Audio input | Format {:?} | Channels {:?} | Sample Rate {} | Buffer Size {:?}",
        config.sample_format(),
        config.channels(),
        config.sample_rate().0,
        config.buffer_size(),
    );

    let input_audio_data = init_ringbuffer(config.sample_rate().0 as usize);

    let is_mono = config.channels() == 1;

    // Clone the Arc for the input device to read data into
    let input_audio_data2 = input_audio_data.clone();

    // Create the input stream that copies data from the input device into
    // the input ringbuffer
    let stream = audio_device
        .build_input_stream(
            &config.config(),
            move |data: &[f32], _info| {
                let mut buffer = input_audio_data2.lock().unwrap();
                if is_mono {
                    buffer.extend(data.iter().copied());
                } else {
                    buffer.extend(data.chunks_exact(2).map(|vals| vals[0] + vals[1] / 2.0));
                }
            },
            |err| {
                println!("Error during audio read: {err:#?}");
            },
            None, // Timeout
        )
        .expect("Failed to build audio input stream");

    // Start recording from the microphone
    stream.play().unwrap();

    // Read from the buffer every 100ms
    let start = std::time::Instant::now();
    while start.elapsed() <= std::time::Duration::from_secs(5) {
        std::thread::sleep_ms(100);
        println!(
            "{:?}",
            &input_audio_data
                .lock()
                .unwrap()
                .to_vec()
                .iter()
                .sum::<f32>()
        );
    }
}

/// Get the default input audio device
pub fn get() -> (Stream, Arc<Mutex<AllocRingBuffer<f32>>>) {
    let audio_device = select_default_input_device().expect("No 'default' audio input found");
    let config = audio_device.default_input_config().unwrap();

    assert!(
        config.sample_format() == cpal::SampleFormat::F32,
        "Only works on audio devices with format of f32 currently."
    );

    println!(
        "Audio input | Format {:?} | Channels {:?} | Sample Rate {} | Buffer Size {:?}",
        config.sample_format(),
        config.channels(),
        config.sample_rate().0,
        config.buffer_size(),
    );

    let input_audio_data = init_ringbuffer(config.sample_rate().0 as usize);

    let is_mono = config.channels() == 1;

    // Clone the Arc for the input device to read data into
    let input_audio_data2 = input_audio_data.clone();

    // Create the input stream that copies data from the input device into
    // the input ringbuffer
    let stream = audio_device
        .build_input_stream(
            &config.config(),
            move |data: &[f32], _info| {
                let mut buffer = input_audio_data2.lock().unwrap();
                if is_mono {
                    buffer.extend(data.iter().copied());
                } else {
                    buffer.extend(data.chunks_exact(2).map(|vals| vals[0] + vals[1] / 2.0));
                }
            },
            |err| {
                println!("Error during audio read: {err:#?}");
            },
            None, // Timeout
        )
        .expect("Failed to build audio input stream");

    (stream, input_audio_data)
}
