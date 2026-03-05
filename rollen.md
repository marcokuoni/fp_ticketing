# Mögliche Rollen im System

## 1. Student

**Zweck:** Hilfe anfordern und Übungen präsentieren.

**Rechte / Funktionen:**

- Ticket erstellen
- Ticket-Typ auswählen
  - Fragen zum Stoff
  - Aktuelle Übung präsentieren
  - Zukünftige Übung präsentieren
  - Vergangene fehlgeschlagene Übung präsentieren

- Priorität / Gewichtung sehen
- Warteschlangenstatus sehen
- Raum wechseln (wenn System ihn umverteilt)
- Ticket schliessen

**Optional:**

- Gruppenarbeit (mehrere Studenten pro Ticket)
- Upload von Code / Screenshots

---

## 2. Betreuer (Tutor / Teaching Assistant)

**Zweck:** Studenten helfen und Tickets bearbeiten.

**Rechte / Funktionen:**

- Tickets im eigenen Raum sehen
- Ticket annehmen
- Ticket abschliessen
- Studenten in den Raum rufen
- Ticket priorisieren (z.B. wenn schnell lösbar)
- Studenten in andere Räume verschieben

**Zusätzlich:**

- Warteschlange im Raum moderieren
- Ticket-Typ korrigieren

---

## 3. Raum-Betreuer (Lead Tutor / Raumverantwortlicher)

Falls mehrere Betreuer pro Raum existieren.

**Rechte / Funktionen:**

- Übersicht über alle Tickets im Raum
- Verteilung der Tickets an Betreuer
- Kapazität des Raumes setzen
- Überlast erkennen
- Tickets in andere Räume delegieren

---

## 4. Dozent / Lehrperson

**Zweck:** Überblick über das gesamte Übungssystem.

**Rechte / Funktionen:**

- Alle Räume sehen
- Warteschlangen global sehen
- Statistik sehen:
  - Wartezeiten
  - Tickettypen
  - Belastung pro Raum

- Tickets priorisieren
- Studenten global umverteilen

---

## 5. Übungsorganisation / Admin

Technische Systemadministration.

**Rechte / Funktionen:**

- Räume anlegen
- Betreuer zu Räumen zuweisen
- Tickettypen definieren
- Gewichtungen konfigurieren
- Dynamische Regeln definieren (z.B. Priorisierung)
- Systemparameter konfigurieren

---

# Systemrollen (technisch)

Wenn du das als Software baust, könnte das RBAC z.B. so aussehen:

| Rolle       | Beschreibung         |
| ----------- | -------------------- |
| `student`   | erstellt Tickets     |
| `tutor`     | bearbeitet Tickets   |
| `room_lead` | moderiert Raum       |
| `lecturer`  | globale Übersicht    |
| `admin`     | Systemadministration |

---

# Erweiterbare Rollen (optional)

## Queue Manager

Automatischer Dienst:

- berechnet Priorität
- verteilt Studenten auf Räume
- erkennt Überlast

## Observer

z.B. für neue Tutoren

- darf nur zuschauen
- keine Tickets bearbeiten

---

# Eventuell sinnvoll: Gruppenstruktur

**Raum**

- ID
- Name
- Kapazität
- aktuelle Betreuer
- Warteschlange

**Ticket**

- Student
- Tickettyp
- Gewicht
- Zeit
- Raum
- Status

---

# Beispiel-Szenario

1. Student erstellt Ticket
   → Typ: _Frage zum Stoff_

2. System berechnet Gewicht

3. Ticket kommt in **Raum-Queue**

4. Wenn Raum überlastet:

→ Student wird automatisch in anderen Raum verschoben

1. Tutor nimmt Ticket an

2. Ticket wird geschlossen

---

# Architektonischer Hinweis

Wichtig für dein System:

**Ticketgewicht = Funktion von**

```
gewicht =
  ticket_typ
  + wartezeit
  + student_gruppe
  + tutor_verfügbarkeit
```

Damit kannst du später **fair scheduling** implementieren.

---

Wenn du möchtest, kann ich dir auch noch:

- eine **saubere Domänenmodellierung (DDD)**
- ein **Datenbankschema**
- oder ein **Scheduling-Algorithmus für die Warteschlange**

für dieses System entwerfen. Das wäre für dein Projekt wahrscheinlich der nächste sinnvolle Schritt.
