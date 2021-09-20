extern crate piston_window;
use piston_window::*;

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
