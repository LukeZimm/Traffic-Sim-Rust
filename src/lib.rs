extern crate fps_counter;
extern crate piston_window;

use fps_counter::*;
use piston_window::Button::Keyboard;
use piston_window::*;
use std::time::{Duration, Instant};
use std::{thread, time};

pub struct Car {
    x: f64,
    y: f64,
    width: f64,
    length: f64,
    vel: f64,
    acc: f64,
    angle: f64,
    color: [f32; 4],
    delta: f64,
}
impl Car {
    pub fn new(x: f64, y: f64) -> Car {
        Car {
            x,
            y,
            width: 30.0,
            length: 50.0,
            vel: 0.0,
            acc: -200.0,
            angle: std::f64::consts::PI / 2.0,
            color: [1.0, 0.0, 0.0, 1.0],
            delta: 1.0 / 400.0,
        }
    }
    pub fn rect(&self) -> [f64; 4] {
        [
            self.x - self.width / 2.0,
            self.y - self.length / 2.0,
            self.width,
            self.length,
        ]
    }
    pub fn step(&mut self) {
        self.x += self.vel * self.angle.cos() * self.delta;
        self.y += self.vel * self.angle.sin() * self.delta;
        self.vel += self.acc * self.delta;
    }
}

pub struct Sim {
    cars: Vec<Car>,
    fps: FPSCounter,
}
impl Sim {
    pub fn new() -> Sim {
        let vec: Vec<Car> = Vec::new();
        Sim {
            cars: vec,
            fps: FPSCounter::default(),
        }
    }
    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car);
    }
    pub fn step(&mut self) {
        for car in &mut self.cars {
            car.step();
        }
    }
    pub fn render<E: GenericEvent>(
        &mut self,
        window: &mut PistonWindow,
        e: &E,
        runtime: &mut RunTime,
    ) {
        window.draw_2d(e, |c, g, _| {
            clear(color::BLACK, g);
            let c = c.trans(200.0, 200.0);
            for car in &self.cars {
                rectangle(car.color, car.rect(), c.transform, g);
            }
        });
        self.step();
        //runtime.frame_delay();
        println!("{} fps", self.fps.tick());
    }
}

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
