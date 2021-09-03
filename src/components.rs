//! List of all components used in the game.

use rltk::{Point, RGB};
use specs::prelude::*;
use specs_derive::*;

/// Component to describe the position
/// of a game entity in the game.
#[derive(Component)]
pub struct Position {
    /// X coordinate of the entity.
    pub x: i32,

    /// Y coordinate of the entity.
    pub y: i32,
}

impl Position {
    /// Crates a new [Point] from the calling
    /// coordinate.
    pub fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

/// Component to describe the render
/// information of an entity.
#[derive(Component)]
pub struct Renderable {
    /// Font symbol of the entity.
    pub symbol: rltk::FontCharType,

    /// Foreground color of the entity.
    pub fg: RGB,

    /// Background color of the entity.
    pub bg: RGB,
}

/// Component for the player entity.
#[derive(Component, Debug)]
pub struct Player {}

/// Component for the field of view implementation.
#[derive(Component)]
pub struct FOV {
    /// Positions in the FOV.
    pub content: Vec<rltk::Point>,

    /// Range of the FOV.
    pub range: i32,

    /// Flag indicating if the
    /// [FOV] should be updated.
    pub is_dirty: bool,
}

/// Component for the monsters.
#[derive(Component, Debug)]
pub struct Monster {}

/// Component to name entities
#[derive(Component, Debug)]
pub struct Name {
    /// The name of the entity
    pub name: String
}
