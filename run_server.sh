#!/bin/bash
cd "$(dirname "$0")"
RUST_LOG=info cargo run --release -p server
