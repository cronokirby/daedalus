extern crate minifb;

use minifb::{Key, WindowOptions, Window};


const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default())
        .expect("Failed to make window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in buffer.iter_mut() {
            *x = 0;
        }
        window.update_with_buffer(&buffer).unwrap();
    }
    println!("foo");
}