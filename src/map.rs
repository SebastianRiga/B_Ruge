//! Game map implementation.

use rltk::{RandomNumberGenerator, Rltk, RGB};

/// Enum describing all available tile
/// types of the game.
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    /// Any floor, walkable.
    FLOOR,
    /// Any wall, not walkable.
    WALL,
}

/// Struct representing the map of
/// a level in the game world.
/// A tile is represented by a [TileType].
pub struct Map {
    /// Width of the map in tiles.
    pub width: i32,

    /// Height of the map in tiles.
    pub height: i32,

    /// Vector containing all tiles in the map
    /// represented by a [TileType].
    pub tiles: Vec<TileType>,
}

impl Map {
    /// Creates a new map with the given `width`
    /// and `height`.
    /// 
    /// All outer coordinates and random positions 
    /// on the map are filled with walls.
    /// 
    /// # Arguments
    /// * `width`: The width of the new map.
    /// * `height`: The height of the new map.
    /// 
    /// # Note
    /// Only use this function for testing.
    /// 
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Map {
            width: width,
            height: height,
            tiles: vec![TileType::FLOOR; width as usize * height as usize],
        };

        // Create outer vertical walls
        for x in 0..map.width {
            map.set(x, 0, TileType::WALL);
            map.set(x, map.max_y(), TileType::WALL);
        }

        // Create outer horizontal walls
        for y in 0..map.height {
            map.set(0, y, TileType::WALL);
            map.set(map.max_x(), y, TileType::WALL);
        }

        // Create a random number generator
        let mut rng = RandomNumberGenerator::new();

        // Create random walls on the map
        for _ in 0..400 {
            let x = rng.roll_dice(1, map.max_x());
            let y = rng.roll_dice(1, map.max_y());

            if (x, y) != (40, 25) {
                map.set(x, y, TileType::WALL);
            }
        }

        map
    }

    /// Gets the [TileType] stored at the given `x`
    /// and `y` position.
    /// 
    /// # Arguments
    /// * `x`: X coordinate of the tile.
    /// * `y`: Y coordinate of the tile.
    /// 
    /// See `map_idx` for the actual access implementation.
    /// 
    pub fn get(&self, x: i32, y: i32) -> TileType {
        self.tiles[map_idx(x, y, self.width)]
    }

    /// Sets the [TileType] of the tile at the given
    /// `x` and `y` position.
    /// 
    /// # Arguments
    /// * `x`: X coordinate of the tile to change.
    /// * `y`: Y coordinate of the tile to change.
    /// * `tile`: The [TileType] to be set at the given position.
    /// 
    /// See `map_idx´ for the actual access implementation.
    /// 
    pub fn set(&mut self, x: i32, y: i32, tile: TileType) -> &mut Self {
        self.tiles[map_idx(x, y, self.width)] = tile;
        self
    }

    /// Returns the maximum x coordinate available
    /// on the map.
    pub fn max_x(&self) -> i32 {
        self.width - 1
    }

    /// Returns the maximum y coordinate available
    /// on the map.
    pub fn max_y(&self) -> i32 {
        self.height - 1
    }

    /// Uses the passed [Rltk] context to draw
    /// the map on the screen.
    /// 
    /// # Arguments
    /// * `ctx`: The [Rltk] context to draw the map with.
    /// 
    pub fn draw(&self, ctx: &mut Rltk) -> &Self {
        // Get starting x and y coordinates.i64
        let (mut x, mut y) = (0, 0);

        // Iterate through all tiles
        for tile in self.tiles.iter() {
            // Match the type of tile and draw the
            // position accordingly.
            match tile {
                TileType::FLOOR => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_u8(0, 0, 0),
                        rltk::to_cp437('.'),
                    );
                }
                TileType::WALL => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0., 1., 0.),
                        RGB::from_u8(0, 0, 0),
                        rltk::to_cp437('#'),
                    );
                }
            }

            // Increase x and y coordinate counter
            x += 1;

            if x > self.max_x() {
                x = 0;
                y += 1;
            }
        }

        self
    }
}

/// Maps the passed `x` and `y` coordinates to a 
/// index in the [Map.tile] vector index using the 
/// ´max_width` and returns it.
///  
/// # Arguments
/// * `x`: X coordinate of the tile.
/// * `y`: Y coordinate of the tile.
/// * `max_width`: The max width of the map.
/// 
fn map_idx(x: i32, y: i32, max_width: i32) -> usize {
    (y as usize * max_width as usize) + x as usize
}
