use std::convert::Infallible;

use async_stream::stream;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, patch, post},
    Json, Router,
};

use crate::{
    domain::{CreateTicketRequest, TicketActionRequest},
    error::AppError,
    service,
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/config", get(get_config))
        .route("/api/tickets", post(create_ticket))
        .route("/api/queues/:room_id", get(get_room_queue))
        .route("/api/tickets/:ticket_id/accept", patch(accept_ticket))
        .route("/api/tickets/:ticket_id/start", patch(start_ticket))
        .route("/api/tickets/:ticket_id/resolve", patch(resolve_ticket))
        .route("/api/tickets/:ticket_id/cancel", patch(cancel_ticket))
        .route("/api/events", get(stream_events))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn get_config(State(state): State<AppState>) -> Json<crate::domain::ConfigResponse> {
    Json(service::get_config(&state).await)
}

async fn create_ticket(
    State(state): State<AppState>,
    Json(req): Json<CreateTicketRequest>,
) -> Result<Json<crate::domain::Ticket>, (StatusCode, String)> {
    service::create_ticket(&state, req)
        .await
        .map(Json)
        .map_err(to_http_error)
}

async fn get_room_queue(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<Json<crate::domain::QueueResponse>, (StatusCode, String)> {
    service::get_room_queue(&state, room_id)
        .await
        .map(Json)
        .map_err(to_http_error)
}

async fn accept_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<uuid::Uuid>,
    Json(req): Json<TicketActionRequest>,
) -> Result<Json<crate::domain::Ticket>, (StatusCode, String)> {
    service::accept_ticket(&state, ticket_id, req)
        .await
        .map(Json)
        .map_err(to_http_error)
}

async fn start_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<uuid::Uuid>,
    Json(_req): Json<TicketActionRequest>,
) -> Result<Json<crate::domain::Ticket>, (StatusCode, String)> {
    service::start_ticket(&state, ticket_id)
        .await
        .map(Json)
        .map_err(to_http_error)
}

async fn resolve_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<uuid::Uuid>,
    Json(_req): Json<TicketActionRequest>,
) -> Result<Json<crate::domain::Ticket>, (StatusCode, String)> {
    service::resolve_ticket(&state, ticket_id)
        .await
        .map(Json)
        .map_err(to_http_error)
}

async fn cancel_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<uuid::Uuid>,
    Json(_req): Json<TicketActionRequest>,
) -> Result<Json<crate::domain::Ticket>, (StatusCode, String)> {
    service::cancel_ticket(&state, ticket_id)
        .await
        .map(Json)
        .map_err(to_http_error)
}

async fn stream_events(
    State(state): State<AppState>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let mut rx = state.events_tx.subscribe();

    let event_stream = stream! {
        while let Ok(event) = rx.recv().await {
            let json = serde_json::to_string(&event).unwrap_or_else(|_| "{}".to_string());
            yield Ok(Event::default()
                .event(event.event_type)
                .id(event.sequence.to_string())
                .data(json));
        }
    };

    Sse::new(event_stream).keep_alive(KeepAlive::default())
}

fn to_http_error(err: AppError) -> (StatusCode, String) {
    (err.status_code(), err.to_string())
}
