//! Collection of functions for the player.

use rltk::{Rltk, VirtualKeyCode, Point};
use specs::prelude::*;
use std::cmp::{max, min};

use super::{Map, Player, Position, State, TileType, FOV, ProcessingState};

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
    let map = ecs.fetch::<Map>();
    let mut fovs = ecs.write_storage::<FOV>();
    let mut players = ecs.write_storage::<Player>();
    let mut positions = ecs.write_storage::<Position>();
    let mut player_ecs_position = ecs.write_resource::<Point>();

    for (_, position, fov) in (&mut players, &mut positions, &mut fovs).join() {
        let new_pos = map.get_tile(position.x + delta_x, position.y + delta_y);

        if new_pos != TileType::WALL {
            position.x = min(79, max(0, position.x + delta_x));
            position.y = min(49, max(0, position.y + delta_y));

            player_ecs_position.x = position.x;
            player_ecs_position.y = position.y;

            fov.is_dirty = true;
        }
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
    match ctx.key {
        None => { return ProcessingState::IDLE }
        Some(key) => match key {
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

            _ => { return ProcessingState::IDLE }
        },
    }
    ProcessingState::RUNNING
}
