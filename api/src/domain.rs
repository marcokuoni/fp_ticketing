use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketType {
    pub id: String,
    pub name: String,
    pub base_weight: i32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TicketStatus {
    Waiting,
    Assigned,
    InProgress,
    Resolved,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: Uuid,
    pub session_id: String,
    pub room_id: String,
    pub student_id: String,
    pub ticket_type_id: String,
    pub topic: String,
    pub details: Option<String>,
    pub status: TicketStatus,
    pub computed_priority: i32,
    pub created_at: DateTime<Utc>,
    pub assigned_tutor_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainEvent {
    pub sequence: u64,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub ticket: Option<Ticket>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketRequest {
    pub session_id: String,
    pub room_id: String,
    pub student_id: String,
    pub ticket_type_id: String,
    pub topic: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketActionRequest {
    pub tutor_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueResponse {
    pub room_id: String,
    pub waiting: Vec<Ticket>,
    pub assigned: Vec<Ticket>,
    pub in_progress: Vec<Ticket>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub rooms: Vec<Room>,
    pub ticket_types: Vec<TicketType>,
}
