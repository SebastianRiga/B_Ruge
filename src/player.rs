//! Collection of functions for the player.

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use super::{Map, Player, Position, State, TileType};

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
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        if map.get(pos.x + delta_x, pos.y + delta_y) != TileType::WALL {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
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
pub fn player_handle_input(game_state: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::W => player_move(0, -1, &mut game_state.ecs),
            VirtualKeyCode::A => player_move(-1, 0, &mut game_state.ecs),
            VirtualKeyCode::S => player_move(0, 1, &mut game_state.ecs),
            VirtualKeyCode::D => player_move(1, 0, &mut game_state.ecs),
            _ => {}
        },
    }
}
