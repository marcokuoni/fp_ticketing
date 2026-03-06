use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::{
        ConfigResponse, CreateTicketRequest, QueueResponse, Ticket, TicketActionRequest,
        TicketStatus,
    },
    error::AppError,
    state::AppState,
};

pub async fn get_config(state: &AppState) -> ConfigResponse {
    let store = state.store.read().await;
    ConfigResponse {
        rooms: store.rooms.values().cloned().collect(),
        ticket_types: store.ticket_types.values().cloned().collect(),
    }
}

pub async fn create_ticket(state: &AppState, req: CreateTicketRequest) -> Result<Ticket, AppError> {
    let mut store = state.store.write().await;

    if !store.rooms.contains_key(&req.room_id) {
        return Err(AppError::RoomNotFound);
    }

    let ticket_type = store
        .ticket_types
        .get(&req.ticket_type_id)
        .ok_or(AppError::TicketTypeNotFound)?;

    if !ticket_type.active {
        return Err(AppError::TicketTypeInactive);
    }

    let waiting_count = store
        .tickets
        .values()
        .filter(|t| t.room_id == req.room_id && t.status == TicketStatus::Waiting)
        .count() as i32;

    let ticket = Ticket {
        id: Uuid::new_v4(),
        session_id: req.session_id,
        room_id: req.room_id,
        student_id: req.student_id,
        ticket_type_id: req.ticket_type_id,
        topic: req.topic,
        details: req.details,
        status: TicketStatus::Waiting,
        computed_priority: ticket_type.base_weight + waiting_count,
        created_at: Utc::now(),
        assigned_tutor_id: None,
    };

    store.tickets.insert(ticket.id, ticket.clone());
    drop(store);

    state.emit_event("TicketCreated", Some(ticket.clone()));
    Ok(ticket)
}

pub async fn get_room_queue(state: &AppState, room_id: String) -> Result<QueueResponse, AppError> {
    let store = state.store.read().await;
    if !store.rooms.contains_key(&room_id) {
        return Err(AppError::RoomNotFound);
    }

    let mut waiting: Vec<_> = store
        .tickets
        .values()
        .filter(|t| t.room_id == room_id && t.status == TicketStatus::Waiting)
        .cloned()
        .collect();
    waiting.sort_by_key(|t| (-t.computed_priority, t.created_at, t.id));

    let assigned = store
        .tickets
        .values()
        .filter(|t| t.room_id == room_id && t.status == TicketStatus::Assigned)
        .cloned()
        .collect();

    let in_progress = store
        .tickets
        .values()
        .filter(|t| t.room_id == room_id && t.status == TicketStatus::InProgress)
        .cloned()
        .collect();

    Ok(QueueResponse {
        room_id,
        waiting,
        assigned,
        in_progress,
    })
}

pub async fn accept_ticket(
    state: &AppState,
    ticket_id: Uuid,
    req: TicketActionRequest,
) -> Result<Ticket, AppError> {
    mutate_ticket(state, ticket_id, |ticket| {
        if ticket.status != TicketStatus::Waiting {
            return Err(AppError::InvalidTransition("ticket must be waiting"));
        }
        ticket.status = TicketStatus::Assigned;
        ticket.assigned_tutor_id = req.tutor_id;
        Ok(())
    })
    .await
}

pub async fn start_ticket(state: &AppState, ticket_id: Uuid) -> Result<Ticket, AppError> {
    mutate_ticket(state, ticket_id, |ticket| {
        if ticket.status != TicketStatus::Assigned {
            return Err(AppError::InvalidTransition("ticket must be assigned"));
        }
        ticket.status = TicketStatus::InProgress;
        Ok(())
    })
    .await
}

pub async fn resolve_ticket(state: &AppState, ticket_id: Uuid) -> Result<Ticket, AppError> {
    mutate_ticket(state, ticket_id, |ticket| {
        if ticket.status != TicketStatus::Assigned && ticket.status != TicketStatus::InProgress {
            return Err(AppError::InvalidTransition(
                "ticket must be assigned or in progress",
            ));
        }
        ticket.status = TicketStatus::Resolved;
        Ok(())
    })
    .await
}

pub async fn cancel_ticket(state: &AppState, ticket_id: Uuid) -> Result<Ticket, AppError> {
    mutate_ticket(state, ticket_id, |ticket| {
        if ticket.status == TicketStatus::Resolved || ticket.status == TicketStatus::Cancelled {
            return Err(AppError::InvalidTransition("ticket already closed"));
        }

        ticket.status = TicketStatus::Cancelled;
        Ok(())
    })
    .await
}

async fn mutate_ticket<F>(state: &AppState, ticket_id: Uuid, mutator: F) -> Result<Ticket, AppError>
where
    F: FnOnce(&mut Ticket) -> Result<(), AppError>,
{
    let mut store = state.store.write().await;
    let ticket = store
        .tickets
        .get_mut(&ticket_id)
        .ok_or(AppError::TicketNotFound)?;

    mutator(ticket)?;
    let updated = ticket.clone();
    drop(store);

    state.emit_event("TicketUpdated", Some(updated.clone()));
    Ok(updated)
}
