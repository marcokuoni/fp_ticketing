## Ubiquitous Language

- **Raum**: Ort, in dem eine betreute Übung stattfindet
- **Session**: Zeitraum, in dem Tickets gezogen werden können (z.B. Übung vom 14:00–16:00)
- **Betreuer**: Person, die Tickets bearbeitet
- **Student**
- **Ticket**: Anfrage eines Studenten
- **Queue**: Warteschlange pro Raum (oder pro Session+Raum)
- **Ticket-Typ**: Kategorie der Anfrage (dynamisch erweiterbar)
- **Gewichtung**: Prioritätsbasis pro Ticket-Typ und evtl. weitere Faktoren
- **Zuteilung**: Ticket ist einem Betreuer zugewiesen
- **Umverteilung**: Student/Ticket wird in einen anderen Raum verschoben

---

## Bounded Contexts

Du kannst es als Monolith starten und später trennen, aber fachlich sind diese Bereiche klar:

### 1) Exercise Operations (Kern-Domäne)

Alles rund um Tickets, Warteschlange, Bearbeitung, Umverteilung.

### 2) Identity & Roles (Support)

Benutzer, Rollen, ggf. Kurszugehörigkeit.

### 3) Configuration (Support)

Ticket-Typen, Gewichtungsregeln, Raum-Konfiguration, Limits.

---

## Aggregates und Invarianten

### Aggregate: `Session`

**Zweck:** Rahmen, in dem Räume aktiv sind und Tickets akzeptiert werden.

**Entity:**

- `Session`
  - `sessionId`
  - `courseId` (optional)
  - `startsAt`, `endsAt`
  - `state`: Planned | Running | Closed

**Invarianten:**

- Tickets dürfen nur erstellt werden, wenn `Session.state == Running`
- Räume sind nur innerhalb einer Session «aktiv»

---

### Aggregate: `Room`

**Zweck:** Kapazität und Betreuungssituation eines Raumes in einer Session.

**Entity:**

- `Room`
  - `roomId`
  - `sessionId`
  - `name`
  - `capacity` (wie viele aktive Fälle gleichzeitig sinnvoll)
  - `assignmentPolicyId` (optional, falls pro Raum andere Regeln gelten)

**Value Objects:**

- `Capacity { maxActiveStudents, maxActiveTicketsPerTutor? }`

**Invarianten:**

- Raum gehört genau zu einer Session
- Kapazität darf nicht negativ sein

> Hinweis: Die Queue selbst würde ich nicht in `Room` speichern, sondern als eigenes Aggregate, sonst wird `Room` bei jedem Queue-Update zum Hotspot.

---

### Aggregate: `Queue` (pro Session+Room)

**Zweck:** Stabile, transaktionale Kontrolle über Reihenfolge, Priorisierung und Statuswechsel.

**Entity:**

- `Queue`
  - `queueId`
  - `sessionId`
  - `roomId`
  - `entries: List<QueueEntryId>` (konzeptionell, nicht zwingend als Liste persistieren)

**Entity:**

- `QueueEntry`
  - `entryId`
  - `ticketId`
  - `enqueuedAt`
  - `computedPriority` (Snapshot)
  - `state`: Waiting | Assigned | InProgress | Done | Cancelled | Moved

**Invarianten:**

- Ein `Ticket` darf in einer Session nur in **genau einer** Queue gleichzeitig `Waiting/Assigned/InProgress` sein
- `Assigned` braucht eine gültige Zuteilung (Assignment)
- `Moved` erzeugt eine neue QueueEntry in der Ziel-Queue (oder referenziert sie)

---

### Aggregate: `Ticket`

**Zweck:** Fachlicher Inhalt der Anfrage und Lebenszyklus.

**Entity:**

- `Ticket`
  - `ticketId`
  - `sessionId`
  - `createdBy: StudentId`
  - `type: TicketTypeId`
  - `topic` (kurzer Text)
  - `details` (optional)
  - `status`: Open | Assigned | InProgress | Resolved | Cancelled
  - `createdAt`, `updatedAt`

**Value Objects:**

- `TicketTypeId`
- `TicketStatus`
- `TicketText` (Topic/Details mit Länge/Validierung)

**Invarianten:**

- Ticket-Typ muss existieren (Configuration Context)
- Status-Übergänge sind strikt (z.B. nicht von Resolved zurück nach Open)

---

