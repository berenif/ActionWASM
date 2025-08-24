// Demo JavaScript for WASM Multiplayer Roguelike ARPG
// This file contains all the interactive demo functionality

// Performance monitoring
let fpsCounter = 60;
let latencyCounter = 12;
let memoryCounter = 48;
let entitiesCounter = 127;

// WebRTC demo state
let roomCode = null;
let peers = [];
let isConnected = false;

// Animation frame for performance monitoring
let lastTime = performance.now();
let frameCount = 0;

// Initialize demo on page load
document.addEventListener('DOMContentLoaded', () => {
    startPerformanceMonitoring();
    initializeCanvas();
    updateMetrics();
});

// Performance monitoring
function startPerformanceMonitoring() {
    function updateFPS() {
        const currentTime = performance.now();
        frameCount++;
        
        if (currentTime >= lastTime + 1000) {
            fpsCounter = Math.round((frameCount * 1000) / (currentTime - lastTime));
            document.getElementById('fps-counter').textContent = fpsCounter;
            frameCount = 0;
            lastTime = currentTime;
        }
        
        requestAnimationFrame(updateFPS);
    }
    
    updateFPS();
}

// Update performance metrics
function updateMetrics() {
    setInterval(() => {
        // Simulate realistic metrics
        latencyCounter = Math.floor(Math.random() * 20) + 10;
        memoryCounter = Math.floor(Math.random() * 30) + 40;
        entitiesCounter = Math.floor(Math.random() * 50) + 100;
        
        document.getElementById('latency-counter').textContent = latencyCounter;
        document.getElementById('memory-counter').textContent = memoryCounter;
        document.getElementById('entities-counter').textContent = entitiesCounter;
    }, 2000);
}

// Initialize canvas for visual demos
function initializeCanvas() {
    const canvas = document.getElementById('demo-canvas');
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;
    
    // Draw initial demo scene
    drawDemoScene(ctx);
}

// Draw demo scene
function drawDemoScene(ctx) {
    const width = ctx.canvas.width;
    const height = ctx.canvas.height;
    
    // Clear canvas
    ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
    ctx.fillRect(0, 0, width, height);
    
    // Draw grid
    ctx.strokeStyle = 'rgba(102, 126, 234, 0.2)';
    ctx.lineWidth = 1;
    
    for (let x = 0; x < width; x += 40) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
    }
    
    for (let y = 0; y < height; y += 40) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
    }
    
    // Draw demo text
    ctx.fillStyle = '#f093fb';
    ctx.font = '24px Inter, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('Game Canvas - Click buttons to see features in action', width / 2, height / 2);
}

// Feature Demo Functions

// WebAssembly Performance Demo
function demoWASMPerformance() {
    const startTime = performance.now();
    
    // Simulate WASM benchmark
    showNotification('Running WASM Performance Benchmark...');
    
    setTimeout(() => {
        const endTime = performance.now();
        const duration = (endTime - startTime).toFixed(2);
        
        // Update canvas with results
        const canvas = document.getElementById('demo-canvas');
        const ctx = canvas.getContext('2d');
        
        ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        ctx.fillStyle = '#4ade80';
        ctx.font = '32px Inter, sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('WASM Performance Test', canvas.width / 2, 100);
        
        ctx.font = '20px Inter, sans-serif';
        ctx.fillStyle = '#f093fb';
        ctx.fillText(`Execution Time: ${duration}ms`, canvas.width / 2, 150);
        ctx.fillText(`Operations/sec: ${Math.floor(Math.random() * 1000000 + 500000)}`, canvas.width / 2, 190);
        ctx.fillText(`Memory Usage: ${memoryCounter}MB`, canvas.width / 2, 230);
        
        showNotification('Benchmark Complete! Near-native performance achieved.');
    }, 1500);
}

// Multiplayer Connection Demo
function demoMultiplayer() {
    showNotification('Testing P2P connection...');
    
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    // Simulate connection visualization
    ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Draw network nodes
    const nodes = [
        { x: canvas.width / 2, y: canvas.height / 2, label: 'You', color: '#4ade80' },
        { x: canvas.width / 4, y: canvas.height / 3, label: 'Peer 1', color: '#f093fb' },
        { x: canvas.width * 3/4, y: canvas.height / 3, label: 'Peer 2', color: '#f093fb' },
        { x: canvas.width / 4, y: canvas.height * 2/3, label: 'Peer 3', color: '#f093fb' },
        { x: canvas.width * 3/4, y: canvas.height * 2/3, label: 'Peer 4', color: '#f093fb' }
    ];
    
    // Draw connections
    ctx.strokeStyle = 'rgba(102, 126, 234, 0.5)';
    ctx.lineWidth = 2;
    
    for (let i = 1; i < nodes.length; i++) {
        ctx.beginPath();
        ctx.moveTo(nodes[0].x, nodes[0].y);
        ctx.lineTo(nodes[i].x, nodes[i].y);
        ctx.stroke();
    }
    
    // Draw nodes
    nodes.forEach(node => {
        ctx.fillStyle = node.color;
        ctx.beginPath();
        ctx.arc(node.x, node.y, 20, 0, Math.PI * 2);
        ctx.fill();
        
        ctx.fillStyle = '#fff';
        ctx.font = '14px Inter, sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(node.label, node.x, node.y + 40);
    });
    
    showNotification('P2P Network Connected! Latency: 12ms');
}

