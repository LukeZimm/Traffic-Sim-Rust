extern crate piston_window;

use piston_window::Button::Keyboard;
use piston_window::*;
use std::time::{Duration, Instant};
use std::{thread, time};

const DELTA: f64 = 1.0 / 60.0;

pub struct Car {
    x: f64,
    y: f64,
    width: f64,
    length: f64,
    vel: f64,
    acc: f64,
    angle: f64,
    color: [f32; 4],
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
        self.x += self.vel * self.angle.cos() * DELTA;
        self.y += self.vel * self.angle.sin() * DELTA;
        self.vel += self.acc * DELTA;
    }
}

pub struct Sim {
    cars: Vec<Car>,
}
impl Sim {
    pub fn new() -> Sim {
        let vec: Vec<Car> = Vec::new();
        Sim { cars: vec }
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
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let mut glyphs = window.load_font(assets.join("Hack-Regular.ttf")).unwrap();
        window.draw_2d(e, |c, g, _| {
            clear(color::BLACK, g);
            let c = c.trans(200.0, 200.0);
            for car in &self.cars {
                rectangle(color::RED, car.rect(), c.transform, g);
            }
            let transform = c.transform.trans(0.0, 0.0);

            /* text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                .draw(
                    &format!("{}", runtime.fps()),
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
            .unwrap(); */
            runtime.fps();
        });
        self.step();
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
        println!("{:?}", self.prev_frame.elapsed());
        if self.prev_frame.elapsed() < self.frame_int {
            let delay = self.frame_int - self.prev_frame.elapsed();
            thread::sleep(delay);
        }
        self.prev_frame = time::Instant::now();
    }
    pub fn fps(&mut self) -> u32 {
        let fps = time::Duration::from_secs(1).as_nanos() / self.prev_frame.elapsed().as_nanos();
        println!("{}", fps);
        self.prev_frame = time::Instant::now();
        fps as u32
    }
}
