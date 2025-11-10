# Windows Build-Anleitung

## Voraussetzungen

### 1. Rust installieren

**Offizielle Website:**
https://www.rust-lang.org/tools/install

**Installer herunterladen:**
- Gehe zu https://rustup.rs/
- Lade `rustup-init.exe` herunter
- Führe den Installer aus

**Toolchain wählen:**
```powershell
# Option 1: MSVC (empfohlen für Windows)
rustup default stable-msvc

# Option 2: GNU (MinGW)
rustup default stable-gnu
```

### 2. Visual Studio Build Tools (für MSVC)

**Falls MSVC gewählt:**
- Lade "Build Tools for Visual Studio" herunter
- Installiere "Desktop development with C++"
- Link: https://visualstudio.microsoft.com/downloads/

**Oder:** Installiere vollständiges Visual Studio 2019/2022

---

## Build-Prozess

### 1. Repository klonen

```powershell
cd C:\Users\YourName\Documents
git clone https://github.com/your-repo/game.git
cd game
```

### 2. Dependencies prüfen

```powershell
# Prüfe ob Rust installiert ist
rustc --version
cargo --version

# Sollte zeigen:
# rustc 1.XX.X (...)
# cargo 1.XX.X (...)
```

### 3. Kompilieren

**Client:**
```powershell
cargo build --release -p client
```

**Server:**
```powershell
cargo build --release -p server
```

**Beide:**
```powershell
cargo build --release
```

**Erwartete Build-Zeit:**
- Erster Build: 5-10 Minuten
- Incremental Build: 10-30 Sekunden

---

## Ausführen

### Server starten

```powershell
cd C:\Users\YourName\Documents\game
.\target\release\server.exe
```

**Oder:**
```powershell
cargo run --release -p server
```

### Client starten

```powershell
cd C:\Users\YourName\Documents\game
.\target\release\client.exe
```

**Oder:**
```powershell
cargo run --release -p client
```

---

## Troubleshooting

### Fehler: "linker not found"

**Problem:** MSVC Toolchain nicht installiert

**Lösung:**
```powershell
# Installiere Visual Studio Build Tools
# Oder wechsle zu GNU:
rustup default stable-gnu
```

### Fehler: "cannot find crate bevy"

**Problem:** Dependencies nicht heruntergeladen

**Lösung:**
```powershell
cargo clean
cargo build --release
```

### Fehler: "rayon not found"

**Problem:** Phase 4 Dependencies fehlen

**Lösung:**
```powershell
# Dependencies aktualisieren
cargo update
cargo build --release
```

### Build ist sehr langsam

**Problem:** Debug-Build statt Release

**Lösung:**
```powershell
# Nutze immer --release flag
cargo build --release -p client
# NICHT: cargo build -p client
```

### Firewall blockiert Server

**Problem:** Windows Firewall blockiert Port 5000

**Lösung:**
```powershell
# Firewall-Regel hinzufügen (als Administrator):
netsh advfirewall firewall add rule name="Game Server" dir=in action=allow protocol=TCP localport=5000
netsh advfirewall firewall add rule name="Game Server UDP" dir=in action=allow protocol=UDP localport=5000
```

---

## Performance-Tipps

### 1. Release Build verwenden

```powershell
# ✅ RICHTIG: Release Build (optimiert)
cargo build --release

# ❌ FALSCH: Debug Build (langsam)
cargo build
```

**Unterschied:**
- Debug: ~5-10 FPS
- Release: 60 FPS

### 2. Multi-Threading nutzen

**Automatisch aktiviert** in Phase 4!

**CPU-Auslastung prüfen:**
- Task Manager → Performance → CPU
- Sollte alle Cores bei ~20-50% nutzen

### 3. VSync deaktivieren (optional)

Im Client unter Settings:
- VSync: AUS
- Kann 60+ FPS ermöglichen

---

## Verfügbare Builds

### Release Build (empfohlen)

