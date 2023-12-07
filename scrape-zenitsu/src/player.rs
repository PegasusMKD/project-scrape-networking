use std::net::SocketAddr;

use crate::networking::*;

use scrape_collision::collider::RigidBodyHandle;

#[derive(Clone)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub server_info: PlayerServerInfo,
    pub health: i32,
    pub handle: RigidBodyHandle,
}

impl Player {
    pub fn new(id: String, username: String, addr: SocketAddr, handle: RigidBodyHandle) -> Self {
        Player {
            id,
            username,
            handle,
            server_info: PlayerServerInfo { addr },
            health: 100,
        }
    }
}
