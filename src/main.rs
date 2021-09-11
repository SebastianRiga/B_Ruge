#![warn(missing_docs)]

//! D&D and NetHack inspired dungeon crawler written in rust.

use rltk::{console, RltkBuilder};
use specs::prelude::*;

mod ui;
mod config;
mod swatch;

mod state;
pub use state::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod rectangle;
pub use rectangle::Rectangle;

mod map;
pub use map::*;

mod systems;
pub use systems::*;

mod entity_factory;
pub use entity_factory::*;

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

/// Bootstraps the game, registers components, initiates systems,
/// creates entities and starts the rendering. After the bootstrapping
/// it calls the [rltk::main_loop] to display the game window.
fn main() -> rltk::BError {
    console::log(
        r#"
             :::::::::            :::::::::  :::    :::  ::::::::  :::::::::: 
             :+:    :+:           :+:    :+: :+:    :+: :+:    :+: :+:         
            +:+    +:+           +:+    +:+ +:+    +:+ +:+        +:+          
           +#++:++#+            +#++:++#:  +#+    +:+ :#:        +#++:++#      
          +#+    +#+           +#+    +#+ +#+    +#+ +#+   +#+# +#+            
         #+#    #+#           #+#    #+# #+#    #+# #+#    #+# #+#             
        ######### ########## ###    ###  ########   ########  ##########   
        
        by Sebastian Riga (c) 2021
        version: 0.2.5
        "#,
    );

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

    // Register components
    game_state.ecs.register::<FOV>();
    game_state.ecs.register::<Name>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Monster>();
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Collision>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Statistics>();
    game_state.ecs.register::<MeleeAttack>();
    game_state.ecs.register::<DamageCounter>();

    // Create the game map
    let map = Map::new_map_with_rooms(config::MAP_WIDTH, config::MAP_HEIGHT);

    // Get a new random number generator for monster placement
    let mut rng = rltk::RandomNumberGenerator::new();

    // Apply the monster creation to all rooms expect for the first.
    // The rng is used to choose a random monster to place
    map.rooms_for_each_skip(1, |idx, room| match rng.roll_dice(1, 2) {
        1 => {
            EntityFactory::new_goblin(
                &room.center(),
                &mut game_state.ecs,
                Option::from(format!("#{}", idx)),
            );
        }
        _ => {
            EntityFactory::new_gremlin(
                &room.center(),
                &mut game_state.ecs,
                Option::from(format!("#{}", idx)),
            );
        }
    });

    // The player is placed in the center of the first room
    let player_position = map.rooms[0].center();

    // Create the player
    let player_entity = EntityFactory::new_player(&player_position, &mut game_state.ecs);

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

    // Start the main loop
    rltk::main_loop(terminal, game_state)
}
