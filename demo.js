// Demo JavaScript for WASM Multiplayer Roguelike ARPG
// This file contains all the interactive demo functionality

// Performance monitoring
let fpsCounter = 60;
let latencyCounter = 12;
let memoryCounter = 48;
let entitiesCounter = 127;

// Expose performance counters to window for monitoring
window.fpsCounter = fpsCounter;
window.latencyCounter = latencyCounter;
window.memoryCounter = memoryCounter;
window.entitiesCounter = entitiesCounter;

// WebRTC demo state
let roomCode = null;
let peers = [];
let isConnected = false;

// Animation frame for performance monitoring
let lastTime = performance.now();
let frameCount = 0;

// Initialize demo on page load
document.addEventListener('DOMContentLoaded', () => {
    console.log('Demo initialization started');
    try {
        startPerformanceMonitoring();
        initializeCanvas();
        updateMetrics();
        console.log('Demo initialization completed successfully');
    } catch (error) {
        console.error('Error during demo initialization:', error);
    }
});

// Add global error handler for debugging
window.addEventListener('error', (event) => {
    console.error('Global error caught:', event.error);
    console.error('Error message:', event.message);
    console.error('Error source:', event.filename);
    console.error('Line:', event.lineno, 'Column:', event.colno);
});

// Add unhandled promise rejection handler
window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
});

// Performance monitoring
let fpsAnimationId = null;
let metricsIntervalId = null;

function startPerformanceMonitoring() {
    // Stop any existing monitoring first
    stopPerformanceMonitoring();
    
    function updateFPS() {
        try {
            const currentTime = performance.now();
            frameCount++;
            
            if (currentTime >= lastTime + 1000) {
                fpsCounter = Math.round((frameCount * 1000) / (currentTime - lastTime));
                window.fpsCounter = fpsCounter; // Update window property
                const fpsElement = document.getElementById('fps-counter');
                if (fpsElement) {
                    fpsElement.textContent = fpsCounter;
                }
                frameCount = 0;
                lastTime = currentTime;
            }
            
            fpsAnimationId = requestAnimationFrame(updateFPS);
        } catch (error) {
            console.error('Error in FPS monitoring:', error);
            stopPerformanceMonitoring();
        }
    }
    
    updateFPS();
    console.log('Performance monitoring started');
}

function stopPerformanceMonitoring() {
    if (fpsAnimationId) {
        cancelAnimationFrame(fpsAnimationId);
        fpsAnimationId = null;
    }
    if (metricsIntervalId) {
        clearInterval(metricsIntervalId);
        metricsIntervalId = null;
    }
}

// Update performance metrics
function updateMetrics() {
    // Clear any existing interval first
    if (metricsIntervalId) {
        clearInterval(metricsIntervalId);
    }
    
    metricsIntervalId = setInterval(() => {
        try {
            // Simulate realistic metrics
            latencyCounter = Math.floor(Math.random() * 20) + 10;
            memoryCounter = Math.floor(Math.random() * 30) + 40;
            entitiesCounter = Math.floor(Math.random() * 50) + 100;
            
            // Update window properties
            window.latencyCounter = latencyCounter;
            window.memoryCounter = memoryCounter;
            window.entitiesCounter = entitiesCounter;
            
            const latencyElement = document.getElementById('latency-counter');
            const memoryElement = document.getElementById('memory-counter');
            const entitiesElement = document.getElementById('entities-counter');
            
            if (latencyElement) latencyElement.textContent = latencyCounter;
            if (memoryElement) memoryElement.textContent = memoryCounter;
            if (entitiesElement) entitiesElement.textContent = entitiesCounter;
            
            // Remove verbose logging to improve performance
            // Only log errors or important events
        } catch (error) {
            console.error('Error updating metrics:', error);
            if (metricsIntervalId) {
                clearInterval(metricsIntervalId);
                metricsIntervalId = null;
            }
        }
    }, 2000);
    console.log('Metrics monitoring started');
}

