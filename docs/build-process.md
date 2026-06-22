---
name: build-process
description: How to build the VPK locally and via CI
metadata: 
  node_type: memory
  type: project
  originSessionId: e677ffc5-a282-4329-9def-1068650ff7e1
---

## Local build

Requires VitaSDK installed at `$HOME/vitasdk`.

```bash
VITASDK=$HOME/vitasdk PATH="$HOME/vitasdk/bin:$HOME/.cargo/bin:$PATH" cargo vita build vpk --release
```

**Prerequisites:**
- VitaSDK (arm-vita-eabi toolchain) at `$HOME/vitasdk`
- Rust nightly-2025-06-01 (pinned in `rust-toolchain.toml`)
- `cargo-vita` installed (`cargo install cargo-vita --locked`)

**Output:** `target/armv7-sony-vita-newlibeabihf/release/vita-save-cloud.vpk`

## CI build

`.github/workflows/build.yml` triggers on `v*` tag pushes and `workflow_dispatch`.

Uses `vitasdk/vitasdk:latest` Docker container (Alpine). Key deps installed via `apk`: `gcc`, `musl-dev`, `perl` (perl needed for ring crate assembly generation).

Rust installed via rustup, cargo-vita built from source. Caches at `/root/.rustup` and `/root/.cargo`.

Release creation needs `contents: write` permission.

**Common CI failures:**
- `apt-get: not found` → container is Alpine, use `apk`
- `perl: not found` → ring's build.rs needs perl for ARM assembly
- `HOME differs from euid` → set `HOME: /root` in env
- `cargo vita: no such command` → cargo-vita install failed silently; check gcc is installed

[[project-overview]] [[version-bumping]]
