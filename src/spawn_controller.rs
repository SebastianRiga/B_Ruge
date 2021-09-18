//! Module for spawning monsters, items and general entities.

/// TODO: Add documentation

use super::{config, entity_factory, rng, Position, Rectangle};
use specs::prelude::*;

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
