use rltk::{field_of_view, console, Point};
use specs::prelude::*;

use super::{Map, Player, Position, FOV, Monster, Name};

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
                fov.is_dirty = false;

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

impl <'a>System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, FOV>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        // Get system data
        let (fovs, player_position, monsters, names) = data;

        // Iterate through all monsters that have an fov
        for (fov, _monster, name) in (&fovs, &monsters, &names).join() {
            // If the fov of the monster contains the player
            // its AI is executed.
            if fov.content.contains(&*player_position) {
                console::log(format!("{} growls aggressively!", name.name));
            }
        }
    }
}
