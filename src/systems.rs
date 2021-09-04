//! Module containing all systems of the game

use rltk::{a_star_search, console, field_of_view, Point};
use specs::prelude::*;

use super::{pythagoras_distance, Collision, Map, Monster, Name, Player, Position, FOV};

/// System that handles the field of view
/// processing. See the implementation below
/// for more details.
pub struct FOVSystem {}

impl<'a> System<'a> for FOVSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, FOV>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // Get the systems data
        let (mut map, entities, mut fovs, positions, players) = data;

        // Find the entities, fov system and positions.
        for (entity, fov, position) in (&entities, &mut fovs, &positions).join() {
            // If the [FOV] is dirty, calculate new
            if fov.is_dirty {
                // Invalidate [FOV] flag
                fov.mark_as_clean();

                // Recalculate the [FOV]
                fov.content.clear();
                fov.content = field_of_view(position.to_point(), fov.range, &*map);
                fov.content.retain(|pos| {
                    pos.x >= 0 && pos.x < map.width && pos.y >= 0 && pos.y < map.height
                });

                // Check if the entity is the [Player]
                let _player = players.get(entity);
                if let Some(_player) = _player {
                    // Clean map fov tiles
                    map.reset_tiles_in_fov();

                    // Set the tiles of all fields in the [FOV]
                    for explored_tile in fov.content.iter() {
                        map.set_explored_tile(explored_tile.x, explored_tile.y, true);
                        map.set_tile_in_fov(explored_tile.x, explored_tile.y, true);
                    }
                }
            }
        }
    }
}

/// Base AI system for all monsters.
pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, FOV>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // Get system data
        let (mut map, mut fovs, player_position, monsters, names, mut positions) = data;

        // Iterate through all monsters that have an fov
        for (fov, _monster, name, position) in (&mut fovs, &monsters, &names, &mut positions).join()
        {
            // If the fov of the monster contains the player
            // its AI is executed.
            if fov.content.contains(&*player_position) {
                let distance_to_player =
                    pythagoras_distance(&position.to_point(), &*player_position);

                if distance_to_player < 1.5 {
                    // TODO: Add attack here
                    console::log(&format!("{} growls aggressively at!", name.name));
                    return;
                }

                let monster_idx = map.coordinates_to_idx(position.x, position.y);
                let player_idx = map.coordinates_to_idx(player_position.x, player_position.y);

                // Calculate path for the monster to chase the player
                let path = a_star_search(monster_idx, player_idx, &mut *map);

                // If a path could successfully be calculated, update the monsters position
                // according to the new coordinates from the path.
                if path.success && path.steps.len() > 1 {
                    let next_position = map.idx_to_coordinates(path.steps[1]);
                    position.update_with_tuple(next_position);
                    fov.mark_as_dirty();
                }
            }
        }
    }
}

/// System updating the properties and tile attributes
/// of the game [Map].
pub struct MapSystem {}

impl<'a> System<'a> for MapSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collision>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, collisions) = data;

        map.refresh_blocked_tiles();

        for (position, _collision) in (&positions, &collisions).join() {
            map.set_tile_is_blocked(position.x, position.y, true);
        }
    }
}
