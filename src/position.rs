/// The `Position` struct represents a 2D position in space, with the `z` coordinate representing the floor.
#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}
