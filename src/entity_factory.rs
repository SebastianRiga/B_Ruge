//! Factory to create entities

use specs::prelude::*;

use super::{
    rng, swatch, Collision, Item, Monster, Name, Player, Position, Potion, Renderable, Statistics,
    FOV,
};

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
            order: 0,
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
            defense: 3,
        })
        .build()
}

/// Creates a new goblin entity through the `ecs`, puts it at
/// the passed `position` and returns it.
///
/// # Arguments
/// * `position`: The x and y coordinates at which the goblin should be placed at.
/// * `ecs`: The `ecs` through which the goblin should be created.
/// * `suffix`: Optional suffix that can be added to the monsters name.
///
pub fn new_goblin(ecs: &mut World, position: Position, suffix: Option<String>) -> Entity {
    let name = Name {
        name: format!("{}{}", "Goblin", suffix.unwrap_or(String::default())),
    };

    let (fg, bg) = swatch::GOBLIN.colors();
    let renderable = Renderable {
        symbol: rltk::to_cp437('o'),
        fg,
        bg,
        order: 1,
    };

    let statistic = Statistics {
        hp_max: 10,
        hp: 10,
        power: 2,
        defense: 1,
    };

    new_monster(ecs, name, renderable, statistic, position)
}

/// Creates a new gremlin entity through the `ecs`, puts it at
/// the passed `position` and returns it.
///
/// # Arguments
/// * `position`: The x and y coordinates at which the gremlin should be placed at.
/// * `ecs`: The `ecs` through which the gremlin should be created.
/// * `suffix`: Optional suffix that can be added to the monsters name.
///
pub fn new_gremlin(ecs: &mut World, position: Position, suffix: Option<String>) -> Entity {
    let (fg, bg) = swatch::GREMLIN.colors();

    let name = Name {
        name: format!("{}{}", "Gremlin", suffix.unwrap_or(String::default())),
    };

    let renderable = Renderable {
        symbol: rltk::to_cp437('g'),
        fg,
        bg,
        order: 1
    };

    let statistic = Statistics {
        hp_max: 16,
        hp: 16,
        power: 4,
        defense: 2,
    };

    new_monster(ecs, name, renderable, statistic, position)
}

pub fn new_health_potion(ecs: &mut World, position: Position) -> Entity {
    let (fg, bg) = swatch::HEALTH_POTION.colors();

    ecs.create_entity()
        .with(position)
        .with(Renderable {
            symbol: rltk::to_cp437('!'),
            fg,
            bg,
            order: 2
        })
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Item {})
        .with(Potion { healing_amount: 8 })
        .build()
}

pub fn random_monster(ecs: &mut World, position: Position) -> Entity {
    let creator = [new_goblin, new_gremlin];
    let upper_bound = creator.len() as i32;

    let index = rng::range(ecs, 0, upper_bound) as usize;

    (creator[index])(ecs, position, None)
}

fn new_monster(
    ecs: &mut World,
    name: Name,
    renderable: Renderable,
    statistic: Statistics,
    position: Position,
) -> Entity {
    ecs.create_entity()
        .with(position)
        .with(renderable)
        .with(name)
        .with(statistic)
        .with(FOV {
            content: Vec::new(),
            range: 8,
            is_dirty: true,
        })
        .with(Monster {})
        .with(Collision {})
        .build()
}
