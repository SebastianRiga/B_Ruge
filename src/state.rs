//! Game state handling module.

use rltk::{GameState, Rltk};
use specs::prelude::*;

use super::{
    player_handle_input, ui_controller, DamageSystem, DialogInterface, DialogResult, FOVSystem,
    ItemCollectionSystem, ItemDropSystem, ItemUseSystem, Map, MapDexSystem, MeleeCombatSystem,
    MonsterAI, Position, Renderable, TargetSelectionState, UseItem,
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

        let mut potion_drink_system = ItemUseSystem {};
        potion_drink_system.run_now(&self.ecs);

        let mut item_drop_system = ItemDropSystem {};
        item_drop_system.run_now(&self.ecs);

        self.ecs.maintain();
    }

    /// Returns the current [ProcessingState] of the
    /// system
    ///
    /// # Note
    /// * If a [DialogInterface] has been registered,
    /// the function always returns [ProcessingState::Dialog].
    fn get_processing_state(&self) -> ProcessingState {
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

    /// Updates the saved [ProcessingState] with the passed value,
    /// by writting it into the `ecs` resource.
    ///
    /// # Arguments
    /// * `new_processing_state`: The new [ProcessingState] of the system.
    ///
    fn update_processing_state(&self, new_processing_state: &ProcessingState) {
        let mut writer = self.ecs.write_resource::<ProcessingState>();
        *writer = *new_processing_state;
    }

    /// Displays the ui of the game on the screen, this includes
    /// the map, message log, status information etc.
    ///
    /// # Arguments
    /// * `ctx`: The context in which the ui should be drawn.
    ///
    fn show_ui(&self, ctx: &mut Rltk) {
        // Fetch the map from the ecs and draw it
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        // Draw base ui
        ui_controller::draw_ui(&self.ecs, ctx);

        // Get all entities with [Position] and [Renderable]
        // attributes and render them on the screen.
        let positions = self.ecs.read_storage::<Position>();
        let renderers = self.ecs.read_storage::<Renderable>();

        // Join get all renderables with a position and collect them in a vec for sorting
        let mut entities = (&positions, &renderers).join().collect::<Vec<_>>();

        // Sort all tuples by the render order set in the renderable
        entities.sort_by(|&first, &second| second.1.order.cmp(&first.1.order));

        // Render entities
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

    /// Fetches the currently saved dialog from the `ecs` and
    /// displays it.
    ///
    /// # Arguments
    /// * `ctx`: The [Rltk] context in which the dialog should be drawn.ui_controller
    ///
    /// # Panics
    /// * If no dialog is stored in the `ecs`.
    ///
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

        // Standard render process
        self.show_ui(ctx);

        let mut show_dialog = false;

        let mut next_processing_state = self.get_processing_state();

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
            ProcessingState::PlayerIsTargeting { range, entity } => {
                let result = ui_controller::draw_player_ranged_targeting(range, &self.ecs, ctx);

                next_processing_state = match result.0 {
                    TargetSelectionState::Cancel => ProcessingState::WaitingForInput,
                    TargetSelectionState::Selecting => {
                        ProcessingState::PlayerIsTargeting { range, entity }
                    }
                    TargetSelectionState::Selected => {
                        let player = self.ecs.fetch::<Entity>();

                        let usage = UseItem {
                            item: entity,
                            target: result.1,
                        };

                        let mut usage_intent = self.ecs.write_storage::<UseItem>();
                        usage_intent.insert(*player, usage).expect("");
                        ProcessingState::PlayerTurn
                    }
                }
            }
            ProcessingState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                next_processing_state = ProcessingState::Internal;
            }
        }

        // Remove all dead/defeated entities from the `ecs`
        DamageSystem::clean_up(&mut self.ecs);

        // Draw the tooltip as the top most ui element. (Only dialogs are higher!)
        ui_controller::draw_tooltips(&self.ecs, ctx);

        // If there is a dialog to display, show it and read the result
        if show_dialog {
            match self.show_dialog(ctx) {
                DialogResult::Consumed { next_state } => {
                    self.ecs.remove::<DialogInterface>();
                    next_processing_state = next_state;
                }
                DialogResult::Waiting => {}
            }
        }

        // Update the processing state
        self.update_processing_state(&next_processing_state);
    }
}

/// Enum describing all processing states
/// the game can be in during execution.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ProcessingState {
    /// Executes all internal systems
    /// and functions before the game
    /// passes control to the player.
    Internal,

    /// The system is displaying a dialog
    /// andis waiting for a key press on
    /// the current dialog.
    Dialog,

    /// The game is waiting for player
    /// input.
    WaitingForInput,

    /// Executes the action
    /// input by the player.
    PlayerTurn,

    /// The player is using a skill/spell/item that requires selecting
    /// a target.
    PlayerIsTargeting {
        /// The range of the skill/spell/item, the player
        /// is targeting with.
        range: i32,

        /// The [Entity] the player is using.
        entity: Entity,
    },

    /// Executes the monsters
    /// actions.
    MonsterTurn,
}
