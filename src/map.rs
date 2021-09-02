//! Game map implementation.

use rltk::{RandomNumberGenerator, Rltk, RGB, Algorithm2D, Point, BaseMap};
use specs::prelude::*;

use std::cmp::{max, min};

use super::{Rectangle, GAME_CONFIG};

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
#[derive(Default)]
pub struct Map {
    /// Width of the map in tiles.
    pub width: i32,

    /// Height of the map in tiles.
    pub height: i32,

    /// Vector containing all tiles in the map
    /// represented by a [TileType].
    pub tiles: Vec<TileType>,

    /// Vector containing all rooms on the map.
    /// Each room is represented by a [Rectangle].
    pub rooms: Vec<Rectangle>,

    /// Vector containing all tiles
    /// the player has already had in
    /// his fov
    pub explored_tiles: Vec<bool>,

    /// Vector containing all tiles
    /// which are currently in the fov.
    pub tiles_in_fov: Vec<bool>
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
    pub fn new_map_test(width: i32, height: i32) -> Self {
        let mut map = Map {
            width: width,
            height: height,
            tiles: vec![TileType::FLOOR; width as usize * height as usize],
            rooms: Vec::new(),
            explored_tiles: vec![false; width as usize * height as usize],
            tiles_in_fov: vec![false; width as usize * height as usize],
        };

        // Create outer vertical walls
        for x in 0..map.width {
            map.set_tile(x, 0, TileType::WALL);
            map.set_tile(x, map.max_y(), TileType::WALL);
        }

        // Create outer horizontal walls
        for y in 0..map.height {
            map.set_tile(0, y, TileType::WALL);
            map.set_tile(map.max_x(), y, TileType::WALL);
        }

        // Create a random number generator
        let mut rng = RandomNumberGenerator::new();

        // Create random walls on the map
        for _ in 0..400 {
            let x = rng.roll_dice(1, map.max_x());
            let y = rng.roll_dice(1, map.max_y());

            if (x, y) != (40, 25) {
                map.set_tile(x, y, TileType::WALL);
            }
        }

        map
    }

