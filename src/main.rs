extern crate piston_window;

use piston_window::*;

mod car;
mod lane;
mod runtime;
mod sim;
mod traffic;

use runtime::RunTime;
use sim::Sim;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Traffic Simulator", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();
    // window.set_bench_mode(true); // Uncomment to remove frame limit
    let mut sim = Sim::new();
    let mut runtime = RunTime::new(400.0);
    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::W) {
                sim.scale_mult(1.1);
            } else if button == Button::Keyboard(Key::S) {
                sim.scale_mult(0.9);
            }
        }
        sim.render(&mut window, &e, &mut runtime);
    }
}
