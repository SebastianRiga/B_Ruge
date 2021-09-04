#![warn(missing_docs)]

//! Main entry point of the game.

use rltk::{console, RltkBuilder};
use specs::prelude::*;

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

mod config;
pub use config::*;

mod systems;
pub use systems::*;

mod entity_factory;
pub use entity_factory::*;

mod tile_factory;
pub use tile_factory::*;

mod functions;
pub use functions::*;

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
    let mut context = RltkBuilder::simple80x50()
        .with_title(GAME_CONFIG.name)
        .build()?;

    // Create the initial game state
    let mut game_state = State {
        ecs: World::new(),
        processing_state: ProcessingState::RUNNING,
    };

    // Enable scan lines for the nostalgic feel.
    // TODO: Need to find a possibility to insert custom shaders.
    context.with_post_scanlines(true);

    // Register components
    game_state.ecs.register::<FOV>();
    game_state.ecs.register::<Name>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Monster>();
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Collision>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Statistics>();

    // Create the game map
    let map = Map::new_map_with_rooms(GAME_CONFIG.window_width, GAME_CONFIG.window_height);

    // Get a new random number generator for monster placement
    let mut rng = rltk::RandomNumberGenerator::new();

    // Apply the monster creation to all rooms expect for the first.
    // The rng is used to choose a random monster to place
    map.apply_to_rooms_skip(1, |idx, room| match rng.roll_dice(1, 2) {
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
    EntityFactory::new_player(&player_position, &mut game_state.ecs);

    // Insert the game resources into the ecs
    game_state.ecs.insert(map);
    game_state.ecs.insert(player_position.to_point());

    // Start the main loop
    rltk::main_loop(context, game_state)
}
