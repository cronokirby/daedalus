use std::collections::HashMap;

use rand::{thread_rng, Rng};

/// Used to represent a single tile
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tile {
    Wall,
    Floor
}

#[derive(Debug)]
struct Room {
    top_left: (usize, usize),
    /// Non inclusive bound
    down_right: (usize, usize)
}

impl Room {
    fn new(pos: (usize, usize), width: usize, height: usize) -> Self {
        let (r, c) = pos;
        Room {
            top_left: pos,
            down_right: (r + height, c + width)
        }
    }

    /// Check whether or not 2 rooms collide
    fn collides_with(&self, that: &Room) -> bool {
        let (a_y1, a_x1) = self.top_left;
        let (a_y2, a_x2) = (self.down_right.0 - 1, self.down_right.1 - 1);
        let (b_y1, b_x1) = that.top_left;
        let (b_y2, b_x2) = (that.down_right.0 - 1, that.down_right.1 - 1);
        if a_y1 <= b_y1 && a_x1 <= b_x1 {
            b_y1 <= a_y2 && b_x1 <= a_x2
        } else {
            a_y1 <= b_y2 && a_x1 <= b_x2
        }
    }
}

/// Generate random tiles
/// Max attempts controls density, more or less.
/// It specifies how many times an overlap can fail before generation ends.
fn random_tiles(width: usize, height: usize, max_attempts: i32) -> Vec<Tile> {
    let mut positions: Vec<(usize, usize)> = Vec::with_capacity(width * height);
    // We use these bounds to make sure that rooms don't touch the edges of walls
    for r in 1..(height - 4) {
        for c in 1..(width - 4) {
            positions.push((r, c));
        }
    }
    let mut attempts = max_attempts;
    let mut rooms: Vec<Room> = Vec::new();
    let mut rng = thread_rng();
    while attempts > 0 {
        let start_pos = rng.choose(&positions).unwrap();
        let room = Room::new(*start_pos, 4, 4);
        let collided = rooms.iter().any(|r| r.collides_with(&room));
        if collided {
            attempts -= 1;
        } else {
            rooms.push(room);
            attempts -= 1;
        }
    }
    // Now we place the tiles
    let mut tiles = vec![Tile::Wall; width * height];
    for room in &rooms {
        let (r0, c0) = room.top_left;
        let (rmax, cmax) = room.down_right;
        for r in r0..rmax {
            for c in c0..cmax {
                tiles[r * width + c] = Tile::Floor;
            }
        }
    }
    tiles
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

    /// Generates a new TileGrid at random
    pub fn random(width: usize, height: usize) -> Self {
        let v = random_tiles(width, height, 200);
        TileGrid {
            tiles: v,
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