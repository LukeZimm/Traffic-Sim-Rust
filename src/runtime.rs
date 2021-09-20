extern crate piston_window;
use piston_window::*;

use std::time::{Duration, Instant};
use std::{thread, time};

pub struct RunTime {
    lifetime: time::Instant,
    frame_int: time::Duration,
    prev_frame: time::Instant,
}
impl RunTime {
    pub fn new(fps: f32) -> RunTime {
        RunTime {
            lifetime: time::Instant::now(),
            frame_int: time::Duration::from_secs_f32(1.0 / fps),
            prev_frame: time::Instant::now(),
        }
    }
    pub fn print(&self) {
        println!("{:?}", self.lifetime.elapsed());
    }
    pub fn frame_delay(&mut self) {
        let now = time::Instant::now();
        let elapsed = self.prev_frame.elapsed();
        let delay;
        if elapsed < self.frame_int {
            delay = self.frame_int - elapsed;
        } else {
            delay = Duration::from_secs(0)
        }
        self.prev_frame = time::Instant::now();
        thread::sleep(delay);
    }
    pub fn fps(&mut self) -> u32 {
        let prev = self.prev_frame.elapsed().as_nanos();
        // println!("{} prev", prev);
        let fps = time::Duration::from_secs(1).as_nanos() / prev;
        println!("{} fps", fps);
        fps as u32
    }
}
