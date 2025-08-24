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
        this.sendSignalingMessage({
          type: 'register',
          id: this.localId
        });
      };

      this.signalingSocket.onmessage = (event) => {
        this.handleSignalingMessage(JSON.parse(event.data));
      };

      this.signalingSocket.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

      this.signalingSocket.onclose = () => {
        console.log('Disconnected from signaling server');
        // Attempt to reconnect after 3 seconds
        setTimeout(() => this.connectToSignalingServer(), 3000);
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
      case 'room-created':
        console.log('Room created:', message.roomId);
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
        break;
      
      case 'room-not-found':
        console.error('Room not found');
        break;
    }
  }

  private handlePeerJoined(peerId: string) {
    console.log('Peer joined:', peerId);
    
    // Create a new peer connection as initiator
    const peer = new SimplePeer({
      initiator: true,
      trickle: true,
      config: {
        iceServers: [
          { urls: 'stun:stun.l.google.com:19302' },
          { urls: 'stun:stun1.l.google.com:19302' }
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
    }
  }

  private handleOffer(message: any) {
    const { from, signal } = message;
    
    // Create a new peer connection as responder
    const peer = new SimplePeer({
      initiator: false,
      trickle: true,
      config: {
        iceServers: [
          { urls: 'stun:stun.l.google.com:19302' },
          { urls: 'stun:stun1.l.google.com:19302' }
        ]
      }
    });

    this.setupPeerHandlers(peer, from);
    
    this.peers.set(from, {
      id: from,
      peer,
      connected: false
    });

    peer.signal(signal);
  }

  private handleAnswer(message: any) {
    const { from, signal } = message;
    const peerData = this.peers.get(from);
    if (peerData) {
      peerData.peer.signal(signal);
    }
  }

  private handleIceCandidate(message: any) {
    const { from, candidate } = message;
    const peerData = this.peers.get(from);
    if (peerData) {
      peerData.peer.signal({ candidate });
    }
  }

  private setupPeerHandlers(peer: SimplePeer.Instance, peerId: string) {
    peer.on('signal', (signal) => {
      this.sendSignalingMessage({
        type: signal.type === 'offer' ? 'offer' : signal.type === 'answer' ? 'answer' : 'ice-candidate',
        to: peerId,
        from: this.localId,
        signal: signal.type === 'offer' || signal.type === 'answer' ? signal : undefined,
        candidate: signal.candidate
      });
    });

    peer.on('connect', () => {
      console.log('Connected to peer:', peerId);
      const peerData = this.peers.get(peerId);
      if (peerData) {
        peerData.connected = true;
      }
      this.onPeerConnected(peerId);
    });

    peer.on('data', (data: any) => {
      this.handlePeerData(peerId, data);
    });

    peer.on('error', (err) => {
      console.error('Peer error:', err);
    });

    peer.on('close', () => {
      console.log('Peer connection closed:', peerId);
      this.peers.delete(peerId);
    });
  }

  private handlePeerData(peerId: string, data: any) {
    try {
      const message = JSON.parse(data.toString());
      this.onPeerMessage(peerId, message);
    } catch (error) {
      console.error('Failed to parse peer data:', error);
    }
  }

  sendToPeer(peerId: string, data: any) {
    const peerData = this.peers.get(peerId);
    if (peerData && peerData.connected) {
      peerData.peer.send(JSON.stringify(data));
    }
  }

  broadcastToAll(data: any) {
    const message = JSON.stringify(data);
    this.peers.forEach((peerData) => {
      if (peerData.connected) {
        peerData.peer.send(message);
      }
    });
  }

  private sendSignalingMessage(message: any) {
    if (this.signalingSocket && this.signalingSocket.readyState === WebSocket.OPEN) {
      this.signalingSocket.send(JSON.stringify(message));
    }
  }

  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  // Event handlers (to be overridden)
  onPeerConnected(peerId: string) {
    // Override this method to handle peer connections
  }

  onPeerMessage(peerId: string, message: any) {
    // Override this method to handle peer messages
  }

  // Public getters
  get connectedPeers(): string[] {
    return Array.from(this.peers.values())
      .filter(p => p.connected)
      .map(p => p.id);
  }

  get isConnected(): boolean {
    return this.signalingSocket?.readyState === WebSocket.OPEN;
  }
}