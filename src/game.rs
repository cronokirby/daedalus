use std::collections::HashMap;

use image;
use image::{GenericImage, Pixel};
use minifb::{Key};
use rand::{thread_rng, Rng};

use tiles::{Tile, TileGrid};

/// Reads an image from a path into a buffer.
/// Panics if the image can't be read for any reason.
fn read_image(path: &str) -> Vec<u32> {
    let img = image::open(path).expect(&format!("Couldn't open {}", path));
    img.pixels().map(|(_, _, p)| {
        let rgba = p.to_rgba();
        let mut r: u32 = rgba.data[3] as u32;
        for i in 0..3 {
            r <<= 8;
            r |= rgba.data[i] as u32;
        }
        r
    }).collect()
}


/// Contains all the necessary info for sprites in the game.
struct SpriteData {
    player: Vec<u32>,
    tiles: HashMap<Tile, Vec<u32>>
}

impl SpriteData {
    fn from_files() -> Self {
        let player = read_image("assets/player.png");
        let floortile = read_image("assets/floortile.png");
        let walltile = read_image("assets/walltile.png");
        let mut tiles = HashMap::new();
        tiles.insert(Tile::Floor, floortile);
        tiles.insert(Tile::Wall, walltile);
        SpriteData {
            player: player,
            tiles: tiles
        }
    }
}


#[derive(Clone, Copy, Debug)]
enum Direction {
    U,
    D,
    L,
    R
}

/// Checks if a position can move in a direction given min and max bounds
fn valid_move(pos: (usize, usize), min: (usize, usize), max: (usize, usize), dir: Direction) -> bool {
    match dir {
        Direction::L => pos.1 > min.1,
        Direction::R => pos.1 < max.1,
        Direction::U => pos.0 > min.0,
        Direction::D => pos.0 < max.0
    }
}

fn move_dir(pos: (usize, usize), dir: Direction) -> (usize, usize) {
    let mut pos = pos;
    match dir {
        Direction::L => pos.1 -= 1,
        Direction::R => pos.1 += 1,
        Direction::U => pos.0 -= 1,
        Direction::D => pos.0 += 1
    }
    pos
}


/// Contains all necessary information for the Game State
pub struct Game {
    /// The current position of the player, row column
    player_pos: (usize, usize),
    /// The last player pos, used to write over movement
    /// Also indicates whether a new dungeon has been generated,
    /// None is so
    last_player_pos: Option<(usize, usize)>,
    /// The bounds (rows, columns) of the world.
    /// All entities must always be contained in this.
    world_bounds: (usize, usize),
    /// The grid containing all background tiles
    grid: TileGrid,
    /// Mapping each sprite to sprite information
    sprite_data: SpriteData,
    /// The size for every sprite
    sprite_size: usize,
    /// Used for delaying updates while animations transitions
    transition: i32
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = TileGrid::random(width, height);
        let sprite_d = SpriteData::from_files();
        let mut rng = thread_rng();
        let mut positions = Vec::with_capacity(width * height);
        for w in 0..width {
            for h in 0..height {
                positions.push((h, w));
            }
        }
        rng.shuffle(&mut positions);
        let player_pos = positions.into_iter()
            .find(|&(r, c)| grid.get((r, c)) == Tile::Floor)
            .unwrap(); // the map will always generate floor tiles
        Game {
            player_pos: player_pos,
            last_player_pos: None, // this can be anything, except (0, 0)
            world_bounds: (height - 1, width - 1),
            grid: grid,
            sprite_data: sprite_d,
            sprite_size: 32,
            transition: 0
        }
    }

    pub fn update(&mut self, keys: &[Key]) {
        // don't update at all if we're transitioning
        if self.transition > 0 {
            self.transition -= 1;
            return
        }
        let mut dir = None;
        for key in keys {
            match key {
                Key::Left => {
                    dir = Some(Direction::L);
                }
                Key::Right => {
                    dir = Some(Direction::R);
                }
                Key::Down => {
                    dir = Some(Direction::D);
                }
                Key::Up => {
                    dir = Some(Direction::U);
                }
                _ => {}
            }
        }
        if let Some(d) = dir {
            if valid_move(self.player_pos, (0, 0), self.world_bounds, d) {
                let new_pos = move_dir(self.player_pos, d);
                if self.grid.get(new_pos) == Tile::Floor {
                    self.last_player_pos = Some(self.player_pos);
                    self.player_pos = move_dir(self.player_pos, d);
                    self.transition = 30;
                }
            }
        }
    }

    pub fn write_to(&self, buffer: &mut [u32]) {
        // Render all the tiles if we're in a new area
        if let None = self.last_player_pos {
            self.grid.write_to(&self.sprite_data.tiles, buffer);
        }
        // Render the player
        let player_sprite = &self.sprite_data.player;
        let (py, px) = self.player_pos;
        let sz = self.sprite_size;
        let base_r = py * sz;
        let base_c = px * sz;
        for r in 0..sz {
            for c in 0..sz {
                let color = player_sprite[r * sz + c];
                if color != 0 {
                    buffer[(base_r + r) * self.grid.width * sz
                           + base_c + c] = color
                }
            }
        }
        // Render over the player's last position
        if let Some(pos) = self.last_player_pos {
            self.grid.write_pos(&self.sprite_data.tiles, pos, buffer);
        }
    }
}