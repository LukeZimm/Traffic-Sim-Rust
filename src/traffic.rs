extern crate piston_window;
use piston_window::*;

use crate::car::Car;
use crate::lane::Lane;

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
