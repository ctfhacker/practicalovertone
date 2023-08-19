//! A wavetable oscillator

use std::f64::consts::PI;

/// Number of samples in the wave table
const WAVE_TABLE_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct WavetableOscillator {
    /// Number of samples to generate per second
    sample_rate: u32,

    /// The default wave table for this oscillator
    wave_table: [f64; WAVE_TABLE_SIZE],

    /// The next index to generate
    index: f64,

    ///
    index_increment: f64,
}

impl crate::Sampler for WavetableOscillator {
    fn sample(&mut self) -> f64 {
        self.get_sample()
    }
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32) -> Self {
        let mut wave_table = [0.0; WAVE_TABLE_SIZE];

        let mut i = 0;
        loop {
            if i >= WAVE_TABLE_SIZE {
                break;
            }

            wave_table[i] = (2.0 * PI * i as f64 / WAVE_TABLE_SIZE as f64).sin();
            i += 1;
        }

        Self {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_index(&mut self, index: f64) {
        self.index = index;
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.index_increment = frequency * WAVE_TABLE_SIZE as f64 / self.sample_rate as f64;
    }

    /// Get the next value from the oscillator
    pub fn get_sample(&mut self) -> f64 {
        // Get the next sample
        let sample = self.linear_interp();

        // Go to the next index
        self.index += self.index_increment;
        self.index %= WAVE_TABLE_SIZE as f64;

        // Return the sample
        return sample;
    }

    /// Return the linearly interpolated value of the current index into the wave table
    pub fn linear_interp(&self) -> f64 {
        // Find the indexes surrounding the current index
        // Index: 2.6 -> [2, 3]
        let truncated_index = (self.index as u32) as usize;
        let next_index = (truncated_index + 1) % WAVE_TABLE_SIZE;

        // Weight of the next index from the current one
        let next_index_weight = self.index - truncated_index as f64;

        // Distance from the previous index to this one
        let truncated_index_weight = 1.0 - next_index_weight;

        // Get the two weighted values from the wave table based on the weights
        let truncated_val = self.wave_table[truncated_index] * truncated_index_weight;
        let next_val = self.wave_table[next_index] * next_index_weight;

        // Return the sum of the two weighted values from the wave table
        truncated_val + next_val
    }
}
