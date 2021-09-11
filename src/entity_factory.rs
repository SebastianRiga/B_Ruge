//! Factory to create entities

use specs::prelude::*;

use super::{Collision, Monster, Name, Player, Position, Renderable, Statistics, FOV, swatch};

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
        let (fg, bg) = swatch::PLAYER.colors();

        ecs.create_entity()
            .with(Position {
                x: position.x,
                y: position.y,
            })
            .with(Renderable {
                symbol: rltk::to_cp437('@'),
                fg,
                bg,
            })
            .with(Player {})
            .with(FOV {
                content: Vec::new(),
                range: 8,
                is_dirty: true,
            })
            .with(Name {
                name: "Rouge".to_string(),
            })
            .with(Statistics {
                hp_max: 30,
                hp: 30,
                power: 5,
                defense: 2,
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
        let (fg, bg) = swatch::GOBLIN.colors();

        ecs.create_entity()
            .with(Position {
                x: position.x,
                y: position.y,
            })
            .with(Renderable {
                symbol: rltk::to_cp437('o'),
                fg,
                bg,
            })
            .with(FOV {
                content: Vec::new(),
                range: 8,
                is_dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{}{}", "Goblin", suffix.unwrap_or(String::default())),
            })
            .with(Collision {})
            .with(Statistics {
                hp_max: 10,
                hp: 10,
                power: 2,
                defense: 1,
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
        let (fg, bg) = swatch::GREMLIN.colors();

        ecs.create_entity()
            .with(Position {
                x: position.x,
                y: position.y,
            })
            .with(Renderable {
                symbol: rltk::to_cp437('g'),
                fg,
                bg,
            })
            .with(FOV {
                content: Vec::new(),
                range: 8,
                is_dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{}{}", "Gremlin", suffix.unwrap_or(String::default())),
            })
            .with(Collision {})
            .with(Statistics {
                hp_max: 16,
                hp: 16,
                power: 4,
                defense: 2,
            })
            .build()
    }
}