// Initialize canvas for visual demos
function initializeCanvas() {
    const canvas = document.getElementById('demo-canvas');
    if (!canvas) {
        console.error('Canvas element not found during initialization!');
        return;
    }
    
    const ctx = canvas.getContext('2d');
    if (!ctx) {
        console.error('Failed to get 2D context for canvas');
        return;
    }
    
    // Set canvas dimensions with fallback
    canvas.width = canvas.offsetWidth || 800;
    canvas.height = canvas.offsetHeight || 600;
    
    console.log(`Canvas initialized with dimensions: ${canvas.width}x${canvas.height}`);
    
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
    
    // Switch to demo tab first
    switchToDemoTab();
    
    // Simulate WASM benchmark
    showNotification('Running WASM Performance Benchmark...');
    
    setTimeout(() => {
        const endTime = performance.now();
        const duration = (endTime - startTime).toFixed(2);
        
        // Update canvas with results
        const canvas = document.getElementById('demo-canvas');
        if (!canvas) return;
        
        const ctx = canvas.getContext('2d');
        if (!ctx) return;
        
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
    // Switch to demo tab first
    switchToDemoTab();
    
    setTimeout(() => {
        showNotification('Testing P2P connection...');
        
        const canvas = document.getElementById('demo-canvas');
        if (!canvas) return;
        
        const ctx = canvas.getContext('2d');
        if (!ctx) return;
        
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
    }, 100);
}

// Global function to stop all animations
function stopAllAnimations() {
    // Stop all animation frames
    if (typeof fpsAnimationId !== 'undefined' && fpsAnimationId) {
        cancelAnimationFrame(fpsAnimationId);
        fpsAnimationId = null;
    }
    if (typeof combatAnimationId !== 'undefined' && combatAnimationId) {
        cancelAnimationFrame(combatAnimationId);
        combatAnimationId = null;
    }
    if (typeof enemyAnimationId !== 'undefined' && enemyAnimationId) {
        cancelAnimationFrame(enemyAnimationId);
        enemyAnimationId = null;
    }
    if (typeof loadingAnimationId !== 'undefined' && loadingAnimationId) {
        cancelAnimationFrame(loadingAnimationId);
        loadingAnimationId = null;
    }
    
    // Clear any intervals
    if (typeof metricsIntervalId !== 'undefined' && metricsIntervalId) {
        clearInterval(metricsIntervalId);
        metricsIntervalId = null;
    }
}

// Helper function to switch to demo tab
function switchToDemoTab() {
    // Stop any running animations before switching
    stopAllAnimations();
    
    // Hide all tabs
    const tabs = document.querySelectorAll('.tab-content');
    tabs.forEach(tab => tab.classList.remove('active'));
    
    // Remove active class from all buttons
    const buttons = document.querySelectorAll('.tab-btn');
    buttons.forEach(btn => btn.classList.remove('active'));
    
    // Show demo tab
    const demoTab = document.getElementById('demo-tab');
    if (demoTab) {
        demoTab.classList.add('active');
    }
    
    // Find and activate the demo button
    buttons.forEach(btn => {
        if (btn.textContent.includes('Live Demo')) {
            btn.classList.add('active');
        }
    });
}

// Roguelike Room Generation Demo
function demoRoguelike() {
    try {
        console.log('Starting roguelike demo...');
        
        // Switch to demo tab first
        switchToDemoTab();
        
        // Small delay to ensure tab is visible
        setTimeout(() => {
            showNotification('Generating procedural dungeon...');
            
            const canvas = document.getElementById('demo-canvas');
            if (!canvas) {
                console.error('Canvas element not found!');
                showNotification('Error: Canvas not found!');
                return;
            }
            
            const ctx = canvas.getContext('2d');
            if (!ctx) {
                console.error('Could not get canvas context!');
                showNotification('Error: Could not get canvas context!');
                return;
            }
            
            // Ensure canvas has proper dimensions
            if (canvas.width === 0 || canvas.height === 0) {
                canvas.width = canvas.offsetWidth || 800;
                canvas.height = canvas.offsetHeight || 600;
            }
            
            console.log(`Canvas dimensions: ${canvas.width}x${canvas.height}`);
            
            // Clear the canvas
            ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
            ctx.fillRect(0, 0, canvas.width, canvas.height);
            
            // Generate random room layout
            const tileSize = 20;
            const roomWidth = Math.floor(canvas.width / tileSize);
            const roomHeight = Math.floor(canvas.height / tileSize);
            
            console.log(`Generating ${roomWidth}x${roomHeight} room...`);
            
            // Generate room tiles
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
            
            // Add some enemies
            ctx.fillStyle = '#ef4444';
            for (let i = 0; i < 5; i++) {
                const enemyX = Math.floor(Math.random() * (roomWidth - 2) + 1) * tileSize;
                const enemyY = Math.floor(Math.random() * (roomHeight - 2) + 1) * tileSize;
                ctx.fillRect(enemyX + 5, enemyY + 5, 10, 10);
            }
            
            // Add some loot
            ctx.fillStyle = '#fbbf24';
            for (let i = 0; i < 3; i++) {
                const lootX = Math.floor(Math.random() * (roomWidth - 2) + 1) * tileSize;
                const lootY = Math.floor(Math.random() * (roomHeight - 2) + 1) * tileSize;
                ctx.beginPath();
                ctx.arc(lootX + 10, lootY + 10, 5, 0, Math.PI * 2);
                ctx.fill();
            }
            
            const seed = Math.floor(Math.random() * 999999);
            showNotification('Room Generated! Seed: ' + seed);
            console.log('Roguelike demo completed successfully!');
        }, 100);
    } catch (error) {
        console.error('Error in demoRoguelike:', error);
        showNotification('Error generating room: ' + error.message);
    }
}

// Combat System Demo
let combatAnimationId = null;

function demoCombat() {
    // Cancel any existing combat animation
    if (combatAnimationId) {
        cancelAnimationFrame(combatAnimationId);
        combatAnimationId = null;
    }
    
    // Switch to demo tab first
    switchToDemoTab();
    
    setTimeout(() => {
        showNotification('Demonstrating combat system...');
        
        const canvas = document.getElementById('demo-canvas');
        if (!canvas) {
            console.error('Canvas not found!');
            return;
        }
        
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            console.error('Could not get canvas context!');
            return;
        }
        
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
                combatAnimationId = requestAnimationFrame(animateCombat);
            } else {
                combatAnimationId = null;
                showNotification('Combat Demo Complete!');
            }
        }
        
        animateCombat();
    }, 100);
}

