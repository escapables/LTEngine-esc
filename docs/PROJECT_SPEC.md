---
summary: 'Product purpose, portability contract, primary workflow, and scope for the LTEngine fork.'
read_when:
  - Deciding product direction or implementation priority.
  - Changing runtime interfaces, packaging, or language workflows.
---

# Project Specification

## Purpose

LTEngine-esc is a portable, offline-first Linux document translator. Its primary validated workflow is Swedish-to-English translation with a local GGUF model. Other language pairs remain supported; Swedish and English are a testing and usability priority, not a hardcoded restriction.

## Product Contract

- Run from an unpacked directory on a Linux system with minimal host dependencies.
- Translate without an internet connection once the binary and model are staged locally.
- Require no external translation API, account, daemon, web server, or loopback network connection.
- Accept document input and produce translated document output; `.txt` is the currently shipped format.
- Keep model selection explicit and support locally supplied GGUF files.

“Offline” describes runtime behavior. Building and initially obtaining a model may require network access. A portable bundle or its accompanying model pack must contain everything needed before entering the offline environment.

## Interfaces

- Direct CLI: text, stdin, and scripted document translation.
- Native Linux GUI: drag-and-drop document input, language selection, progress, translated preview, and save-as.
- Reusable Rust interfaces may be extracted where they simplify the CLI, native GUI, and testing.
- The existing LibreTranslate-compatible localhost server and browser UI are legacy code to remove after CLI parity.

The current release retains the server as its no-subcommand default but now ships direct text, stdin, and `.txt` document translation. Server removal and portable packaging remain roadmap work.

## Portability Standard

A portable release is complete when it can be copied to a clean supported Linux system and can:

1. Start without installing project-specific packages.
2. Load a staged GGUF model without network access.
3. Translate a Swedish document to English without opening a TCP listener.
4. Write the result to a user-selected local path.
5. Report actionable errors for missing models, unsupported files, and insufficient memory.

Build tools such as Rust, CMake, clang, and a C++ compiler are development dependencies, not intended end-user runtime requirements.

## Product Priorities

1. Reliable Swedish-to-English document translation.
2. Portable offline Linux operation with minimal runtime dependencies.
3. Direct CLI operation without localhost networking.
4. Basic native drag-and-drop document translation without a web server.
5. Additional language pairs and suitable local GGUF models.
6. A small maintainable runtime dependency surface.

## Non-Goals

- Cloud-hosted inference or required external services.
- Restricting translation to Swedish and English.
- Running or maintaining an HTTP API, web server, or browser-based UI long term.
- Bundling every supported model into every release artifact.
