---
name: sqlite-vfs
description: The SQLite VFS fix that allows reading ur0:/shell/db/app.db for real game names and icons
metadata: 
  node_type: memory
  type: project
  originSessionId: e677ffc5-a282-4329-9def-1068650ff7e1
---

## Problem

SQLite on Vita (`SceSqlite_stub`) cannot open `ur0:/shell/db/app.db` because Sony's filesystem is read-only and the standard VFS refuses the path.

## Solution

Integrated VitaShell's custom SQLite VFS (`c/vita_sqlite_vfs.c`, ~183 lines, GPL-3.0) from TheOfficialFloW/VitaShell.

This VFS registers as `"psp2_rw"` and overrides `xOpen`/`xDelete` with raw `sceIo*` calls, bypassing Sony's read-only restriction.

**Key functions:**
- `sqlite_init()` — registers the custom VFS (replaces `sqlite3_initialize()`)
- `sqlite_exit()` — unregisters it (replaces `sqlite3_shutdown()`)

## Integration

In `c/tai.c`:
- `extern int sqlite_init(void);` / `extern int sqlite_exit(void);`
- Called instead of standard `sqlite3_initialize()`/`sqlite3_shutdown()`

In `build.rs`:
- `vita_sqlite_vfs.c` compiled via `cc::Build`
- `libVitaShellUser_stub_weak.a` linked (provides `VitaShellUser` exports at `./c/`)

## Important

- `SceSqlite_stub` provides the SQLite API (link-time)
- The custom VFS only changes the I/O layer — all SQL operations go through firmware SQLite
- `sqlite3.c` (8.8MB amalgamation) was removed — it was a leftover from the original dev, never compiled, and inflated repo language stats to 98% C

**Why:** Without this, the app showed raw title IDs instead of game names, and no game icons.

[[project-overview]] [[build-process]]
