/*

example.rs 
Feel free to modify this example to suit your needs

atengine is a product of Ubuntufanboy and can be found at https://www.github.com/Ubuntufanboy/atengine

This product is in a very early development stage. Expect bugs and several updates to the engine as this project grows!
*/
mod blit;
mod frame;
mod update;

use std::thread::sleep;
use std::time::{Duration, Instant};
use frame::img_to_frame;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use crate::update::magic_speed_up;

fn main() {
    let frames_path = "path/to/frames";
    let mut frame_number = 1;
    let fps = 10; // Any more than 10 is pretty hard on engine. Feel free to change
    let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
    let mut next_frame_time = Instant::now();
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("audio.mp3").unwrap()); // can play sound too
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());
    loop {
        if Instant::now() >= next_frame_time {
            next_frame_time += frame_duration;
            let frame = img_to_frame(&format!("{}/frame{}.png", frames_path, frame_number)); // You
                                                                                             // can
                                                                                             // even
                                                                                             // load
                                                                                             // images
                                                                                             // into
                                                                                             // frames
                                                                                             // using
                                                                                             // frame.rs
            frame.print_whole_frame(); // blits every pixel in the frame using blit.rs
            frame_number += 1;
            print!("\x1b[H"); // fixes screen.
        }
    }
}

