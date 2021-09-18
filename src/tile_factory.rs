//! Factory to create entityless tiles

use super::{swatch, Renderable};

/// Factory to produce basic tile
/// renderables, which are not entities.
pub struct TileFactory {}

impl TileFactory {
    /// Create a new floor tile
    pub fn new_floor() -> Renderable {
        let (fg, bg) = swatch::FLOOR.colors();

        Renderable {
            symbol: rltk::to_cp437('.'),
            fg,
            bg,
            order: -1,
        }
    }

    /// Create a new wall tile
    pub fn new_wall() -> Renderable {
        let (fg, bg) = swatch::WALL.colors();

        Renderable {
            symbol: rltk::to_cp437('#'),
            fg,
            bg,
            order: -1,
        }
    }
}
