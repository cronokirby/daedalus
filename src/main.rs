extern crate image;
extern crate minifb;
extern crate rand;
use minifb::{Key, Window, WindowOptions};

mod tiles;
mod game;
use game::{Game};



fn main() {
    const WIDTH: usize = 40 * 32;
    const HEIGHT: usize = 30 * 32;
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut game = Game::new(40, 30);
    let window_opts = WindowOptions::default();
    let mut window = Window::new(
        "Daedalus",
        WIDTH,
        HEIGHT,
        window_opts
        ).expect("Failed to make window");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let k = window.get_keys();
        if let Some(keys) = k {
            game.update(&keys);
        }
        game.write_to(&mut buffer);
        window.update_with_buffer(&buffer).expect("Failed to update window");
    }
}