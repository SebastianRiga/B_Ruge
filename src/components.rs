//! List of all components used in the game.

use rltk::RGB;
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
