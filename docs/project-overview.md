---
name: project-overview
description: Save Sync — PS Vita homebrew app for manual cloud save sync between two modded Vitas
metadata: 
  node_type: memory
  type: project
  originSessionId: e677ffc5-a282-4329-9def-1068650ff7e1
---

Save Sync is a Rust + C PS Vita homebrew that backs up game saves and syncs them via a self-hosted HTTP server. Forked from Save Cloud Vita (Baidu Cloud integration). All Baidu/Chinese code removed, replaced with a Fastify server and Rust HTTP client via ureq.

**Stack:**
- Vita client: Rust + C, libvita2d, nightly toolchain (`nightly-2025-06-01`), cargo-vita
- Server: Node.js + Fastify + TypeScript (in `server/`)
- CI: GitHub Actions with `vitasdk/vitasdk:latest` Docker container

**Title ID:** `SAVSYNC01`
**Package name:** `vita-save-cloud` (kept from fork to not break Cargo.toml references)
**App display name:** Save Sync

[[build-process]] [[sqlite-vfs]] [[version-bumping]]
