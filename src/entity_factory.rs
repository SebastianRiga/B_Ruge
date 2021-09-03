//! Factory to create entities

use rltk::RGB;
use specs::prelude::*;

use super::{Player, Position, Renderable, FOV, Monster, Name};

/// Factory to create instances of all
/// entities in the game.
pub struct EntityFactory {}

impl EntityFactory {
    /// Creates a new player entity through the `ecs`, puts it at
    /// the passed `position` and returns it.
    ///
    /// # Arguments
    /// * `position`: The x and y coordinates at which the player should be placed at.
    /// * `ecs`: The `ecs` through which the player should be created.
    ///
    pub fn new_player(position: &Position, ecs: &mut World) -> Entity {
        ecs.create_entity()
            .with(Position {
                x: position.x,
                y: position.y,
            })
            .with(Renderable {
                symbol: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .with(FOV {
                content: Vec::new(),
                range: 8,
                is_dirty: true,
            })
            .with(Name {
                name: "Rouge".to_string()
            })
            .build()
    }

    /// Creates a new goblin entity through the `ecs`, puts it at
    /// the passed `position` and returns it.
    ///
    /// # Arguments
    /// * `position`: The x and y coordinates at which the goblin should be placed at.
    /// * `ecs`: The `ecs` through which the goblin should be created.
    /// * `suffix`: Optinal suffix that can be added to the monsters name.
    ///
    pub fn new_goblin(position: &Position, ecs: &mut World, suffix: Option<String>) -> Entity {
        ecs.create_entity()
            .with(Position {
                x: position.x,
                y: position.y,
            })
            .with(Renderable {
                symbol: rltk::to_cp437('o'),
                fg: RGB::from_u8(169, 169, 169),
                bg: RGB::named(rltk::BLACK),
            })
            .with(FOV {
                content: Vec::new(),
                range: 8,
                is_dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{}{}", "Goblin", suffix.unwrap_or(String::default()))
            })
            .build()
    }

    /// Creates a new gremlin entity through the `ecs`, puts it at
    /// the passed `position` and returns it.
    ///
    /// # Arguments
    /// * `position`: The x and y coordinates at which the gremlin should be placed at.
    /// * `ecs`: The `ecs` through which the gremlin should be created.
    /// * `suffix`: Optinal suffix that can be added to the monsters name.
    ///
    pub fn new_gremlin(position: &Position, ecs: &mut World, suffix: Option<String>) -> Entity {
        ecs.create_entity()
            .with(Position {
                x: position.x,
                y: position.y,
            })
            .with(Renderable {
                symbol: rltk::to_cp437('g'),
                fg: RGB::from_u8(124, 252, 0),
                bg: RGB::named(rltk::BLACK),
            })
            .with(FOV {
                content: Vec::new(),
                range: 8,
                is_dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{}{}", "Gremlin", suffix.unwrap_or(String::default()))
            })
            .build()
    }
}
