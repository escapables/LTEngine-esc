---
summary: 'Public overview of LTEngine development priorities.'
read_when:
  - Reviewing project direction.
  - Choosing an area to contribute to.
---

# Roadmap

## Current Status

- Inherited local LibreTranslate-compatible server using GGUF language models.
- Direct text/stdin translation through the shared core without a listener.
- Default Gemma3 4B model; CPU, CUDA, Metal, and Vulkan builds.
- `.txt` file translation foundation and release-triggered platform builds.
- Direct document CLI and portable release acceptance are not implemented yet.

## Near Term

- Extract translation behavior from HTTP handlers.
- Add direct `.txt` document translation to the shipped text/stdin CLI.
- Translate long documents in bounded paragraph slices with progress and ordered reassembly.
- Design with `$visual-companion`, then add a native drag-and-drop document UI.
- Remove Actix, LibreTranslate endpoints, and browser resources after CLI parity.
- Verify Swedish-to-English translation offline on a clean Linux system.
- Add sentence splitting for long documents.
- Improve language detection for short text.

## Later

- Evaluate more LLM families and publish comparative benchmarks.
- Extract a reusable Rust library and then evaluate language bindings.
- Verify GPU release builds and rehearse the release workflow with a test tag.
- Expand document formats after reliable `.txt` translation.

Detailed milestones and every queued task live in `docs/PRIMARY_TODO.md`.
