extern crate fps_counter;
extern crate piston_window;
use fps_counter::*;
use piston_window::*;

use crate::car::Car;
use crate::lane::Lane;
use crate::runtime::RunTime;
use crate::traffic::Traffic;

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
