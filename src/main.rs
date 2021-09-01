#![warn(missing_docs)]

//! Main entry point of the game.

use rltk::{RltkBuilder, RGB};
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

mod scribbles;
pub use scribbles::*;

/// Bootstraps the game, registers components, initiates systems,
/// creates entities and starts the rendering. After the bootstrapping
/// it calls the [rltk::main_loop] to display the game window.
fn main() -> rltk::BError {
    let mut context = RltkBuilder::simple80x50()
        .with_title(GAME_CONFIG.name)
        .build()?;

    let mut game_state = State { ecs: World::new() };

    context.with_post_scanlines(true);

    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();

    game_state
        .ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            symbol: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    let map = Map::new(GAME_CONFIG.window_width, GAME_CONFIG.window_height);
    game_state.ecs.insert(map);

    rltk::main_loop(context, game_state)
}
