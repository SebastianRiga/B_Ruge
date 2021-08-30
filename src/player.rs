use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use super::{Player, Position, State};

pub fn player_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

pub fn player_handle_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::W => player_move(0, -1, &mut gs.ecs),
            VirtualKeyCode::A => player_move(-1, 0, &mut gs.ecs),
            VirtualKeyCode::S => player_move(0, 1, &mut gs.ecs),
            VirtualKeyCode::D => player_move(1, 0, &mut gs.ecs),
            _ => {}
        },
    }
}
