---
summary: 'Source of truth for LTEngine runtime contract and API behavior.'
read_when:
  - Changing server runtime behavior.
  - Changing direct CLI or offline packaging behavior.
  - Updating API endpoints.
  - Modifying model loading behavior.
---

# LTEngine Runtime Reference

This document is the repository-owned source of truth for the LTEngine runtime.

The durable product direction lives in `docs/PROJECT_SPEC.md`. This file distinguishes shipped behavior from the portable target.

## Scope

- Current application: direct text/stdin CLI plus a legacy local HTTP API server.
- Primary binary: `ltengine` (Rust/actix-web).
- API format: LibreTranslate-compatible JSON over HTTP.
- LLM backend: llama.cpp via the `llama-cpp-2` binding.
- Model format: GGUF (default: Gemma3 family from HuggingFace).
- Primary validation pair: Swedish to English; supported languages remain broader.

## Portable Target

- Run from an unpacked Linux directory with no project-specific installation.
- Translate text, stdin, and documents directly through the CLI without opening a TCP listener.
- Offer a native drag-and-drop GUI that calls the same Rust core without a web server or browser bridge.
- Load a staged local GGUF model and require no network or external service at runtime.
- Remove the HTTP server and browser UI after equivalent direct workflows exist.
- Reuse the extracted translation core for validation, prompting, inference, formatting, and detection metadata.

Direct document translation, the native GUI, and release-grade portable packaging are not implemented yet.

## Runtime Contract

- HTTP server binds to `127.0.0.1:5050` by default.
- `translate` handles direct text or stdin without opening a TCP listener.
- Compatible with LibreTranslate API endpoints.
- Local inference only - no external API calls.
- Models downloaded on-demand from HuggingFace or loaded from local path.

For an offline run, supply a model already present on disk with `--model-file`; automatic model retrieval is not offline-safe.

If this contract changes, update this file first.

## Direct Text CLI

Translate an argument:

```bash
./ltengine translate --source sv --target en --text 'Hej världen!' --model-file ./models/model.gguf
```

Translate stdin:

```bash
printf 'Hej världen!\n' | ./ltengine translate --source auto --target en --stdin --model-file ./models/model.gguf
```

Exactly one of `--text` or `--stdin` is required. `--source` accepts a language code or `auto`; `--target` requires a supported code. `--model` defaults to `gemma3-4b`, while `--model-file` selects a staged local GGUF file. Translated text is written to stdout with a trailing newline. Model-loading diagnostics and actionable input, validation, or inference errors use stderr and a non-zero exit status.

Running `ltengine` without a subcommand still starts the temporary HTTP server.

## Architecture

1. Client sends HTTP POST to `/translate` with JSON body.
2. actix-web handler receives request and validates interface-specific limits in `main.rs`.
3. `translation.rs` validates languages/format, builds the prompt, calls inference, and formats output.
4. `llm.rs` manages the LLM context and token generation behind the core's `Inference` boundary.
5. Response returns JSON with `translatedText` and optional detection metadata.

## Key Files

- `ltengine/src/main.rs`: HTTP server setup and request handlers.
- `ltengine/src/cli.rs`: direct command contract, stdin/text execution, and CLI tests.
- `ltengine/src/translation.rs`: reusable translation behavior and controlled-engine tests.
- `ltengine/src/llm.rs`: LLM initialization and inference.
- `ltengine/src/prompt.rs`: Translation prompt templates.
- `ltengine/src/languages.rs`: Supported language codes.
- `ltengine/src/models.rs`: Model configuration and download.
- `ltengine/Cargo.toml`: Dependencies and features (cuda, metal, vulkan).

## Supported API Surface

The server currently supports:

- `POST /translate` - Translate text
  - Request: `{"q": "Hello", "source": "en", "target": "es"}`
  - Response: `{"translatedText": "¡Hola!"}`
  - Supports `source: "auto"` for auto-detection
  - Returns `detectedLanguage` when source is "auto"
  - Returns HTTP 500 when model inference fails; source text is not returned as a false translation
- `POST /detect` - Detect language of text
  - Request: `{"q": "Hello world"}`
  - Response: `[{"language": "en", "confidence": 99}]`
- `GET /languages` - List supported languages
- `GET /frontend/settings` - Frontend configuration
- `POST /translate_file` - Document translation
  - Request: Multipart form with `file`, `source`, `target`, and optional `format` fields
  - Supported formats: `.txt` (text files only, initially)
  - Current limits: 10 MiB upload size and the server `char_limit` for decoded text
  - Current errors use 400 for invalid type, oversize input, invalid UTF-8, or missing fields
  - Current limitation: the multipart form does not accept `api_key`
  - Response: `{"translatedFileUrl": "/download/{uuid}"}`
  - Intended download TTL: 1 hour
  - Current limitation: cleanup is not called, so downloads remain until process exit
- `GET /download/{id}` - Download translated file
  - Response: Binary file download
  - Current lifetime: until server process exit

**Not implemented (return 501):**
- `POST /suggest` - Translation suggestions

## Model Configuration

Models are specified via command line:

```bash
./ltengine -m gemma3-4b          # Use the default Gemma3 4B
./ltengine --model-file /path/to/model.gguf  # Use custom model
```

Supported model aliases:
- `gemma3-1b` - 1GB RAM / 2GB VRAM (testing only)
- `gemma3-4b` - 4GB RAM / 4GB VRAM (default)
- `gemma3-12b` - 8GB RAM / 10GB VRAM
- `gemma3-27b` - 16GB RAM / 18GB VRAM (best quality)

## Build

```bash
git clone https://github.com/escapables/LTEngine-esc.git --recursive
cd LTEngine-esc
cargo build --release
```

## Run

```bash
./target/release/ltengine
# Or with specific model
./target/release/ltengine -m gemma3-4b
```

## Hardware Acceleration

Feature flags for build:
- `cuda` - NVIDIA CUDA support
- `metal` - Apple Metal support (macOS)
- `vulkan` - Vulkan support

Example:
```bash
cargo build --release --features cuda
```

## Troubleshooting

- "Out of memory": Use a smaller model (gemma3-1b or gemma3-4b).
- "Model download failed": Check internet connection or use `--model-file` with local GGUF.
- Slow inference: Enable hardware acceleration feature (cuda/metal/vulkan).

## Change Playbooks

### Add a new API endpoint

1. Add route handler in `main.rs`.
2. Add request/response types using serde.
3. Add tests for the new endpoint.

### Add a new model alias

1. Add model configuration in `src/models.rs`.
2. Update model documentation in `README.md` and this file.

## Guardrails

- Do not add HTTP/API features while the server is pending removal.
- Keep Swedish-to-English document translation in representative acceptance fixtures.
- Maintain offline capability: no required external APIs, runtime downloads, or loopback socket for the target primary workflow.
- Treat build dependencies separately from end-user runtime requirements.
- Keep this file synchronized with actual shipped behavior.

## Freshness Checklist

Update this file whenever any of the following changes:

- API endpoint coverage in `main.rs`
- Model configuration in `models.rs`
- Build process or feature flags
- Runtime contract (API compatibility)
