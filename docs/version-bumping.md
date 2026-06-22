---
name: version-bumping
description: How to bump the version — all places that need updating and the release workflow
metadata: 
  node_type: memory
  type: project
  originSessionId: e677ffc5-a282-4329-9def-1068650ff7e1
---

## Version bump checklist

1. **`Cargo.toml`** — `version = "X.Y.Z"` (line 3). This feeds `env!("CARGO_PKG_VERSION")` used by the splash screen and `vita-mksfoex` for the SFO.
2. **`README.md`** — the release badge uses `img.shields.io/github/v/release/unveroleone/vita-save-sync` (dynamic, auto-updates from latest GitHub release — no manual change needed).
3. **Git tag** — `git tag vX.Y.Z && git push origin vX.Y.Z`. The tag triggers CI (`.github/workflows/build.yml`) which builds the VPK and creates a GitHub Release.

## No other hardcoded versions

- `src/ui/ui_desktop.rs` uses `concat!("v", env!("CARGO_PKG_VERSION"))` — no manual update
- `src/constant.rs` ABOUT_TEXT no longer contains a version number
- Version in the `.sfo` comes from `Cargo.toml` via `cargo-vita`

## Release flow

```
bump Cargo.toml → git commit → git push → git tag vX.Y.Z → git push vX.Y.Z
   → CI builds VPK → creates GitHub Release with VPK attached
```

**VPK download URL pattern:** `https://github.com/unveroleone/vita-save-sync/releases/download/vX.Y.Z/vita-save-cloud.vpk`

[[project-overview]] [[build-process]]
