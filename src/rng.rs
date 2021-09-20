//! Module for random number generation

use chrono::Utc;
use rltk::{console, RandomNumberGenerator};
use specs::prelude::*;

/// Registers a the `rng` handler with the passed `ecs`.
///
/// # Arguments
/// * `ecs`: The [World] in which the `rng` handler should be registered.
///
/// Notes
/// * The seed for the `rng` handler is calculated through the current
/// system time in nanoseconds.
/// * This action must be performed befor any other function from the module
/// can be safely called! If no handler is registered all other functions
/// will panic!
///
pub fn register(ecs: &mut World) {
    let seed = Utc::now().timestamp_nanos() as u64;
    let rng = RandomNumberGenerator::seeded(seed);

    console::log(&format!("Game running with seed: {}", seed));

    ecs.insert(rng);
}

/// Rolls dice, using the classic 3d6 type.
///
/// # Arguments
/// * `ecs`: The [World] with which the `rng` handler was registered.
/// * `n`: The number of dice
/// * `die_type`: Size of the dice / Amount of sides.
///
/// # Panics
/// * If no `rng` handler is registered in the passed `ecs`.
///
/// # See also
/// * [register]
///
pub fn roll_dice(ecs: &mut World, n: i32, die_type: i32) -> i32 {
    if is_registered(ecs) {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        return rng.roll_dice(n, die_type);
    }
    panic!("Called 'roll_dice' function of module rng without registering it with the ecs!");
}

/// Returns a random number in the range from `start` to `end`.
/// The range includes the `start` parameter but excludes the `end`
/// parameter!
///
/// # Arguments
/// * `ecs`: The [World] in which the `rng` handler is registered.
/// * `start`: The start of the range from which the random number should be picked (Inclusive!).
/// * `end`: The end of the range from which the random number should be picked (Exclusive!).
///
/// # Panics
/// * If no `rng` handler is registered in the
///
pub fn range(ecs: &mut World, start: i32, end: i32) -> i32 {
    if is_registered(ecs) {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        return rng.range(start, end);
    }
    panic!("Called 'roll_dice' function of module rng without registering it with the ecs!");
}

/// Shorthand convenience function that returns `true` if
/// a `rng` handler is registered with the passed `ecs`.
///
/// # Arguments
/// * `ecs`: The [World] to check for a registered `rng` handler.
///
fn is_registered(ecs: &mut World) -> bool {
    ecs.has_value::<RandomNumberGenerator>()
}
