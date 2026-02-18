# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Implemented `/translate_file` endpoint for `.txt` file translation with multipart form upload
- Added `GET /download/{id}` endpoint for retrieving translated files (1-hour TTL)
- Added `FileStore` with in-memory storage for translated files

### Fixed

- Changed default server host binding from `0.0.0.0` to `127.0.0.1` for Windows compatibility.

### Added

- CI release workflow (`.github/workflows/release.yml`) for multi-platform builds (Linux, macOS, Windows).
- Documentation system: `docs/README.md`, `ROADMAP.md`, `PORTABLE_APP.md`, `STYLE.md`, `RELEASING.md`, `WORKFLOW.md`, `linux-dev-setup.md`.
- `scripts/docs-list.mjs` for documentation discovery.
- `bin/validate-docs` and `bin/test-gate` for doc validation and testing.

### Changed

- Disabled CI on normal pushes; builds now only run on release publish events.
- Updated `.gitignore` with project-relevant patterns (Rust artifacts, IDE configs, local docs).
- Updated dependencies: actix-web 4.12, clap 4.5, serde 1.0.228, serde_json 1.0.149, anyhow 1.0.101.
- Removed `build.yml` workflow that was incorrectly triggering on pushes despite being "disabled".

### Removed

- Removed commented-out code blocks from `llm.rs` and `main.rs`.
- Removed unused lifetime elision patterns in `llm.rs`.

### Fixed

- Fixed README examples to use `127.0.0.1` instead of `0.0.0.0`.
- Fixed README port references from `5000` to `5050`.

### Changed

- Updated PORTABLE_APP.md with complete API endpoint list (/detect, /frontend/settings).
- Updated ROADMAP.md with current PRIMARY_TODO progress (5/6 steps).
- Applied cargo fmt formatting across entire ltengine codebase.
- Fixed all cargo clippy warnings in ltengine crate.

### Documentation

- Added placeholder docs for non-applicable features (elevation, scripting, mislabel-inventory).
- Added `CODE_OF_CONDUCT.md` with front matter for validation compliance.
- Updated `PORTABLE_APP.md` to reflect actual host binding behavior.
- Documented GPU backend build requirements (Vulkan SDK, CUDA Toolkit) in PRIMARY_TODO.md.
- Verified CUDA 13.1 build on Windows with Ninja generator.
- Verified CUDA runtime works on NVIDIA RTX 3060 (3GB VRAM, 35 layers offloaded).
