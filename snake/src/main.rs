use draw::to_coord_u32;
use game::Game;
use piston_window::{
    clear, types::Color, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

mod draw;
mod game;
mod snake;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |context, graphic_buffer, _device| {
            clear(BACK_COLOR, graphic_buffer);
            game.draw(&context, graphic_buffer);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
