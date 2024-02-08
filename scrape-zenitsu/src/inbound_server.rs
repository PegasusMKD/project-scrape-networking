use std::collections::BTreeSet;

use std::io::{self, ErrorKind};

use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

use prost::Message;

use crate::input_messages::*;
use crate::message_queue::*;

pub struct InboundServer {
    pub socket: Arc<UdpSocket>,         // Shared
    pub message_queue: ConcurrentMessageQueue, // Shared

    in_usage: bool,
}

impl InboundServer {
    pub fn new(socket: UdpSocket) -> Self {
        InboundServer {
            socket: Arc::new(socket),
            message_queue: Arc::new(Mutex::new(BTreeSet::new())),
            in_usage: false,
        }
    }

    pub async fn peek_latest_order(&self) -> i32 {
        if let Some(item) = self.message_queue.lock().await.last() {
            return item.order;
        }

        return 0;
    }

    pub async fn wait_incoming_messages(&mut self) -> io::Result<()> {
        let mut buf = vec![0; 128];
        let result = self.socket.recv_from(&mut buf).await;

        match result {
            Err(ref e) => {
                if e.raw_os_error().is_some_and(|err| err == 10054)
                    || e.kind() == ErrorKind::WouldBlock
                {
                    return Ok(());
                }
            }
            _ => (),
        }

        let (size, addr) = result.expect("Data should be filled in...");
        let mut clean_buf = bytes::Bytes::from(buf[..size].to_vec());
        let event = GameEvent::decode(&mut clean_buf)?;

        let last_order = self.peek_latest_order().await;
        self.message_queue.lock().await.insert(QueuedMessage {
            data: event,
            addr,
            order: last_order + 1,
        });
        Ok(())
    }
}
