#!/bin/bash

RUST_URL="https://rvn-rust.onrender.com/compute/fibonacci/35"
NODE_URL="https://rvn-node.onrender.com/compute/fibonacci/35"

THREADS=5   
CONNECTIONS=100  
DURATION=30s  


echo "Running performance test on Rust API at $RUST_URL..."
wrk -t$THREADS -c$CONNECTIONS -d$DURATION $RUST_URL
echo "Rust API test complete."

echo ""


echo "Running performance test on Node.js API at $NODE_URL..."
wrk -t$THREADS -c$CONNECTIONS -d$DURATION $NODE_URL
echo "Node.js API test complete."
