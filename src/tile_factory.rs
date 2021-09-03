//! Factory to create entityless tiles

use rltk::RGB;

use super::Renderable;

/// Factory to produce basic tile
/// renderables, which are not entities.
pub struct TileFactory {}

impl TileFactory {
    /// Create a new floor tile
    pub fn new_floor() -> Renderable {
        Renderable {
            symbol: rltk::to_cp437('.'),
            fg: RGB::from_u8(141, 163, 153),
            bg: RGB::named(rltk::BLACK),
        }
    }

    /// Create a new wall tile
    pub fn new_wall() -> Renderable {
        Renderable {
            symbol: rltk::to_cp437('#'),
            fg: RGB::from_u8(220, 206, 26),
            bg: RGB::named(rltk::BLACK),
        }
    }
}
