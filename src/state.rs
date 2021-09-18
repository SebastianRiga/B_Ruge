//! Game state handling module

/// TODO: Finish documentation
use rltk::{GameState, Rltk};
use specs::prelude::*;

use super::{
    player_handle_input, ui_controller, DamageSystem, DialogInterface, DialogResult, FOVSystem,
    ItemCollectionSystem, Map, MapDexSystem, MeleeCombatSystem, MonsterAI, Position,
    PotionDrinkSystem, Renderable,
};
use crate::ItemDropSystem;

/// Struct describing the current state of the game
/// and providing access to the underlying `ECS`
/// system provided by [rltk].
pub struct State {
    /// Provides access to resource container [World],
    /// that makes up the `ECS`.
    pub ecs: World,
}

impl State {
    /// Execute the systems of the game.
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

        let mut item_collection_system = ItemCollectionSystem {};
        item_collection_system.run_now(&self.ecs);

        let mut potion_drink_system = PotionDrinkSystem {};
        potion_drink_system.run_now(&self.ecs);

        let mut item_drop_system = ItemDropSystem {};
        item_drop_system.run_now(&self.ecs);

        self.ecs.maintain();
    }

    fn get_current_processing_state(&self) -> ProcessingState {
        let has_dialog = self.ecs.has_value::<DialogInterface>();

        let next_processing_state: ProcessingState;
        {
            let current_processing_state = *self.ecs.fetch::<ProcessingState>();
            next_processing_state = if has_dialog {
                ProcessingState::Dialog
            } else {
                current_processing_state
            }
        }

        return next_processing_state;
    }

    /// Displays the ui of the game on the screen, this includes
    /// the map, message log, status information etc.
    ///
    /// # Arguments
    /// * `ctx`: The context in which the ui should be drawn.
    ///
    fn show_ui(&self, ctx: &mut Rltk) {
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        ui_controller::draw_ui(&self.ecs, ctx);

        // Get all entities with [Position] and [Renderable]
        // attributes and render them on the screen.
        let positions = self.ecs.read_storage::<Position>();
        let renderers = self.ecs.read_storage::<Renderable>();

        let mut entities = (&positions, &renderers).join().collect::<Vec<_>>();
        entities.sort_by(|&first, &second| second.1.order.cmp(&first.1.order));

        for (position, renderable) in entities.iter() {
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

    fn show_dialog(&mut self, ctx: &mut Rltk) -> DialogResult {
        let mut dialog = self.ecs.fetch_mut::<DialogInterface>();
        dialog.show(&self.ecs, ctx)
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

        let mut show_dialog = false;

        let mut next_processing_state = self.get_current_processing_state();

        match next_processing_state {
            ProcessingState::Dialog => {
                self.run_systems();
                self.ecs.maintain();
                show_dialog = true;
            }
            ProcessingState::Internal => {
                self.run_systems();
                self.ecs.maintain();
                next_processing_state = ProcessingState::WaitingForInput;
            }
            ProcessingState::WaitingForInput => {
                next_processing_state = player_handle_input(self, ctx);
            }
            ProcessingState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                next_processing_state = ProcessingState::MonsterTurn;
            }
            ProcessingState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                next_processing_state = ProcessingState::Internal;
            }
        }

        DamageSystem::clean_up(&mut self.ecs);

        // Standard render process
        self.show_ui(ctx);

        // If there is a dialog to display, show it and read the result
        if show_dialog {
            if self.show_dialog(ctx) == DialogResult::Consumed {
                self.ecs.remove::<DialogInterface>();
                next_processing_state = ProcessingState::Internal;
            }
        }

        // Update the processing state
        {
            let mut run_writer = self.ecs.write_resource::<ProcessingState>();
            *run_writer = next_processing_state;
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

    /// The is displaying a dialog and
    /// is waiting for a key press for
    /// current dialog.
    Dialog,

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
