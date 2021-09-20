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
    pub fn display<G>(&self, c: Context, g: &mut G)
    where
        G: Graphics,
    {
        rectangle(self.color, self.rect(), c.transform, g);
    }
}

pub struct Lane {
    center: [f64; 2], // meters
    width: f64,       // meters
    direction: f64,   // radians from +x
    length: f64,      // meters
}
impl Lane {
    const LANE_WIDTH: f64 = 4.4;
    pub fn new(x: f64, y: f64, length: f64) -> Lane {
        Lane {
            center: [x, y],
            width: 4.4,
            direction: std::f64::consts::FRAC_PI_2,
            length,
        }
    }
    pub fn display<G>(&self, c: Context, g: &mut G, scale: f64)
    where
        G: Graphics,
    {
        // add loop to draw lines
        let dash_width = 0.2 * scale;
        let dash_length = 3.08 * scale;
        let dash_gap = 8.0 * scale;
        let dashes = (self.length * scale / dash_gap).floor() as u32;
        for i in 0..dashes {
            rectangle(
                color::WHITE,
                [
                    self.center[0] * scale - self.width / 2.0 * scale - dash_width / 2.0,
                    self.center[1] * scale + self.length / 2.0 * scale
                        - dash_length / 2.0
                        - (i as f64) * dash_gap,
                    dash_width,
                    dash_length,
                ],
                c.transform,
                g,
            );
            rectangle(
                color::WHITE,
                [
                    self.center[0] * scale + self.width / 2.0 * scale - dash_width / 2.0,
                    self.center[1] * scale + self.length / 2.0 * scale
                        - dash_length / 2.0
                        - (i as f64) * dash_gap,
                    dash_width,
                    dash_length,
                ],
                c.transform,
                g,
            );
        }
    }
}

pub struct Sim {
    traffic: Traffic,
    fps: FPSCounter,
}
impl Sim {
    pub fn new() -> Sim {
        let mut traffic = Traffic::new();
        traffic.add_car(Car::new(100.0, 0.0));
        traffic.add_lane(Lane::new(0.0, 0.0, 500.0));
        traffic.add_lane(Lane::new(20.0, 0.0, 500.0));
        Sim {
            traffic,
            fps: FPSCounter::default(),
        }
    }
    pub fn step(&mut self) {
        self.traffic.step();
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
            self.traffic.display(c, g);
            /* for car in &self.cars {
                rectangle(car.color, car.rect(), c.transform, g);
            } */
        });
        self.step();
        //runtime.frame_delay();
        println!("{} fps", self.fps.tick());
    }
}

pub struct Traffic {
    cars: Vec<Car>,
    lanes: Vec<Lane>,
}
impl Traffic {
    pub fn new() -> Traffic {
        let vec1: Vec<Car> = Vec::new();
        let vec2: Vec<Lane> = Vec::new();
        Traffic {
            cars: vec1,
            lanes: vec2,
        }
    }
    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car);
    }
    pub fn add_lane(&mut self, lane: Lane) {
        self.lanes.push(lane);
    }
    pub fn display<G>(&self, c: Context, g: &mut G)
    where
        G: Graphics,
    {
        for lane in &self.lanes {
            lane.display(c, g, 2.0); // buggy when less than 2.0
        }
        for car in &self.cars {
            car.display(c, g);
        }
    }
    pub fn step(&mut self) {}
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
