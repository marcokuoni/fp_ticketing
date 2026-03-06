import { $, component$, useSignal, useStore, useVisibleTask$ } from '@builder.io/qwik';
import { api } from './api';
import type { QueueResponse, Room, TicketType } from './types';

export default component$(() => {
  const config = useStore<{ rooms: Room[]; ticketTypes: TicketType[] }>({ rooms: [], ticketTypes: [] });
  const selectedRoomId = useSignal('room-a');
  const queue = useStore<QueueResponse>({ roomId: 'room-a', waiting: [], assigned: [], inProgress: [] });
  const studentId = useSignal('student-001');
  const tutorId = useSignal('tutor-001');
  const selectedTypeId = useSignal('question');
  const topic = useSignal('Need help with exercise step 3');
  const errorMessage = useSignal('');

  const loadConfig = $(async () => {
    try {
      const data = await api.getConfig();
      config.rooms = data.rooms;
      config.ticketTypes = data.ticketTypes;
    } catch (error) {
      errorMessage.value = error instanceof Error ? error.message : 'Could not load configuration';
    }
  });

  const loadQueue = $(async () => {
    try {
      const data = await api.getQueue(selectedRoomId.value);
      queue.roomId = data.roomId;
      queue.waiting = data.waiting;
      queue.assigned = data.assigned;
      queue.inProgress = data.inProgress;
    } catch (error) {
      errorMessage.value = error instanceof Error ? error.message : 'Could not load queue';
    }
  });

  const createTicket = $(async () => {
    try {
      errorMessage.value = '';
      await api.createTicket({
        sessionId: 'session-1',
        roomId: selectedRoomId.value,
        studentId: studentId.value,
        ticketTypeId: selectedTypeId.value,
        topic: topic.value,
      });
      await loadQueue();
    } catch (error) {
      errorMessage.value = error instanceof Error ? error.message : 'Could not create ticket';
    }
  });

  const action = $(async (ticketId: string, kind: 'accept' | 'start' | 'resolve' | 'cancel') => {
    try {
      errorMessage.value = '';
      await api.ticketAction(ticketId, kind, tutorId.value);
      await loadQueue();
    } catch (error) {
      errorMessage.value = error instanceof Error ? error.message : 'Could not update ticket';
    }
  });

  useVisibleTask$(async ({ cleanup }) => {
    await loadConfig();
    await loadQueue();

    const es = api.subscribeEvents();
    es.onmessage = async () => {
      await loadQueue();
    };
    es.addEventListener('TicketCreated', async () => await loadQueue());
    es.addEventListener('TicketUpdated', async () => await loadQueue());

    cleanup(() => es.close());
  });

  return (
    <main>
      <h1>Exercise Help Queue System</h1>
      {errorMessage.value && <p class="error">{errorMessage.value}</p>}

      <section class="card">
        <h2>Create ticket (Student)</h2>
        <label>
          Student ID
          <input value={studentId.value} onInput$={(e) => (studentId.value = (e.target as HTMLInputElement).value)} />
        </label>
        <label>
          Room
          <select
            value={selectedRoomId.value}
            onChange$={async (e) => {
              selectedRoomId.value = (e.target as HTMLSelectElement).value;
              await loadQueue();
            }}
          >
            {config.rooms.map((room) => (
              <option key={room.id} value={room.id}>
                {room.name}
              </option>
            ))}
          </select>
        </label>
        <label>
          Ticket type
          <select
            value={selectedTypeId.value}
            onChange$={(e) => (selectedTypeId.value = (e.target as HTMLSelectElement).value)}
          >
            {config.ticketTypes
              .filter((ticketType) => ticketType.active)
              .map((ticketType) => (
                <option key={ticketType.id} value={ticketType.id}>
                  {ticketType.name}
                </option>
              ))}
          </select>
        </label>
        <label>
          Topic
          <input value={topic.value} onInput$={(e) => (topic.value = (e.target as HTMLInputElement).value)} />
        </label>
        <button onClick$={createTicket}>Create ticket</button>
      </section>

      <section class="card">
        <h2>Room queue (Tutor)</h2>
        <label>
          Tutor ID
          <input value={tutorId.value} onInput$={(e) => (tutorId.value = (e.target as HTMLInputElement).value)} />
        </label>

        <h3>Waiting</h3>
        {queue.waiting.map((ticket) => (
          <article key={ticket.id} class="ticket">
            <strong>{ticket.topic}</strong>
            <span>Student: {ticket.studentId}</span>
            <span>Priority: {ticket.computedPriority}</span>
            <div>
              <button onClick$={() => action(ticket.id, 'accept')}>Accept</button>
              <button onClick$={() => action(ticket.id, 'cancel')}>Cancel</button>
            </div>
          </article>
        ))}

        <h3>Assigned</h3>
        {queue.assigned.map((ticket) => (
          <article key={ticket.id} class="ticket">
            <strong>{ticket.topic}</strong>
            <span>Tutor: {ticket.assignedTutorId}</span>
            <div>
              <button onClick$={() => action(ticket.id, 'start')}>Start</button>
              <button onClick$={() => action(ticket.id, 'resolve')}>Resolve</button>
            </div>
          </article>
        ))}

        <h3>In progress</h3>
        {queue.inProgress.map((ticket) => (
          <article key={ticket.id} class="ticket">
            <strong>{ticket.topic}</strong>
            <button onClick$={() => action(ticket.id, 'resolve')}>Resolve</button>
          </article>
        ))}
      </section>
    </main>
  );
});
