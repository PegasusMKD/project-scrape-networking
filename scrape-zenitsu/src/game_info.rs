use std::borrow::BorrowMut;
use std::net::SocketAddr;
use std::u128;

use scrape_collision::collider::{ColliderHandle, TOIStatus};
use scrape_collision::rapier::IntoRapier;
use scrape_collision::{
    collider::CharacterCollision, collider::GameCollider, helpers::IntoDirection,
};

use crate::bullet::BasicBullet;
use crate::game_state::GameState;
use crate::input_messages;
use crate::input_messages::*;
use crate::output_messages;
use crate::player::*;

use crate::geometry::*;
use rand::Rng;

use crate::bullet::{Bullet, TickUpdate};

use output_messages::update_game_event::UpdateEvent;

pub struct GameInfo {
    players: Vec<Player>,
    bullets: Vec<Bullet>,
    state: GameState,
    collider: GameCollider,

    // DEBUG
    counter: i32,
}

impl GameInfo {
    pub fn get_addresses(&self) -> Vec<SocketAddr> {
        self.players
            .iter()
            .map(|player| player.server_info.addr)
            .collect()
    }

    pub fn add_player(&mut self, data: PlayerJoined, addr: SocketAddr) -> Option<UpdateEvent> {
        let player_exists = self
            .players
            .iter()
            .any(|player| player.server_info.addr == addr);
        if !player_exists {
            let mut rng = rand::thread_rng();
            let position = Position {
                x: rng.gen_range(2.0..10.0),
                y: 5.0,
                z: rng.gen_range(2.0..10.0),
            };
            let (body_handle, collider_handle) = self
                .collider
                .load_entity(vec![position.x, position.y, position.z]);
            let player = Player::new(
                data.id.clone(),
                data.username.clone(),
                addr,
                body_handle,
                collider_handle,
            );
            self.players.push(player);
            return Some(UpdateEvent::AddedPlayer(output_messages::AddedPlayer {
                id: data.id,
                username: data.username,
            }));
        }

        None
    }

    pub fn remove_player(&mut self, _data: PlayerLeft, addr: SocketAddr) -> Option<UpdateEvent> {
        let player_exists = self
            .players
            .iter()
            .position(|player| player.server_info.addr == addr);
        if let Some(pos) = player_exists {
            let player = self.players.swap_remove(pos);
            self.collider.unload_entity(player.body_handle);
            return Some(UpdateEvent::RemovedPlayer(output_messages::RemovedPlayer {
                id: player.id,
            }));
        }

        None
    }

    pub fn move_player(&mut self, data: Move, addr: SocketAddr) -> Option<UpdateEvent> {
        let player_exists = self
            .players
            .iter()
            .position(|player| player.server_info.addr == addr);
        if let Some(pos) = player_exists {
            let player = self.players.get_mut(pos).unwrap();
            let desired = vec![data.distance_x, data.distance_y, data.distance_z];
            let calculated_position = self
                .collider
                .calculate_movement(player.body_handle, Vec::new(), desired)
                .next_position;

            let player_body = self.collider.get_mut_entity(player.body_handle);
            player_body
                .set_next_kinematic_translation(player_body.translation() + calculated_position);
            let next_position = player_body.next_position().translation;
            return Some(UpdateEvent::ChangedPlayerPosition(
                output_messages::ChangedPlayerPosition {
                    id: player.id.clone(),
                    x: next_position.x,
                    y: next_position.y,
                    z: next_position.z,
                },
            ));
        }

        None
    }

    pub fn update_camera_rotation(
        &mut self,
        payload: input_messages::UpdateCamera,
        addr: SocketAddr,
    ) -> Option<UpdateEvent> {
        let player = self.find_player_by_address(addr);
        if player.is_none() {
            return None;
        }

        let direction = payload.direction.unwrap();
        let w = payload.w;

        println!("Updated rotation: {:?} - w: {}", direction, w);

        self.collider.update_entity_rotation(
            player.unwrap().body_handle,
            direction.direction_x,
            direction.direction_y,
            direction.direction_z,
            w,
        );

        None
    }

