extern crate minifb;

use minifb::{Key, WindowOptions, Window};

mod tiles;
use tiles::{TileMap};


fn main() {
    const WIDTH: usize = 40 * 32;
    const HEIGHT: usize = 30 * 32;
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut tiles = TileMap::new(40, 30);
    tiles.set(0, 0, tiles::Tile::White);
    tiles.write_to(&mut buffer);
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