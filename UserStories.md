## US-1 Ticket ziehen

**Als Student** möchte ich ein Ticket erstellen können damit ich Hilfe bekomme.

**Use Case**

1. Student öffnet Ticket-UI.
2. Student wählt Ticket-Typ.
3. Student optional: Topic/Notiz eingeben.
4. System erstellt Ticket und reiht es in die Warteschlange eines Raums ein.

**Akzeptanzkriterien**

- Ticket-Typ ist Pflicht.
- Ticket wird einer aktiven Session zugeordnet.
- Ticket erhält Status `Open` und QueueEntry `Waiting`.
- Student sieht sofort seine Position oder eine Wartezeit-Schätzung (eine von beiden reicht im MVP).

---

## US-2 Ticket-Typen auswählen

**Als Student** möchte ich vordefinierte Ticket-Typen auswählen können damit die Betreuer wissen, worum es geht.

**Use Case**

1. Student klickt «Ticket erstellen».
2. System zeigt aktive Ticket-Typen an.
3. Student wählt einen Typ.

**Akzeptanzkriterien**

- Nur aktive Ticket-Typen werden angezeigt.
- Ticket-Typen sind ohne Deploy/Code-Change erweiterbar (z.B. in Admin-UI oder Config).

---

## US-3 Status meines Tickets sehen

**Als Student** möchte ich den Status meines Tickets sehen damit ich weiss, ob ich warten, bereit sein oder wechseln soll.

**Use Case**

1. Student öffnet «Mein Ticket».
2. System zeigt Status (Waiting/Assigned/InProgress/Done) und Raum.

**Akzeptanzkriterien**

- Status wird in Echtzeit oder via Polling aktualisiert (MVP: Polling OK).
- Wenn Ticket umverteilt wurde, sieht Student den neuen Raum.

---

## US-4 Ticket abbrechen

**Als Student** möchte ich mein Ticket abbrechen können falls ich es nicht mehr brauche.

**Use Case**

1. Student klickt «Ticket abbrechen».
2. System setzt Ticket auf `Cancelled` und entfernt es aus der aktiven Warteschlange.

**Akzeptanzkriterien**

- Abbrechen ist nur möglich, solange Ticket nicht `Resolved/Done` ist.
- Ticket verschwindet aus der Queue und wird nicht mehr Betreuern angeboten.

---

## US-5 Ticketliste im Raum sehen

**Als Betreuer** möchte ich die Warteschlange meines Raums sehen damit ich Tickets in sinnvoller Reihenfolge bearbeiten kann.

**Use Case**

1. Betreuer öffnet Raum-Ansicht.
2. System zeigt alle `Waiting` Tickets mit Typ, Wartezeit, Priorität.

**Akzeptanzkriterien**

- Sortierung ist konsistent (nach `computedPriority` und `enqueuedAt` als Tie-Breaker).
- Betreuer sieht mindestens: Ticket-Typ, Wartezeit, Student (oder anonymisierte ID).

---

## US-6 Ticket annehmen

**Als Betreuer** möchte ich ein Ticket annehmen können damit klar ist, wer sich darum kümmert.

**Use Case**

1. Betreuer klickt «Annehmen» bei einem Ticket.
2. System erstellt Assignment und setzt Ticket/QueueEntry Status auf `Assigned`.

**Akzeptanzkriterien**

- Ein Ticket kann nur von einem Betreuer gleichzeitig angenommen werden.
- Bei Race Conditions gewinnt genau ein Betreuer, andere erhalten «bereits vergeben».

---

## US-7 Ticket bearbeiten starten

**Als Betreuer** möchte ich ein angenommenes Ticket als «in Bearbeitung» markieren damit die Warteschlange korrekt bleibt.

**Use Case**

1. Betreuer klickt «Start».
2. System setzt Status auf `InProgress`.

**Akzeptanzkriterien**

- Start ist nur möglich, wenn Ticket `Assigned` ist.
- Student sieht «In Bearbeitung» und den Raum/Betreuer (optional).

