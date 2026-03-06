import type { ConfigResponse, QueueResponse, Ticket } from './types';

const API_BASE = (import.meta.env.VITE_API_BASE as string | undefined) ?? 'http://localhost:3000';

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...(init?.headers ?? {}) },
    ...init,
  });

  if (!response.ok) {
    throw new Error(`Request failed (${response.status}) for ${path}`);
  }

  return (await response.json()) as T;
}

export const api = {
  getConfig: () => request<ConfigResponse>('/api/config'),
  getQueue: (roomId: string) => request<QueueResponse>(`/api/queues/${roomId}`),
  createTicket: (payload: {
    sessionId: string;
    roomId: string;
    studentId: string;
    ticketTypeId: string;
    topic: string;
  }) =>
    request<Ticket>('/api/tickets', {
      method: 'POST',
      body: JSON.stringify(payload),
    }),
  ticketAction: (ticketId: string, action: 'accept' | 'start' | 'resolve' | 'cancel', tutorId: string) =>
    request<Ticket>(`/api/tickets/${ticketId}/${action}`, {
      method: 'PATCH',
      body: JSON.stringify({ tutorId }),
    }),
  subscribeEvents: () => new EventSource(`${API_BASE}/api/events`),
};
