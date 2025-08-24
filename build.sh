#!/bin/bash

# Build script for WASM Multiplayer Game
set -e

echo "🚀 Starting build process..."

# Check for required tools
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ $1 is not installed. Please install it first."
        exit 1
    fi
}

echo "📋 Checking dependencies..."
check_command rustc
check_command cargo
check_command wasm-pack
check_command node
check_command npm

# Install Rust target if not present
echo "🦀 Setting up Rust WASM target..."
rustup target add wasm32-unknown-unknown || true

# Build WASM module
echo "🔨 Building WASM module..."
cd game
cargo clean
wasm-pack build --target web --out-dir ../client/src/wasm --release

# Optimize WASM if wasm-opt is available
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing WASM with wasm-opt..."
    find ../client/src/wasm -name "*.wasm" -exec wasm-opt -O3 -o {} {} \;
fi

cd ..

# Install client dependencies
echo "📦 Installing client dependencies..."
cd client
npm install

# Build client
echo "🏗️ Building client application..."
npm run build

cd ..

# Create dist directory if it doesn't exist
if [ ! -d "dist" ]; then
    echo "📁 Creating dist directory..."
    mkdir -p dist
fi

# Copy public assets
echo "📋 Copying public assets..."
cp -r public/* dist/ 2>/dev/null || true

echo "✅ Build complete! Output in dist/ directory"
echo "📊 Build size:"
du -sh dist/

# Optional: Start a local server to test
if [ "$1" = "--serve" ]; then
    echo "🌐 Starting local server..."
    cd dist
    python3 -m http.server 8000 || python -m SimpleHTTPServer 8000
fi