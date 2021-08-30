pub struct Rectangle {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, widht: i32, height: i32) -> Rectangle {
        Rectangle {
            left: x,
            top: y,
            right: x + widht,
            bottom: y + height,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let x = (self.left + self.right) / 2;
        let y = (self.top + self.bottom) / 2;
        (x, y)
    }
}