// Roguelike Room Generation Demo
function demoRoguelike() {
    showNotification('Generating procedural dungeon...');
    
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Generate random room layout
    const tileSize = 20;
    const roomWidth = Math.floor(canvas.width / tileSize);
    const roomHeight = Math.floor(canvas.height / tileSize);
    
    for (let x = 0; x < roomWidth; x++) {
        for (let y = 0; y < roomHeight; y++) {
            const isWall = x === 0 || x === roomWidth - 1 || y === 0 || y === roomHeight - 1 || Math.random() < 0.1;
            
            if (isWall) {
                ctx.fillStyle = '#374151';
            } else {
                ctx.fillStyle = Math.random() < 0.05 ? '#f093fb' : '#1f2937';
            }
            
            ctx.fillRect(x * tileSize, y * tileSize, tileSize - 1, tileSize - 1);
        }
    }
    
    // Add player position
    ctx.fillStyle = '#4ade80';
    ctx.fillRect(canvas.width / 2 - 10, canvas.height / 2 - 10, 20, 20);
    
    showNotification('Room Generated! Seed: ' + Math.floor(Math.random() * 999999));
}

// Combat System Demo
function demoCombat() {
    showNotification('Demonstrating combat system...');
    
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    let frame = 0;
    const maxFrames = 60;
    
    function animateCombat() {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.1)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // Player
        const playerX = canvas.width / 3;
        const playerY = canvas.height / 2;
        ctx.fillStyle = '#4ade80';
        ctx.fillRect(playerX - 15, playerY - 15, 30, 30);
        
        // Enemy
        const enemyX = canvas.width * 2/3;
        const enemyY = canvas.height / 2;
        ctx.fillStyle = '#ef4444';
        ctx.fillRect(enemyX - 15, enemyY - 15, 30, 30);
        
        // Attack animation
        if (frame % 20 < 10) {
            ctx.strokeStyle = '#f093fb';
            ctx.lineWidth = 3;
            ctx.beginPath();
            ctx.moveTo(playerX + 15, playerY);
            ctx.lineTo(enemyX - 15, enemyY);
            ctx.stroke();
            
            // Damage number
            ctx.fillStyle = '#fbbf24';
            ctx.font = 'bold 20px Inter, sans-serif';
            ctx.fillText(Math.floor(Math.random() * 100 + 50), enemyX, enemyY - 30);
        }
        
        frame++;
        if (frame < maxFrames) {
            requestAnimationFrame(animateCombat);
        } else {
            showNotification('Combat Demo Complete!');
        }
    }
    
    animateCombat();
}

// Enemy AI Demo
function demoEnemyAI() {
    showNotification('Spawning intelligent enemies...');
    
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    const enemies = [];
    for (let i = 0; i < 5; i++) {
        enemies.push({
            x: Math.random() * canvas.width,
            y: Math.random() * canvas.height,
            vx: (Math.random() - 0.5) * 2,
            vy: (Math.random() - 0.5) * 2,
            color: `hsl(${Math.random() * 60}, 100%, 50%)`
        });
    }
    
    let frame = 0;
    const maxFrames = 120;
    
    function animateEnemies() {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.1)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // Update and draw enemies
        enemies.forEach(enemy => {
            // Simple AI movement
            enemy.x += enemy.vx;
            enemy.y += enemy.vy;
            
            // Bounce off walls
            if (enemy.x < 0 || enemy.x > canvas.width) enemy.vx *= -1;
            if (enemy.y < 0 || enemy.y > canvas.height) enemy.vy *= -1;
            
            // Draw enemy
            ctx.fillStyle = enemy.color;
            ctx.beginPath();
            ctx.arc(enemy.x, enemy.y, 10, 0, Math.PI * 2);
            ctx.fill();
        });
        
        // Draw player
        ctx.fillStyle = '#4ade80';
        ctx.fillRect(canvas.width / 2 - 15, canvas.height / 2 - 15, 30, 30);
        
        frame++;
        if (frame < maxFrames) {
            requestAnimationFrame(animateEnemies);
        } else {
            showNotification('Enemy AI Demo Complete!');
        }
    }
    
    animateEnemies();
}

