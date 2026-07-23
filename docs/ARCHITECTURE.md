---
summary: 'LTEngine architecture, file map, data flow, and guardrails.'
read_when:
  - Changing CLI, document, model, or inference flow.
  - Adding model support or runtime state.
---

# Architecture

> LTEngine-esc — portable, offline-first Linux document translation through direct local interfaces.

Current code offers direct text, stdin, and `.txt` document translation. The inherited Actix server, LibreTranslate endpoints, API/download state, and browser resources were removed after CLI parity. Later work adds long-document slicing and a native GUI.

## Stack

- Rust 2024 Cargo workspace; primary crate `ltengine`.
- Clap command-line interface.
- llama-cpp-2 submodule binding for local GGUF inference.
- Hugging Face model acquisition or local `--model-file`.
- Optional CUDA, Metal, and Vulkan Cargo features.

## File Map

| Path | Responsibility |
| --- | --- |
| `ltengine/src/main.rs` | CLI bootstrap, model resolution, and LLM initialization |
| `ltengine/src/cli.rs` | Command parsing, text/stdin/document dispatch, output, and CLI tests |
| `ltengine/src/document.rs` | Bounded UTF-8 `.txt` input, safe output creation, layout preservation, and filesystem tests |
| `ltengine/src/translation.rs` | Interface-independent validation, prompting, inference orchestration, and formatting |
| `ltengine/src/llm.rs` | llama.cpp model context, serialized inference, and token generation |
| `ltengine/src/models.rs` | Model aliases and local/remote model resolution |
| `ltengine/src/prompt.rs` | Translation prompt construction |
| `ltengine/src/languages.rs` | Supported language-code mapping |
| `.github/workflows/release.yml` | Release-triggered platform builds and asset uploads |
| `bin/` | Local docs/build/test verification |

## Direct CLI Flow

1. Clap requires `translate`, parses required source/target and model options, and selects exactly one input mode.
2. The selected GGUF model resolves and loads without creating a listener or API state.
3. `cli::run_translate` reads text/stdin or delegates bounded `.txt` I/O to `document.rs`.
4. The translation core validates supplied language codes/format and creates system/user prompts.
5. `LLM::run_prompt` serializes local inference through `prompt_lock`.
6. Text goes to stdout; documents go to a selected new path; diagnostics and actionable failures go to stderr.

With `--source auto`, the prompt omits a fixed source language and delegates recognition to the model. The runtime does not calculate or emit source-detection metadata.

## Target Interface Flow

1. CLI or native GUI adapter validates document input and output choices.
2. Document pipeline separates paragraphs and splits oversized paragraphs within a token budget.
3. Shared translation core and model service translate slices sequentially.
4. Pipeline reassembles slices in order while preserving paragraph separators.
5. CLI writes output; GUI reports progress, previews output, and offers save-as.

Swedish-to-English documents are the primary acceptance path. Language and model abstractions remain general enough for other supported pairs.

## Runtime State

- `Args`: parsed direct CLI configuration.
- `LLM`: loaded model and serialized inference lock.
- No HTTP application state, file store, database, listener, or external translation API.

## Architecture Rules

- Keep model loading, translation, and document processing independent from interface parsing.
- Expose future slice progress/errors without coupling the core to a GUI toolkit.
- Do not require a listener, daemon, loopback network, or runtime model download for portable operation.
- Do not reintroduce HTTP endpoints or browser resources.
- Treat document content, filenames, and output paths as untrusted.
- Make resource limits explicit and test their boundaries.
- Keep blocking inference off future GUI event loops.
- Record shipped runtime behavior in `docs/PORTABLE_APP.md`.

## Known Debt

- Inference is globally serialized due to suspected llama.cpp thread-safety behavior.
- Whole-document inference is model-context limited until paragraph slicing.
- Automatic source handling relies on the translation model; short-text detection remains queued for evaluation.
- Portable artifacts have not passed clean-host offline acceptance.

## Verification

| Command | Purpose |
| --- | --- |
| `bin/validate-docs` | Docs metadata, index, TODO, roadmap, handoff checks |
| `bin/loc-check` | Authored-file size guardrail |
| `bin/verify-fast` | Docs, LOC, formatting, and compile checks |
| `bin/test-gate` | Full format, clippy, and test gate |
