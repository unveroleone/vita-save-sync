# Save Sync for PS Vita

Manual cloud save-sync tool for two modded PS Vita devices backed by a self-hosted HTTP server.
Forked from [Save Cloud Vita](https://github.com/save-cloud) — Baidu Cloud replaced with a simple custom API.

## Features

- Local save backup to `ux0:data/save-sync/backups/<TITLEID>/`
- Upload saves to a self-hosted server
- Download newer saves from the server
- Restore saves with automatic pre-restore safety backup
- Sync overview with per-game status badges (In Sync, Upload Needed, Download Available, Conflict)
- Sync All: upload changed, download newer, stop on conflicts
- Device pairing with a static API token
- Simple list-style UI built on `libvita2d`

## Out of scope (MVP)

- Background auto-sync
- Trophy sync
- Google Drive / Dropbox / Baidu backends
- Multi-user admin UI
- Save file merging

## Project structure

```
save-cloud-vita/
  src/               # PS Vita Rust client (vita-rust + libvita2d)
  server/            # Self-hosted sync server (Node.js + Fastify)
  assets/            # App icon, splash, livearea templates
  c/                 # C FFI: vita2d, taihen, IME
```

## Quick start

### 1. Run the server

```bash
cd server
cp .env.example .env
# Edit .env: set USER_TOKEN, USER_NAME, DATA_DIR
docker compose up -d
```

Or bare metal:

```bash
cd server && cp .env.example .env
pnpm install && pnpm run build
USER_TOKEN=your-secret USER_NAME=you DATA_DIR=/data/vita-save-sync node dist/index.js
```

The server listens on port 3000. Put it behind Nginx Proxy Manager or Cloudflare Tunnel for HTTPS (the Vita client supports TLS via iTLS-Enso + rustls).

### 2. Build the Vita app

Requires macOS or Linux with:

```bash
# One-time setup
brew install cmake                      # macOS
git clone https://github.com/vitasdk/vdpm && cd vdpm
./bootstrap-vitasdk.sh && ./install-all.sh
# Add to ~/.zshrc: export VITASDK=/usr/local/vitasdk
# Add to ~/.zshrc: export PATH=$VITASDK/bin:$PATH
rustup install nightly-2025-06-01
rustup component add rust-src --toolchain nightly-2025-06-01
cargo +nightly install cargo-vita
# Then in the project directory:
rustup override set nightly-2025-06-01
git submodule update --init
cargo vita build vpk --release
```

The VPK lands at `target/armv7-sony-vita-newlibeabihf/release/vita-save-cloud.vpk`.

### 3. Install and configure on Vita

1. Transfer the VPK to your Vita and install
2. Open the app, press R to switch to the Sync view
3. Press Triangle to open Settings
4. Enter your server URL, API token, and device name
5. Press "Test Connection" to verify

### 3. Usage

**Titles view (L):**
- Browse installed games as icons
- Press Circle: backup/restore local zip saves
- Press Triangle: game menu (backup all, update account ID, delete)

**Sync view (R):**
- See all games with sync status badges
- Press X: Sync All (upload changed, download newer, skip conflicts)
- Press Triangle: Settings

**Per-game save drawer (from Titles, Circle):**
- Local tab: backup to zip, restore from zip, upload individual backup (SELECT)
- Server tab: upload to server, download from server, download & restore

## Server API

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| `GET` | `/api/status` | No | Health check, returns server version |
| `POST` | `/api/pair` | Token in body | Register device |
| `GET` | `/api/manifest` | Bearer | Get cloud manifest |
| `PUT` | `/api/manifest` | Bearer | Update manifest metadata |
| `PUT` | `/api/save/:titleId` | Bearer | Upload save zip with hash verification |
| `GET` | `/api/save/:titleId` | Bearer | Download latest save zip |

## Folder layout on Vita

```
ux0:data/save-sync/
  config.json          # server URL, token, device name
  manifest.json        # local sync baseline (auto-managed)
  backups/
    PCSE00001/
      meta.json
      2026-06-21 15.42.00.zip
  downloads/           # staging before restore
  logs/
    latest.log
```

## Build

```bash
cargo vita build vpk --release
```

Requires:
- [VitaSDK](https://github.com/vitasdk)
- [vita-rust](https://github.com/vita-rust)
- `vita` cargo subcommand

## Credits

Forked from [Save Cloud Vita](https://github.com/save-cloud) by iamcco.
Uses [VitaShell](https://github.com/TheOfficialFloW/VitaShell) modules for kernel access.
