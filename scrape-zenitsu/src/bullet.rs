use std::string;

use scrape_collision::collider::{
    CharacterCollision, ColliderHandle, GameCollider, RigidBodyHandle,
};
use uuid::Uuid;

use crate::input_messages::Direction;
use crate::{geometry::*, output_messages};

#[derive(Clone)]
pub struct BulletInfo {
    pub id: uuid::Uuid,
    pub position: Position,
    velocity: Velocity,
    fired_by: ColliderHandle,
    body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
}

impl BulletInfo {
    pub fn update_position_in(
        &mut self,
        collider: &mut GameCollider,
        delta: u128,
    ) -> (
        Vec<CharacterCollision>,
        output_messages::UpdateBulletPosition,
    ) {
        let mut desired = vec![];
        desired.push((delta as f32 / 1000.0) * self.velocity.velocity_x);
        desired.push((delta as f32 / 1000.0) * self.velocity.velocity_y);
        desired.push((delta as f32 / 1000.0) * self.velocity.velocity_z);

        let mut exclude_filter = Vec::new();
        exclude_filter.push(self.fired_by);
        // println!("Fired By: {:#?}", self.fired_by);

        let movement = collider.calculate_movement(self.body_handle, exclude_filter, desired);
        if !movement.collisions.is_empty() {
            // println!("Movement collisions: {:?}", movement.collisions);
        }

        let calculated_position = movement.next_position;
        let bullet_body = collider.get_mut_entity(self.body_handle);
        bullet_body.set_next_kinematic_translation(bullet_body.translation() + calculated_position);
        let next_position = bullet_body.next_position().translation;

        (
            movement.collisions,
            output_messages::UpdateBulletPosition {
                id: self.id.to_string(),
                x: next_position.x,
                y: next_position.y,
                z: next_position.z,
                destroy: false,
            },
        )
    }
}

#[derive(Clone)]
pub struct BasicBullet {
    pub bullet_info: BulletInfo,
    speed: f32,
    damage: i16,
}

impl BasicBullet {
    pub fn new(
        collider: &mut GameCollider,
        position: Position,
        direction: Direction,
        fired_by: ColliderHandle,
    ) -> Self {
        let default_speed = 0.1;

        let (body_handle, collider_handle) =
            collider.load_entity(vec![position.x, position.y, position.z]);

        Self {
            bullet_info: BulletInfo {
                id: uuid::Uuid::new_v4(),
                position,
                velocity: Velocity::new(direction, default_speed),
                fired_by,
                body_handle,
                collider_handle,
            },
            speed: default_speed,
            damage: 20,
        }
    }
}

#[derive(Clone)]
pub enum Bullet {
    Basic { bullet: BasicBullet },
}

pub trait TickUpdate {
    fn update_position(
        &mut self,
        collider: &mut GameCollider,
        delta: u128,
    ) -> (
        Vec<CharacterCollision>,
        output_messages::UpdateBulletPosition,
    );

    fn get_body_handle(&self) -> RigidBodyHandle;
    fn get_collider_handle(&self) -> ColliderHandle;
    fn in_vector(&self, bullets: &Vec<String>) -> bool;
}

impl TickUpdate for Bullet {
    fn update_position(
        &mut self,
        collider: &mut GameCollider,
        delta: u128,
    ) -> (
        Vec<CharacterCollision>,
        output_messages::UpdateBulletPosition,
    ) {
        match self {
            Self::Basic { bullet } => bullet.bullet_info.update_position_in(collider, delta),
        }
    }

    fn get_body_handle(&self) -> RigidBodyHandle {
        match self {
            Bullet::Basic { bullet } => bullet.bullet_info.body_handle,
        }
    }

    fn get_collider_handle(&self) -> ColliderHandle {
        match self {
            Bullet::Basic { bullet } => bullet.bullet_info.collider_handle,
        }
    }

    fn in_vector(&self, bullets: &Vec<String>) -> bool {
        match self {
            Self::Basic { bullet } => bullets.contains(&bullet.bullet_info.id.to_string()),
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
