//! Module containing all systems of the game

/// TODO: Add inline documentation for system executions
use rltk::{a_star_search, console, field_of_view, Point, VirtualKeyCode};
use specs::prelude::*;

use super::{
    exceptions, pythagoras_distance, Collision, DamageCounter, DialogInterface, DialogOption,
    DropItem, GameLog, HealingEffect, Loot, Map, MeleeAttack, Monster, Name, PickupItem, Player,
    Position, ProcessingState, Statistics, UseItem, FOV
};
use crate::{Consumable, DamageEffect, SoundEffect};

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
        // Entities
        Entities<'a>,
        // Read resources
        WriteExpect<'a, Map>,            // Read the game map from the ecs
        ReadExpect<'a, Point>,           // Read the player position from the ecs
        ReadExpect<'a, Entity>,          // Read the player entity form the ecs
        ReadExpect<'a, ProcessingState>, // Get the current processing state of the game
        // Read storages
        ReadStorage<'a, Monster>, // Get all monster components
        // Write storages
        WriteStorage<'a, FOV>,         // Get all fov components
        WriteStorage<'a, Position>,    // Get all position components
        WriteStorage<'a, MeleeAttack>, // Get all melee attacker components
    );

    fn run(&mut self, data: Self::SystemData) {
        // Get system data
        let (
            entities,
            mut map,
            player_position,
            player_entity,
            processing_state,
            monsters,
            mut fovs,
            mut positions,
            mut melee_attacks,
        ) = data;

        if *processing_state != ProcessingState::MonsterTurn {
            return;
        }

        // Iterate through all monsters that have an fov
        for (entity, fov, _monster, position) in
            (&entities, &mut fovs, &monsters, &mut positions).join()
        {
            let distance_to_player = pythagoras_distance(&position.to_point(), &*player_position);

            if distance_to_player < 1.5 {
                let melee_attack = MeleeAttack {
                    target: *player_entity,
                };

                let error_message = exceptions::get_add_melee_damage_error_message(&entity);

                melee_attacks
                    .insert(entity, melee_attack)
                    .expect(&error_message);

                return;
            }

            // If the fov of the monster contains the player
            // its AI is executed.
            if fov.content.contains(&*player_position) {
                let monster_idx = map.coordinates_to_idx(position.x, position.y);
                let player_idx = map.coordinates_to_idx(player_position.x, player_position.y);

                // Calculate path for the monster to chase the player
                let path = a_star_search(monster_idx, player_idx, &mut *map);

                // If a path could successfully be calculated, update the monsters position
                // according to the new coordinates from the path.
                if path.success && path.steps.len() > 1 {
                    // Unblock old tile for the remaining monsters in the loop
                    map.set_tile_is_blocked(position.x, position.y, false);

                    // Calculate the next position the monster will move to
                    let next_position = map.idx_to_coordinates(path.steps[1]);

                    // Update the monster position
                    position.update_with_tuple(next_position);

                    // Block the tile the monster has walked to
                    map.set_tile_is_blocked(next_position.0, next_position.1, true);

                    // Mark the fov of the monster as dirty so it can be recalculated for the monster
                    fov.mark_as_dirty();
                }
            }
        }
    }
}

/// System updating the properties and tile attributes
/// of the game [Map].
pub struct MapDexSystem {}

impl<'a> System<'a> for MapDexSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collision>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, positions, collisions) = data;

        // Clear all tile contents and remove all blocked entries
        map.clear_tile_contents();
        map.refresh_blocked_tiles();

        // Iterate through all entities that have a position
        for (position, entity) in (&positions, &entities).join() {
            let collision = collisions.get(entity);

            // Refresh blocked tiles if the entity has collision
            if let Some(_) = collision {
                map.set_tile_is_blocked(position.x, position.y, true);
            }

            // Refresh tile contents by adding the entity
            map.tile_contents_push(position.x, position.y, entity);
        }
    }
}

/// System to handle melee combat interactions.
pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, MeleeAttack>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Statistics>,
        WriteStorage<'a, DamageCounter>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut game_log, mut attackers, names, statistics, mut damage_counter) = data;

        for (_, attacker, name, statistic) in (&entities, &attackers, &names, &statistics).join() {
            if statistic.hp > 0 {
                let target = attacker.target;

                let target_statistics = statistics.get(target).unwrap();

                if target_statistics.hp > 0 {
                    let target_name = names.get(target).unwrap();

                    let damage = i32::max(0, statistic.power - target_statistics.defense);

                    if damage == 0 {
                        game_log.messages_push(&format!(
                            "{} was unable to break {}'s defenses",
                            &name.name, &target_name.name
                        ));
                    } else {
                        game_log.messages_push(&format!(
                            "{} hits {} for {} damage!",
                            &name.name, &target_name.name, damage
                        ));
                        DamageCounter::add_damage_taken(&mut damage_counter, target, damage);
                    }
                }
            }
        }

        attackers.clear();
    }
}

/// System that takes all the damage inflicted to an entity,
/// adds up the damage and subtracts it from the entities
/// health.
pub struct DamageSystem {}

