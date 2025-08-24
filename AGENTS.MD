# AGENTS.MD - Project Rules and Guidelines

## Project Overview
Building a lightweight, responsive multiplayer browser game using WebAssembly (WASM) technology, designed for GitHub Pages deployment with WebRTC-based multiplayer functionality.

## Core Architecture Rules

### 1. Technology Stack
- **WebAssembly Framework**: Use Rust with wasm-bindgen and wasm-pack (latest stable versions)
- **Game Engine**: Bevy 0.14+ (latest Rust game engine with excellent WASM support)
- **Frontend Framework**: Vite + TypeScript for build tooling and modern development
- **Networking**: WebRTC via simple-peer or native WebRTC API for P2P multiplayer
- **Signaling Server**: Lightweight Node.js/Deno server for WebRTC signaling
- **UI Framework**: Minimal CSS with CSS Grid/Flexbox, no heavy UI libraries

### 2. Performance Guidelines
- **Bundle Size**: Keep total WASM + JS bundle under 2MB gzipped
- **Initial Load**: First paint under 2 seconds on 3G connection
- **Frame Rate**: Target 60 FPS on mid-range devices
- **Memory Usage**: Stay under 256MB RAM usage
- **Asset Loading**: Use progressive loading for game assets

### 3. Multiplayer Architecture
- **Network Model**: Peer-to-peer with host migration capability
- **State Sync**: Deterministic lockstep or client-server authoritative
- **Latency Compensation**: Client-side prediction with server reconciliation
- **Max Players**: Design for 2-8 players initially
- **Fallback**: WebSocket fallback if WebRTC fails

### 4. Development Principles
- **Mobile-First**: Design for touch controls and small screens first
- **Progressive Enhancement**: Game should work offline in single-player mode
- **Accessibility**: Include keyboard navigation and screen reader support where possible
- **Cross-Browser**: Support Chrome, Firefox, Safari (latest 2 versions)

### 5. Project Structure
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
│   └── server.ts       # Minimal signaling
├── public/             # Static assets
└── dist/               # Build output for GitHub Pages
```

### 6. Build & Deployment Rules
- **CI/CD**: Use GitHub Actions for automated builds
- **Optimization**: Enable WASM optimization (wasm-opt level 3)
- **Compression**: Use Brotli/Gzip for all assets
- **Caching**: Implement service worker for offline play
- **Versioning**: Use semantic versioning for releases

### 7. Code Quality Standards
- **Rust**: Follow Rust idioms, use clippy and rustfmt
- **TypeScript**: Strict mode, ESLint with recommended rules
- **Testing**: Unit tests for game logic, integration tests for networking
- **Documentation**: Document all public APIs and game mechanics

### 8. Security Considerations
- **Input Validation**: Validate all player inputs on authoritative peer
- **Rate Limiting**: Limit network messages per second
- **Sandboxing**: WASM provides memory safety by default
- **CORS**: Configure properly for GitHub Pages

### 9. Game Design Constraints
- **Simplicity**: Keep mechanics simple for low latency
- **Responsiveness**: All actions should feel immediate
- **Fairness**: Implement lag compensation fairly
- **Scalability**: Design systems that can handle player growth

### 10. Monitoring & Analytics
- **Performance**: Track FPS, load times, memory usage
- **Errors**: Log and report WASM panics and JS errors
- **Gameplay**: Track player sessions and game metrics
- **Privacy**: Respect user privacy, minimal data collection

## Development Workflow

1. **Local Development**: 
   - Run `cargo watch` for Rust hot-reload
   - Use Vite dev server for frontend
   - Local signaling server for testing

2. **Testing**:
   - Unit tests: `cargo test`
   - Integration: Playwright for E2E
   - Performance: Lighthouse CI

3. **Deployment**:
   - Push to main triggers GitHub Action
   - Builds WASM, bundles assets
   - Deploys to GitHub Pages

## Key Decisions Log

| Decision | Rationale | Date |
|----------|-----------|------|
| Rust + Bevy | Best WASM performance, modern ECS | 2024 |
| WebRTC | Low latency P2P networking | 2024 |
| GitHub Pages | Free hosting, good CDN | 2024 |
| Vite | Fast builds, great DX | 2024 |

## Resources & References

- [Bevy WASM Guide](https://bevyengine.org/learn/guides/wasm/)
- [WebRTC for Games](https://webrtcforthecurious.com/)
- [WASM Performance Tips](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/)
- [GitHub Pages Deployment](https://docs.github.com/en/pages)

## Update Protocol
This document should be updated whenever:
- Major architectural decisions are made
- Performance targets change
- New technologies are adopted
- Lessons learned from production issues