// Enemy AI Demo
let enemyAnimationId = null;

function demoEnemyAI() {
    // Cancel any existing enemy animation
    if (enemyAnimationId) {
        cancelAnimationFrame(enemyAnimationId);
        enemyAnimationId = null;
    }
    
    // Switch to demo tab first
    switchToDemoTab();
    
    setTimeout(() => {
        showNotification('Spawning intelligent enemies...');
        
        const canvas = document.getElementById('demo-canvas');
        if (!canvas) return;
        
        const ctx = canvas.getContext('2d');
        if (!ctx) return;
        
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
                enemyAnimationId = requestAnimationFrame(animateEnemies);
            } else {
                enemyAnimationId = null;
                showNotification('Enemy AI Demo Complete!');
            }
        }
        
        animateEnemies();
    }, 100);
}

// Inventory Demo
function demoInventory() {
    // Switch to demo tab first
    switchToDemoTab();
    
    setTimeout(() => {
        showNotification('Opening inventory system...');
        
        const canvas = document.getElementById('demo-canvas');
        if (!canvas) {
            console.error('Canvas not found!');
            return;
        }
        
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            console.error('Could not get canvas context!');
            return;
        }
        
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
    }, 100);
}

// Mobile Touch Demo
function demoMobile() {
    showNotification('Touch controls activated!');
    
    const canvas = document.getElementById('demo-canvas');
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
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
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
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

let loadingAnimationId = null;

function simulateGameLaunch(mode, description) {
    // Cancel any existing loading animation
    if (loadingAnimationId) {
        cancelAnimationFrame(loadingAnimationId);
        loadingAnimationId = null;
    }
    
    const canvas = document.getElementById('demo-canvas');
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
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
            loadingAnimationId = requestAnimationFrame(animateLoading);
        } else {
            loadingAnimationId = null;
            ctx.fillStyle = '#4ade80';
            ctx.font = '24px Inter, sans-serif';
            ctx.fillText('Ready to Play!', canvas.width / 2, canvas.height / 2 + 100);
        }
    }
    
    animateLoading();
}

// Add notification animations once
(function() {
    if (!document.getElementById('notification-styles')) {
        const style = document.createElement('style');
        style.id = 'notification-styles';
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
            @keyframes slideOut {
                from {
                    transform: translateX(0);
                    opacity: 1;
                }
                to {
                    transform: translateX(100%);
                    opacity: 0;
                }
            }
        `;
        document.head.appendChild(style);
    }
})();

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
    
    document.body.appendChild(notification);
    
    // Remove after 3 seconds
    setTimeout(() => {
        notification.style.animation = 'slideOut 0.3s ease';
        setTimeout(() => {
            if (notification.parentNode) {
                document.body.removeChild(notification);
            }
        }, 300);
    }, 3000);
}