use crate::input_messages::Direction;

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone)]
pub struct Velocity {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
}

impl Velocity {
    pub fn new(direction: Direction, speed: f32) -> Self {
        Self {
            velocity_x: direction.direction_x * speed,
            velocity_y: direction.direction_y * speed,
            velocity_z: direction.direction_z * speed,
        }
    }
}
