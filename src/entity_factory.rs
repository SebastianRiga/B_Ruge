//! Factory to create entities

use specs::prelude::*;

use super::{
    r, rng, swatch, Collision, Consumable, HealingEffect, Item, Monster, Name, Player, Position,
    Renderable, Statistics, FOV,
};
use crate::{DamageEffect, Ranged, SoundEffect};

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

// --------------------------------------- Monsters ------------------------------------------------

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
        name: format!("{}{}", "Goblin", suffix.unwrap_or_default()),
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
        order: 1,
    };

    let statistic = Statistics {
        hp_max: 16,
        hp: 16,
        power: 4,
        defense: 2,
    };

    new_monster(ecs, name, renderable, statistic, position)
}

/// Creates a random monster in the `ecs` at the passed `position`.
///
/// * Arguments
/// * `ecs`: The [World] in which the monster should be created.
/// * `position`: The [Position] at which the monster should be placed.
///
pub fn random_monster(ecs: &mut World, position: Position) -> Entity {
    let generators = [new_goblin, new_gremlin];
    let upper_bound = generators.len() as i32;

    let index = rng::range(ecs, 0, upper_bound) as usize;

    (generators[index])(ecs, position, None)
}

/// Creates a new monster in the passed `ecs` and attaches the supplied
/// `name`, `renderable`, `statistic` and `position` components.
///
/// # Arguments
/// * `ecs`: The [World] the monster should be added to.
/// * `name`: The [Name] of the monster.
/// * `renderable`: The [Renderable] information of the monster.
/// * `statistic`: The `Statistic` data of the monster for battle.
/// * `position`: The [Position] of the monster in the world.
///
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

// ----------------------------------------- Items -------------------------------------------------

/// Creates a new `Potion` entity at the supplied `position` in the passed `ecs`.
///
/// # Arguments
/// * `ecs`: The [World] in which the `potion` should be created.
/// * `position`: The [Position] at which the potion should be placed.
///
pub fn new_potion_health(ecs: &mut World, position: Position) -> Entity {
    let (fg, bg) = swatch::POTION_HEALTH.colors();

    ecs.create_entity()
        .with(position)
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Renderable {
            symbol: rltk::to_cp437('!'),
            fg,
            bg,
            order: 2,
        })
        .with(Item {})
        .with(Consumable {})
        .with(HealingEffect { healing_amount: 8 })
        .with(SoundEffect {
            resource: r::sfx::DRINKING_WAV.to_string(),
        })
        .build()
}

pub fn new_scroll_magic_missile(ecs: &mut World, position: Position) -> Entity {
    let (fg, bg) = swatch::SCROLL_MAGIC_MISSILE.colors();

    ecs.create_entity()
        .with(position)
        .with(Name {
            name: "Scroll of Magic Missile".to_string(),
        })
        .with(Renderable {
            symbol: rltk::to_cp437('?'),
            fg,
            bg,
            order: 2,
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(DamageEffect { damage_amount: 16 })
        .with(SoundEffect {
            resource: r::sfx::MAGIC_MISSILES_WAV.to_string(),
        })
        .build()
}

pub fn random_item(ecs: &mut World, position: Position) {
    let generators = [new_potion_health, new_scroll_magic_missile];
    let upper_bound = generators.len() as i32;

    let index = rng::range(ecs, 0, upper_bound) as usize;

    (generators[index])(ecs, position);
}
