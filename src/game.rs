use std::collections::HashMap;

extern crate image;
use image::{GenericImage, Pixel};

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
    tiles: HashMap<Tile, [u32; 32 * 32]>
}

impl SpriteData {
    fn from_files() -> Self {
        let player = read_image("assets/P.png");
        let mut tiles = HashMap::new();
        tiles.insert(Tile::Floor, [0xFF_FF_FF_FF; 32 * 32]);
        tiles.insert(Tile::Wall, [0xFF_00_00_00; 32 * 32]);
        SpriteData {
            player: player,
            tiles: tiles
        }
    }
}


/// Contains all necessary information for the Game State
pub struct Game {
    /// The current position of the player
    player_pos: (usize, usize),
    /// The bounds (width, height) of the world.
    /// All entities must always be contained in this.
    world_bounds: (usize, usize),
    /// The grid containing all background tiles
    grid: TileGrid,
    /// Mapping each sprite to sprite information
    sprite_data: SpriteData,
    /// The size for every sprite
    sprite_size: usize
}


impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = TileGrid::new(width, height);
        let sprite_d = SpriteData::from_files();
        Game {
            player_pos: (0, 0),
            world_bounds: (width, height),
            grid: grid,
            sprite_data: sprite_d,
            sprite_size: 32
        }
    }


    pub fn write_to(&self, buffer: &mut [u32]) {
        // Render all the tiles
        self.grid.write_to(&self.sprite_data.tiles, buffer);
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
    }
}