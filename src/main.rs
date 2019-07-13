extern crate piston_window;

mod draw;
mod paddle;
mod game;
mod block;
mod ball;
//mod rectangle;

use piston_window::*;
use piston_window::types::Color;

use game::Game;

const BACK_COLOR: Color = [0.5,0.5,0.5,1.0];

fn main() {
    let (width, height) = (400.0, 400.0);

    let mut window: PistonWindow = WindowSettings::new(
        "Breakout",
        [width, height],
    ).exit_on_esc(true)
    .build()
    .unwrap();

    //let mut keys = HashSet::new();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        /*match event {
            Event::Input(inp, None) => {
                match inp {
                    Event::Input::PressEvent(Button::Keyboard(key)) => {
                        game.keys.insert(key);
                    }
                    Event::Input::ReleaseEvent(Button::Keyboard(key)) => {
                        game.keys.remove(&key);
                    }
                }
            }
        }*/
        if let Some(Button::Keyboard(key)) = event.press_args() {
            //game.key_pressed(key);
            game.keys.insert(key);
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            game.keys.remove(&key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}