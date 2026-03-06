# API (Axum)

DDD-orientierte Struktur:

- `domain.rs` – Domänenmodelle (Ticket, Status, Events, DTOs)
- `service.rs` – Anwendungslogik / Use-Case-Services (Invarianten + Übergänge)
- `state.rs` – In-Memory Store + Event-Sequenzierung + Seed-Daten
- `http.rs` – HTTP-Adapter (Routing, Request/Response-Mapping)
- `error.rs` – zentralisierte Fehler inkl. HTTP-Status-Mapping

## Run

```bash
cargo run
```

Server startet auf `http://localhost:3000`.

## Endpoints

- `GET /health`
- `GET /api/config`
- `POST /api/tickets`
- `GET /api/queues/:room_id`
- `PATCH /api/tickets/:ticket_id/accept`
- `PATCH /api/tickets/:ticket_id/start`
- `PATCH /api/tickets/:ticket_id/resolve`
- `PATCH /api/tickets/:ticket_id/cancel`
- `GET /api/events` (SSE)
