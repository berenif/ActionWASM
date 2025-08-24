#!/bin/bash

# Simple script to serve the demo locally

echo "🎮 WASM Multiplayer Roguelike ARPG - Demo Server"
echo "================================================"
echo ""
echo "Starting demo server..."
echo ""

# Check if Python is available
if command -v python3 &> /dev/null; then
    echo "📡 Server running at: http://localhost:8080/demo.html"
    echo "Press Ctrl+C to stop the server"
    echo ""
    python3 -m http.server 8080
elif command -v python &> /dev/null; then
    echo "📡 Server running at: http://localhost:8080/demo.html"
    echo "Press Ctrl+C to stop the server"
    echo ""
    python -m SimpleHTTPServer 8080
else
    echo "❌ Python is not installed. Please install Python to run the demo server."
    echo "Alternatively, you can open demo.html directly in your browser."
    exit 1
fi