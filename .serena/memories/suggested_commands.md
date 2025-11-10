# Suggested Commands

## Running the Project

### Start Server (Port 5000)
```bash
cd /home/max/code/game
./run_server.sh
# Or manually:
RUST_LOG=info cargo run --release -p server
```

### Start Client
```bash
cd /home/max/code/game
./run_client.sh
# Or manually:
cargo run --release -p client
```

## Building

### Build All
```bash
cargo build --release
```

### Build Specific Package
```bash
cargo build --release -p client
cargo build --release -p server
cargo build --release -p shared
```

### Development Build (faster, but slower runtime)
```bash
cargo build -p client
```

## Testing

### Run All Tests
```bash
cargo test
```

### Run Server Tests Only
```bash
cargo test -p server
# Or from server directory:
cd server && cargo test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Test Results
- Server: 19/19 auth tests, 3/3 db tests
- Client: No automated tests yet

## Debugging

### Server with Logs
```bash
RUST_LOG=info cargo run --release -p server
# Or debug level:
RUST_LOG=debug cargo run --release -p server
```

### Client with Logs
```bash
RUST_LOG=info cargo run --release -p client
# Specific module:
RUST_LOG=client::networking=debug cargo run -p client
```

### Network Debugging
```bash
# Watch server logs
tail -f server.log | grep -i "message\|error\|auth"
```

## Database Operations

### Reset Database
```bash
rm game.db
cargo run -p server  # DB will be recreated with migrations
```

### Backup Database
```bash
cp game.db game.db.backup
```

### View Database
```bash
sqlite3 game.db "SELECT * FROM users;"
sqlite3 game.db "SELECT * FROM characters;"
```

## Performance

### Parallel Build (8 cores)
```bash
cargo build --release -j 8
```

### Clean Build Cache
```bash
cargo clean
cargo build --release
```

## Utilities

### Check Compilation Without Building
```bash
cargo check -p client
cargo check -p server
```

### Format Code (if rustfmt configured)
```bash
cargo fmt
```

### Clippy Lints (if clippy available)
```bash
cargo clippy
```

## In-Game Commands (Dev Mode)
- **K Key:** +1000 XP (dev cheat)
- **F3:** Toggle dev panel
- **F5:** Toggle free camera mode
- **F1-F4:** Collision debug visualization
- **ESC:** Pause menu

## Git Workflow
```bash
git status
git add .
git commit -m "message"
git push
```
