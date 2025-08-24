import SimplePeer from 'simple-peer';

export interface PeerData {
  id: string;
  peer: SimplePeer.Instance;
  connected: boolean;
}

export class WebRTCManager {
  private peers: Map<string, PeerData> = new Map();
  private signalingSocket: WebSocket | null = null;
  private localId: string;
  private roomId: string | null = null;
  private isHost: boolean = false;
  private reconnectAttempts: number = 0;
  private maxReconnectAttempts: number = 5;

  constructor() {
    this.localId = this.generateId();
  }

  init() {
    this.connectToSignalingServer();
  }

  private connectToSignalingServer() {
    const wsUrl = this.getSignalingUrl();
    
    try {
      this.signalingSocket = new WebSocket(wsUrl);
      
      this.signalingSocket.onopen = () => {
        console.log('Connected to signaling server');
        this.reconnectAttempts = 0;
        this.sendSignalingMessage({
          type: 'register',
          id: this.localId
        });
      };

      this.signalingSocket.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          this.handleSignalingMessage(message);
        } catch (error) {
          console.error('Failed to parse signaling message:', error);
        }
      };

      this.signalingSocket.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

      this.signalingSocket.onclose = () => {
        console.log('Disconnected from signaling server');
        // Attempt to reconnect with exponential backoff
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
          const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 10000);
          this.reconnectAttempts++;
          console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
          setTimeout(() => this.connectToSignalingServer(), delay);
        }
      };
    } catch (error) {
      console.error('Failed to connect to signaling server:', error);
    }
  }

  private getSignalingUrl(): string {
    const hostname = window.location.hostname;
    if (hostname === 'localhost' || hostname === '127.0.0.1') {
      return 'ws://localhost:8080';
    }
    // For production, use a deployed signaling server
    // You can use a free service like Glitch or Railway for the signaling server
    return 'wss://your-signaling-server.herokuapp.com';
  }

  createRoom(roomId: string) {
    this.roomId = roomId;
    this.isHost = true;
    this.sendSignalingMessage({
      type: 'create-room',
      roomId,
      hostId: this.localId
    });
  }

  joinRoom(roomId: string) {
    this.roomId = roomId;
    this.isHost = false;
    this.sendSignalingMessage({
      type: 'join-room',
      roomId,
      peerId: this.localId
    });
  }

  private handleSignalingMessage(message: any) {
    switch (message.type) {
      case 'welcome':
        console.log('Registered with ID:', message.id);
        if (message.id) {
          this.localId = message.id;
        }
        break;

      case 'room-created':
        console.log('Room created:', message.roomId);
        this.onRoomCreated?.(message.roomId);
        break;
      
      case 'room-joined':
        console.log('Joined room:', message.roomId);
        this.onRoomJoined?.(message.roomId);
        break;

      case 'peer-joined':
        this.handlePeerJoined(message.peerId);
        break;
      
      case 'peer-left':
        this.handlePeerLeft(message.peerId);
        break;
      
      case 'offer':
        this.handleOffer(message);
        break;
      
      case 'answer':
        this.handleAnswer(message);
        break;
      
      case 'ice-candidate':
        this.handleIceCandidate(message);
        break;
      
      case 'room-full':
        console.error('Room is full');
        this.onError?.('Room is full');
        break;
      
      case 'room-not-found':
        console.error('Room not found');
        this.onError?.('Room not found');
        break;

      case 'error':
        console.error('Signaling error:', message.message);
        this.onError?.(message.message);
        break;
    }
  }

  private handlePeerJoined(peerId: string) {
    console.log('Peer joined:', peerId);
    
    // Avoid creating duplicate connections
    if (this.peers.has(peerId)) {
      console.warn('Peer already exists:', peerId);
      return;
    }
    
    // Create a new peer connection as initiator
    const peer = new SimplePeer({
      initiator: true,
      trickle: true,
      config: {
        iceServers: [
          { urls: 'stun:stun.l.google.com:19302' },
          { urls: 'stun:stun1.l.google.com:19302' },
          { urls: 'stun:stun2.l.google.com:19302' }
        ]
      }
    });

    this.setupPeerHandlers(peer, peerId);
    
    this.peers.set(peerId, {
      id: peerId,
      peer,
      connected: false
    });
  }

  private handlePeerLeft(peerId: string) {
    console.log('Peer left:', peerId);
    const peerData = this.peers.get(peerId);
    if (peerData) {
      peerData.peer.destroy();
      this.peers.delete(peerId);
      this.onPeerDisconnected?.(peerId);
    }
  }

  private handleOffer(message: any) {
    const { from, signal } = message;
    
    // Avoid creating duplicate connections
    if (this.peers.has(from)) {
      console.warn('Peer already exists:', from);
      return;
    }
    
    // Create a new peer connection as responder
    const peer = new SimplePeer({
      initiator: false,
      trickle: true,
      config: {
        iceServers: [
          { urls: 'stun:stun.l.google.com:19302' },
          { urls: 'stun:stun1.l.google.com:19302' },
          { urls: 'stun:stun2.l.google.com:19302' }
        ]
      }
    });

    this.setupPeerHandlers(peer, from);
    
    this.peers.set(from, {
      id: from,
      peer,
      connected: false
    });

    // Signal the offer to establish connection
    try {
      peer.signal(signal);
    } catch (error) {
      console.error('Failed to signal offer:', error);
    }
  }

  private handleAnswer(message: any) {
    const { from, signal } = message;
    const peerData = this.peers.get(from);
    if (peerData) {
      try {
        peerData.peer.signal(signal);
      } catch (error) {
        console.error('Failed to signal answer:', error);
      }
    }
  }

  private handleIceCandidate(message: any) {
    const { from, candidate } = message;
    const peerData = this.peers.get(from);
    if (peerData && candidate) {
      try {
        peerData.peer.signal({ candidate });
      } catch (error) {
        console.error('Failed to add ICE candidate:', error);
      }
    }
  }

  private setupPeerHandlers(peer: SimplePeer.Instance, peerId: string) {
    peer.on('signal', (signal) => {
      // Determine message type based on signal content
      let messageType = 'ice-candidate';
      if (signal.type === 'offer') messageType = 'offer';
      else if (signal.type === 'answer') messageType = 'answer';
      
      this.sendSignalingMessage({
        type: messageType,
        to: peerId,
        from: this.localId,
        signal: signal.type ? signal : undefined,
        candidate: signal.candidate
      });
    });

    peer.on('connect', () => {
      console.log('Connected to peer:', peerId);
      const peerData = this.peers.get(peerId);
      if (peerData) {
        peerData.connected = true;
      }
      this.onPeerConnected?.(peerId);
    });

    peer.on('data', (data: any) => {
      this.handlePeerData(peerId, data);
    });

    peer.on('error', (err) => {
      console.error(`Peer error with ${peerId}:`, err);
      this.onPeerError?.(peerId, err.message);
    });

    peer.on('close', () => {
      console.log('Peer connection closed:', peerId);
      this.peers.delete(peerId);
      this.onPeerDisconnected?.(peerId);
    });
  }

  private handlePeerData(peerId: string, data: any) {
    try {
      const message = JSON.parse(data.toString());
      this.onPeerMessage?.(peerId, message);
    } catch (error) {
      console.error('Failed to parse peer data:', error);
    }
  }

  sendToPeer(peerId: string, data: any) {
    const peerData = this.peers.get(peerId);
    if (peerData && peerData.connected) {
      try {
        peerData.peer.send(JSON.stringify(data));
      } catch (error) {
        console.error(`Failed to send to peer ${peerId}:`, error);
      }
    }
  }

  broadcastToAll(data: any) {
    const message = JSON.stringify(data);
    this.peers.forEach((peerData) => {
      if (peerData.connected) {
        try {
          peerData.peer.send(message);
        } catch (error) {
          console.error(`Failed to broadcast to peer ${peerData.id}:`, error);
        }
      }
    });
  }

  private sendSignalingMessage(message: any) {
    if (this.signalingSocket && this.signalingSocket.readyState === WebSocket.OPEN) {
      try {
        this.signalingSocket.send(JSON.stringify(message));
      } catch (error) {
        console.error('Failed to send signaling message:', error);
      }
    } else {
      console.warn('Cannot send message: WebSocket not connected');
    }
  }

  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  // Event handlers (to be overridden)
  onPeerConnected?: (peerId: string) => void;
  onPeerDisconnected?: (peerId: string) => void;
  onPeerMessage?: (peerId: string, message: any) => void;
  onPeerError?: (peerId: string, error: string) => void;
  onRoomCreated?: (roomId: string) => void;
  onRoomJoined?: (roomId: string) => void;
  onError?: (error: string) => void;

  // Public getters
  get connectedPeers(): string[] {
    return Array.from(this.peers.values())
      .filter(p => p.connected)
      .map(p => p.id);
  }

  get isConnected(): boolean {
    return this.signalingSocket?.readyState === WebSocket.OPEN;
  }

  get peerId(): string {
    return this.localId;
  }

  // Cleanup method
  destroy() {
    // Close all peer connections
    this.peers.forEach(peerData => {
      peerData.peer.destroy();
    });
    this.peers.clear();

    // Close WebSocket connection
    if (this.signalingSocket) {
      this.signalingSocket.close();
      this.signalingSocket = null;
    }
  }
}