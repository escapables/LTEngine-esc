---
summary: 'Rust style guide used by this project.'
read_when:
  - Writing or reviewing Rust code.
  - Running style cleanup work.
---

# LTEngine Rust Style

## Overview

This project follows standard Rust conventions with some project-specific preferences.

## Formatting

- Use `rustfmt` with default settings.
- Run `cargo fmt` before committing.
- Line length: 100 characters (configure in `rustfmt.toml` if needed).

## Linting

- Address all `cargo clippy` warnings.
- Prefer explicit over implicit (avoid `.into()` when type conversion is not obvious).
- Use `?` operator for error propagation instead of `match` when appropriate.

## Naming Conventions

Follow standard Rust naming:
- `PascalCase` for types, traits, enums, struct names
- `snake_case` for functions, variables, modules, macros
- `SCREAMING_SNAKE_CASE` for constants, statics
- `PascalCase` for enum variants

## Error Handling

- Use `anyhow` for application-level error handling.
- Use `thiserror` for library error types (in workspace dependencies).
- Prefer `Result<T, E>` over panics for recoverable errors.
- Use `expect()` with a descriptive message only for invariant violations.

## Documentation

- Document all public APIs with doc comments (`///`).
- Include examples in doc comments where helpful.
- Use `//` for implementation comments explaining "why", not "what".

## Imports

Group imports in this order:
1. Standard library (`std::`)
2. External crates
3. Internal crate modules (`crate::`)
4. Super modules (`super::`)

Separate groups with blank lines:

```rust
use std::path::PathBuf;

use anyhow::Result;

use crate::translation::Inference;
```

## Module Structure

- Keep modules focused on a single responsibility.
- Prefer flat module hierarchies over deep nesting.
- Re-export public items at the crate root or appropriate module boundary.

## Types

- Use strong typing - prefer newtype wrappers over raw primitives.
- Use `Option<T>` and `Result<T, E>` instead of sentinel values.
- Leverage Rust's type system to make invalid states unrepresentable.

## Blocking Work

- Keep the CLI synchronous unless concurrency provides measured value.
- LLM inference is blocking and serialized.
- Move inference off a future native GUI event loop.

## Unsafe Code

- Minimize unsafe code - prefer safe Rust alternatives.
- When unsafe is necessary (e.g., for C bindings to llama.cpp):
  - Document the safety invariants
  - Keep unsafe blocks as small as possible
  - Wrap in safe abstractions where possible

## Testing

- Write unit tests for pure functions.
- Write parsing and filesystem tests for CLI boundaries.
- Use `cargo test` to run all tests.
- Tests go in `#[cfg(test)]` modules or `tests/` directory.

## Dependencies

- Keep dependencies minimal - evaluate before adding.
- Pin major versions in `Cargo.toml`.
- Use workspace dependencies for shared crates (see `Cargo.toml`).

## Performance

- Avoid premature optimization - write clear code first.
- Use `cargo bench` for benchmarks if needed.
- Profile before optimizing.
- Consider feature flags for expensive optional functionality.

## FFI (llama-cpp-rs)

The project uses `llama-cpp-rs` for LLM inference:
- FFI bindings are handled by the library
- Use the safe Rust wrappers provided by `llama-cpp-2`
- Feature flags control hardware acceleration: `cuda`, `metal`, `vulkan`

## CI/Automation

- Code must pass `cargo check`, `cargo clippy`, and `cargo test`.
- Format with `cargo fmt`.
- All documentation builds with `cargo doc`.
