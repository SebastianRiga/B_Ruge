//! Module for color management

use rltk::RGB;

/// The default background color for entities and tiles. 
pub const DEFAULT_BG_COLOR: (u8, u8, u8) = (0, 0, 0);

/// Type alias for an `rgb` color `tuple`.
type U8Color = (u8, u8, u8);

/// A struct describing the foreground and
/// background color of an entity or tiles.
pub struct Pallet(pub U8Color, pub U8Color);

impl Pallet {
    /// Transforms the [Pallet]'s foreground
    /// and background [U8Color] tuples to 
    /// [RGB] structs and returns them in a tuple
    /// in the same order `(fg, bg)`.
    pub fn colors(&self) -> (RGB, RGB) {
        let fg = self.0;
        let bg = self.1;

        (
            RGB::from_u8(fg.0, fg.1, fg.2),
            RGB::from_u8(bg.0, bg.1, bg.2),
        )
    }
}

/// The player entity's color.
pub const PLAYER: Pallet = Pallet(rltk::YELLOW, DEFAULT_BG_COLOR);

/// The goblin entity's color.
pub const GOBLIN: Pallet = Pallet((169, 169, 169), DEFAULT_BG_COLOR);

/// The gremlin entity's color.
pub const GREMLIN: Pallet = Pallet((124, 252, 0), DEFAULT_BG_COLOR);

/// The floor tile's color.
pub const FLOOR: Pallet = Pallet((141, 163, 153), DEFAULT_BG_COLOR);

/// The wall tile's color.
pub const WALL: Pallet = Pallet((220, 206, 26), DEFAULT_BG_COLOR);

/// The color for the message box ui.
pub const MESSAGE_BOX: Pallet = Pallet(rltk::WHITE, DEFAULT_BG_COLOR);

/// The color for the player's health text on the ui.
pub const PLAYER_HEALTH_TEXT: Pallet = Pallet(rltk::YELLOW, DEFAULT_BG_COLOR);

/// The color for the player's health bar.
pub const PLAYER_HEALTH_BAR: Pallet = Pallet(rltk::RED, DEFAULT_BG_COLOR);

/// The color of the mouse cursor tile.
pub const MOUSE_CURSOR: U8Color = (220, 206, 26);

/// Color for the tooltips.
pub const TOOLTIP: Pallet = Pallet(rltk::WHITE, rltk::DARKRED);