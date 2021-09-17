extern crate piston_window;

use piston_window::*;

use traffic::Car;
use traffic::RunTime;
use traffic::Sim;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Traffic Simulator", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();
    // window.set_bench_mode(true); // Uncomment to remove frame limit
    let mut sim = Sim::new();
    sim.add_car(Car::new(0.0, 0.0));
    let mut runtime = RunTime::new(400.0);
    while let Some(e) = window.next() {
        sim.render(&mut window, &e, &mut runtime);
    }

    /* println!("running");

    let mut window: PistonWindow = WindowSettings::new("Traffic Simulator", [400, 400])
    .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut car = Car::new(0.0, 200.0);

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            let mult: f64 = 10.0;

            if button == Keyboard(Key::R) {
                car.y = 200.0;
            }
        }
        window.draw_2d(&e, |c, g, _| {
            clear(color::BLACK, g);
            let c = c.trans(200.0, 200.0);
            let rect = car.rect();
            rectangle(color::RED, rect, c.transform, g);
            car.step();
            std::thread::sleep(time::Duration::from_millis(1000 / 60));
        });
    } */
}
