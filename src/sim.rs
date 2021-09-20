extern crate fps_counter;
extern crate piston_window;
use fps_counter::*;
use piston_window::*;

use crate::car::Car;
use crate::lane::Lane;
use crate::runtime::RunTime;
use crate::traffic::Traffic;

const MIN_SCALE: f64 = 1.4;
const MAX_SCALE: f64 = 30.0;
const DEFAULT_SCALE: f64 = 4.0;

pub struct Sim {
    traffic: Traffic,
    fps: FPSCounter,
    scale: f64,
}
impl Sim {
    pub fn new() -> Sim {
        let mut traffic = Traffic::new();
        traffic.add_car(Car::new(0.0, 0.0));
        traffic.add_lane(Lane::new(0.0, 0.0, 500.0));
        traffic.add_lane(Lane::new(20.0, 0.0, 500.0));
        Sim {
            traffic,
            fps: FPSCounter::default(),
            scale: DEFAULT_SCALE,
        }
    }
    pub fn scale_up(&mut self, add: f64) {
        if self.scale + add <= MAX_SCALE {
            self.scale += add;
        } else {
            self.scale = MAX_SCALE
        }
    }
    pub fn scale_down(&mut self, sub: f64) {
        if self.scale - sub >= MIN_SCALE {
            self.scale -= sub;
        } else {
            self.scale = MIN_SCALE
        }
    }
    pub fn scale_mult(&mut self, mult: f64) {
        let temp = self.scale * mult;
        if temp >= MIN_SCALE && temp <= MAX_SCALE {
            self.scale = temp;
        } else if temp < MIN_SCALE {
            self.scale = MIN_SCALE;
        } else if temp > MAX_SCALE {
            self.scale = MAX_SCALE;
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
            self.traffic.display(c, g, self.scale);
            /* for car in &self.cars {
                rectangle(car.color, car.rect(), c.transform, g);
            } */
        });
        self.step();
        //runtime.frame_delay();
        println!("{} fps", self.fps.tick());
    }
}
