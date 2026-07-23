# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Added direct text and stdin translation through the `translate` CLI subcommand.
- Added bounded UTF-8 `.txt` translation to a selected non-existing output path.
- Extracted reusable translation behavior behind a controlled inference boundary.

### Fixed

- Return translation inference failures instead of silently returning the source text.

### Added

- CI release workflow (`.github/workflows/release.yml`) for multi-platform builds (Linux, macOS, Windows).
- Documentation system: `docs/README.md`, `ROADMAP.md`, `PORTABLE_APP.md`, `STYLE.md`, `RELEASING.md`, `WORKFLOW.md`, `linux-dev-setup.md`.
- `scripts/docs-list.mjs` for documentation discovery.
- `bin/validate-docs` and `bin/test-gate` for doc validation and testing.

### Changed

- Disabled CI on normal pushes; builds now only run on release publish events.
- Updated `.gitignore` with project-relevant patterns (Rust artifacts, IDE configs, local docs).
- Reduced direct runtime dependencies to the CLI, model acquisition, inference, decoding, and error-handling crates.
- Removed `build.yml` workflow that was incorrectly triggering on pushes despite being "disabled".

### Removed

- Removed commented-out code blocks from `llm.rs` and `main.rs`.
- Removed unused lifetime elision patterns in `llm.rs`.
- Removed the Actix HTTP runtime, LibreTranslate endpoints, API/download state, browser resources, and HTTP-only dependencies.
- Removed the unused `whatlang` detection metadata path; `--source auto` now delegates recognition to the model.

### Changed

- Updated PORTABLE_APP.md for the direct CLI-only runtime.
- Updated ROADMAP.md with current PRIMARY_TODO progress (5/6 steps).
- Applied cargo fmt formatting across entire ltengine codebase.
- Fixed all cargo clippy warnings in ltengine crate.

### Documentation

- Added placeholder docs for non-applicable features (elevation, scripting, mislabel-inventory).
- Added `CODE_OF_CONDUCT.md` with front matter for validation compliance.
- Updated runtime, architecture, dependency, release, and development docs after HTTP removal.
- Documented GPU backend build requirements (Vulkan SDK, CUDA Toolkit) in PRIMARY_TODO.md.
- Verified CUDA 13.1 build on Windows with Ninja generator.
- Verified CUDA runtime works on NVIDIA RTX 3060 (3GB VRAM, 35 layers offloaded).
