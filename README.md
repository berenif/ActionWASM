# WASM Multiplayer Arena 🎮

A lightweight, responsive multiplayer browser game built with WebAssembly (WASM) and WebRTC for peer-to-peer networking. Designed for deployment on GitHub Pages.

## 🚀 Features

- **WebAssembly Performance**: Core game logic written in Rust/Bevy for near-native performance
- **P2P Multiplayer**: Real-time multiplayer using WebRTC with minimal latency
- **Responsive Design**: Mobile-first approach with touch controls support
- **Progressive Enhancement**: Works offline in single-player mode
- **Lightweight**: < 2MB bundle size (gzipped)
- **Modern UI**: Beautiful, animated interface with dark theme

## 🏗️ Architecture

```
┌─────────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Game Engine   │────▶│   Frontend   │────▶│    WebRTC       │
│  (Rust/Bevy)    │     │ (TypeScript) │     │   Networking    │
│     [WASM]      │     │    [Vite]    │     │  (SimplePeer)   │
└─────────────────┘     └──────────────┘     └─────────────────┘
                               │                      │
                               ▼                      ▼
                        ┌──────────────┐     ┌─────────────────┐
                        │  GitHub      │     │   Signaling     │
                        │   Pages      │     │    Server       │
                        └──────────────┘     └─────────────────┘
```

## 📁 Project Structure

```
/
├── game/                 # Rust/WASM game logic
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── game.rs      # Core game logic
│   │   ├── network.rs   # WebRTC networking
│   │   └── systems/     # ECS systems
│   └── Cargo.toml
├── client/              # TypeScript frontend
│   ├── src/
│   │   ├── main.ts     # Client entry
│   │   ├── webrtc.ts   # WebRTC handling
│   │   └── ui.ts       # UI components
│   └── package.json
├── signaling/          # Signaling server
│   └── server.js       # WebSocket signaling
└── AGENTS.MD           # Development rules
```

## 🛠️ Development Setup

### Prerequisites

- Rust (latest stable)
- Node.js 18+
- wasm-pack
- npm or yarn

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/wasm-multiplayer-game.git
   cd wasm-multiplayer-game
   ```

2. **Install Rust dependencies**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   cargo install wasm-pack
   ```

3. **Build the WASM module**
   ```bash
   cd game
   wasm-pack build --target web --out-dir ../client/src/wasm
   ```

4. **Install and run the client**
   ```bash
   cd ../client
   npm install
   npm run dev
   ```

5. **Run the signaling server** (for multiplayer)
   ```bash
   cd ../signaling
   npm install
   npm start
   ```

## 🎮 How to Play

### Single Player Mode
1. Open the game in your browser
2. Click "Practice Mode"
3. Use WASD or Arrow keys to move

### Multiplayer Mode
1. **Create a Room**: Click "Create Room" to generate a room code
2. **Join a Room**: Enter the room code and click "Join Room"
3. Share the room code with friends (up to 8 players)

### Controls
- **Movement**: WASD or Arrow Keys
- **Action**: Space
- **Menu**: ESC

## 🚢 Deployment

### GitHub Pages

The project is configured for automatic deployment to GitHub Pages:

1. Push to the `main` branch
2. GitHub Actions will automatically:
   - Build the WASM module
   - Bundle the client
   - Deploy to GitHub Pages

### Manual Deployment

```bash
# Build for production
cd game
wasm-pack build --target web --out-dir ../client/src/wasm --release

cd ../client
npm run build

# The dist/ folder contains the production build
```

### Signaling Server Deployment

For production multiplayer, deploy the signaling server to a platform like:
- Heroku
- Railway
- Render
- Your own VPS

Update the WebSocket URL in `client/src/webrtc.ts`:
```typescript
return 'wss://your-signaling-server.com';
```

## 🔧 Configuration

### Performance Tuning

Edit `game/Cargo.toml` for WASM optimization:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
strip = true        # Strip symbols
```

### Network Settings

Configure in `client/src/webrtc.ts`:
```typescript
const iceServers = [
  { urls: 'stun:stun.l.google.com:19302' },
  // Add TURN servers for better connectivity
];
```

## 📊 Performance Targets

- **Bundle Size**: < 2MB gzipped
- **Initial Load**: < 2s on 3G
- **Frame Rate**: 60 FPS
- **Memory Usage**: < 256MB
- **Network Latency**: < 50ms (P2P)

## 🧪 Testing

```bash
# Run Rust tests
cd game
cargo test

# Run TypeScript tests
cd client
npm test

# Run E2E tests
npm run test:e2e
```

## 📝 Development Guidelines

See [AGENTS.MD](./AGENTS.MD) for detailed development rules and architecture decisions.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Follow the guidelines in AGENTS.MD
4. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 🙏 Acknowledgments

- [Bevy Engine](https://bevyengine.org/) - Rust game engine
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - WASM bindings
- [SimplePeer](https://github.com/feross/simple-peer) - WebRTC library
- [Vite](https://vitejs.dev/) - Frontend tooling

## 🐛 Known Issues

- WebRTC may not work behind strict firewalls without TURN servers
- Safari requires user interaction before playing audio
- Mobile browsers may have reduced performance

## 📮 Contact

For questions or support, please open an issue on GitHub.

---

**Live Demo**: [https://yourusername.github.io/wasm-multiplayer-game](https://yourusername.github.io/wasm-multiplayer-game)