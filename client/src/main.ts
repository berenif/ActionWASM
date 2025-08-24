import './style.css';
import { WebRTCManager } from './webrtc';
import { UIManager } from './ui';
import { GameLoader } from './game-loader';

// Initialize the game
async function init() {
  console.log('Initializing WASM Multiplayer Game...');
  
  // Create UI manager
  const uiManager = new UIManager();
  uiManager.init();
  
  // Load WASM game module
  const gameLoader = new GameLoader();
  
  // Initialize WebRTC for multiplayer
  const webrtcManager = new WebRTCManager();
  
  // Set up WebRTC event handlers
  webrtcManager.onPeerConnected = (peerId: string) => {
    console.log('Peer connected:', peerId);
    uiManager.updatePlayerCount(webrtcManager.connectedPeers.length + 1);
    uiManager.updateConnectionStatus(true, 'Connected');
    
    // Send initial game state to new peer
    webrtcManager.sendToPeer(peerId, {
      type: 'welcome',
      playerId: webrtcManager.peerId
    });
  };
  
  webrtcManager.onPeerDisconnected = (peerId: string) => {
    console.log('Peer disconnected:', peerId);
    uiManager.updatePlayerCount(webrtcManager.connectedPeers.length + 1);
    
    if (webrtcManager.connectedPeers.length === 0) {
      uiManager.updateConnectionStatus(false, 'No peers connected');
    }
  };
  
  webrtcManager.onPeerMessage = (peerId: string, message: any) => {
    console.log('Message from peer:', peerId, message);
    // Handle game messages here
    if (gameLoader && message.type === 'game-update') {
      gameLoader.updateGameState(message.data);
    }
  };
  
  webrtcManager.onError = (error: string) => {
    console.error('WebRTC error:', error);
    uiManager.updateConnectionStatus(false, error);
  };
  
  webrtcManager.onRoomCreated = (roomId: string) => {
    uiManager.showRoomId(roomId);
  };
  
  webrtcManager.onRoomJoined = (roomId: string) => {
    console.log('Successfully joined room:', roomId);
  };
  
  // Initialize WebRTC connection
  webrtcManager.init();
  
  // Update connection status periodically
  setInterval(() => {
    const isConnected = webrtcManager.isConnected;
    if (isConnected) {
      const peerCount = webrtcManager.connectedPeers.length;
      const status = peerCount > 0 ? `${peerCount} peer${peerCount !== 1 ? 's' : ''} connected` : 'Connected to server';
      uiManager.updateConnectionStatus(true, status);
    } else {
      uiManager.updateConnectionStatus(false, 'Connecting...');
    }
  }, 2000);
  
  // Handle room joining
  uiManager.onJoinRoom = (roomId: string) => {
    if (!roomId || roomId.length !== 6) {
      console.error('Invalid room ID');
      return;
    }
    webrtcManager.joinRoom(roomId.toUpperCase());
  };
  
  // Handle room creation
  uiManager.onCreateRoom = () => {
    const roomId = generateRoomId();
    webrtcManager.createRoom(roomId);
  };
  
  // Initialize game loader
  try {
    await gameLoader.init();
    
    // Start the game when ready
    gameLoader.onReady = () => {
      console.log('Game loaded and ready!');
      uiManager.hideLoader();
      
      // Start game timer
      let gameTime = 0;
      setInterval(() => {
        gameTime++;
        uiManager.updateGameTime(gameTime);
      }, 1000);
    };
  } catch (error) {
    console.error('Failed to initialize game:', error);
    uiManager.hideLoader();
    // Game will fall back to JavaScript implementation
  }
  
  // Handle page unload
  window.addEventListener('beforeunload', () => {
    webrtcManager.destroy();
  });
}

function generateRoomId(): string {
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
  let roomId = '';
  for (let i = 0; i < 6; i++) {
    roomId += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return roomId;
}

// Error handling
window.addEventListener('error', (event) => {
  console.error('Global error:', event.error);
});

window.addEventListener('unhandledrejection', (event) => {
  console.error('Unhandled promise rejection:', event.reason);
});

// Start initialization when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init().catch(console.error);
}