### Aggregate: `Assignment` (Zuteilung Ticket ↔ Betreuer)

**Zweck:** Explizit modellieren, wer gerade verantwortlich ist.

**Entity:**

- `Assignment`
  - `assignmentId`
  - `ticketId`
  - `tutorId`
  - `roomId` (wo wird es bearbeitet)
  - `assignedAt`
  - `acceptedAt?`
  - `endedAt?`
  - `state`: Proposed | Accepted | Ended | Revoked

**Invarianten:**

- Pro Ticket höchstens eine aktive Assignment (`Proposed/Accepted`)
- `Accepted` nur, wenn Tutor dem Raum in der Session zugeordnet ist (oder globaler Tutor)

---

## Support-Modelle

### Entity: `Tutor` / `Student` / `User`

Im Identity-Context, nicht im Kern «anreichern», sondern nur IDs referenzieren.

- `User { userId, displayName }`
- `Tutor { tutorId == userId, skills?, activeSessionIds? }`
- `Student { studentId == userId, groupId? }`

---

## Konfiguration als Domäne (wichtig wegen «dynamisch erweiterbar»)

### Aggregate: `TicketType`

- `ticketTypeId`
- `name` (z.B. «Fragen zum Stoff»)
- `description`
- `isActive`
- `baseWeight` (Default-Gewichtung)
- `allowedInSessions?` (optional)

### Aggregate: `WeightingRuleSet`

Wenn du komplexer willst als nur `baseWeight`.

- `ruleSetId`
- `name`
- `rules: List<Rule>`
- `version`
- `effectiveFrom`

**Rule Beispiele:**

- Wartezeit erhöht Priorität nach X Minuten
- «Vergangene failed Übung präsentieren» bekommt zusätzlich +N
- Anfängergruppe vs Fortgeschrittene (optional)
- Tutor-Auslastung beeinflusst Umverteilung, nicht Ticket-Priorität

---

## Domain Services (fachliche Logik, die nicht in ein Aggregate passt)

### `PriorityCalculator`

Input:

- Ticket (Typ, createdAt)
- Queue-Kontext (Wartezeit)
- RuleSet
  Output:
- `computedPriority`

### `LoadBalancer` / `RoomRebalancer`

Input:

- aktive Queues, Tutor-Kapazitäten, Queue-Längen, Service-Level-Ziele
  Output:
- Vorschläge: `MoveTicket(ticketId, fromRoomId, toRoomId)`

Wichtig: Umverteilung ist oft **Policy/Service**, nicht Aggregate-intern.

---

## Domain Events (für Audit, UI-Updates, Stats)

- `TicketCreated(ticketId, sessionId, studentId, typeId)`
- `TicketEnqueued(ticketId, roomId, priority)`
- `TicketAssigned(ticketId, tutorId, roomId)`
- `TicketStarted(ticketId, tutorId)`
- `TicketResolved(ticketId)`
- `TicketCancelled(ticketId)`
- `TicketMoved(ticketId, fromRoomId, toRoomId, reason)`
- `TutorCapacityChanged(tutorId, sessionId, newCapacity)`
- `RoomCapacityChanged(roomId, sessionId, newCapacity)`

---

## Zustandsmodell (vereinfacht)

**Ticket.status**

- Open → Assigned → InProgress → Resolved
- Open/Assigned/InProgress → Cancelled

**QueueEntry.state**

- Waiting → Assigned → InProgress → Done
- Waiting/Assigned → Moved
- Waiting → Cancelled

---

## Datenmodell-Hinweis (damit es sauber skalieren kann)

- **Ticket** bleibt die «Quelle» für Inhalt/Status
- **QueueEntry** ist die «Quelle» für Reihenfolge/Priorität und kann häufig aktualisiert werden
- `computedPriority` ist ein Snapshot, damit die Queue stabil bleibt auch wenn Regeln später geändert werden
  (Regeländerungen gelten dann z.B. nur für neue Tickets oder nach Recompute-Job)

---

Wenn du willst, kann ich als nächsten Schritt eins davon liefern:

1. ein konkretes **ERD / SQL-Schema** (Postgres/MySQL) passend zu obigem Modell
2. eine **API-Schnittstelle** (REST oder JSON-RPC) inkl. Endpoints und Payloads
3. ein **Prioritäts- und Umverteilungs-Algorithmus** (fair, keine Starvation, gewichtete Warteschlange)
