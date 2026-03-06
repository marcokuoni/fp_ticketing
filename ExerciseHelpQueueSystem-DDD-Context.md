# Exercise Help Queue System — DDD Context (Qwik + Axum, In-Memory, Live Updates)

This document consolidates the current domain and architecture decisions from the repository into one implementation-ready context file.

- Frontend: **Qwik**
- Backend: **Rust + Axum**
- Persistence: **none** (runtime in-memory state only)
- Configuration: **static, file-based** (`TOML` or `YAML`), loaded on startup (optional hot reload)
- Updates: **dynamic** (recommended: **SSE**; WebSocket optional)
- Primary goal: stable and deterministic operation for simultaneous supervised exercises across multiple rooms with multiple tutors.

---

## 1) Domain Goal and Scope

The system manages supervised exercise help sessions where students create help tickets, tutors process them, and the platform enforces fair queueing plus controlled rebalancing across rooms.

It is optimized for:

- predictable queue behavior
- clear ownership and assignment semantics
- race-safe tutor actions
- low operational complexity for MVP delivery

---

## 2) Ubiquitous Language

- **Room**: physical or virtual location where supervised exercise support happens
- **Session**: bounded period during which tickets can be created and processed
- **Tutor**: staff member processing tickets
- **Student**: ticket creator/owner
- **Ticket**: help request created by a student
- **Queue**: ordered waiting line per `Session + Room`
- **Ticket Type**: configurable request category (e.g., “question”, “present exercise”)
- **Weight / Priority**: ranking basis derived from type and policy
- **Assignment**: explicit responsibility link between ticket and tutor
- **Rebalancing**: moving waiting tickets between rooms under overload policy

---

## 3) Bounded Contexts

### 3.1 Exercise Operations (Core Domain)

Owns ticket lifecycle, queue ordering, tutor assignment, and room-to-room ticket movement.

### 3.2 Identity & Roles (Supporting)

Owns users and role mapping (`student`, `tutor`, `room_lead`, `lecturer`, `admin`).

### 3.3 Configuration (Supporting)

Owns static configuration such as ticket types, weighting baselines, room capacities, and assignment/rebalancing policies.

---

## 4) Aggregates and Invariants

### 4.1 `Session`

**Purpose**: timebox that gates all queue operations.

**Fields**

- `session_id`
- `course_id?`
- `starts_at`, `ends_at`
- `state: Planned | Running | Closed`

**Invariants**

- tickets can only be created when `state == Running`
- rooms are active only within the owning session

### 4.2 `Room`

**Purpose**: operating environment and capacity envelope within a session.

**Fields**

- `room_id`
- `session_id`
- `name`
- `capacity` (e.g., max active students)
- `assignment_policy_id?`

**Value object**

- `Capacity { max_active_students, max_active_tickets_per_tutor? }`

**Invariants**

- belongs to exactly one session
- capacity values must be non-negative

### 4.3 `Queue` (per `Session + Room`)

**Purpose**: stable ordering and state progression for processing work.

**Fields**

- `queue_id`
- `session_id`
- `room_id`
- `entries`

### 4.4 `QueueEntry`

**Fields**

- `entry_id`
- `ticket_id`
- `enqueued_at`
- `computed_priority` (snapshot)
- `state: Waiting | Assigned | InProgress | Done | Cancelled | Moved`

**Invariants**

- per session, a ticket may be active (`Waiting/Assigned/InProgress`) in exactly one queue
- `Assigned` requires a valid active assignment
- `Moved` must create/point to a target room queue entry

### 4.5 `Ticket`

**Purpose**: request content and lifecycle.

**Fields**

- `ticket_id`
- `session_id`
- `created_by: StudentId`
- `type: TicketTypeId`
- `topic`
- `details?`
- `status: Open | Assigned | InProgress | Resolved | Cancelled`
- `created_at`, `updated_at`

**Invariants**

- type must be an active configured type
- status transitions are strict (no implicit rollback from resolved)

### 4.6 `Assignment`

**Purpose**: explicit ticket ↔ tutor responsibility relation.

**Fields**

- `assignment_id`
- `ticket_id`
- `tutor_id`
- `room_id`
- `assigned_at`
- `accepted_at?`
- `ended_at?`
- `state: Proposed | Accepted | Ended | Revoked`

**Invariants**

- at most one active assignment (`Proposed/Accepted`) per ticket
- accepted assignments require tutor eligibility for room/session policy

---

## 5) Lifecycle Models

### 5.1 Ticket status

- `Open -> Assigned -> InProgress -> Resolved`
- `Open/Assigned/InProgress -> Cancelled`

### 5.2 QueueEntry state

