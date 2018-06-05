extern crate image;
use image::{GenericImage, Pixel};
extern crate minifb;
use minifb::{Key, WindowOptions, Window};

mod tiles;
use tiles::{TileGrid};
mod game;
use game::{Game};



fn main() {
    const WIDTH: usize = 40 * 32;
    const HEIGHT: usize = 30 * 32;
    let mut buffer = vec![0; WIDTH * HEIGHT];
    /*
    let mut tiles = TileGrid::new(40, 30);
    tiles.set(0, 0, tiles::Tile::Floor);
    tiles.write_to(&mut buffer);*/
    let game = Game::new(40, 30);
    game.write_to(&mut buffer);
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default())
        .expect("Failed to make window");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).expect("Failed to draw!");
    }
}