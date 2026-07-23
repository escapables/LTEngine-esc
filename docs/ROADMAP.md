---
summary: 'Public overview of LTEngine development priorities.'
read_when:
  - Reviewing project direction.
  - Choosing an area to contribute to.
---

# Roadmap

## Current Status

- CLI-only local GGUF translation with no HTTP server, browser UI, or listener.
- Direct text, stdin, and bounded UTF-8 `.txt` translation through the shared core.
- Default Gemma3 4B model; CPU, CUDA, Metal, and Vulkan builds.
- Safe user-selected document output without overwriting existing files.
- Portable release acceptance is not implemented yet.

## Near Term

- Translate long documents in bounded paragraph slices with progress and ordered reassembly.
- Design with `$visual-companion`, then add a native drag-and-drop document UI.
- Verify Swedish-to-English translation offline on a clean Linux system.
- Add sentence splitting for long documents.
- Improve language detection for short text.
- Migrate to the upstream-proven official `llama-cpp-2` binding.

## Later

- Evaluate more LLM families and publish comparative benchmarks.
- Extract a reusable Rust library and then evaluate language bindings.
- Verify GPU release builds and rehearse the release workflow with a test tag.
- Expand document formats after reliable `.txt` translation.

Detailed milestones and every queued task live in `docs/PRIMARY_TODO.md`.
