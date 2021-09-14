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
        VirtualKeyCode::A => "A",
        VirtualKeyCode::B => "B",
        VirtualKeyCode::C => "C",
        VirtualKeyCode::D => "D",
        VirtualKeyCode::E => "E",
        VirtualKeyCode::F => "F",
        VirtualKeyCode::G => "G",
        VirtualKeyCode::H => "H",
        VirtualKeyCode::I => "I",
        VirtualKeyCode::J => "J",
        VirtualKeyCode::K => "K",
        VirtualKeyCode::L => "L",
        VirtualKeyCode::M => "M",
        VirtualKeyCode::N => "N",
        VirtualKeyCode::O => "O",
        VirtualKeyCode::P => "P",
        VirtualKeyCode::Q => "Q",
        VirtualKeyCode::R => "R",
        VirtualKeyCode::S => "S",
        VirtualKeyCode::T => "T",
        VirtualKeyCode::U => "U",
        VirtualKeyCode::V => "V",
        VirtualKeyCode::W => "W",
        VirtualKeyCode::X => "X",
        VirtualKeyCode::Y => "Y",
        VirtualKeyCode::Z => "Z",
        VirtualKeyCode::Escape => "Escape",
        _ => "Key",
    }
}

pub fn i32_to_alpha_key(value: i32) -> VirtualKeyCode {
    match value {
        0 => VirtualKeyCode::A,
        1 => VirtualKeyCode::B,
        2 => VirtualKeyCode::C,
        3 => VirtualKeyCode::D,
        4 => VirtualKeyCode::E,
        5 => VirtualKeyCode::F,
        6 => VirtualKeyCode::G,
        7 => VirtualKeyCode::H,
        8 => VirtualKeyCode::I,
        9 => VirtualKeyCode::J,
        10 => VirtualKeyCode::K,
        11 => VirtualKeyCode::L,
        12 => VirtualKeyCode::M,
        13 => VirtualKeyCode::N,
        14 => VirtualKeyCode::O,
        15 => VirtualKeyCode::P,
        16 => VirtualKeyCode::Q,
        17 => VirtualKeyCode::R,
        18 => VirtualKeyCode::S,
        19 => VirtualKeyCode::T,
        20 => VirtualKeyCode::U,
        21 => VirtualKeyCode::V,
        22 => VirtualKeyCode::W,
        23 => VirtualKeyCode::X,
        24 => VirtualKeyCode::Y,
        25 => VirtualKeyCode::Z,
        _ => panic!(format!("i32 {} can't be converted to an alpha key, because it is out of range (0-25)!", value)),
    }
}
