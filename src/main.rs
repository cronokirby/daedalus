extern crate minifb;

use minifb::{Key, WindowOptions, Window};


/// Used to modify pixels but have them printed larger
struct Buffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize
}

impl Buffer {
    /// Constructs a new buffer.
    /// Width and Height are the number of faux pixels,
    /// and not actual pixels
    fn new(width: usize, height: usize) -> Self {
        Buffer { buffer: vec![0; width * height * 4],
                 width: width,
                 height: height }
    }

    /// Return the bounds of the buffer, in faux pixels.
    fn bounds(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get(&self, row: usize, column: usize) -> u32 {
        self.buffer[row * self.width * 4 + column * 2]
    }

    fn set(&mut self, row: usize, column: usize, color: u32) {
       let mut this_row = row * self.width * 4 + column * 2;
       for _ in 0..2 {
            for i in 0..2 {
                self.buffer[this_row + i] = color;
            }
            this_row += self.width * 2;
       }
    }

    /// Draws the buffer to a window
    fn update_window(&self, window: &mut minifb::Window) {
        window.update_with_buffer(&self.buffer).expect("Failed to draw.");
    }
}


fn main() {
    const WIDTH: usize = 416;
    const HEIGHT: usize = 320;
    let mut buffer: Buffer = Buffer::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH * 2,
        HEIGHT * 2,
        WindowOptions::default())
        .expect("Failed to make window");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.update_window(&mut window);
    }
    println!("foo");
}