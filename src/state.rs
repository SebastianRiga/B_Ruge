use rltk::{GameState, Rltk};
use specs::prelude::*;

use super::{player_handle_input, Position, Renderable};

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_handle_input(self, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.symbol)
        }
    }
}
