use std::net::SocketAddr;

use crate::networking::*;

use scrape_collision::collider::{ColliderHandle, RigidBodyHandle};

#[derive(Clone)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub server_info: PlayerServerInfo,
    pub health: i32,
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle
}

impl Player {
    pub fn new(id: String, username: String, addr: SocketAddr, body_handle: RigidBodyHandle, collider_handle: ColliderHandle) -> Self {
        Player {
            id,
            username,
            body_handle,
            collider_handle,
            server_info: PlayerServerInfo { addr },
            health: 100,
        }
    }
}
