//! Collection of helper functions

use rltk::{DistanceAlg, Point};

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
