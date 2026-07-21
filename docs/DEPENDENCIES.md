---
summary: 'LTEngine dependency roles and update rules.'
read_when:
  - Adding or updating dependencies.
  - Changing model download, HTTP, or llama.cpp integration.
---

# Dependencies

## Current Application Dependencies

| Dependency | Role | Source |
| --- | --- | --- |
| actix-web / actix-multipart | HTTP server and multipart parsing | `ltengine/Cargo.toml` |
| llama-cpp-2 | Local GGUF inference and acceleration backends | `llama-cpp-rs` git submodule |
| hf-hub | On-demand model download | Cargo registry |
| clap | CLI parsing | Cargo registry |
| serde / serde_json | LibreTranslate-compatible request/response data | Cargo registry |
| whatlang | Current language detection | Cargo registry |
| uuid | Translated-file download identifiers | Cargo registry |
| static-files crates | Compile-time embedded frontend | Cargo registry |
| encoding_rs | Token byte decoding | Cargo registry |

`Cargo.lock` is the exact resolved source of truth. `Cargo.toml` and `ltengine/Cargo.toml` own declared versions and feature flags.

Actix, multipart, UUID download storage, and static-file crates are inherited HTTP-only dependencies. Remove them after TODO 5–7 provide CLI parity; do not build new behavior on them.

`hf-hub` supports convenient model acquisition but is not part of the offline runtime path. Portable operation uses a staged GGUF file.

## Development Toolchain

- Stable Rust and Cargo.
- CMake, clang, and a C++ compiler for llama.cpp bindings.
- Optional CUDA Toolkit, Metal toolchain, or Vulkan SDK for acceleration.

These are build dependencies. A portable release must not require users to install the Rust or native compilation toolchain.

## Portable Runtime Contract

- One Linux binary plus its required shared system libraries, identified per release with `ldd` or equivalent.
- A local GGUF model, shipped separately when artifact size makes bundling impractical.
- No Python, Node.js, package manager, web server, external API, or runtime model download.
- No undocumented project-specific files outside the portable directory.

The current release artifacts have not yet passed this contract. Measure actual dynamic linkage before claiming broad Linux portability.

## Rules

- Health-check new dependencies before addition: maintenance, releases, adoption, license.
- Prefer existing crates or standard library for small tasks.
- Keep the llama-cpp submodule revision and Rust binding compatible.
- Treat HTTP crates and browser assets as temporary migration dependencies.
- Prefer dependencies that can ship inside the portable artifact without host setup.
- Review `Cargo.lock` diffs for transitive changes.
- Record notable dependency decisions or health checks here.
