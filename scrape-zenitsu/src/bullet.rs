use scrape_collision::collider::{GameCollider, RigidBodyHandle};

use crate::input_messages::Direction;
use crate::{geometry::*, output_messages};

pub struct BulletInfo {
    pub id: uuid::Uuid,
    pub position: Position,
    velocity: Velocity,
    handle: RigidBodyHandle,
}

impl BulletInfo {
    pub fn update_position_in(
        &mut self,
        collider: &mut GameCollider,
        delta: u128,
    ) -> output_messages::UpdateBulletPosition {
        let mut desired = vec![];
        desired.push((delta as f32 / 1000.0) * self.velocity.velocity_x);
        desired.push((delta as f32 / 1000.0) * self.velocity.velocity_y);
        desired.push((delta as f32 / 1000.0) * self.velocity.velocity_z);

        let calculated_position = collider.calculate_movement(self.handle, desired);
        let bullet_body = collider.get_mut_entity(self.handle);
        bullet_body.set_next_kinematic_translation(bullet_body.translation() + calculated_position);
        let next_position = bullet_body.next_position().translation;

        output_messages::UpdateBulletPosition {
            id: self.id.to_string(),
            x: next_position.x,
            y: next_position.y,
            z: next_position.z,
        }
    }
}

pub struct BasicBullet {
    pub bullet_info: BulletInfo,
    speed: f32,
    damage: i16,
}

impl BasicBullet {
    pub fn new(collider: &mut GameCollider, position: Position, direction: Direction) -> Self {
        let default_speed = 2.0;

        let handle = collider.load_entity(vec![position.x, position.y, position.z]);

        Self {
            bullet_info: BulletInfo {
                id: uuid::Uuid::new_v4(),
                position,
                velocity: Velocity::new(direction, default_speed),
                handle,
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
    fn update_position(
        &mut self,
        collider: &mut GameCollider,
        delta: u128,
    ) -> output_messages::UpdateBulletPosition;
}

impl TickUpdate for Bullet {
    fn update_position(
        &mut self,
        collider: &mut GameCollider,
        delta: u128,
    ) -> output_messages::UpdateBulletPosition {
        match self {
            Self::Basic { bullet } => bullet.bullet_info.update_position_in(collider, delta),
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
