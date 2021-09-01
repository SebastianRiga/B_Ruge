use rltk::{GameState, Rltk};
use specs::prelude::*;

use super::{player_handle_input, Map, Position, Renderable};

/// Struct describing the current state of the game
/// and providing access to the underlying `ECS`
/// system provided by [rltk].
pub struct State {
    /// Provides access to resource container [World],
    /// that makes up the `ECS`.
    pub ecs: World,
}

impl GameState for State {
    /// Gets called every frame of the game.
    /// Used to  execute render logic, executes systems
    /// and handle inputs.
    /// 
    /// # Arguments
    /// * `ctx`: The [Rltk] context of the `ecs`.
    /// 
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear screen
        ctx.cls();

        // Read player input
        player_handle_input(self, ctx);

        // Create the map
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        // Get all entities with [Postion] and [Renderable]
        // attributes and render them on the screen.
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.symbol)
        }
    }
}
