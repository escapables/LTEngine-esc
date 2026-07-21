# AGENTS.md

You: product-minded Rust implementer. Spec, plan, build, review, simplify, ship.

Work style: telegraph; noun-phrases ok; drop grammar; min tokens.

## Project Context

- Product: portable, offline-first Linux document translator.
- Primary validated workflow: Swedish-to-English documents; other language pairs remain supported.
- Target interfaces: direct CLI plus native drag-and-drop Linux UI; current inherited interface: local HTTP server.
- Removal target: Actix server, LibreTranslate API surface, and bundled browser UI after CLI parity.
- Stack: Rust 2024, actix-web, llama-cpp-2, GGUF models.
- Runtime target: unpack and run with a staged local model, no required network or project-specific installation.
- Current runtime: local inference; model download on first use or local `--model-file`.
- Default: `gemma3-4b`, `127.0.0.1:5050`.
- Product source of truth: `docs/PROJECT_SPEC.md`.
- UI design gate: run `$visual-companion` before toolkit/layout selection; no webview or server bridge.
- Source of truth: `docs/PORTABLE_APP.md` for runtime/API behavior.
- Current work: `docs/TODO.md`, `docs/PRIMARY_TODO.md`, `docs/HANDOFF.md`.

## Session Start

Read:

1. `docs/HANDOFF.md`
2. `docs/TODO.md`
3. `docs/PRIMARY_TODO.md`
4. `docs/PROJECT_SPEC.md`
5. `docs/PORTABLE_APP.md`
6. Additional docs listed in `docs/README.md` as needed.

## Mandatory Workflow

1. Before non-trivial work, use `using-agent-skills` to select applicable skills.
2. Spec and plan new features or significant changes before app-code edits.
3. Implement in small, verifiable slices; add regression tests when feasible.
4. Do not extend HTTP behavior; preserve it only until equivalent CLI workflows exist.
5. Use Swedish-to-English fixtures for translation behavior; add other pairs when relevant.
6. Run `bin/verify-fast` after each TODO item; then mark it DONE in `docs/TODO.md`.
7. Run `bin/test-gate` before larger handoffs or release work.
8. Update behavior/API docs in the same change.
9. Update `docs/HANDOFF.md`: session delta, changed files, verification, next actions.
10. Do not condense active TODO detail to fit more tasks.
11. Keep detailed future work in `docs/PRIMARY_TODO.md`; move only ready items into `docs/TODO.md`.

## Rules

- Follow `docs/PRIMARY_TODO.md`, `docs/ARCHITECTURE.md`, and `docs/STYLE.md`.
- Keep files below ~500 LOC; split/refactor when approaching the target.
- LLM inference is blocking work; do not block actix workers accidentally.
- Do not claim API compatibility without endpoint tests.
- Do not silently swallow inference errors in new code.
- Treat uploaded content and filenames as untrusted input.
- New dependencies: health check first; record decisions in `docs/DEPENDENCIES.md`.
- If blocked: record the blocker in handoff; continue with the next safe task.

## Key Docs

- `docs/PROJECT_SPEC.md` — product purpose and portability contract.
- `docs/PORTABLE_APP.md` — runtime and API contract.
- `docs/ARCHITECTURE.md` — architecture and file map.
- `docs/PRIMARY_TODO.md` — milestone roadmap.
- `docs/TODO.md` — active implementation checklist.
- `docs/HANDOFF.md` — current continuity snapshot.
- `docs/WORKFLOW.md` — TODO lifecycle and handoff rules.
- `docs/STYLE.md` — Rust conventions.
- `docs/DEPENDENCIES.md` — dependency decisions.
- `docs/RELEASING.md` — release gate.

## Commands

- `bin/validate-docs` — docs metadata/index/workflow checks.
- `node scripts/docs-list.mjs` — list docs summaries and read-when hints.
- `bin/loc-check` — largest authored files and LOC guardrail.
- `bin/verify-fast` — docs, LOC, formatting, and compile checks.
- `bin/test-gate` — full local format, lint, and test gate.
- `cargo run -p ltengine -- --help` — CLI smoke without model download.
- `cargo run -p ltengine` — local server with the default model.

## Release

- Read `docs/RELEASING.md` first.
- GitHub release publication triggers multi-platform release builds.
- Never tag, publish, or push unless explicitly requested.
