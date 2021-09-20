extern crate piston_window;
use piston_window::*;

pub struct Lane {
    center: [f64; 2], // meters
    width: f64,       // meters
    direction: f64,   // radians from +x
    length: f64,      // meters
}
impl Lane {
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
