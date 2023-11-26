use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::input_messages::GameEvent;

use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct QueuedMessage {
    pub order: i32,
    pub data: GameEvent,
    pub addr: SocketAddr,
}

pub type ConcurrentMessageQueue = Arc<Mutex<BTreeSet<QueuedMessage>>>;

impl PartialEq for QueuedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.addr == other.addr
    }
}

impl PartialOrd for QueuedMessage {
    fn lt(&self, other: &Self) -> bool {
        self.order < other.order
    }

    fn le(&self, other: &Self) -> bool {
        self.order <= other.order
    }

    fn gt(&self, other: &Self) -> bool {
        self.order > other.order
    }

    fn ge(&self, other: &Self) -> bool {
        self.order >= other.order
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.order.cmp(&other.order))
    }
}

impl Eq for QueuedMessage {}

impl Ord for QueuedMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return Ordering::Equal;
        }

        self.order.cmp(&other.order)
    }
}
