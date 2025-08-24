import './style.css';
import './landing.css';
import { WebRTCManager } from './webrtc';
import { UIManager } from './ui';
import { GameLoader } from './game-loader';

// Game state
let gameStarted = false;
let gameMode: 'practice' | 'multiplayer' = 'practice';
let webrtcManager: WebRTCManager | null = null;
let uiManager: UIManager | null = null;
let gameLoader: GameLoader | null = null;

// Initialize landing page interactions
function initLandingPage() {
  const playButton = document.getElementById('play-button');
  const practiceButton = document.getElementById('practice-mode');
  const multiplayerButton = document.getElementById('multiplayer-mode');
  const landingPage = document.getElementById('landing-page');
  const gameContainer = document.getElementById('game-container');
  
  // Play button - starts practice mode by default
  playButton?.addEventListener('click', () => {
    startGame('practice');
  });
  
  // Practice mode button
  practiceButton?.addEventListener('click', () => {
    startGame('practice');
  });
  
  // Multiplayer mode button
  multiplayerButton?.addEventListener('click', () => {
    startGame('multiplayer');
  });
  
  // Keyboard shortcut - Enter to start
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && !gameStarted) {
      startGame('practice');
    }
  });
  
  // Menu button in game
  document.getElementById('menu-button')?.addEventListener('click', () => {
    returnToMenu();
  });
  
  // Restart button on death screen
  document.getElementById('restart-button')?.addEventListener('click', () => {
    restartGame();
  });
}

// Start the game
async function startGame(mode: 'practice' | 'multiplayer') {
  if (gameStarted) return;
  
  gameStarted = true;
  gameMode = mode;
  
  // Hide landing page, show game container
  const landingPage = document.getElementById('landing-page');
  const gameContainer = document.getElementById('game-container');
  
  if (landingPage) landingPage.style.display = 'none';
  if (gameContainer) gameContainer.style.display = 'block';
  
  // Initialize game
  await initGame();
  
  // If multiplayer mode, show connection UI
  if (mode === 'multiplayer' && uiManager) {
    uiManager.showMultiplayerUI();
  }
}

// Return to main menu
function returnToMenu() {
  const landingPage = document.getElementById('landing-page');
  const gameContainer = document.getElementById('game-container');
  
  if (landingPage) landingPage.style.display = 'block';
  if (gameContainer) gameContainer.style.display = 'none';
  
  // Clean up game
  if (gameLoader) {
    gameLoader.cleanup();
  }
  
  // Disconnect WebRTC if in multiplayer
  if (webrtcManager) {
    webrtcManager.disconnect();
  }
  
  gameStarted = false;
}

// Restart the game
function restartGame() {
  // Hide death screen
  const deathScreen = document.getElementById('death-screen');
  if (deathScreen) deathScreen.style.display = 'none';
  
  // Reload game
  if (gameLoader) {
    gameLoader.restart();
  }
  
  // Reset UI
  updateGameUI({
    health: 100,
    maxHealth: 100,
    room: 1,
    gold: 0,
    souls: 0
  });
}

// Initialize the game
async function initGame() {
  console.log('Initializing Roguelike ARPG...');
  
  // Create UI manager
  uiManager = new UIManager();
  uiManager.init();
  
  // Load WASM game module
  gameLoader = new GameLoader();
  
  // Initialize WebRTC for multiplayer
  if (gameMode === 'multiplayer') {
    webrtcManager = new WebRTCManager();
    setupWebRTC();
  }
  
  try {
    // Load the game
    await gameLoader.load();
    console.log('Game loaded successfully!');
    
    // Start the game
    gameLoader.start();
    
    // Set up game event listeners
    setupGameEventListeners();
    
  } catch (error) {
    console.error('Failed to load game:', error);
    uiManager.showError('Failed to load game. Please refresh the page.');
  }
}

// Set up WebRTC event handlers
function setupWebRTC() {
  if (!webrtcManager || !uiManager) return;
  
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
    uiManager.showError(error);
  };
}

// Set up game event listeners
function setupGameEventListeners() {
  // Listen for game events from WASM
  window.addEventListener('game-event', (event: any) => {
    const { type, data } = event.detail;
    
    switch (type) {
      case 'health-update':
        updateHealthBar(data.current, data.max);
        break;
      case 'room-change':
        showRoomTransition(data.roomNumber, data.roomType);
        break;
      case 'player-death':
        showDeathScreen(data);
        break;
      case 'stats-update':
        updateGameUI(data);
        break;
    }
  });
}

// Update health bar
function updateHealthBar(current: number, max: number) {
  const healthFill = document.querySelector('.health-fill') as HTMLElement;
  const healthText = document.querySelector('.health-text') as HTMLElement;
  
  if (healthFill) {
    const percentage = (current / max) * 100;
    healthFill.style.width = `${percentage}%`;
  }
  
  if (healthText) {
    healthText.textContent = `${current}/${max}`;
  }
}

// Show room transition
function showRoomTransition(roomNumber: number, roomType: string) {
  const transitionScreen = document.getElementById('room-transition');
  const roomNumberEl = document.getElementById('next-room-number');
  const roomTypeEl = document.querySelector('.room-type');
  
  if (transitionScreen && roomNumberEl && roomTypeEl) {
    roomNumberEl.textContent = roomNumber.toString();
    roomTypeEl.textContent = roomType;
    
    transitionScreen.style.display = 'flex';
    
    // Hide after 2 seconds
    setTimeout(() => {
      transitionScreen.style.display = 'none';
    }, 2000);
  }
}

// Show death screen
function showDeathScreen(stats: any) {
  const deathScreen = document.getElementById('death-screen');
  
  if (deathScreen) {
    // Update stats
    const roomsCleared = document.getElementById('rooms-cleared');
    const enemiesKilled = document.getElementById('enemies-killed');
    const goldCollected = document.getElementById('gold-collected');
    const runTime = document.getElementById('run-time');
    
    if (roomsCleared) roomsCleared.textContent = stats.roomsCleared || '0';
    if (enemiesKilled) enemiesKilled.textContent = stats.enemiesKilled || '0';
    if (goldCollected) goldCollected.textContent = stats.goldCollected || '0';
    if (runTime) {
      const minutes = Math.floor(stats.runTime / 60);
      const seconds = Math.floor(stats.runTime % 60);
      runTime.textContent = `${minutes}:${seconds.toString().padStart(2, '0')}`;
    }
    
    deathScreen.style.display = 'flex';
  }
}

// Update game UI
function updateGameUI(data: any) {
  if (data.room !== undefined) {
    const roomNumber = document.getElementById('room-number');
    if (roomNumber) roomNumber.textContent = data.room.toString();
  }
  
  if (data.gold !== undefined) {
    const gold = document.getElementById('gold');
    if (gold) gold.textContent = data.gold.toString();
  }
  
  if (data.souls !== undefined) {
    const souls = document.getElementById('souls');
    if (souls) souls.textContent = data.souls.toString();
  }
  
  if (data.health !== undefined && data.maxHealth !== undefined) {
    updateHealthBar(data.health, data.maxHealth);
  }
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
  initLandingPage();
});

// Handle page visibility changes
document.addEventListener('visibilitychange', () => {
  if (gameLoader) {
    if (document.hidden) {
      gameLoader.pause();
    } else {
      gameLoader.resume();
    }
  }
});

// Handle window resize
window.addEventListener('resize', () => {
  if (gameLoader) {
    gameLoader.handleResize();
  }
});