- `Waiting -> Assigned -> InProgress -> Done`
- `Waiting/Assigned -> Moved`
- `Waiting -> Cancelled`

---

## 6) Domain Services

### 6.1 `PriorityCalculator`

Computes `computed_priority` from:

- ticket type base weight
- waiting-time contribution
- optional rule-set modifiers

> Recommendation for deterministic MVP ordering: sort by `(computed_priority desc, enqueued_at asc, ticket_id asc)`.

### 6.2 `RoomRebalancer`

Evaluates queue pressure and tutor capacity to suggest/execute moves:

- candidate selection from overloaded room(s)
- destination room selection with spare capacity
- movement of `Waiting` tickets only in MVP

---

## 7) Domain Events

Core event types for audit, projections, and live UI updates:

- `TicketCreated`
- `TicketEnqueued`
- `TicketAssigned`
- `TicketStarted`
- `TicketResolved`
- `TicketCancelled`
- `TicketMoved`
- `TutorCapacityChanged`
- `RoomCapacityChanged`

Event emission should be synchronous with in-memory command handling so SSE subscribers always observe a consistent post-command state.

---

## 8) Configuration Model (File-Based)

Load once at startup from `TOML` or `YAML`.

### 8.1 Suggested top-level config sections

- `sessions` (or session template for runtime bootstrap)
- `rooms`
- `ticket_types`
- `weighting_rules`
- `assignment_policy`
- `rebalancing_policy`

### 8.2 Ticket type config fields

- `id`
- `name`
- `description?`
- `active`
- `base_weight`

### 8.3 Hot reload (optional)

- safe for non-structural policy changes (weights, thresholds)
- emit config-changed events and re-evaluate only future tickets for MVP
- avoid retroactive full re-prioritization unless explicitly enabled

---

## 9) In-Memory Architecture (Axum)

### 9.1 State ownership

Use a single process-level application state (e.g., `Arc<RwLock<AppState>>`) containing:

- sessions
- rooms
- queues + entries
- tickets
- assignments
- read-optimized indices (ticket -> queue entry, room -> waiting list, tutor -> active assignments)

### 9.2 Determinism and concurrency

- process commands under write lock with small critical sections
- enforce invariants atomically per command
- use monotonic sequence numbers for events to support strict client ordering

### 9.3 Recovery implications

Because state is in-memory only:

- restart clears operational data
- config survives (file-based)
- this is acceptable for target MVP; persistence can be introduced later via repository interfaces

---

## 10) API and Real-Time Updates

### 10.1 Command/query split (recommended)

- **Commands** (`POST`/`PATCH`): create ticket, accept/start/resolve/cancel, move ticket
- **Queries** (`GET`): queue view per room, my ticket, tutor workload, session overview

### 10.2 SSE as default live-update transport

Provide endpoint(s), e.g.:

- `GET /events?session_id=...`
- optional filters: room, role, actor

Server emits ordered events with:

- sequence number
- event type
- timestamp
- normalized payload

SSE advantages here:

- simpler than WebSocket for server->client push
- works well for dashboard and status updates
- straightforward reconnection using `Last-Event-ID`

WebSocket remains optional for future bidirectional needs.

---

## 11) Role-Centered Capability Map (RBAC)

- `student`: create/cancel own ticket, read own status
- `tutor`: list room queue, accept/start/resolve assigned work
- `room_lead`: tutor capabilities + room-level balancing actions
- `lecturer`: global read visibility + optional global prioritization controls
- `admin`: configuration and operational setup management

---

## 12) User-Story Coverage (MVP Slices)

### Slice A — Core queue

US-1, US-2, US-3, US-5, US-6, US-8

### Slice B — Lifecycle hygiene

US-4, US-7, US-9

### Slice C — Load distribution + admin

US-10, US-11, US-12

This preserves fast delivery while retaining a clean path to full room balancing and administration.

---

## 13) Non-Goals (Current Scope)

- durable storage/database integration
- cross-process distribution
- advanced analytics/BI
- generalized workflow engine

These can be layered later without changing the core domain language.

---

## 14) Implementation Notes for Qwik + Axum

- keep frontend views projection-oriented (student view, tutor room board, admin config view)
- subscribe once to SSE and fan out events into local store/signals
- derive optimistic UI transitions only where server conflict risk is low
- for race-sensitive actions (`accept ticket`), always trust server result and reconcile UI from events

---

## 15) Summary

This DDD context intentionally keeps the runtime simple (single process, in-memory state, static config) while preserving strict domain boundaries and invariants. It is suitable for supervised exercise operations with multiple rooms and tutors, and supports live coordination through SSE-backed event propagation.
