/// Used to represent a single tile
#[derive(Clone)]
pub enum Tile {
    Black,
    White
}

pub fn tile_pixels(t: &Tile) -> &[u32] {
    match t {
        Tile::White => &[0xFF_FF_FF_FF; 32 * 32],
        Tile::Black => &[0; 32 * 32]
    }
}

/// Used to represent a tile map
pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    pub size: usize
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        TileMap {
            tiles: vec![Tile::Black; width * height],
            width: width,
            height: height,
            size: 32
        }
    }

    pub fn get(&self, row: usize, column: usize) -> &Tile {
        &self.tiles[row * self.width + column]
    }

    pub fn set(&mut self, row: usize, column: usize, t: Tile) {
        self.tiles[row * self.width + column] = t;
    }

    pub fn write_to(&self, buffer: &mut [u32]) {
        for r in 0..self.height {
            for c in 0..self.width {
                let tile = &self.tiles[r * self.width + c];
                let arr = tile_pixels(tile);
                let base_r = r * self.size;
                let base_c = c * self.size;
                for rt in 0..self.size {
                    for ct in 0..self.size {
                        buffer[(base_r + rt) * self.width * self.size + base_c + ct] = arr[rt * self.size + ct];
                    }
                }
            }
        }
    }
}