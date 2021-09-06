//! Game state handling module

use rltk::{GameState, Rltk};
use specs::prelude::*;

use super::{
    player_handle_input, DamageSystem, FOVSystem, Map, MapDexSystem, MeleeCombatSystem, MonsterAI,
    Position, Renderable,
};

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

        let mut monster_ai = MonsterAI {};
        monster_ai.run_now(&self.ecs);

        let mut map_dex = MapDexSystem {};
        map_dex.run_now(&self.ecs);

        let mut melee_combat_system = MeleeCombatSystem {};
        melee_combat_system.run_now(&self.ecs);

        let mut damage_system = DamageSystem {};
        damage_system.run_now(&self.ecs);

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

        let mut next_processing_state: ProcessingState;
        {
            let current_processing_state = *self.ecs.fetch::<ProcessingState>();
            next_processing_state = current_processing_state;
        }

        match next_processing_state {
            ProcessingState::Internal => {
                self.run_systems();
                next_processing_state = ProcessingState::WaitingForInput;
            }
            ProcessingState::WaitingForInput => {
                next_processing_state = player_handle_input(self, ctx);
            }
            ProcessingState::PlayerTurn => {
                self.run_systems();
                next_processing_state = ProcessingState::MonsterTurn;
            }
            ProcessingState::MonsterTurn => {
                self.run_systems();
                next_processing_state = ProcessingState::Internal;
            }
        }

        {
            let mut run_writer = self.ecs.write_resource::<ProcessingState>();
            *run_writer = next_processing_state;
        }

        DamageSystem::clean_up(&mut self.ecs);

        // Create the map
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        // Get all entities with [Position] and [Renderable]
        // attributes and render them on the screen.
        let positions = self.ecs.read_storage::<Position>();
        let renderers = self.ecs.read_storage::<Renderable>();

        for (position, renderable) in (&positions, &renderers).join() {
            if map.is_tile_in_fov(position.x, position.y) {
                ctx.set(
                    position.x,
                    position.y,
                    renderable.fg,
                    renderable.bg,
                    renderable.symbol,
                )
            }
        }
    }
}

/// Enum describing all processing states
/// the game can be in during execution.
#[derive(PartialEq, Copy, Clone)]
pub enum ProcessingState {
    /// Executes all internal systems
    /// and functions before the game
    /// passes control to the player.
    Internal,

    /// The game is waiting for player
    /// input.
    WaitingForInput,

    /// Executes the action
    /// input by the player.
    PlayerTurn,

    /// Executes the monsters
    /// actions.
    MonsterTurn,
}