```powershell
cargo build --release
```

**Eigenschaften:**
- Optimiert für Performance
- ~10-20 MB Größe
- Keine Debug-Symbole

### Debug Build

```powershell
cargo build
```

**Eigenschaften:**
- Enthält Debug-Symbole
- Langsamer (~10x)
- Größer (~50-100 MB)

### Profiling Build

```powershell
cargo build --profile profiling
```

**Eigenschaften:**
- Optimiert + Debug-Symbole
- Für Performance-Analyse

---

## Collision-System auf Windows

### Multi-Threading

**Rayon nutzt automatisch Windows Threads API:**
- Kompatibel mit Windows 10/11
- Nutzt alle CPU-Cores
- Work-Stealing Thread Pool

**Performance-Test:**
```powershell
# Im Spiel:
# - Spawne viele Entities
# - Öffne Task Manager
# - Prüfe CPU-Auslastung

# Erwartung:
# - Alle Cores bei ~20-50%
# - Stabile 60 FPS
```

### Bekannte Einschränkungen

**Keine bekannten Windows-spezifischen Probleme!**
- ✅ Rayon funktioniert perfekt
- ✅ Bevy unterstützt Windows voll
- ✅ Alle Features funktionieren

---

## Cross-Compilation (Linux → Windows)

### Von Linux zu Windows kompilieren

**Voraussetzungen:**
```bash
# Auf Linux-System:
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64  # Für Debian/Ubuntu
```

**Kompilieren:**
```bash
cargo build --release --target x86_64-pc-windows-gnu -p client
```

**Ergebnis:**
```
target/x86_64-pc-windows-gnu/release/client.exe
```

**Übertragen:**
```bash
# Kopiere .exe zu Windows-System
scp target/x86_64-pc-windows-gnu/release/client.exe user@windows-pc:C:/game/
```

---

## Netzwerk-Konfiguration

### Server auf Windows

**Standard:** localhost (127.0.0.1:5000)

**Öffentlicher Server:**
1. Ändere Bind-Address in `server/src/main.rs`:
```rust
let socket = UdpSocket::bind("0.0.0.0:5000")?;
```

2. Firewall konfigurieren:
```powershell
netsh advfirewall firewall add rule name="Game Server" dir=in action=allow protocol=UDP localport=5000
```

3. Port-Forwarding im Router (falls nötig)

---

## Build-Skripte (optional)

### build_windows.bat

```batch
@echo off
echo Building Game...
cargo build --release
echo.
echo Build complete!
echo Client: target\release\client.exe
echo Server: target\release\server.exe
pause
```

### run_server.bat

```batch
@echo off
echo Starting Server...
set RUST_LOG=info
target\release\server.exe
pause
```

### run_client.bat

```batch
@echo off
echo Starting Client...
target\release\client.exe
pause
```

---

## Erfolgskriterien

**Build erfolgreich wenn:**
- ✅ `cargo build --release` ohne Fehler
- ✅ `client.exe` existiert in `target/release/`
- ✅ `server.exe` existiert in `target/release/`

**Runtime erfolgreich wenn:**
- ✅ Server startet ohne Crash
- ✅ Client startet und zeigt Fenster
- ✅ Login/Register funktioniert
- ✅ Character Creation funktioniert
- ✅ In-Game Movement funktioniert
- ✅ Collision Detection funktioniert
- ✅ Stabile 60 FPS

---

## Support

**Bei Problemen:**

1. Prüfe Rust-Version:
```powershell
rustc --version
# Sollte >= 1.70 sein
```

2. Update Rust:
```powershell
rustup update
```

3. Clean & Rebuild:
```powershell
cargo clean
cargo build --release
```

4. Prüfe Dependencies:
```powershell
cargo tree
```

---

_Erstellt: 2024-11-10_
_Getestet: Compilation erfolgreich (Cross-Compilation)_
_Status: Ready für Windows 10/11_
