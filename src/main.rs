extern crate piston_window;

use piston_window::*;
use piston_window::Button::Keyboard;
use piston_window::Key;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();

    let mut x = 0.0;
    let mut y = 0.0;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [x, y, 100.0, 100.0],
                      context.transform,
                      graphics);


        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => {
                    y -= 10.0;
                }
                Key::Down => {
                    y += 10.0;
                }
                Key::Left => {
                    x -= 10.0;
                }
                Key::Right => {
                    x += 10.0;
                }
                _ => {}
            }
        }
    }
}