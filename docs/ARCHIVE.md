---
summary: 'Archived completed LTEngine milestones and task history.'
read_when:
  - Reviewing completed roadmap work.
  - Tracing tasks collapsed from TODO.md.
---

# Archived Milestones

## DONE v0.1 Core API and Modernization

| What | Result |
| --- | --- |
| Translation API | `/translate`, `/detect`, `/languages`, frontend settings, and `/suggest` compatibility stub |
| Local inference | llama-cpp-2 GGUF inference with Gemma3 aliases and local model paths |
| Acceleration | CPU plus CUDA, Metal, and Vulkan build features |
| File translation foundation | `.txt` upload, in-memory UUID downloads, and one-hour TTL scaffold |
| Release automation | Release-triggered Linux, macOS, and Windows artifact workflow |
| Repository modernization | Dependency updates, formatting/clippy cleanup, docs tooling, and stale code cleanup |

## DONE Documentation Review Tasks 1–4

1. Defined first-party documentation review scope and mismatch inventory.
2. Fixed P0 runtime/model/release documentation inaccuracies.
3. Added contributor guidance and copy-pasteable setup documentation.
4. Improved docs ownership, indexing, and maintainer workflow labels.

## SUPERSEDED HTTP Tasks

The 2026-07-21 CLI-only product decision superseded these tasks without treating them as completed:

| Former task | Preserved requirement | Disposition |
| --- | --- | --- |
| 5 file error semantics | `/translate_file`: 415 type, 413 size, 400 fields/UTF-8, API-key enforcement, download expiry | Replace with direct CLI validation and errors in TODO 5–7 |
| 6 file integration tests | Upload URL/download content, expiry, auto source, source-equals-target | Replace HTTP boundary coverage with CLI document tests in TODO 6–7 |
| 7 file size semantics | Configurable 10 MiB byte limit; accept multiline input without `char_limit` | Retained in TODO 7 |
| 8 file API docs | Document fields, limits, status codes, lifetime, and API-key behavior | Replace with CLI/runtime documentation during TODO 6–8 |
| 11 inference concurrency | Safely remove the global request-time inference mutex | No concurrent server requests after TODO 8; revisit only for library callers |
| 12 request cancellation | Stop inference after an HTTP client disconnects | Removed with HTTP runtime |
| 13 split server modules | Reduce oversized `main.rs` by separating handlers and state | Replaced by core extraction and server deletion in TODO 5 and 8 |
| 20 Docker decision | Decide whether to publish a server container | Rejected; portable CLI directory is the distribution target |

Nothing above was completed by the decision. Reusable validation, limits, model behavior, and tests were carried into the replacement CLI tasks.
