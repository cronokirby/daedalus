use std::cmp::{min};
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
fn random_tiles(width: usize, height: usize, max_attempts: i32, buffer: &mut [Tile]) {
    let mut positions: Vec<(usize, usize)> = Vec::with_capacity(width * height);
    // We use these bounds to make sure that rooms don't touch the edges of walls
    for r in 1..(height - 3) {
        for c in 1..(width - 3) {
            positions.push((r, c));
        }
    }
    let mut attempts = max_attempts;
    let mut rooms: Vec<Room> = Vec::new();
    let mut rng = thread_rng();
    let sizes: Vec<usize> = (0..7).collect();
    while attempts > 0 {
        let &(r, c) = rng.choose(&positions).unwrap();
        // Chooses room sizes between 3 and 7, but makes sure not to hit the edge
        let w = *rng.choose(&sizes[3..min(7, width - c)]).unwrap();
        let h = *rng.choose(&sizes[3..min(7, height - r)]).unwrap();
        let room = Room::new((r, c), w, h);
        let larger = Room::new((r - 1, c - 1), w + 2, h + 2);
        let collided = rooms.iter().any(|r| r.collides_with(&larger));
        if collided {
            attempts -= 1;
        } else {
            rooms.push(room);
        }
    }
    // Now we place the tiles
    for room in &rooms {
        let (r0, c0) = room.top_left;
        let (rmax, cmax) = room.down_right;
        for r in r0..rmax {
            for c in c0..cmax {
                buffer[r * width + c] = Tile::Floor;
            }
        }
    }
}

// the bounds here are strict, no tile can be placed there
fn adjacent(pos: (usize, usize), bounds: (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = Vec::with_capacity(4);
    let (r, c) = pos;
    let (r_max, c_max) = bounds;
    if r > 0 {
        v.push((r - 1, c));
    }
    if c > 0 {
        v.push((r, c - 1));
    }
    if r + 1 < r_max {
        v.push((r + 1, c));
    }
    if c + 1 < c_max {
        v.push((r, c + 1))
    }
    v
}

fn valid_tiles(pos: (usize, usize), bounds: (usize, usize), grid: &[Tile]) -> Vec<(usize, usize)> {
    let width = bounds.1;
    adjacent(pos, bounds).into_iter().filter(|p| {
        // Any tile next to us that's already filled must be an already explored path,
        // so we don't even consider them potential candidates
        if grid[p.0 * width + p.1] == Tile::Wall {
            let adjacent = adjacent(*p, bounds);
            // We want to avoid joining tunnels accidentally
            adjacent.into_iter().filter(|p| *p != pos).all(|(r, c)| grid[r * width + c] == Tile::Wall)
        } else {
            false
        }
    }).collect()
}

/// Generates a random maze, through depth first search
fn random_maze(width: usize, height: usize) -> Vec<Tile> {
    let mut tiles = vec![Tile::Wall; width * height];
    let mut rng = thread_rng();
    let mut stack = Vec::new();
    stack.push((0, 0));
    while !stack.is_empty() {
        // we can unwrap because we know that the stack isn't empty
        let pos = stack.last().unwrap().clone();
        tiles[pos.0 * width + pos.1] = Tile::Floor;
        let valid = valid_tiles(pos, (height, width), &tiles);
        match rng.choose(&valid) {
            // There are valid position lefts, and we start searching from that position
            Some(new_pos) => { stack.push(*new_pos) }
            // There are no valid positions left, so we go back to the previous one
            None => { stack.pop(); }
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
    /// Generates a new TileGrid at random
    pub fn random(width: usize, height: usize) -> Self {
        //let v = random_tiles(width, height, 1000);
        let mut v = random_maze(width, height);
        random_tiles(width, height, 100, &mut v);
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