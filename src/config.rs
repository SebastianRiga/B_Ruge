//! Global game settings.

/// Struct to manage the globaly constant
/// values of the game.
pub struct Config {
    /// Name of the game.
    pub name: &'static str,

    /// Window width in tiles.
    pub window_width: i32,

    /// Window height in tiles.
    pub window_height: i32,

    /// Maximum number of rooms allowed
    /// on the map.
    pub max_rooms: i32,

    /// Minimum size allowed for a room on
    /// the map.
    pub min_room_size: i32,

    /// Maxmimum size allowed for a room
    /// on the map.
    pub max_room_size: i32,
}

/// Immutable global object containing
/// the constant values of the game.
/// 
/// Usable for size and space calculations.
pub const GAME_CONFIG: Config = Config {
    name: "B_Ruge",
    window_width: 80,
    window_height: 50,
    max_rooms: 30,
    min_room_size: 6,
    max_room_size: 10,
};
