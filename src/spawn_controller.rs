//! Module for spawning monsters, items and general entities.

use super::{config, entity_factory, rng, Position, Rectangle};
use specs::prelude::*;

/// Spawns monsters and items in the passed room [Rectangle],
/// based on the parameters set in the game's [config].
///
/// # Arguments
/// * `ecs`: The [World] in which the [Entity] structs will be saved.
/// * `room`: The room from the [Map] in which the monsters and items
/// should be spawned.
///
/// # See also
/// * [place_entities_in_room]
///
pub fn spawn_in_room(ecs: &mut World, room: &Rectangle) {
    let mut monster_spawn_positions: Vec<Position> = Vec::new();
    let mut item_spawn_positions: Vec<Position> = Vec::new();

    let monster_amount = rng::roll_dice(ecs, 1, config::MAX_MONSTERS_PER_ROOM + 2) - 3;
    let item_amount = rng::roll_dice(ecs, 1, config::MAX_ITEMS_PER_ROOM + 2) - 3;

    // Place monsters
    place_entities_in_room(ecs, monster_amount, room, &mut monster_spawn_positions);

    // Place items
    place_entities_in_room(ecs, item_amount, room, &mut item_spawn_positions);

    // Create monsters
    for position in monster_spawn_positions.iter().copied() {
        entity_factory::random_monster(ecs, position);
    }

    // Create items
    for position in item_spawn_positions.iter().copied() {
        entity_factory::new_health_potion(ecs, position);
    }
}

/// Convenience function that creates monster or item entities
/// in accordance to the passed `max_placement` parameter and
/// the positions which are already occupied by a monster as
/// indicated biy the container [Vec]
///
/// # Arguments
/// * `ecs`: The [World] in which the entities should be stored.
/// * `max_placements`: Maximum amount of entities that can be placed.
/// * `room`: Reference to the room [Rectangle] from the [Map], in which
/// the entities should be placed.
/// * `container`: [Vec] storing the spawn positions of the monsters.
///  
fn place_entities_in_room(
    ecs: &mut World,
    max_placements: i32,
    room: &Rectangle,
    container: &mut Vec<Position>,
) {
    for _ in 0..max_placements {
        let mut is_placed = false;

        while !is_placed {
            let x = room.left + rng::roll_dice(ecs, 1, i32::abs(room.right - room.left));
            let y = room.top + rng::roll_dice(ecs, 1, i32::abs(room.bottom - room.top));

            let position = Position { x, y };

            if !container.contains(&position) {
                container.push(position);
                is_placed = true;
            }
        }
    }
}
