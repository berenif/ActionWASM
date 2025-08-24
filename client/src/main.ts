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
  await gameLoader.init();
  
  // Initialize WebRTC for multiplayer
  const webrtcManager = new WebRTCManager();
  webrtcManager.init();
  
  // Handle room joining
  uiManager.onJoinRoom = (roomId: string) => {
    webrtcManager.joinRoom(roomId);
  };
  
  // Handle room creation
  uiManager.onCreateRoom = () => {
    const roomId = generateRoomId();
    webrtcManager.createRoom(roomId);
    uiManager.showRoomId(roomId);
  };
  
  // Start the game when ready
  gameLoader.onReady = () => {
    console.log('Game loaded and ready!');
    uiManager.hideLoader();
  };
}

function generateRoomId(): string {
  return Math.random().toString(36).substring(2, 8).toUpperCase();
}

// Start initialization when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}