impl DamageSystem {
    /// Removes all entities which have been defeated in the last executed turn
    /// from the `ecs`.
    ///
    /// # Arguments
    /// * `ecs`: The [World] from which the defeated entities should be removed.
    ///
    pub fn clean_up(ecs: &mut World) {
        let mut defeated_entities: Vec<Entity> = Vec::new();
        let mut player_died = false;

        {
            let entities = ecs.entities();
            let names = ecs.read_storage::<Name>();
            let players = ecs.read_storage::<Player>();
            let mut game_log = ecs.write_resource::<GameLog>();
            let statistics = ecs.read_storage::<Statistics>();

            for (entity, statistic) in (&entities, &statistics).join() {
                if statistic.hp < 1 {
                    let player = players.get(entity);

                    if let Some(_) = player {
                        let player_name = names.get(entity).unwrap();
                        console::log(&format!("Player {} has died!", player_name.name));
                        player_died = true;
                    }

                    let monster_name = names.get(entity);

                    if let Some(name) = monster_name {
                        defeated_entities.push(entity);
                        game_log.messages_push(&format!("{} has died", name.name));
                    }
                }
            }
        }

        if player_died {
            DialogInterface::register_dialog(
                ecs,
                "An untimely end".to_string(),
                Some(
                    "You have died while exploring the dungeon! Restart the game and try again."
                        .to_string(),
                ),
                vec![DialogOption {
                    description: "Quit the game".to_string(),
                    key: VirtualKeyCode::Q,
                    args: vec![],
                    callback: Box::new(|_, ctx, _| {
                        ctx.quit();
                        ProcessingState::Internal
                    }),
                }],
                false,
            )
        }

        ecs.delete_entities(&defeated_entities)
            .expect("Unable to clean up defeated entities!");
    }
}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, Statistics>,
        WriteStorage<'a, DamageCounter>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut statistics, mut damage_counters) = data;

        for (mut statistic, damage_counter) in (&mut statistics, &damage_counters).join() {
            statistic.hp -= damage_counter.damage_values.iter().sum::<i32>();
        }

        damage_counters.clear();
    }
}

/// System that handles the `PickupItem` requests of all
/// [Entity] objects and adds the corresponding Item to their
/// inventory by registering a respective [Loot] component.
pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, PickupItem>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Loot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_log, names, mut pickups, mut positions, mut backpack) = data;

        for pickup in pickups.join() {
            positions.remove(pickup.item);

            let loot = Loot {
                owner: pickup.collector,
            };

            backpack
                .insert(pickup.item, loot)
                .expect("Unable to insert collectable into backpack!");

            let collector_name = names.get(pickup.collector).unwrap();
            let item_name = names.get(pickup.item).unwrap();
            let message = format!("{} picked up {}.", collector_name.name, item_name.name);

            game_log.messages_push(&message);
        }

        pickups.clear();
    }
}

/// System that handles [DropItem] requests
/// of all [Entity] objects and removes the
/// corresponding `Item` from their inventory
/// and set it [Position] to render it on the map.
pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Loot>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, DropItem>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut game_log, names, mut loot, mut positions, mut drops) = data;

        for (entity, drop) in (&entities, &drops).join() {
            let entity_position = positions.get(entity).unwrap();

            let drop_position = Position {
                x: entity_position.x,
                y: entity_position.y,
            };

            positions.insert(drop.item, drop_position).expect("");
            loot.remove(drop.item);

            let entity_name = &names.get(entity).unwrap().name;
            let item_name = &names.get(drop.item).unwrap().name;

            let log_message = format!("{} drops {}", entity_name, item_name);

            game_log.messages_push(&log_message);
        }

        drops.clear();
    }
}

/// System used for processing [UseItem] requests in
/// the `ecs`.
pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, HealingEffect>,
        ReadStorage<'a, DamageEffect>,
        ReadStorage<'a, SoundEffect>,
        WriteStorage<'a, UseItem>,
        WriteStorage<'a, Statistics>,
        WriteStorage<'a, DamageCounter>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            map,
            mut game_log,
            names,
            consumables,
            healing_effects,
            damage_effects,
            sound_effects,
            mut use_items,
            mut statistics,
            mut damage_counters,
        ) = data;

        for (entity, use_item_request, statistic) in (&entities, &use_items, &mut statistics).join()
        {
            let user_name = names.get(entity).expect("");
            let item_name = names.get(use_item_request.item).expect("");

            // Check if the item has an associated sound effect
            if let Some(sound_effect) = sound_effects.get(use_item_request.item) {
                sound_effect.play();
            }

            // Check if the used item is a healing item

            if let Some(healing_effect) = healing_effects.get(use_item_request.item) {
                statistic.hp = i32::min(
                    statistic.hp_max,
                    statistic.hp + healing_effect.healing_amount,
                );

                let message = format!(
                    "{} uses the {}, restoring {} health.",
                    user_name.name, item_name.name, healing_effect.healing_amount
                );
                game_log.messages_push(&message);
            }

            // Check if the item is inflicting damage to a target

            if let Some(damage_effect) = damage_effects.get(use_item_request.item) {
                let target_point = use_item_request.target.expect("");
                let hit_entities = map.tile_contents_get(target_point.x, target_point.y);

                if hit_entities.is_empty() {
                    let message = format!(
                        "{} uses {}, but hits nothing.",
                        user_name.name, item_name.name,
                    );

                    game_log.messages_push(&message);
                } else {
                    for target in hit_entities.iter() {
                        let target_name = names.get(*target).expect("");
                        DamageCounter::add_damage_taken(
                            &mut damage_counters,
                            *target,
                            damage_effect.damage_amount,
                        );

                        let message = format!(
                            "{} uses {}, hitting {} for {} damage.",
                            user_name.name,
                            item_name.name,
                            target_name.name,
                            damage_effect.damage_amount
                        );

                        game_log.messages_push(&message);
                    }
                }
            }

            // Check if the used item is a consumable

            if let Some(_) = consumables.get(use_item_request.item) {
                entities.delete(use_item_request.item).expect(&format!(
                    "Unable to delete potion with entity id {} after usage.",
                    use_item_request.item.id()
                ));
            }
        }

        use_items.clear();
    }
}
