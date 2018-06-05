extern crate image;
extern crate minifb;
use minifb::{Key, Scale, Window, WindowOptions};

mod tiles;
mod game;
use game::{Game};



fn main() {
    const WIDTH: usize = 20 * 32;
    const HEIGHT: usize = 15 * 32;
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut game = Game::new(20, 15);
    let mut window_opts = WindowOptions::default();
    window_opts.scale = Scale::X2;
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