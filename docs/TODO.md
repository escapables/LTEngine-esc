---
summary: 'Active LTEngine implementation checklist.'
read_when: [Starting a session, Looking for the next implementation task.]
---

# TODO

// DONE 1–4 — 2026-02 documentation review scope plus P0, P1, and P2 remediation.
// DONE 5 — reusable translation core with controlled-inference tests and propagated errors.
// DONE 6 — direct text/stdin CLI with controlled-engine coverage and clean stdout.
// DONE 7 — bounded UTF-8 `.txt` input writes a safe selected output path with layout preservation.

### 8. Remove HTTP runtime
Task: Remove the web server after direct CLI text and document parity.
Scope:
- Delete Actix handlers/state, static browser resources, and HTTP-only dependencies.
- Remove host, port, API-key, download-store, and LibreTranslate compatibility behavior.
Done when:
- Binary opens no listener; CLI tests, `bin/test-gate`, and updated runtime docs pass.

### 21. Official llama binding
Task: Port upstream's migration to the official `llama-cpp-2` crate before adding Gemma 4.
Scope:
- Move to the upstream-proven official crate/version and adapt its API and warning cleanup.
- Update the lockfile; remove the submodule only after Gemma 3 regression coverage passes.
Done when:
- Gemma 3 loads/translates and `bin/test-gate` passes without importing Docker or HTTP changes.

### 22. Gemma 4 support
Task: Port upstream Gemma 4 E4B support without removing existing Gemma 3 choices.
Scope:
- Add the E4B GGUF model entry and verified download metadata.
- Port template fallback, thinking cleanup, and useful model/template logging with regression tests.
Done when:
- Gemma 3 4B and Gemma 4 E4B both load and translate representative inputs.
- Gemma 3 4B remains the default pending benchmark evidence.

### 23. T480 translation benchmark
Task: Compare Gemma 3 4B and Gemma 4 E4B on the target ThinkPad T480.
Scope:
- Measure translation quality, startup time, tokens per second, peak RAM, and model size.
- Use matched multilingual fixtures and settings; record hardware and reproducible commands.
Done when:
- Results identify whether Gemma 4's quality gain justifies its higher resource cost.
- Benchmark method and results are documented.

### 24. Default model decision
Task: Select the project default from the T480 benchmark results.
Scope:
- Promote Gemma 4 E4B only if it provides the preferred quality/resource trade-off.
- Otherwise retain Gemma 3 4B; align defaults, tests, README, runtime docs, and model tables.
Done when:
- The decision and evidence are documented and all default-model references agree.
- `bin/verify-fast` and the focused default-model regression test pass.
