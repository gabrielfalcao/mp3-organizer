use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use minifb::{Key, Window, WindowOptions};
use num_complex::Complex;
use rustfft::FftPlanner;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FFT_SIZE: usize = 1024; // Must be a power of 2

pub fn generate_waveform() {
    // 1. Setup shared thread-safe buffer for raw audio samples
    let audio_buffer = Arc::new(Mutex::new(vec![0.0f32; FFT_SIZE]));
    let audio_buffer_clone = Arc::clone(&audio_buffer);

    // 2. Initialize CPAL Audio Capture
    let host = cpal::default_host();
    let device =
        host.default_input_device().expect("No audio input device found. Plug in a microphone!");

    let config =
        device.default_input_config().expect("Failed to get default input config").config();

    // Spawn the audio input stream
    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if let Ok(mut buffer) = audio_buffer_clone.lock() {
                    // Shift old data out, push new data in (sliding window)
                    let to_move = buffer.len().saturating_sub(data.len());
                    if to_move > 0 {
                        buffer.copy_within(data.len().., 0);
                        let end = buffer.len() - data.len();
                        buffer[end..].copy_from_slice(&data[..data.len()]);
                    } else {
                        buffer.copy_from_slice(&data[..FFT_SIZE]);
                    }
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    // 3. Initialize Minifb Graphics Window
    let mut window =
        Window::new("Rust Real-Time Audio Visualizer", WIDTH, HEIGHT, WindowOptions::default())
            .unwrap_or_else(|e| panic!("{}", e));

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS

    let mut frame_buffer = vec![0u32; WIDTH * HEIGHT];
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(FFT_SIZE);

    // 4. Main Rendering Loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear frame buffer to black
        for pixel in frame_buffer.iter_mut() {
            *pixel = 0x000000;
        }

        // Get snapshot of current audio samples
        let samples = {
            let buffer = audio_buffer.lock().unwrap();
            buffer.clone()
        };

        // Convert audio samples into Complex numbers for FFT processing
        let mut fft_buffer: Vec<Complex<f32>> =
            samples.iter().map(|&sample| Complex::new(sample, 0.0)).collect();

        // Perform FFT (Transforms Time-Domain data to Frequency-Domain data)
        fft.process(&mut fft_buffer);

        // Calculate magnitude for the first half of the FFT results (positive frequencies)
        let num_bars = WIDTH;
        let bin_size = (FFT_SIZE / 2) / num_bars;

        for bar_idx in 0..num_bars {
            // Aggregate frequency bins for each horizontal pixel column
            let mut magnitude = 0.0;
            let start_bin = bar_idx * bin_size;
            for bin in start_bin..(start_bin + bin_size).min(FFT_SIZE / 2) {
                magnitude += fft_buffer[bin].norm();
            }
            magnitude /= bin_size as f32;

            // Amplify and scale the magnitude to fit screen height
            let mut bar_height = (magnitude * 300.0) as usize;
            if bar_height > HEIGHT {
                bar_height = HEIGHT;
            }

            // Draw a vertical bar on the frame buffer
            for y in (HEIGHT - bar_height)..HEIGHT {
                let pixel_idx = y * WIDTH + bar_idx;
                if pixel_idx < frame_buffer.len() {
                    // Draw neon cyan bars (RGB: 0, 255, 255)
                    frame_buffer[pixel_idx] = 0x00FFFF;
                }
            }
        }

        // Display the drawn frame
        window.update_with_buffer(&frame_buffer, WIDTH, HEIGHT).unwrap();
    }
}
