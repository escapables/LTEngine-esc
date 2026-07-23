---
summary: 'Source of truth for LTEngine runtime and direct CLI behavior.'
read_when:
  - Changing direct CLI or offline packaging behavior.
  - Modifying model loading or translation behavior.
  - Verifying removed HTTP behavior stays removed.
---

# LTEngine Runtime Reference

This document is the repository-owned source of truth for shipped LTEngine runtime behavior. The durable product direction lives in `docs/PROJECT_SPEC.md`.

## Scope

- Current application: direct text, stdin, and UTF-8 `.txt` translation through a required CLI subcommand.
- Primary binary: `ltengine` (Rust).
- LLM backend: llama.cpp through the `llama-cpp-2` binding.
- Model format: GGUF; `gemma3-4b` is the default alias.
- Primary validation pair: Swedish to English; supported language pairs remain broader.
- Removed surface: HTTP server, LibreTranslate endpoints, API/download state, and browser UI.

The native GUI, long-document slicing, clean-host offline acceptance, and release-grade portable packaging are not implemented yet.

## Runtime Contract

- `translate` handles text, stdin, or `.txt` documents without opening a TCP listener.
- Running without a subcommand fails with CLI usage; it does not load a model or start a server.
- The binary exposes no HTTP or LibreTranslate-compatible API.
- Inference is local; no external translation API is called.
- Models load from `--model-file` or are acquired on demand through the selected model alias.

For offline use, stage a compatible GGUF file and pass `--model-file`. Automatic model retrieval is not offline-safe.

## Direct Text CLI

Translate an argument:

```bash
./ltengine translate --source sv --target en \
  --text 'Hej världen!' --model-file ./models/model.gguf
```

Translate stdin:

```bash
printf 'Hej världen!\n' | ./ltengine translate \
  --source auto --target en --stdin --model-file ./models/model.gguf
```

Exactly one of `--text`, `--stdin`, or `--input` is required. `--source` accepts a supported language code or `auto`; `--target` requires a supported code. With `auto`, source recognition is delegated to the model and no detection metadata is emitted.

Translated text is written to stdout with a trailing newline. Model-loading diagnostics and actionable input, validation, or inference errors use stderr and a non-zero exit status.

## Direct Document CLI

```bash
./ltengine translate --source sv --target en \
  --input ./source.txt --output ./translated.txt \
  --model-file ./models/model.gguf
```

Document mode accepts UTF-8 `.txt` input and requires a `.txt` output path. The default byte limit is 10 MiB; `--max-input-bytes` configures it. Leading/trailing whitespace, line endings, and internal model-produced multiline structure are preserved. Existing outputs and input/output aliases are rejected. Output is created only after input validation and successful inference. A write failure may leave a partial newly created output and reports that explicitly.

Whole-document inference remains limited by the model context until paragraph slicing is implemented.

## Runtime Flow

1. Clap validates the required `translate` subcommand, model options, required source/target arguments, and exclusive input mode.
2. `models.rs` resolves a staged GGUF path or downloads the selected alias.
3. `llm.rs` initializes local llama.cpp inference.
4. `cli.rs` reads text/stdin or delegates bounded document I/O to `document.rs`.
5. `translation.rs` validates the supplied language codes and format, builds the prompt, calls inference, and formats output.

## Key Files

- `ltengine/src/main.rs`: CLI bootstrap and model initialization.
- `ltengine/src/cli.rs`: command contract, text/stdin execution, document dispatch, and CLI tests.
- `ltengine/src/document.rs`: bounded document I/O, path safety, layout preservation, and tests.
- `ltengine/src/translation.rs`: reusable translation behavior and controlled-engine tests.
- `ltengine/src/llm.rs`: LLM initialization and inference.
- `ltengine/src/prompt.rs`: translation prompt templates.
- `ltengine/src/languages.rs`: supported language-code mapping.
- `ltengine/src/models.rs`: model aliases and local/remote model resolution.
- `ltengine/Cargo.toml`: dependencies and acceleration features.

## Model Configuration

Model options are global and may appear before or after `translate`:

```bash
./ltengine translate --source sv --target en --text 'Hej' -m gemma3-4b
./ltengine translate --source sv --target en --text 'Hej' \
  --model-file ./models/model.gguf
```

Supported aliases:

- `gemma3-1b` - 1 GB RAM / 2 GB VRAM (testing only)
- `gemma3-4b` - 4 GB RAM / 4 GB VRAM (default)
- `gemma3-12b` - 8 GB RAM / 10 GB VRAM
- `gemma3-27b` - 16 GB RAM / 18 GB VRAM (best quality)

## Build

```bash
git clone https://github.com/escapables/LTEngine-esc.git --recursive
cd LTEngine-esc
cargo build --release
```

Acceleration features: `cuda`, `metal`, and `vulkan`.

```bash
cargo build --release --features cuda
```

## Troubleshooting

- "Out of memory": use a smaller model such as `gemma3-1b` or `gemma3-4b`.
- "Model download failed": check connectivity or pass a staged GGUF with `--model-file`.
- Slow inference: enable an available acceleration feature.
- Missing command: run `./ltengine --help`, then use `./ltengine translate ...`.

## Change Playbooks

### Change CLI behavior

1. Update Clap arguments in `src/cli.rs`.
2. Add parsing and controlled-inference regressions.
3. Keep stdout limited to translated text and stderr for diagnostics/errors.
4. Update this file and README examples.

### Add a model alias

1. Add model configuration in `src/models.rs`.
2. Update model tables in `README.md` and this file.
3. Add or update the focused model regression.

## Guardrails

- Do not reintroduce HTTP, browser, daemon, or loopback runtime behavior.
- Keep Swedish-to-English document translation in representative acceptance fixtures.
- Keep other supported language pairs functional.
- Treat document content, filenames, and output paths as untrusted.
- Maintain offline capability through staged local models.
- Keep build dependencies separate from end-user runtime requirements.

## Freshness Checklist

Update this file when changing:

- CLI arguments or stdout/stderr behavior
- document limits, formats, or path safety
- model configuration or loading
- build process or acceleration features
- runtime network/offline behavior
