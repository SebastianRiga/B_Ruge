//! Collection of functions for the player.

use std::cmp::{max, min};

use rltk::{a_star_search, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::{DialogInterface, DialogOption};

use super::{
    Map, MeleeAttack, Player, PlayerPathing, Position, ProcessingState, State, Statistics, FOV,
    config,
};

/// Moves the [Player] entity through its stored [Position]
/// in the `ecs` by adding the `delta_x` and `delta_y` to it.
///
/// # Arguments
///
/// * `delta_x`: The movement delta in x direction.
/// * `delta_y`: The movement delta in y direction.
/// * `ecs` : Mutable reference to the `ecs`.
///
/// # Note
/// If the coordinate the player tries to move to is out of
/// bounds or not walkable, the player wont be moved.
///  
pub fn player_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // Fetch map from ecs
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();

    // Write ecs storages
    let mut fovs = ecs.write_storage::<FOV>();
    let players = ecs.write_storage::<Player>();
    let mut positions = ecs.write_storage::<Position>();
    let mut melee_attacks = ecs.write_storage::<MeleeAttack>();
    let mut player_ecs_position = ecs.write_resource::<Point>();

    // Read ecs storages
    let statistics = ecs.read_storage::<Statistics>();

    for (entity, _, position, fov) in (&entities, &players, &mut positions, &mut fovs).join() {
        let new_position = Position {
            x: position.x + delta_x,
            y: position.y + delta_y,
        };

        for target in map.tile_contents_get(new_position.x, new_position.y).iter() {
            let enemy = statistics.get(*target);

            if let Some(_) = enemy {
                let attack = MeleeAttack { target: *target };

                melee_attacks.insert(entity, attack).expect(&format!(
                    "Adding melee attack from player agianst entity with id {} failed!",
                    entity.id()
                ));
            }
        }

        let is_new_position_blocked = map.is_tile_blocked(new_position.x, new_position.y);

        if !is_new_position_blocked {
            position.x = min(config::WINDOW_WIDTH - 1, max(0, new_position.x));
            position.y = min(config::WINDOW_HEIGHT - 1, max(0, new_position.y));

            player_ecs_position.x = position.x;
            player_ecs_position.y = position.y;

            fov.is_dirty = true;
        }
    }
}

fn player_moved_after_click(ecs: &mut World) -> Option<(i32, i32)> {
    let map = ecs.write_resource::<Map>();
    let player_ecs_position = ecs.write_resource::<Point>();
    let mut pathing_writer = ecs.write_resource::<PlayerPathing>();

    match pathing_writer.pop() {
        Some(idx) => {
            let (x, y) = map.idx_to_coordinates(idx);
            let delta_x = x - player_ecs_position.x;
            let delta_y = y - player_ecs_position.y;

            Some((delta_x, delta_y))
        }
        None => None,
    }
}

fn handle_new_click_to_move(ecs: &mut World, ctx: &Rltk) {
    let fovs = ecs.read_storage::<FOV>();
    let mut map = ecs.write_resource::<Map>();
    let player = ecs.read_resource::<Entity>();
    let player_ecs_position = ecs.write_resource::<Point>();
    let mut pathing_writer = ecs.write_resource::<PlayerPathing>();

    let mouse_position = ctx.mouse_point();

    let start_idx = map.coordinates_to_idx(player_ecs_position.x, player_ecs_position.y);
    let end_idx = map.coordinates_to_idx(mouse_position.x, mouse_position.y);

    let blocked_tiles = map.blocked_tiles.clone();
    map.refresh_blocked_tiles();

    let mut path = a_star_search(start_idx, end_idx, &mut *map);

    map.blocked_tiles = blocked_tiles;

    let fov = fovs.get(*player).unwrap();

    if path.success && path.steps.len() > 1 && fov.contains(&mouse_position) {
        path.steps.remove(0);
        path.steps.reverse();
        pathing_writer.update(&mut path.steps);
    }
}

/// Handles the [Player] movement through user input.
///
/// # Arguments
///
/// * `game_state`: Reference to the current state of the game for `ecs` access.
/// * `ctx`: Reference to the context of the `ecs` to read the key input.
///
pub fn player_handle_input(game_state: &mut State, ctx: &mut Rltk) -> ProcessingState {
    match player_moved_after_click(&mut game_state.ecs) {
        Some((delta_x, delta_y)) => {
            player_move(delta_x, delta_y, &mut game_state.ecs);
            return ProcessingState::PlayerTurn;
        }
        None => {}
    }

    match ctx.key {
        Some(key) => match key {
            // Cardinal directions
            VirtualKeyCode::W
            | VirtualKeyCode::Up
            | VirtualKeyCode::Numpad8
            | VirtualKeyCode::K => player_move(0, -1, &mut game_state.ecs),

            VirtualKeyCode::A
            | VirtualKeyCode::Left
            | VirtualKeyCode::Numpad4
            | VirtualKeyCode::H => player_move(-1, 0, &mut game_state.ecs),

            VirtualKeyCode::S
            | VirtualKeyCode::Down
            | VirtualKeyCode::Numpad2
            | VirtualKeyCode::J => player_move(0, 1, &mut game_state.ecs),

            VirtualKeyCode::D
            | VirtualKeyCode::Right
            | VirtualKeyCode::Numpad6
            | VirtualKeyCode::L => player_move(1, 0, &mut game_state.ecs),

            // Diagonal directions
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Q => player_move(-1, -1, &mut game_state.ecs),

            VirtualKeyCode::Numpad9 | VirtualKeyCode::E => player_move(1, -1, &mut game_state.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::Y => player_move(-1, 1, &mut game_state.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::X => player_move(1, 1, &mut game_state.ecs),

            VirtualKeyCode::Escape => {
                DialogInterface::register_dialog(
                    &mut game_state.ecs,
                    "Quit game?".to_string(),
                    "Are you sure you want to end your current run?".to_string(),
                    &[DialogOption {
                        description: "Quit".to_string(),
                        key: VirtualKeyCode::Y,
                        callback: |ctx| ctx.quit(),
                    }],
                );
            }

            _ => return ProcessingState::WaitingForInput,
        },
        None => {
            if ctx.left_click {
                handle_new_click_to_move(&mut game_state.ecs, ctx);
            }
            return ProcessingState::WaitingForInput;
        }
    }

    ProcessingState::PlayerTurn
}
