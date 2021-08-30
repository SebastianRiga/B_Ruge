#![warn(missing_docs)]

//! dsfasdf

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

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Bionic_Rouge")
        .build()?;

    let mut game_state = State { ecs: World::new() };

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

    rltk::main_loop(context, game_state)
}