---

## US-8 Ticket abschliessen

**Als Betreuer** möchte ich ein Ticket abschliessen können damit der Student aus der Warteschlange raus ist.

**Use Case**

1. Betreuer klickt «Abschliessen».
2. System setzt Ticket `Resolved` und QueueEntry `Done`.

**Akzeptanzkriterien**

- Abschliessen ist nur möglich, wenn Ticket `Assigned` oder `InProgress` ist.
- Ticket erscheint nicht mehr in der aktiven Queue.

---

## US-9 Dynamische Gewichtung anwenden

**Als System** möchte ich Tickets anhand einer konfigurierbaren Gewichtung priorisieren damit wichtige Anfragen schneller drankommen.

**Use Case**

1. Ticket wird erstellt.
2. System berechnet `computedPriority` aus Ticket-Typ-Basisgewicht und Wartezeit.
3. Queue sortiert danach.

**Akzeptanzkriterien**

- Ticket-Typ hat ein `baseWeight`.
- Wartezeit erhöht Priorität oder wirkt zumindest als Tie-Breaker (MVP: Tie-Breaker reicht).
- Änderung am `baseWeight` wirkt für neue Tickets (MVP) oder optional für alle via Recompute-Job (nicht nötig).

---

## US-10 Umverteilung bei Überlast

**Als System** möchte ich Studenten/Tickets in andere Räume verschieben wenn ein Raum überlastet ist damit die Gesamtauslastung ausgeglichen wird.

**Use Case**

1. System erkennt: Raum A ist über Kapazität (Queue-Länge oder aktive Tickets pro Betreuer).
2. System sucht Raum B mit freier Kapazität.
3. System verschiebt ein oder mehrere `Waiting` Tickets nach B.
4. Student bekommt Hinweis «Bitte in Raum B».

**Akzeptanzkriterien**

- Es werden nur Tickets im Status `Waiting` verschoben (MVP).
- Verschobene Tickets behalten ihre Reihenfolge-Fairness (z.B. enqueuedAt bleibt relevant).
- Student sieht den neuen Raum eindeutig.

---

## US-11 Admin: Räume und Betreuer konfigurieren

**Als Admin/Übungsorganisation** möchte ich Räume und Betreuer einer Session zuweisen können damit das System weiss, wer wo arbeitet.

**Use Case**

1. Admin legt Session an (Start/Ende).
2. Admin legt Räume an und setzt Kapazität.
3. Admin weist Betreuer Räumen zu.

**Akzeptanzkriterien**

- Session muss Running sein, damit Tickets gezogen werden können.
- Räume ohne Betreuer dürfen optional Tickets annehmen (MVP: entweder verhindern oder erlauben aber warnen, entscheide dich).

---

## US-12 Admin: Ticket-Typen pflegen

**Als Admin** möchte ich Ticket-Typen inkl. Gewichtung pflegen können damit neue Kategorien ohne Code-Change entstehen.

**Use Case**

1. Admin erstellt/editiert Ticket-Typ (Name, baseWeight, aktiv).
2. System zeigt neue Typen sofort beim Ticket erstellen.

**Akzeptanzkriterien**

- Ticket-Typ kann aktiviert/deaktiviert werden.
- baseWeight ist eine Zahl in definiertem Bereich (z.B. 0–100 oder -100–100).

---

# Minimales MVP-Release in 3 «Scheiben»

Damit du wirklich schnell zu einem funktionierenden System kommst:

1. **Basis Queue**

- US-1, US-2, US-3, US-5, US-6, US-8

1. **Lebenszyklus + Hygiene**

- US-4, US-7, US-9

1. **Lastverteilung + Admin**

- US-10, US-11, US-12

---

Wenn du willst, kann ich dir als nächstes daraus direkt ein **Use-Case-Diagramm (textuell)** oder ein **Backlog mit Priorität, Aufwand (S/M/L) und Abhängigkeiten** machen.
