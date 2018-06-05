extern crate image;
use image::{GenericImage, Pixel};
extern crate minifb;
use minifb::{Key, Scale, Window, WindowOptions};

mod tiles;
use tiles::{TileGrid};
mod game;
use game::{Game};



fn main() {
    const WIDTH: usize = 20 * 32;
    const HEIGHT: usize = 15 * 32;
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let game = Game::new(20, 15);
    game.write_to(&mut buffer);
    let mut window_opts = WindowOptions::default();
    window_opts.scale = Scale::X2;
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        window_opts)
        .expect("Failed to make window");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).expect("Failed to draw!");
    }
}