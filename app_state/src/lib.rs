//! The shared state between the application and the hot reloadable library

/*
    let mut oscillator = WavetableOscillator::new(44100);
    oscillator.set_frequency(440.0);

    for _ in 0..12000 {
        let x = oscillator.get_sample();
        println!("{x:?}");
    }
*/

use std::sync::{Arc, Mutex};

use cpal::Stream;
use egui::plot::{MarkerShape, PlotPoints};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use synth::oscillator::WavetableOscillator;

pub struct AppState {
    /// Current tick of the state
    pub tick: usize,

    /// An audio device
    pub audio_device: Option<Stream>,

    /// The audio data
    pub audio_data: Arc<Mutex<AllocRingBuffer<f32>>>,

    /// The oscillator used to generate tones
    pub oscillator: WavetableOscillator,

    /// Buffer containing sound data
    pub buffer: Vec<f64>,

    /// The frequency currently displayed
    pub frequency: f64,

    /// Whether or not to draw the graph
    pub draw: bool,

    /// Whether to record audio or not
    pub record: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let mut oscillator = WavetableOscillator::new(44100);
        let mut buffer = Vec::new();

        Self {
            tick: 0,
            audio_device: None,
            audio_data: Arc::new(Mutex::new(AllocRingBuffer::new(1))),
            oscillator,
            buffer,
            frequency: 100.0,
            draw: true,
            record: false,
        }
    }
}

impl AppState {
    pub fn points(&self) -> PlotPoints {
        // Gather all of the audio input points
        self.audio_data
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(x, y)| x % 4 == 0)
            .map(|(x, y)| [x as f64, *y as f64])
            .collect()
    }
}
