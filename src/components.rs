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
    /// Creates a new [Position] with the `x` and `y` coordinates
    /// of the `tuple`. Tuple format: (`x`, `y`).
    pub fn new_from_tuple(tuple: (i32, i32)) -> Self {
        Position {
            x: tuple.0,
            y: tuple.1,
        }
    }

    /// Crates a new [Point] from the calling
    /// coordinate
    pub fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Updates the the fields of the position with the
    /// passed `x` and `y` coordinates.
    pub fn update(&mut self, x: i32, y: i32) -> &Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Updates the fields of the [Position] with the data from
    /// the tuple, where the tuple has the format `(x, y)`.
    pub fn update_with_tuple(&mut self, tuple: (i32, i32)) -> &Self {
        self.update(tuple.0, tuple.1)
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

impl FOV {
    /// Set the [FOV] to dirty, which triggers
    /// system execution in the next processing
    /// cycle.
    pub fn mark_as_dirty(&mut self) -> &Self {
        self.is_dirty = true;
        self
    }

    /// Mark the [FOV] as clean, meaning no further
    /// view processing for the associated entity is
    /// needed.
    pub fn mark_as_clean(&mut self) -> &Self {
        self.is_dirty = false;
        self
    }
}

/// Component for the monsters.
#[derive(Component, Debug)]
pub struct Monster {}

/// Component to name entities
#[derive(Component, Debug)]
pub struct Name {
    /// The name of the entity
    pub name: String,
}

/// Component that designates a an associated
/// entity as blocking, meaning it can't be walked
/// over.
#[derive(Component, Debug)]
pub struct Collision {}

/// Component describing the
/// combat stats of an entity.
#[derive(Component, Debug)]
pub struct Statistics {
    /// Maximum hp of the entity.
    pub hp_max: i32,
    
    /// Current hp of the entity.
    pub hp: i32,
    
    /// Attack power of the entity.
    pub power: i32,
    
    /// Defense capabilities of the entity.
    pub defense: i32,
}
