use rltk::{Rltk, GameState, RltkBuilder};

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    
    let context = RltkBuilder::simple80x50()
    .with_title("Bionic_Rouge")
    .build()?;

    let game_state = State {};

    rltk::main_loop(context, game_state)
}
