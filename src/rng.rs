//! Module for random number generation

use chrono::Utc;
use rltk::{console, DiceType, RandomNumberGenerator};
use specs::prelude::*;

pub fn register(ecs: &mut World) {
    let seed = Utc::now().timestamp_nanos() as u64;
    let rng = RandomNumberGenerator::seeded(seed);

    console::log(&format!("Game running with seed: {}", seed));

    ecs.insert(rng);
}

pub fn roll(ecs: &mut World, dice: DiceType) -> i32 {
    if is_registered(ecs) {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        return rng.roll(dice);
    }
    panic!("Called 'roll_dice' function of module rng without registering it with the ecs!");
}

pub fn roll_dice(ecs: &mut World, n: i32, die_type: i32) -> i32 {
    if is_registered(ecs) {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        return rng.roll_dice(n, die_type);
    }
    panic!("Called 'roll_dice' function of module rng without registering it with the ecs!");
}

pub fn range(ecs: &mut World, start: i32, end: i32) -> i32 {
    if is_registered(ecs) {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        return rng.range(start, end);
    }
    panic!("Called 'roll_dice' function of module rng without registering it with the ecs!");
}

fn is_registered(ecs: &mut World) -> bool {
    ecs.has_value::<RandomNumberGenerator>()
}
