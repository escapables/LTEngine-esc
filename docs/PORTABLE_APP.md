---
summary: 'Source of truth for LTEngine runtime contract, architecture, and server behavior.'
read_when:
  - Changing server runtime behavior.
  - Updating API endpoints.
  - Modifying model loading behavior.
---

# LTEngine App Reference

This document is the repository-owned source of truth for the LTEngine runtime.

## Scope

- Local AI machine translation HTTP API server.
- Primary binary: `ltengine` (Rust/actix-web).
- API format: LibreTranslate-compatible JSON over HTTP.
- LLM backend: llama.cpp via llama-cpp-rs.
- Model format: GGUF (default: Gemma3 family from HuggingFace).

## Runtime Contract

- HTTP server binds to `127.0.0.1:5050` by default.
- Compatible with LibreTranslate API endpoints.
- Local inference only - no external API calls.
- Models downloaded on-demand from HuggingFace or loaded from local path.

If this contract changes, update this file first.

## Architecture

1. Client sends HTTP POST to `/translate` with JSON body.
2. actix-web handler receives request in `main.rs`.
3. `llm.rs` manages the LLM context and token generation.
4. `prompt.rs` constructs translation prompts for the LLM.
5. `languages.rs` provides language code mappings.
6. Response returns JSON with `translatedText` field.

## Key Files

- `ltengine/src/main.rs`: HTTP server setup and request handlers.
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
- `POST /detect` - Detect language of text
  - Request: `{"q": "Hello world"}`
  - Response: `[{"language": "en", "confidence": 99}]`
- `GET /languages` - List supported languages
- `GET /frontend/settings` - Frontend configuration

**Not implemented (return 501):**
- `POST /translate_file` - Document translation
- `POST /suggest` - Translation suggestions

## Model Configuration

Models are specified via command line:

```bash
./ltengine -m gemma3-4b          # Use default Gemma3 4B
./ltengine --model-file /path/to/model.gguf  # Use custom model
```

Supported model aliases:
- `gemma3-1b` - 1GB RAM / 2GB VRAM (testing only)
- `gemma3-4b` - 4GB RAM / 4GB VRAM (default)
- `gemma3-12b` - 8GB RAM / 10GB VRAM
- `gemma3-27b` - 16GB RAM / 18GB VRAM (best quality)

## Build

```bash
git clone https://github.com/LibreTranslate/LTEngine --recursive
cd LTEngine
cargo build [--features cuda,vulkan,metal] --release
```

## Run

```bash
./target/release/ltengine
# Or with specific model
./target/release/ltengine -m gemma3-12b
```

## Hardware Acceleration

Feature flags for build:
- `cuda` - NVIDIA CUDA support
- `metal` - Apple Metal support (macOS)
- `vulkan` - Vulkan support
- `native` - Native CPU optimizations

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

- Keep API compatible with LibreTranslate where possible.
- Maintain offline capability - no required external APIs at runtime.
- Keep this file synchronized with actual shipped behavior.

## Freshness Checklist

Update this file whenever any of the following changes:

- API endpoint coverage in `main.rs`
- Model configuration in `models.rs`
- Build process or feature flags
- Runtime contract (API compatibility)
