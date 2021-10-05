// #![deny(warnings)]
#![warn(missing_docs)]

//! D&D and NetHack inspired dungeon crawler written in rust.

use rltk::RltkBuilder;
use specs::prelude::*;

mod config;
mod dialog_factory;
mod entity_factory;
mod exceptions;
mod rng;
mod spawn_controller;
mod swatch;
mod ui_controller;

mod audio;

mod state;
pub use state::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod rectangle;
pub use rectangle::*;

mod map;
pub use map::*;

mod systems;
pub use systems::*;

mod tile_factory;
pub use tile_factory::*;

mod functions;
pub use functions::*;

mod dialog;
pub use dialog::*;

mod data;
pub use data::*;

mod scribbles;
pub use scribbles::*;

mod res;

/// Bootstraps the game, registers components, initiates systems,
/// creates entities and starts the rendering. After the bootstrapping
/// it calls the [rltk::main_loop] to display the game window.
fn main() -> rltk::BError {
    config::log_starting_message();

    // Create a new terminal
    let mut terminal = RltkBuilder::simple(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)?
        .with_title(config::GAME_NAME)
        .with_fullscreen(false)
        .build()?;

    // Enable scan lines for the nostalgic feel.
    // TODO: Need to find a possibility to insert custom shaders.
    terminal.with_post_scanlines(true);

    // Create the initial game state
    let mut game_state = State { ecs: World::new() };

    // Register random number generator
    rng::register(&mut game_state.ecs);

    // Register components
    register_components(&mut game_state.ecs);

    // Create the game map
    let map = Map::new(&mut game_state.ecs, config::MAP_WIDTH, config::MAP_HEIGHT);

    // Apply the monster creation to all rooms expect for the first.
    // The rng is used to choose a random monster to place
    map.rooms_for_each_skip(1, |_, room| {
        spawn_controller::spawn_in_room(&mut game_state.ecs, room);
    });

    // The player is placed in the center of the first room
    let player_position = map.rooms[0].center();

    // Create the player
    let player_entity = entity_factory::new_player(&player_position, &mut game_state.ecs);

    // Create the games message logger
    let game_log = GameLog::new();

    // Create the player pathing object
    let player_pathing = PlayerPathing::new();

    // Insert the game resources into the ecs
    game_state.ecs.insert(map);
    game_state.ecs.insert(player_entity);
    game_state.ecs.insert(player_position.to_point());
    game_state.ecs.insert(game_log);
    game_state.ecs.insert(player_pathing);

    // Set the initial processing state of the game
    game_state.ecs.insert(ProcessingState::Internal);

    match audio::init() {
        Err(error) => println!(
            "An error occurred while initializing the audio controller: {}!",
            error
        ),
        Ok(_) => {
            audio::play_background(res::music::HELLISH_WAV);
            audio::play_ambiance(res::ambiance::DUNGEON01_WAV);
        }
    }

    // Start the main loop
    rltk::main_loop(terminal, game_state)
}
