use crate::input_messages::Direction;
use crate::{geometry::*, output_messages};

pub struct BulletInfo {
    pub id: uuid::Uuid,
    pub position: Position,
    velocity: Velocity,
}

impl BulletInfo {
    pub fn update_position_in(&mut self, delta: u128) -> output_messages::UpdateBulletPosition {
        self.position.x += (delta as f32 / 1000.0) * self.velocity.velocity_x as f32;
        self.position.y += (delta as f32 / 1000.0) * self.velocity.velocity_y as f32;
        self.position.z += (delta as f32 / 1000.0) * self.velocity.velocity_z as f32;
        output_messages::UpdateBulletPosition {
            id: self.id.to_string(),
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
        }
    }
}

pub struct BasicBullet {
    pub bullet_info: BulletInfo,
    speed: f32,
    damage: i16,
}

impl BasicBullet {
    pub fn new(position: Position, direction: Direction) -> Self {
        let default_speed = 2.0;

        Self {
            bullet_info: BulletInfo {
                id: uuid::Uuid::new_v4(),
                position,
                velocity: Velocity::new(direction, default_speed),
            },
            speed: default_speed,
            damage: 20,
        }
    }
}

pub enum Bullet {
    Basic { bullet: BasicBullet },
}

pub trait TickUpdate {
    fn update_position(&mut self, delta: u128) -> output_messages::UpdateBulletPosition;
}

impl TickUpdate for Bullet {
    fn update_position(&mut self, delta: u128) -> output_messages::UpdateBulletPosition {
        match self {
            Self::Basic { bullet } => bullet.bullet_info.update_position_in(delta),
        }
    }
}

impl output_messages::CreateBullet {
    pub fn new(bullet_info: &BulletInfo) -> Self {
        Self {
            id: bullet_info.id.to_string(),
            x: bullet_info.position.x,
            y: bullet_info.position.y,
            z: bullet_info.position.z,
        }
    }
}
