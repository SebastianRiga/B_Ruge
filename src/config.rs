//! Global game settings.

/// The current version of the game.
pub const GAME_VERSION: &'static str = "v0.2.6";

/// The name of the game, needed for display on the
/// window and in-game.
pub const GAME_NAME: &'static str = "B_Ruge";

/// The width of the game's window.
pub const WINDOW_WIDTH: i32 = 80;

/// The height of the game's window.
pub const WINDOW_HEIGHT: i32 = 50;

/// The width of the in-game map.
pub const MAP_WIDTH: i32 = 80;

/// The height of the in-game map.
pub const MAP_HEIGHT: i32 = 40;

/// The maximum amount of rooms the
/// map can display.
pub const MAX_ROOMS: i32 = 30;

/// The minimum size of a room on
/// the map.
pub const MIN_ROOM_SIZE: i32 = 6;

/// The maximum size of a room on
/// the map.
pub const MAX_ROOM_SIZE: i32 = 10;
