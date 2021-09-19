//! Module for color management

use rltk::RGB;

/// The default background color for entities and tiles.
pub const DEFAULT_BG_COLOR: (u8, u8, u8) = (0, 0, 0);

/// Type alias for an `rgb` color `tuple`.
type U8Color = (u8, u8, u8);

/// A struct describing the foreground and
/// background color of an entity or tile.
pub struct Pallet(pub U8Color, pub U8Color);

impl Pallet {
    /// Transforms the [Pallet]'s foreground
    /// and background [U8Color] tuples to
    /// [RGB] structs and returns them in a tuple
    /// in the same order of `(fg, bg)`.
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
pub const PLAYER: Pallet = Pallet(rltk::GOLD, DEFAULT_BG_COLOR);

/// The goblin entity's color.
pub const GOBLIN: Pallet = Pallet((169, 169, 169), DEFAULT_BG_COLOR);

/// The gremlin entity's color.
pub const GREMLIN: Pallet = Pallet((124, 252, 0), DEFAULT_BG_COLOR);

/// The floor tile's color.
pub const FLOOR: Pallet = Pallet((141, 163, 153), DEFAULT_BG_COLOR);

/// The wall tile's color.
pub const WALL: Pallet = Pallet(rltk::GOLDENROD, DEFAULT_BG_COLOR);

/// The color for the message box ui.
pub const MESSAGE_BOX: Pallet = Pallet(rltk::WHITE, DEFAULT_BG_COLOR);

/// The color for the player's health text on the ui.
pub const PLAYER_HEALTH_TEXT: Pallet = Pallet(rltk::GOLD, DEFAULT_BG_COLOR);

/// The color for the player's health bar.
pub const PLAYER_HEALTH_BAR: Pallet = Pallet(rltk::RED, DEFAULT_BG_COLOR);

/// The color of the mouse cursor tile.
pub const MOUSE_CURSOR: U8Color = rltk::GOLD;

/// Color for the tooltips.
pub const TOOLTIP: Pallet = Pallet(rltk::WHITE, rltk::DARKRED);

/// Color pallet for the health potion item.
pub const HEALTH_POTION: Pallet = Pallet(rltk::CRIMSON, DEFAULT_BG_COLOR);

/// The color pallet for dialog titles.
pub const DIALOG_TITLE: Pallet = Pallet(rltk::WHITE, DEFAULT_BG_COLOR);

/// The color pallet for dialog dismiss/cancel buttons.
pub const DIALOG_DISMISS_BUTTON: Pallet = Pallet(rltk::WHITE, DEFAULT_BG_COLOR);

/// The color pallet for dialog frames.
pub const DIALOG_FRAME: Pallet = Pallet(rltk::WHITE, DEFAULT_BG_COLOR);

/// Color pallet for dialog options.
pub const DIALOG_OPTION: Pallet = Pallet(rltk::LIGHT_GOLDENROD, DEFAULT_BG_COLOR);
