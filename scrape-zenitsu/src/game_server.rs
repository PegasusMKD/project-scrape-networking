use std::collections::BTreeSet;
use std::io;
use std::net::SocketAddr;

use crate::inbound_server::InboundServer;
use crate::utility;

use std::time::Duration;

use prost::Message;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

use crate::message_queue::*;
use crate::{game_info::*, input_messages};

use bytes::BytesMut;

use crate::input_messages::game_event::*;
use crate::input_messages::*;
use crate::output_messages::update_game_event::UpdateEvent;
use crate::output_messages::UpdateGameEvent;

pub struct GameServer {
    socket: Arc<Mutex<UdpSocket>>,         // Shared
    message_queue: ConcurrentMessageQueue, // Shared

    game: GameInfo,             // Outgoing
    write_buf: bytes::BytesMut, // Outgoing
}

impl GameServer {
    pub fn with_inbound(server: &InboundServer) {
        let outbound_message_queue = Arc::clone(&server.message_queue);
        let outbound_socket = Arc::clone(&server.socket);
        let mut game_server = GameServer::new(outbound_message_queue, outbound_socket);
        tokio::spawn(async move {
            let _ = game_server.tick_process().await;
        });
    }

    pub fn new(queue: ConcurrentMessageQueue, socket: Arc<Mutex<UdpSocket>>) -> Self {
        Self {
            message_queue: queue,
            socket,
            game: GameInfo::default(),
            write_buf: BytesMut::new(),
        }
    }

    async fn flush_queue(&self) -> BTreeSet<QueuedMessage> {
        let mut queue = self.message_queue.lock().await;
        let response_queue: BTreeSet<QueuedMessage> = queue.clone();
        queue.clear();
        response_queue
    }

    pub async fn tick_process(&mut self) -> io::Result<()> {
        println!("Started ticks...");
        let mut interval = tokio::time::interval(Duration::from_millis(16));
        let mut last_loop = utility::current_time();
        loop {
            interval.tick().await;

            let start = utility::current_time();
            let delta = start - last_loop;

            let _ = self.process_all_events(delta as f32).await;

            self.game.game_tick(delta);

            last_loop = start;
        }
    }

    async fn process_all_events(&mut self, delta: f32) -> io::Result<()> {
        let mut queue = self.flush_queue().await;

        while let Some(message) = queue.pop_first() {
            // println!("Processed the message: {:?}", message);
            let update = self.process_event(message.data, delta, message.addr);
            self.process_update(update).await?;
            self.write_buf.clear();
        }

        Ok(())
    }

    fn process_event(
        &mut self,
        event: GameEvent,
        delta: f32,
        addr: SocketAddr,
    ) -> Option<UpdateEvent> {
        let ev = event.event.unwrap();
        match ev {
            Event::Joined(payload) => self.game.add_player(payload, addr),
            Event::Move(payload) => self.game.move_player(payload, delta, addr),
            Event::Left(payload) => self.game.remove_player(payload, addr),
            Event::Shoot(payload) => self.game.shoot_bullet(payload, addr),
        }
    }

    async fn process_update(&mut self, event: Option<UpdateEvent>) -> io::Result<()> {
        if event == None {
            return Ok(());
        }

        let addrs = self.game.get_addresses();
        let ev = UpdateGameEvent {
            update_event: event,
        };

        self.write_buf.reserve(ev.encoded_len());
        ev.encode(&mut self.write_buf)?;

        for addr in addrs {
            let result = self.socket.lock().await.try_send_to(&self.write_buf, addr);
            if result.is_err() {
                self.game
                    .remove_player(input_messages::PlayerLeft { id: String::new() }, addr);
            }
        }

        Ok(())
    }
}