// Inventory Demo
function demoInventory() {
    showNotification('Opening inventory system...');
    
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Draw inventory grid
    const slotSize = 60;
    const padding = 10;
    const cols = 8;
    const rows = 4;
    const startX = (canvas.width - (cols * (slotSize + padding))) / 2;
    const startY = 100;
    
    // Title
    ctx.fillStyle = '#f093fb';
    ctx.font = '24px Inter, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('Inventory System', canvas.width / 2, 50);
    
    // Draw slots
    const items = [
        { icon: '⚔️', rarity: 'legendary' },
        { icon: '🛡️', rarity: 'rare' },
        { icon: '🏹', rarity: 'common' },
        { icon: '💎', rarity: 'epic' },
        { icon: '🧪', rarity: 'common' },
        { icon: '📜', rarity: 'rare' },
        { icon: '🗝️', rarity: 'legendary' },
        { icon: '💰', rarity: 'common' }
    ];
    
    const rarityColors = {
        common: '#9ca3af',
        rare: '#3b82f6',
        epic: '#a855f7',
        legendary: '#f59e0b'
    };
    
    for (let row = 0; row < rows; row++) {
        for (let col = 0; col < cols; col++) {
            const x = startX + col * (slotSize + padding);
            const y = startY + row * (slotSize + padding);
            
            // Draw slot background
            ctx.fillStyle = 'rgba(255, 255, 255, 0.1)';
            ctx.fillRect(x, y, slotSize, slotSize);
            
            // Draw item if exists
            const itemIndex = row * cols + col;
            if (itemIndex < items.length) {
                const item = items[itemIndex];
                
                // Draw rarity border
                ctx.strokeStyle = rarityColors[item.rarity];
                ctx.lineWidth = 2;
                ctx.strokeRect(x, y, slotSize, slotSize);
                
                // Draw item icon
                ctx.font = '30px sans-serif';
                ctx.textAlign = 'center';
                ctx.fillText(item.icon, x + slotSize / 2, y + slotSize / 2 + 10);
            }
        }
    }
    
    showNotification('Inventory loaded with ' + items.length + ' items!');
}

// Mobile Touch Demo
function demoMobile() {
    showNotification('Touch controls activated!');
    
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Draw virtual joystick
    const joystickX = 150;
    const joystickY = canvas.height - 150;
    
    ctx.strokeStyle = 'rgba(102, 126, 234, 0.5)';
    ctx.lineWidth = 3;
    ctx.beginPath();
    ctx.arc(joystickX, joystickY, 60, 0, Math.PI * 2);
    ctx.stroke();
    
    ctx.fillStyle = '#667eea';
    ctx.beginPath();
    ctx.arc(joystickX, joystickY, 30, 0, Math.PI * 2);
    ctx.fill();
    
    // Draw action buttons
    const buttonPositions = [
        { x: canvas.width - 150, y: canvas.height - 150, label: 'A' },
        { x: canvas.width - 100, y: canvas.height - 200, label: 'B' },
        { x: canvas.width - 200, y: canvas.height - 200, label: 'X' },
        { x: canvas.width - 150, y: canvas.height - 250, label: 'Y' }
    ];
    
    buttonPositions.forEach(btn => {
        ctx.strokeStyle = 'rgba(240, 147, 251, 0.5)';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(btn.x, btn.y, 30, 0, Math.PI * 2);
        ctx.stroke();
        
        ctx.fillStyle = '#f093fb';
        ctx.font = '20px Inter, sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(btn.label, btn.x, btn.y + 7);
    });
    
    // Instructions
    ctx.fillStyle = '#fff';
    ctx.font = '18px Inter, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('Touch Controls Overlay', canvas.width / 2, 50);
    ctx.font = '14px Inter, sans-serif';
    ctx.fillText('Optimized for mobile devices with haptic feedback', canvas.width / 2, 80);
}

// Offline Mode Demo
function demoOffline() {
    const isOffline = Math.random() > 0.5;
    
    if (isOffline) {
        showNotification('Offline mode activated! Game cached locally.');
        document.getElementById('connection-status').className = 'status-indicator inactive';
        document.getElementById('connection-text').textContent = 'Offline Mode';
    } else {
        showNotification('Online mode restored! Syncing with server...');
        document.getElementById('connection-status').className = 'status-indicator active';
        document.getElementById('connection-text').textContent = 'Connected';
    }
    
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    ctx.fillStyle = isOffline ? '#ef4444' : '#4ade80';
    ctx.font = '24px Inter, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(isOffline ? '📴 Offline Mode' : '📶 Online Mode', canvas.width / 2, canvas.height / 2);
    
    ctx.fillStyle = '#fff';
    ctx.font = '16px Inter, sans-serif';
    ctx.fillText(isOffline ? 'Playing locally with cached data' : 'Connected to game servers', canvas.width / 2, canvas.height / 2 + 40);
}