    fn find_player_by_address(&self, addr: SocketAddr) -> Option<&Player> {
        let player_exists = self
            .players
            .iter()
            .position(|player| player.server_info.addr == addr);
        if let Some(pos) = player_exists {
            let player = self.players.get(pos).unwrap();
            return Some(player);
        }

        None
    }

    pub fn shoot_bullet(
        &mut self,
        _payload: input_messages::Shoot,
        addr: SocketAddr,
    ) -> Option<UpdateEvent> {
        let player = self.find_player_by_address(addr);
        if player.is_none() {
            return None;
        }

        let player_body_handle = player.unwrap().body_handle;
        let player_collider_handle = player.unwrap().collider_handle;

        let player_body = self.collider.get_entity(player_body_handle);
        let player_position = player_body.translation();
        let player_rotation = player_body.rotation().into_inner().into_direction();
        println!(
            "Direction: x: {} - y: {} - z: {}",
            player_rotation.x, player_rotation.y, player_rotation.z
        );
        let position = Position {
            x: player_position.x,
            y: player_position.y,
            z: player_position.z,
        };
        // TODO: Fix
        let basic_bullet = BasicBullet::new(
            &mut self.collider,
            position,
            Direction {
                direction_x: player_rotation.x,
                direction_y: player_rotation.y,
                direction_z: player_rotation.z,
            },
            player_collider_handle,
        );

        let response = Some(UpdateEvent::CreateBullet(
            output_messages::CreateBullet::new(&basic_bullet.bullet_info),
        ));
        self.bullets.push(Bullet::Basic {
            bullet: basic_bullet,
        });
        response
    }

    pub fn update_bullets(&mut self, delta: u128) -> Option<UpdateEvent> {
        let mut bullet_changes: Vec<output_messages::UpdateBulletPosition> = Vec::new();
        if self.players.is_empty() {
            return None;
        }
        for bullet in self.bullets.iter_mut() {
            let (collisions, mut updates) = bullet.update_position(&mut self.collider, delta);
            let collided = Self::bullet_collided(&self.players, bullet, collisions);
            if collided {
                updates.destroy = true;
            }
            bullet_changes.push(updates.clone());
        }

        let to_destroy = bullet_changes
            .iter()
            .filter(|bul| bul.destroy)
            .map(|bul| bul.id.clone())
            .collect();
        self.bullets = self
            .bullets
            .clone()
            .into_iter()
            .filter(|bul| !bul.in_vector(&to_destroy))
            .collect();

        Some(UpdateEvent::UpdateAllBullets(
            output_messages::UpdateAllBullets {
                update_bullet_position: bullet_changes,
            },
        ))
    }

    pub fn bullet_collided(
        players: &Vec<Player>,
        bullet: &Bullet,
        collisions: Vec<CharacterCollision>,
    ) -> bool {
        let player_colliders: Vec<ColliderHandle> = players
            .iter()
            .map(|player| player.collider_handle)
            .collect();
        let useful_collisions: Vec<&CharacterCollision> = collisions
            .iter()
            .filter(|col| player_colliders.contains(&col.handle))
            .collect();
        if !useful_collisions.is_empty() {
            println!("Bullet collider: {:#?}", bullet.get_collider_handle());
            return true;
        }

        false
    }

    pub fn game_tick(&mut self, delta: u128) -> Vec<Option<UpdateEvent>> {
        let mut events = Vec::new();
        let bullet_update = self.update_bullets(delta);
        events.push(bullet_update);
        self.collider.run_step();
        events
    }
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            players: Vec::new(),
            bullets: Vec::new(),
            state: GameState {},
            collider: GameCollider::new("./data/environment.gltf".to_string()),
            counter: 0,
        }
    }
}
