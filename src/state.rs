//! Game state handling module

use rltk::{GameState, Rltk};
use specs::prelude::*;

use super::{player_handle_input, FOVSystem, Map, Position, Renderable, MonsterAI};

/// Struct describing the current state of the game
/// and providing access to the underlying `ECS`
/// system provided by [rltk].
pub struct State {
    /// Provides access to resource container [World],
    /// that makes up the `ECS`.
    pub ecs: World,

    /// Controls the games execution state.
    pub processing_state: ProcessingState,
}

impl State {
    /// Exectue the systems of the game.
    fn run_systems(&mut self) {
        let mut fov_system = FOVSystem {};
        let mut monster_ai = MonsterAI {};

        fov_system.run_now(&self.ecs);
        monster_ai.run_now(&self.ecs);

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
        if self.processing_state == ProcessingState::RUNNING {
            self.run_systems();
            self.processing_state = ProcessingState::IDLE;
        } else {
                // Read player input to get next processing step
            self.processing_state = player_handle_input(self, ctx);
        }

        // Create the map
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        // Get all entities with [Postion] and [Renderable]
        // attributes and render them on the screen.
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (position, renderables) in (&positions, &renderables).join() {
            if map.is_tile_in_fov(position.x, position.y) {
                ctx.set(
                    position.x,
                    position.y,
                    renderables.fg,
                    renderables.bg,
                    renderables.symbol,
                )
            }
        }
    }
}

/// Enum describing all processing states
/// the game can be in during execution.
#[derive(PartialEq, Copy, Clone)]
pub enum ProcessingState {
    /// The game is ideling and waiting for input.
    /// No actions are performed during this state.
    IDLE,
    
    /// The game is actively executing systems and
    /// functions, e.g. the player has pressed a key
    /// or otherwise triggered an event.
    RUNNING
}
