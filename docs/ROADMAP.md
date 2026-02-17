---
summary: 'Prioritized roadmap for LTEngine development.'
read_when:
  - Planning upcoming work.
  - Tracking what ships next.
  - Deciding near-term implementation priorities.
---

# Roadmap

## Current Status (2026-02-17)

- Core translation API functional with Gemma3 models.
- Supports CPU, CUDA, Metal, and Vulkan backends.
- PRIMARY_TODO modernization: 6 of 6 steps complete.

## Immediate Priority (Next Session)

**BLOCKED until PRIMARY_TODO complete:** Codebase modernization is required before new features.

1. ~~Implement CI release workflow for automated builds.~~ (Done - release.yml created)
2. Add `/translate_file` endpoint for document translation.
3. Implement sentence splitting for longer texts.
4. Add better language detection for short texts (port LexiLang to Rust).

## Completed Foundations

- Basic `/translate` endpoint with LibreTranslate compatibility.
- LLM inference via llama-cpp-rs.
- Model download from HuggingFace.
- Auto language detection using whatlang.
- Support for multiple hardware acceleration backends.
- Release workflow (`.github/workflows/release.yml`) created.

## In Progress / Planned

- [x] **PRIMARY_TODO:** Codebase cleanup and modernization (DONE - 6/6 steps)
- [ ] Remove mutex block limiting concurrent translation requests.
- [ ] Cancel inference when HTTP connections are aborted.
- [ ] Add `/translate_file` endpoint for document translation.
- [ ] Sentence splitting for long text chunking.
- [ ] Better language detection for short texts.
- [ ] Test/add more LLM models beyond Gemma3.
- [ ] Comparative benchmarks vs proprietary services.
- [ ] Command line inference mode (`./ltengine translate`).
- [ ] Library mode with bindings for other languages.
- [x] ~~Automated CI builds and releases.~~ (Workflow created, needs GPU verification)

## Release Automation Status

- [x] Workflow file created (`.github/workflows/release.yml`)
- [x] Multi-platform builds configured (Linux, macOS, Windows)
- [x] Smoke tests configured
- [x] Release asset upload configured
- [ ] GPU offloading verified in CI
- [ ] Docker image publication (optional)
- [ ] Dry run on test tag completed

## Guardrails

- Keep API compatible with LibreTranslate.
- Maintain offline capability.
- Prioritize translation quality over speed (within reason).
- Keep docs aligned with actual shipped behavior.
- **No new features until PRIMARY_TODO modernization complete.**
