use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use chrono::Utc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::domain::{DomainEvent, Room, Ticket, TicketType};

#[derive(Default)]
pub struct InMemoryStore {
    pub rooms: HashMap<String, Room>,
    pub ticket_types: HashMap<String, TicketType>,
    pub tickets: HashMap<Uuid, Ticket>,
}

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<RwLock<InMemoryStore>>,
    pub events_tx: broadcast::Sender<DomainEvent>,
    sequence: Arc<AtomicU64>,
}

impl AppState {
    pub fn new(mut store: InMemoryStore) -> Self {
        seed_defaults(&mut store);
        let (events_tx, _) = broadcast::channel(256);

        Self {
            store: Arc::new(RwLock::new(store)),
            events_tx,
            sequence: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn emit_event(&self, event_type: &str, ticket: Option<Ticket>) {
        let event = DomainEvent {
            sequence: self.sequence.fetch_add(1, Ordering::SeqCst) + 1,
            event_type: event_type.to_owned(),
            timestamp: Utc::now(),
            ticket,
        };

        let _ = self.events_tx.send(event);
    }
}

fn seed_defaults(store: &mut InMemoryStore) {
    store.rooms.insert(
        "room-a".to_string(),
        Room {
            id: "room-a".into(),
            name: "Room A".into(),
            capacity: 8,
        },
    );
    store.rooms.insert(
        "room-b".to_string(),
        Room {
            id: "room-b".into(),
            name: "Room B".into(),
            capacity: 8,
        },
    );

    store.ticket_types.insert(
        "question".to_string(),
        TicketType {
            id: "question".into(),
            name: "Question about material".into(),
            base_weight: 20,
            active: true,
        },
    );
    store.ticket_types.insert(
        "present-current".to_string(),
        TicketType {
            id: "present-current".into(),
            name: "Present current exercise".into(),
            base_weight: 30,
            active: true,
        },
    );
}
