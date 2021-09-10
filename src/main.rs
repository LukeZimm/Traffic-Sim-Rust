use flo_draw::canvas::*;
use flo_draw::*;

use std::thread;
use std::time::Duration;

//use flo_canvas::*;

struct Car {
    col: Color,
    width: f64,
    length: f64,
    x: f64,
    y: f64,

    vel: f64,
    acc: f64,
}

impl Car {
    pub fn new_car(col: Color, x: f64, y: f64) -> Car {
        Car {
            col: col,
            width: 50.0,
            length: 100.0,
            x: x,
            y: y,
            vel: -15.0,
            acc: 0.2,
        }
    }

    pub fn update(&mut self) {
        self.y += self.vel;
        self.vel += self.acc;
    }
}

fn main() {
    with_2d_graphics(|| {
        let mut cars = [
            // Create new cars
            Car::new_car(Color::Rgba(1.0, 0.0, 0.0, 1.0), 0.0, 100.0),
            Car::new_car(Color::Rgba(0.0, 0.0, 1.0, 1.0), 100.0, 150.0),
            Car::new_car(Color::Rgba(0.0, 1.0, 0.0, 1.0), 200.0, 200.0),
            Car::new_car(Color::Rgba(1.0, 1.0, 1.0, 1.0), 300.0, 250.0),
            Car::new_car(Color::Rgba(0.2, 0.2, 0.2, 1.0), 400.0, 300.0),
            Car::new_car(Color::Rgba(1.0, 1.0, 0.0, 1.0), 500.0, 350.0),
            Car::new_car(Color::Rgba(0.5, 0.0, 1.0, 1.0), 600.0, 400.0),
            Car::new_car(Color::Rgba(1.0, 0.5, 0.0, 1.0), 700.0, 450.0),
        ];
        let canvas = create_canvas_window("Hello World");

        loop {
            for car in cars.iter_mut() {
                car.update(); // Move cars
            }

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0)); // yercdhuybkgjtgre897n5rdgry87uhi nfodrety7b8 hedg8ny7ut4rfgdt8ug49rt
                gc.canvas_height(1000.0); // mkofbcg09u8jfthrui8ktrgf8oi7ytvrcfkbyufrvutg7498ert 5gr9e8rtf489edrytu84r4ty748r6t7y87y77777777777777777777
                gc.center_region(0.0, 0.0, 1000.0, 1000.0); // mklopkkjjjjjjnb7fgt

                for car in cars.iter() {
                    gc.rect(
                        (car.x - car.width / 2.0) as f32,
                        (car.y - car.length / 2.0) as f32,
                        (car.x + car.width / 2.0) as f32,
                        (car.y + car.length / 2.0) as f32,
                    );
                    gc.fill_color(car.col);
                    gc.fill();
                }
            });
            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));
        }
    });
}
