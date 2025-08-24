import { WebSocketServer } from 'ws';
import { createServer } from 'http';
import { randomBytes } from 'crypto';
import dotenv from 'dotenv';

dotenv.config();

const PORT = process.env.PORT || 8080;
const MAX_PLAYERS_PER_ROOM = 8;

// Create HTTP server
const server = createServer((req, res) => {
  // Health check endpoint
  if (req.url === '/health') {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('OK');
    return;
  }
  
  // CORS headers for browser requests
  res.writeHead(200, {
    'Content-Type': 'text/plain',
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type'
  });
  res.end('WebRTC Signaling Server');
});

// Create WebSocket server
const wss = new WebSocketServer({ server });

// Store rooms and clients
const rooms = new Map();
const clients = new Map();
// Reverse mapping for efficient WebSocket to Client lookup
const wsToClient = new WeakMap();

class Room {
  constructor(id, hostId) {
    this.id = id;
    this.hostId = hostId;
    this.players = new Set();
    this.createdAt = Date.now();
  }

  addPlayer(playerId) {
    if (this.players.size >= MAX_PLAYERS_PER_ROOM) {
      return false;
    }
    this.players.add(playerId);
    return true;
  }

  removePlayer(playerId) {
    this.players.delete(playerId);
    if (playerId === this.hostId && this.players.size > 0) {
      // Migrate host to another player
      this.hostId = Array.from(this.players)[0];
    }
  }

  isEmpty() {
    return this.players.size === 0;
  }

  toJSON() {
    return {
      id: this.id,
      hostId: this.hostId,
      players: Array.from(this.players),
      playerCount: this.players.size
    };
  }
}

class Client {
  constructor(ws, id) {
    this.ws = ws;
    this.id = id;
    this.roomId = null;
    this.isAlive = true;
  }

  send(message) {
    if (this.ws.readyState === 1) { // WebSocket.OPEN
      this.ws.send(JSON.stringify(message));
    }
  }

  joinRoom(roomId) {
    this.roomId = roomId;
  }

  leaveRoom() {
    this.roomId = null;
  }
}

// Handle WebSocket connections
wss.on('connection', (ws) => {
  const clientId = generateId();
  const client = new Client(ws, clientId);
  clients.set(clientId, client);
  wsToClient.set(ws, client);

  console.log(`Client connected: ${clientId}`);

  // Send welcome message
  client.send({
    type: 'welcome',
    id: clientId
  });

  // Set up ping/pong for connection health
  ws.on('pong', () => {
    client.isAlive = true;
  });

  ws.on('message', (data) => {
    try {
      const message = JSON.parse(data.toString());
      handleMessage(client, message);
    } catch (error) {
      console.error('Failed to parse message:', error);
      client.send({
        type: 'error',
        message: 'Invalid message format'
      });
    }
  });

  ws.on('close', () => {
    handleDisconnect(client);
  });

  ws.on('error', (error) => {
    console.error(`WebSocket error for client ${clientId}:`, error);
  });
});

function handleMessage(client, message) {
  switch (message.type) {
    case 'create-room':
      handleCreateRoom(client, message);
      break;
    
    case 'join-room':
      handleJoinRoom(client, message);
      break;
    
    case 'leave-room':
      handleLeaveRoom(client);
      break;
    
    case 'offer':
    case 'answer':
    case 'ice-candidate':
      handleSignaling(client, message);
      break;
    
    case 'get-room-info':
      handleGetRoomInfo(client, message);
      break;
    
    default:
      client.send({
        type: 'error',
        message: `Unknown message type: ${message.type}`
      });
  }
}

function handleCreateRoom(client, message) {
  const roomId = message.roomId || generateRoomId();
  
  if (rooms.has(roomId)) {
    client.send({
      type: 'error',
      message: 'Room already exists'
    });
    return;
  }

  const room = new Room(roomId, client.id);
  room.addPlayer(client.id);
  rooms.set(roomId, room);
  client.joinRoom(roomId);

  client.send({
    type: 'room-created',
    roomId: roomId,
    room: room.toJSON()
  });

  console.log(`Room created: ${roomId} by ${client.id}`);
}

