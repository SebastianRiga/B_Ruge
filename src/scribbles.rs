//! Module for small tests.

/// T1
fn get(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Main test method to exectue in the main 
/// file. Only used for small tests.
pub fn scribble() {
    let mut vec = vec![0; 80 * 50];

    vec[get(79, 49)] = 1;
}
