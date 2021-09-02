//! Game state handling module

use rltk::{GameState, Rltk};
use specs::prelude::*;

use super::{player_handle_input, Map, Position, Renderable, FOVSystem};

/// Struct describing the current state of the game
/// and providing access to the underlying `ECS`
/// system provided by [rltk].
pub struct State {
    /// Provides access to resource container [World],
    /// that makes up the `ECS`.
    pub ecs: World,
}

impl State {
    /// Exectue the systems of the game.
    fn run_systems(&mut self) {
        let mut fov_system = FOVSystem {};
        fov_system.run_now(&self.ecs);
        self.ecs.maintain();
    }
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

        // Execute the systems of the state.
        self.run_systems();

        // Read player input
        player_handle_input(self, ctx);

        // Create the map
        let map = self.ecs.fetch::<Map>();
        map.draw(&self.ecs, ctx);

        // Get all entities with [Postion] and [Renderable]
        // attributes and render them on the screen.
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.symbol)
        }
    }
}
