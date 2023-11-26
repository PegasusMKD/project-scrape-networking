pub mod input_messages {
    include!(concat!(env!("OUT_DIR"), "/input_messages.rs"));
}

pub mod output_messages {
    include!(concat!(env!("OUT_DIR"), "/output_messages.rs"));
}

pub mod bullet;
pub mod game_info;
pub mod game_server;
pub mod game_state;
pub mod geometry;
pub mod inbound_server;
pub mod message_queue;
pub mod networking;
pub mod player;
pub mod utility;
