//! Rectangle for drawing rooms.

use super::Position;

/// Struct to represent any square
/// shape to draw on the map from
/// the coordinates (left, top) to
/// (right, bottom).
///
/// # Examples
///
/// (left, top) --> +-----------# \
///                 |           | \
///                 |           | \
///                 |           | \
///                 #-----------+ <-- (right, bottom)
///
#[derive(Default, Debug, Clone)]
pub struct Rectangle {
    /// Left x coordinate of the rectangle.
    pub left: i32,

    /// Top y coordinate of the rectangle.
    pub top: i32,

    /// Right x coordinate of the rectangle.
    pub right: i32,

    /// Bottom y coordinate of the rectangle.
    pub bottom: i32,
}

impl Rectangle {
    /// Creates a new rectangle at the coordinates (`x`, `y`)
    /// and draws it with the passed `width` and `height`.
    ///
    /// # Arguments
    ///
    /// * `left`: Top left x coordinate of the rectangle.
    /// * `top`: Top y coordinate of the rectangle.
    /// * `width`: Width of the rectangle.
    /// * `height`: Height of the rectangle.
    ///
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rectangle {
            left: x,
            top: y,
            right: x + width,
            bottom: y + height,
        }
    }

    /// Checks if the calling [Rectangle] overlaps with
    /// another [Rectangle]. Returns true if they overlap
    /// and false otherwise.
    ///
    /// # Arguments
    /// * `other`: The rectangle for which the overlapping should be checked.
    ///
    pub fn overlaps(&self, other: &Rectangle) -> bool {
        self.left <= other.right
            && self.right >= other.left
            && self.top <= other.bottom
            && self.bottom >= other.top
    }

    /// Returns the center coordinate
    /// of the rectangle as a [Position].
    pub fn center(&self) -> Position {
        let x = (self.left + self.right) / 2;
        let y = (self.top + self.bottom) / 2;
        Position { x, y }
    }
}
