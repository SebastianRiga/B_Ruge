//! Collection of helper functions

use chrono::{Timelike, Utc};
use rltk::{DistanceAlg, Point, VirtualKeyCode};

/// Calculates the distance between the `start` and `end` point
/// using [DistanceAlg::Pythagoras] and returns the result.
///
/// # Arguments
/// * `start`: The starting point.
/// * `end`: The end point.
///
pub fn pythagoras_distance(start: &Point, end: &Point) -> f32 {
    DistanceAlg::Pythagoras.distance2d(*start, *end)
}

/// Returns teh current timestamp as a [String] in
/// the format `HH:mm aa`. 
pub fn timestamp_formatted() -> String {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    let appendix = if is_pm { "PM" } else { "AM" };

    format!("{:02}:{:02} {}", hour, now.minute(), appendix)
}

/// Converts the passed [VirtualKeyCode] to a [str].
/// 
/// # Arguments
/// * `key`: The [VirtualKeyCode] to convert.
/// 
pub fn virtual_key_code_to_string(key: VirtualKeyCode) -> &'static str {
    match key {
        VirtualKeyCode::Y => "Y",
        _ => "Key",
    }
}