    /// Creates a new map with the given `width`
    /// and `height`.
    ///
    /// Adds rooms of random widht and height
    /// at random positions to the map. Rooms
    /// are connected through vertical and
    /// horizontal intersections.
    ///
    /// Every room is represented through a
    /// [Rectangle].
    ///
    /// # Arguments
    /// * `width`: The width of the new map.
    /// * `height`: The height of the new map.
    ///
    pub fn new_map_with_rooms(width: i32, height: i32) -> Self {
        // Create the base map struct
        let mut map = Map {
            width: width,
            height: height,
            tiles: vec![TileType::WALL; width as usize * height as usize],
            rooms: Vec::new(),
            explored_tiles: vec![false; width as usize * height as usize],
            tiles_in_fov: vec![false; width as usize * height as usize],
        };

        // Create a random number generator
        let mut rng = RandomNumberGenerator::new();

        // Create as many rooms as defined in the [GAME_CONFIG]
        for _ in 0..GAME_CONFIG.max_rooms {
            // Calc the [Rectangle] width and height args
            let room_width = rng.range(GAME_CONFIG.min_room_size, GAME_CONFIG.max_room_size);
            let room_height = rng.range(GAME_CONFIG.min_room_size, GAME_CONFIG.max_room_size);

            // Calc the x and y position of the top left corner of the [Rectangle].
            let x = rng.roll_dice(1, width - room_width - 1) - 1;
            let y = rng.roll_dice(1, height - room_height - 1) - 1;

            // Create the new room
            let room = Rectangle::new(x, y, room_width, room_height);

            // Check if the new room overlaps with any of the existing rooms.
            let mut can_place = true;

            for existing_room in map.rooms.iter() {
                if room.overlaps(existing_room) {
                    can_place = false;
                    break;
                }
            }

            if can_place {
                // Draw the room
                map.draw_room(&room);

                // Create the intersections between the new and the previous room.
                if !map.rooms.is_empty() {
                    let new_room_center = room.center();
                    let previous_room_center = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.draw_horizontal_intersection(
                            previous_room_center.x,
                            new_room_center.x,
                            previous_room_center.y,
                        );
                        map.draw_vertical_intersection(
                            previous_room_center.y,
                            new_room_center.y,
                            new_room_center.x,
                        );
                    } else {
                        map.draw_vertical_intersection(
                            previous_room_center.y,
                            new_room_center.y,
                            previous_room_center.x,
                        );
                        map.draw_horizontal_intersection(
                            previous_room_center.x,
                            new_room_center.x,
                            new_room_center.y,
                        );
                    }
                }

                // Add room to the map.
                map.rooms.push(room);
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
    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
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
    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) -> &mut Self {
        self.tiles[map_idx(x, y, self.width)] = tile;
        self
    }

    /// Gets the flag stored at the given `x`
    /// and `y` position, that indicates if the
    /// tile is explored.
    ///
    /// # Arguments
    /// * `x`: X coordinate of the tile.
    /// * `y`: Y coordinate of the tile.
    ///
    /// See `map_idx` for the actual access implementation.
    ///
    pub fn get_explored_tile(&self, x: i32, y: i32) -> bool {
        self.explored_tiles[map_idx(x, y, self.width)]
    }

    /// Sets the flag that indicates wether or not
    /// the tile has been explored
    ///
    /// # Arguments
    /// * `x`: X coordinate of the tile to change.
    /// * `y`: Y coordinate of the tile to change.
    /// * `explored`: Flag that indicates if the position is explored.
    ///
    /// See `map_idx´ for the actual access implementation.
    ///
    pub fn set_explored_tile(&mut self, x: i32, y: i32, explored: bool) -> &mut Self {
        self.explored_tiles[map_idx(x, y, self.width)] = explored;
        self
    }

    /// Gets the flag stored at the given `x`
    /// and `y` position, that indicates if the
    /// tile is in the fov.
    ///
    /// # Arguments
    /// * `x`: X coordinate of the tile.
    /// * `y`: Y coordinate of the tile.
    ///
    /// See `map_idx` for the actual access implementation.
    ///
    pub fn get_tile_in_fov(&self, x: i32, y: i32) -> bool {
        self.tiles_in_fov[map_idx(x, y, self.width)]
    }

    /// Sets the flag that indicates wether or not
    /// the tile is in the fov.
    ///
    /// # Arguments
    /// * `x`: X coordinate of the tile to change.
    /// * `y`: Y coordinate of the tile to change.
    /// * `is_in_fov`: Flag that indicates if the position is in the fov.
    ///
    /// See `map_idx´ for the actual access implementation.
    ///
    pub fn set_tile_in_fov(&mut self, x: i32, y: i32, is_in_fov: bool) -> &mut Self {
        self.tiles_in_fov[map_idx(x, y, self.width)] = is_in_fov;
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

    /// Checks if the given coordinate is within the bounds of the
    /// map. Returns a [Result], which contains the map index at the
    /// given coordinate. Otherwise a appropriate error message is returned.
    ///
    /// # Arguments
    /// * `x`: X coordinate of the position.
    /// * `y`: Y coordinate of the position.
    ///
    pub fn check_idx(&self, x: i32, y: i32) -> Result<usize, String> {
        // Get map idx of the position
        let idx = map_idx(x, y, self.width);

        // Return the idx if it is in bounds
        if idx > 0 && idx < self.width as usize * self.height as usize {
            return Ok(idx);
        }

        // Return an error if the idx is out of bounds
        let err = format!(
            "Index {} is out of bounds in map {} * {}!",
            idx, self.width, self.height
        );
        Err(err)
    }

    /// Draws the passed room on the map by changing the
    /// [TileType] of the positions included in the [Rectangle]
    /// to [TileType::FLOOR].
    ///
    /// # Arguments
    /// * `room`: The room to draw on the map.
    ///
    pub fn draw_room(&mut self, room: &Rectangle) -> &Self {
        // Iterate the room coords and set the positions to a floor tile
        for x in room.left + 1..=room.right {
            for y in room.top + 1..=room.bottom {
                self.set_tile(x, y, TileType::FLOOR);
            }
        }

        self
    }

    /// Draws the passed list of rooms on the map by changing the
    /// [TileType] of the positions included in the [Rectangle]
    /// to [TileType::FLOOR].
    ///
    /// # Arguments
    /// * `rooms`: List of rooms to draw on the map.
    ///
    /// # See also
    /// * See draw_room for the actual drawing implementation of
    /// a [Rectangle].
    ///
    pub fn draw_rooms(&mut self, rooms: &[&Rectangle]) -> &Self {
        for room in rooms.iter() {
            self.draw_room(room);
        }

        self
    }

    /// Uses the passed [Rltk] context to draw
    /// the map on the screen.
    ///
    /// # Arguments
    /// * `ctx`: The [Rltk] context to draw the map with.
    ///
    pub fn draw(&self, _ecs: &World, ctx: &mut Rltk) -> &Self {

        // Get starting x and y coordinates.i64
        let (mut x, mut y) = (0, 0);

        // Iterate through all tiles
        for (idx, tile) in self.tiles.iter().enumerate() {
            if self.explored_tiles[idx] {
                // Draw the tile
                self.draw_tile(x, y, tile, ctx);
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

    /// Draws a horizontal intersection from the start to the end x coordinate at the
    /// given y position, by setting the [TileType] of the effected positions to
    /// [TileType::FLOOR].
    ///
    /// # Arguments
    /// * `start_x`: The starting x coordinate of the intersection.
    /// * `end_x`: The end x coordinate of the intersection.
    /// * `y`: The y coordinate of the intersection.
    ///
    /// # Notes
    /// * If any of the passed coordinates is out of bounds, the drawing is stopped. Does
    /// not throw.
    /// * The function always iterates from the minimum to the maximum of the coordinates.
    ///
    /// # See also
    /// * See check_idx for the safety measure to handle indices which are out of bounds.
    ///
    fn draw_horizontal_intersection(&mut self, start_x: i32, end_x: i32, y: i32) -> &Self {
        // Iterate from the minimum passed x coordinate to the maximum
        for x in min(start_x, end_x)..=max(start_x, end_x) {
            // If the idx is within bounds, set a floor tile
            match self.check_idx(x, y) {
                Ok(idx) => {
                    self.tiles[idx] = TileType::FLOOR;
                }
                Err(_) => {
                    // no-op
                }
            }
        }

        self
    }

    /// Draws a vertical intersection from the start to the end y coordinate at the
    /// given x position, by setting the [TileType] of the effected positions to
    /// [TileType::FLOOR].
    ///
    /// # Arguments
    /// * `start_y`: The starting y coordinate of the intersection.
    /// * `end_y`: The end y coordinate of the intersection.
    /// * `x`: The x coordinate of the intersection.
    ///
    /// # Notes
    /// * If any of the passed coordinates is out of bounds, the drawing is stopped. Does
    /// not throw.
    /// * The function always iterates from the minimum to the maximum of the coordinates.
    ///
    /// # See also
    /// * See check_idx for the safety measure to handle indices which are out of bounds.
    ///
    fn draw_vertical_intersection(&mut self, start_y: i32, end_y: i32, x: i32) -> &Self {
        // Iterate from the minimum passed y coordinate to the maximum
        for y in min(start_y, end_y)..=max(start_y, end_y) {
            match self.check_idx(x, y) {
                // If the idx is within bounds, set a floor tile
                Ok(idx) => {
                    self.tiles[idx] = TileType::FLOOR;
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }

        self
    }

    // Match the type of tile and draw it according to the position.
    fn draw_tile(&self, x: i32, y: i32, tile: &TileType, ctx: &mut Rltk) -> &Self {
        let symbol;
        let mut foreground_color;

        match tile {
            TileType::FLOOR => {
                symbol = rltk::to_cp437('.');
                foreground_color = RGB::from_f32(0., 0.5, 0.5);
            }
            TileType::WALL => {
                symbol =  rltk::to_cp437('#');
                foreground_color =  RGB::from_f32(0., 1., 0.);
            }
        }

        if !self.tiles_in_fov[map_idx(x, y, self.width)] {
            foreground_color = foreground_color.to_greyscale();
        }

        ctx.set(x, y, foreground_color, RGB::from_f32(0., 0., 0.), symbol);

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

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::WALL
    }
}
