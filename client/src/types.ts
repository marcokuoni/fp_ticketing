export type Room = { id: string; name: string; capacity: number };

export type TicketType = {
  id: string;
  name: string;
  baseWeight: number;
  active: boolean;
};

export type TicketStatus = 'Waiting' | 'Assigned' | 'InProgress' | 'Resolved' | 'Cancelled';

export type Ticket = {
  id: string;
  sessionId: string;
  roomId: string;
  studentId: string;
  ticketTypeId: string;
  topic: string;
  details?: string;
  status: TicketStatus;
  computedPriority: number;
  assignedTutorId?: string | null;
};

export type QueueResponse = {
  roomId: string;
  waiting: Ticket[];
  assigned: Ticket[];
  inProgress: Ticket[];
};

export type ConfigResponse = {
  rooms: Room[];
  ticketTypes: TicketType[];
};
