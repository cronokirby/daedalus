use std::collections::HashMap;


/// Used to represent a single tile
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tile {
    Wall,
    Floor
}

/// Used to represent a tile map
pub struct TileGrid {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    pub size: usize
}

impl TileGrid {
    pub fn new(width: usize, height: usize) -> Self {
        TileGrid {
            tiles: vec![Tile::Floor; width * height],
            width: width,
            height: height,
            size: 32
        }
    }

    /// Writes a single tile to a buffer
    pub fn write_pos(&self, tilemap: &HashMap<Tile, Vec<u32>>, pos: (usize, usize), buffer: &mut [u32]) {
        let (r, c) = pos;
        let tile = &self.tiles[r * self.width + c];
        let arr = &tilemap[tile];
        let base_r = r * self.size;
        let base_c = c * self.size;
        for rt in 0..self.size {
            for ct in 0..self.size {
                buffer[(base_r + rt) * self.width * self.size + base_c + ct] = arr[rt * self.size + ct];
            }
        }
    }

    /// Writes all the tiles to a mutable buffer.
    /// The buffer must be large enough to contain all the pixels.
    pub fn write_to(&self, tilemap: &HashMap<Tile, Vec<u32>>, buffer: &mut [u32]) {
        for r in 0..self.height {
            for c in 0..self.width {
                self.write_pos(tilemap, (r, c), buffer);
            }
        }
    }
}