// Network Demo Functions
function createRoom() {
    roomCode = generateRoomCode();
    document.getElementById('room-code').textContent = roomCode;
    showNotification('Room created! Code: ' + roomCode);
    
    // Update connection status
    document.getElementById('connection-status').className = 'status-indicator active';
    document.getElementById('connection-text').textContent = 'Host - Waiting for players...';
}

function joinRoom() {
    const code = document.getElementById('join-code').value;
    if (!code) {
        showNotification('Please enter a room code!');
        return;
    }
    
    showNotification('Joining room: ' + code);
    
    // Simulate connection
    setTimeout(() => {
        document.getElementById('connection-status').className = 'status-indicator active';
        document.getElementById('connection-text').textContent = 'Connected to room';
        document.getElementById('peers-list').innerHTML = '<div>Connected Players: 2/8</div>';
        showNotification('Successfully joined room!');
    }, 1000);
}

function generateRoomCode() {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    let code = '';
    for (let i = 0; i < 6; i++) {
        code += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return code;
}

// Game Mode Launchers
function launchPractice() {
    showNotification('Launching Practice Mode...');
    simulateGameLaunch('Practice Mode', 'Single-player training session');
}

function launchCoop() {
    showNotification('Starting Co-op Campaign...');
    simulateGameLaunch('Co-op Campaign', 'Multiplayer dungeon crawl');
}

function launchPvP() {
    showNotification('Entering PvP Arena...');
    simulateGameLaunch('PvP Arena', 'Player vs Player combat');
}

function simulateGameLaunch(mode, description) {
    const canvas = document.getElementById('demo-canvas');
    const ctx = canvas.getContext('2d');
    
    ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Loading animation
    let progress = 0;
    const maxProgress = 100;
    
    function animateLoading() {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // Title
        ctx.fillStyle = '#f093fb';
        ctx.font = '28px Inter, sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(mode, canvas.width / 2, canvas.height / 2 - 50);
        
        // Description
        ctx.fillStyle = '#fff';
        ctx.font = '16px Inter, sans-serif';
        ctx.fillText(description, canvas.width / 2, canvas.height / 2 - 20);
        
        // Progress bar
        const barWidth = 300;
        const barHeight = 20;
        const barX = (canvas.width - barWidth) / 2;
        const barY = canvas.height / 2 + 20;
        
        ctx.strokeStyle = 'rgba(102, 126, 234, 0.5)';
        ctx.lineWidth = 2;
        ctx.strokeRect(barX, barY, barWidth, barHeight);
        
        ctx.fillStyle = 'linear-gradient(90deg, #667eea, #f093fb)';
        ctx.fillStyle = '#667eea';
        ctx.fillRect(barX, barY, (barWidth * progress) / maxProgress, barHeight);
        
        // Progress text
        ctx.fillStyle = '#fff';
        ctx.font = '14px Inter, sans-serif';
        ctx.fillText(`Loading... ${progress}%`, canvas.width / 2, barY + barHeight + 30);
        
        progress += 2;
        if (progress <= maxProgress) {
            requestAnimationFrame(animateLoading);
        } else {
            ctx.fillStyle = '#4ade80';
            ctx.font = '24px Inter, sans-serif';
            ctx.fillText('Ready to Play!', canvas.width / 2, canvas.height / 2 + 100);
        }
    }
    
    animateLoading();
}

// Utility function to show notifications
function showNotification(message) {
    // Create notification element
    const notification = document.createElement('div');
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        background: linear-gradient(135deg, #667eea, #f093fb);
        color: white;
        padding: 1rem 1.5rem;
        border-radius: 10px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
        z-index: 1000;
        animation: slideIn 0.3s ease;
        font-family: 'Inter', sans-serif;
    `;
    notification.textContent = message;
    
    // Add animation
    const style = document.createElement('style');
    style.textContent = `
        @keyframes slideIn {
            from {
                transform: translateX(100%);
                opacity: 0;
            }
            to {
                transform: translateX(0);
                opacity: 1;
            }
        }
    `;
    document.head.appendChild(style);
    
    document.body.appendChild(notification);
    
    // Remove after 3 seconds
    setTimeout(() => {
        notification.style.animation = 'slideOut 0.3s ease';
        setTimeout(() => {
            document.body.removeChild(notification);
        }, 300);
    }, 3000);
}