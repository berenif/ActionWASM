export class UIManager {
  private container: HTMLElement | null = null;
  private menuElement: HTMLElement | null = null;
  private gameCanvas: HTMLCanvasElement | null = null;
  private loaderElement: HTMLElement | null = null;
  
  // Event callbacks
  public onJoinRoom: ((roomId: string) => void) | null = null;
  public onCreateRoom: (() => void) | null = null;

  init() {
    this.createHTML();
    this.attachEventListeners();
  }

  private createHTML() {
    // Get or create main container
    this.container = document.getElementById('app') || document.body;
    
    // Clear container
    this.container.innerHTML = '';
    
    // Create the HTML structure
    this.container.innerHTML = `
      <div id="game-container">
        <!-- Loading Screen -->
        <div id="loader" class="loader-container">
          <div class="loader">
            <div class="loader-spinner"></div>
            <p>Loading game assets...</p>
          </div>
        </div>

        <!-- Main Menu -->
        <div id="menu" class="menu-container">
          <div class="menu-content">
            <h1 class="game-title">WASM Multiplayer Arena</h1>
            <div class="menu-buttons">
              <button id="create-room-btn" class="menu-btn primary">
                <span class="btn-icon">🎮</span>
                Create Room
              </button>
              <div class="join-section">
                <input 
                  type="text" 
                  id="room-input" 
                  class="room-input" 
                  placeholder="Enter Room Code"
                  maxlength="6"
                  pattern="[A-Z0-9]{6}"
                />
                <button id="join-room-btn" class="menu-btn secondary">
                  <span class="btn-icon">🔗</span>
                  Join Room
                </button>
              </div>
              <button id="single-player-btn" class="menu-btn tertiary">
                <span class="btn-icon">👤</span>
                Practice Mode
              </button>
            </div>
            <div id="room-info" class="room-info hidden">
              <p>Room Code:</p>
              <div class="room-code" id="room-code"></div>
              <button id="copy-code-btn" class="copy-btn">Copy</button>
            </div>
          </div>
        </div>

        <!-- Game Canvas -->
        <canvas id="game-canvas"></canvas>

        <!-- In-Game UI Overlay -->
        <div id="game-ui" class="game-ui hidden">
          <div class="game-stats">
            <div class="stat-item">
              <span class="stat-label">Players:</span>
              <span id="player-count" class="stat-value">1</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Score:</span>
              <span id="score" class="stat-value">0</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Time:</span>
              <span id="game-time" class="stat-value">0:00</span>
            </div>
          </div>
          <div class="connection-status" id="connection-status">
            <span class="status-indicator"></span>
            <span class="status-text">Connecting...</span>
          </div>
        </div>

        <!-- Controls Help -->
        <div id="controls-help" class="controls-help">
          <h3>Controls</h3>
          <div class="control-item">
            <kbd>W</kbd><kbd>A</kbd><kbd>S</kbd><kbd>D</kbd> or <kbd>↑</kbd><kbd>←</kbd><kbd>↓</kbd><kbd>→</kbd> - Move
          </div>
          <div class="control-item">
            <kbd>Space</kbd> - Action
          </div>
          <div class="control-item">
            <kbd>Esc</kbd> - Menu
          </div>
        </div>
      </div>
    `;

    // Store references
    this.menuElement = document.getElementById('menu');
    this.gameCanvas = document.getElementById('game-canvas') as HTMLCanvasElement;
    this.loaderElement = document.getElementById('loader');
  }

  private attachEventListeners() {
    // Create room button
    const createBtn = document.getElementById('create-room-btn');
    createBtn?.addEventListener('click', () => {
      if (this.onCreateRoom) {
        this.onCreateRoom();
        this.showGame();
      }
    });

    // Join room button
    const joinBtn = document.getElementById('join-room-btn');
    const roomInput = document.getElementById('room-input') as HTMLInputElement;
    
    joinBtn?.addEventListener('click', () => {
      const roomId = roomInput?.value.trim().toUpperCase();
      if (roomId && roomId.length === 6 && this.onJoinRoom) {
        this.onJoinRoom(roomId);
        this.showGame();
      } else {
        this.showError('Please enter a valid 6-character room code');
      }
    });

    // Enter key in room input
    roomInput?.addEventListener('keypress', (e) => {
      if (e.key === 'Enter') {
        joinBtn?.click();
      }
    });

    // Single player button
    const singleBtn = document.getElementById('single-player-btn');
    singleBtn?.addEventListener('click', () => {
      this.showGame();
    });

    // Copy room code button
    const copyBtn = document.getElementById('copy-code-btn');
    copyBtn?.addEventListener('click', () => {
      const roomCode = document.getElementById('room-code')?.textContent;
      if (roomCode) {
        navigator.clipboard.writeText(roomCode).then(() => {
          copyBtn.textContent = 'Copied!';
          setTimeout(() => {
            copyBtn.textContent = 'Copy';
          }, 2000);
        });
      }
    });

    // ESC key to show menu
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        this.toggleMenu();
      }
    });
  }

  showRoomId(roomId: string) {
    const roomInfo = document.getElementById('room-info');
    const roomCode = document.getElementById('room-code');
    
    if (roomInfo && roomCode) {
      roomCode.textContent = roomId;
      roomInfo.classList.remove('hidden');
    }
  }

  showGame() {
    if (this.menuElement) {
      this.menuElement.classList.add('hidden');
    }
    
    const gameUI = document.getElementById('game-ui');
    if (gameUI) {
      gameUI.classList.remove('hidden');
    }

    const controlsHelp = document.getElementById('controls-help');
    if (controlsHelp) {
      setTimeout(() => {
        controlsHelp.style.opacity = '0';
        setTimeout(() => {
          controlsHelp.style.display = 'none';
        }, 500);
      }, 5000);
    }
  }

  showMenu() {
    if (this.menuElement) {
      this.menuElement.classList.remove('hidden');
    }
    
    const gameUI = document.getElementById('game-ui');
    if (gameUI) {
      gameUI.classList.add('hidden');
    }
  }

  toggleMenu() {
    if (this.menuElement?.classList.contains('hidden')) {
      this.showMenu();
    } else {
      this.showGame();
    }
  }

  hideLoader() {
    if (this.loaderElement) {
      this.loaderElement.style.opacity = '0';
      setTimeout(() => {
        if (this.loaderElement) {
          this.loaderElement.style.display = 'none';
        }
      }, 300);
    }
  }

  updateConnectionStatus(connected: boolean, text?: string) {
    const statusElement = document.getElementById('connection-status');
    if (statusElement) {
      const indicator = statusElement.querySelector('.status-indicator');
      const statusText = statusElement.querySelector('.status-text');
      
      if (indicator) {
        indicator.className = `status-indicator ${connected ? 'connected' : 'disconnected'}`;
      }
      
      if (statusText) {
        statusText.textContent = text || (connected ? 'Connected' : 'Disconnected');
      }
    }
  }

  updatePlayerCount(count: number) {
    const playerCount = document.getElementById('player-count');
    if (playerCount) {
      playerCount.textContent = count.toString();
    }
  }

  updateScore(score: number) {
    const scoreElement = document.getElementById('score');
    if (scoreElement) {
      scoreElement.textContent = score.toString();
    }
  }

  updateGameTime(seconds: number) {
    const timeElement = document.getElementById('game-time');
    if (timeElement) {
      const minutes = Math.floor(seconds / 60);
      const secs = seconds % 60;
      timeElement.textContent = `${minutes}:${secs.toString().padStart(2, '0')}`;
    }
  }

  private showError(message: string) {
    // Create and show error toast
    const toast = document.createElement('div');
    toast.className = 'error-toast';
    toast.textContent = message;
    document.body.appendChild(toast);
    
    setTimeout(() => {
      toast.classList.add('show');
    }, 10);
    
    setTimeout(() => {
      toast.classList.remove('show');
      setTimeout(() => {
        document.body.removeChild(toast);
      }, 300);
    }, 3000);
  }
}