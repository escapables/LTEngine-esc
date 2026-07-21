---
summary: 'LTEngine architecture, file map, data flow, and guardrails.'
read_when:
  - Changing server structure or inference flow.
  - Adding endpoints, model support, or runtime state.
---

# Architecture

> LTEngine-esc — portable, offline-first Linux document translation through direct local interfaces.

Current code offers direct text/stdin translation plus the inherited HTTP server. Both use the reusable translation core. The next steps add document CLI/native GUI adapters, then remove Actix and the bundled browser UI.

## Stack

- Rust 2024 Cargo workspace; primary crate `ltengine`.
- actix-web HTTP server with embedded static frontend resources.
- llama-cpp-2 submodule binding for local GGUF inference.
- Hugging Face model download or local `--model-file`.
- Optional CUDA, Metal, and Vulkan Cargo features.

## File Map

| Path | Responsibility |
| --- | --- |
| `ltengine/src/main.rs` | Runtime dispatch, actix application, handlers, file store; HTTP removal target |
| `ltengine/src/cli.rs` | Command parsing, text/stdin input, direct translation output, and CLI tests |
| `ltengine/src/translation.rs` | Interface-independent validation, prompting, inference orchestration, formatting, and detection metadata |
| `ltengine/src/llm.rs` | llama.cpp model context, serialized inference, token generation |
| `ltengine/src/models.rs` | Model aliases and local/remote model resolution |
| `ltengine/src/prompt.rs` | Translation prompt construction |
| `ltengine/src/languages.rs` | Supported languages and whatlang detection |
| `ltengine/src/error_response.rs` | HTTP error response mapping |
| `ltengine/resources/` | Embedded browser frontend and vendored static assets |
| `.github/workflows/release.yml` | Release-triggered platform builds and asset uploads |
| `bin/` | Local docs/build/test verification |

## Request Flow

1. actix parses JSON, form, or multipart input.
2. Handler validates interface-specific required fields, API key, and limits.
3. The translation core validates languages/format and creates system/user prompts.
4. The core calls the model through the `Inference` boundary; `LLM::run_prompt` serializes access through `prompt_lock`.
5. llama.cpp generates tokens locally; inference failures return to the caller.
6. The core formats translated text and detection metadata; the handler creates the LibreTranslate-compatible JSON response.

File translation additionally stores translated bytes in an in-memory UUID-keyed store. A one-hour TTL and cleanup method exist, but cleanup is never called; current downloads remain until process exit.

## Direct CLI Flow

1. Clap validates `translate`, source/target, model options, and exactly one text/stdin input.
2. The selected GGUF model loads without creating an HTTP server or file store.
3. `cli::run_translate` reads input and calls the shared translation core.
4. Translated text goes to stdout; loading diagnostics and actionable failures go to stderr.

## Target Interface Flow

1. CLI or native GUI adapter validates document input and output choices.
2. Document pipeline separates paragraphs and splits oversized paragraphs within a token budget.
3. Shared translation core and model service translate slices sequentially.
4. Pipeline reassembles slices in order while preserving paragraph separators.
5. CLI writes output; GUI reports progress, previews output, and offers save-as.

Swedish-to-English documents are the primary acceptance path. Language and model abstractions must remain general enough for other supported pairs.

## Runtime State

- `Args`: parsed CLI/server configuration; wrapped in `Arc` only for legacy server mode.
- `Arc<LLM>`: loaded model plus inference lock.
- `Arc<Mutex<FileStore>>`: translated download content.
- No database or external translation API.

## Architecture Rules

- Keep request validation separate from translation behavior where practical.
- Keep model loading, translation, and document processing independent from interface parsing.
- Expose slice progress and errors without coupling the core to a specific UI toolkit.
- Do not require a listener, daemon, network, or runtime model download for direct portable operation.
- Remove HTTP dependencies and static frontend resources only after CLI parity protects current translation use cases.
- Treat uploaded content, multipart fields, UUIDs, and filenames as untrusted.
- Avoid holding file-store locks during inference.
- Keep blocking inference from starving actix workers.
- Preserve API response compatibility with LibreTranslate where implemented.
- Make resource limits explicit and test their boundaries.
- Record runtime/API behavior in `docs/PORTABLE_APP.md`.

## Known Debt

- `main.rs` remains a temporary mix of runtime dispatch, HTTP state, handlers, and server bootstrap; it is now below 500 LOC.
- Inference is globally serialized due to suspected llama.cpp thread-safety behavior.
- `/translate_file` lacks enforced expiry, integration tests, and full API-key/error semantics.

## Verification

| Command | Purpose |
| --- | --- |
| `bin/validate-docs` | Docs metadata, index, TODO, roadmap, handoff checks |
| `bin/loc-check` | Authored-file size guardrail |
| `bin/verify-fast` | Docs, LOC, formatting, and compile checks |
| `bin/test-gate` | Full format, clippy, and test gate |