function handleJoinRoom(client, message) {
  const { roomId } = message;
  
  if (!rooms.has(roomId)) {
    client.send({
      type: 'room-not-found',
      roomId: roomId
    });
    return;
  }

  const room = rooms.get(roomId);
  
  if (!room.addPlayer(client.id)) {
    client.send({
      type: 'room-full',
      roomId: roomId
    });
    return;
  }

  client.joinRoom(roomId);

  // Notify the joiner
  client.send({
    type: 'room-joined',
    roomId: roomId,
    room: room.toJSON()
  });

  // Notify existing players
  broadcastToRoom(roomId, {
    type: 'peer-joined',
    peerId: client.id
  }, client.id);

  console.log(`Client ${client.id} joined room ${roomId}`);
}

function handleLeaveRoom(client) {
  if (!client.roomId) return;

  const room = rooms.get(client.roomId);
  if (room) {
    room.removePlayer(client.id);
    
    if (room.isEmpty()) {
      rooms.delete(client.roomId);
      console.log(`Room ${client.roomId} deleted (empty)`);
    } else {
      // Notify remaining players
      broadcastToRoom(client.roomId, {
        type: 'peer-left',
        peerId: client.id
      }, client.id);
    }
  }

  console.log(`Client ${client.id} left room ${client.roomId}`);
  client.leaveRoom();
}

function handleSignaling(client, message) {
  const { to } = message;
  
  if (!to || !clients.has(to)) {
    client.send({
      type: 'error',
      message: 'Target peer not found'
    });
    return;
  }

  const targetClient = clients.get(to);
  
  // Forward the signaling message
  targetClient.send({
    ...message,
    from: client.id
  });
}

function handleGetRoomInfo(client, message) {
  const { roomId } = message;
  
  if (!rooms.has(roomId)) {
    client.send({
      type: 'room-not-found',
      roomId: roomId
    });
    return;
  }

  const room = rooms.get(roomId);
  client.send({
    type: 'room-info',
    room: room.toJSON()
  });
}

function handleDisconnect(client) {
  console.log(`Client disconnected: ${client.id}`);
  
  // Leave room if in one
  if (client.roomId) {
    handleLeaveRoom(client);
  }
  
  // Remove from clients map
  clients.delete(client.id);
}

function broadcastToRoom(roomId, message, excludeId = null) {
  const room = rooms.get(roomId);
  if (!room) return;

  room.players.forEach(playerId => {
    if (playerId !== excludeId) {
      const client = clients.get(playerId);
      if (client) {
        client.send(message);
      }
    }
  });
}

function generateId() {
  // Use crypto-secure random generation
  const timestamp = Date.now().toString(36);
  const random = randomBytes(8).toString('hex');
  return timestamp + random;
}

function generateRoomId() {
  // Generate crypto-secure 6-character room code
  const bytes = randomBytes(4);
  const hex = bytes.toString('hex').toUpperCase();
  return hex.slice(0, 6);
}

// Heartbeat to detect disconnected clients
const heartbeatInterval = setInterval(() => {
  wss.clients.forEach((ws) => {
    const client = wsToClient.get(ws);
    if (client) {
      if (!client.isAlive) {
        ws.terminate();
        handleDisconnect(client);
        return;
      }
      client.isAlive = false;
      ws.ping();
    }
  });
}, 30000);

// Clean up old empty rooms periodically
const cleanupInterval = setInterval(() => {
  const now = Date.now();
  const maxAge = 60 * 60 * 1000; // 1 hour
  
  rooms.forEach((room, roomId) => {
    if (room.isEmpty() && (now - room.createdAt) > maxAge) {
      rooms.delete(roomId);
      console.log(`Cleaned up old room: ${roomId}`);
    }
  });
}, 5 * 60 * 1000); // Every 5 minutes

// Start server
server.listen(PORT, () => {
  console.log(`Signaling server running on port ${PORT}`);
  console.log(`WebSocket endpoint: ws://localhost:${PORT}`);
  console.log(`Health check: http://localhost:${PORT}/health`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
  console.log('SIGTERM received, closing server...');
  
  clearInterval(heartbeatInterval);
  clearInterval(cleanupInterval);
  
  wss.close(() => {
    server.close(() => {
      console.log('Server closed');
      process.exit(0);
    });
  });
});