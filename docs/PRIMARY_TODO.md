---
summary: 'LTEngine implementation roadmap organized by milestone.'
read_when:
  - Planning upcoming implementation work.
  - Promoting tasks into the active TODO.
  - Reviewing milestone progress.
---

# Roadmap

> Target: portable offline Linux document translation through direct CLI/native GUI interfaces and local GGUF inference.

## NOT DONE v0.2 Portable CLI

| What | Result | Coding tips |
| --- | --- | --- |
| Translation core | DONE — model-backed translation independent from Actix handlers | Controlled-engine tests preserve language behavior and errors |
| Text CLI | DONE — arguments and stdin translate without a listener | Translated text on stdout; diagnostics/errors on stderr |
| Document CLI | DONE — bounded UTF-8 `.txt` input writes a selected new output path | Swedish-to-English, layout, limits, encoding, and path safety tested |
| HTTP removal | Actix, API state, and browser assets removed after parity | Remove dependencies last |

Active task: TODO 8 in `docs/TODO.md`.

## NOT DONE v0.3 Long Text Quality

| What | Result | Coding tips |
| --- | --- | --- |
| Sentence splitting | Long text split, translated, and reassembled safely | Preserve paragraph structure |
| Document slicing (29) | Documents around 20,000 words translate sequentially by paragraph; oversized paragraphs split safely | Budget by tokens; preserve order and separators; expose progress/errors |
| Short-text detection | Better short-text language detection, evaluating a LexiLang port | Benchmark before replacing whatlang |

Queued: 9 sentence splitting; 10 short-text language detection; 29 paragraph-sliced long-document pipeline.

## NOT DONE v0.4 Portable Runtime

| What | Result | Coding tips |
| --- | --- | --- |
| Offline smoke | Clean Linux host translates with staged binary and model | Disable network during verification |
| Runtime dependencies | Document and minimize required host libraries | Inspect release binary linkage |
| Portable layout | Define binary, model directory, licenses, and examples | Keep large model packs separable |
| UI design (27) | Selected native drag/drop layout for input, languages, progress, preview, and save | Run `$visual-companion`; persist mockups and rationale before toolkit choice |
| Native UI (28) | Basic Linux GUI calls the Rust core directly without a listener or browser bridge | Health-check toolkit; default Swedish to English; test file drop and save |

Former tasks 11–13 are dispositioned in `docs/ARCHIVE.md`; their reusable concerns moved into TODO 7–8 or the completed core/text CLI.

## NOT DONE v0.5 Models and Interfaces

| What | Result | Coding tips |
| --- | --- | --- |
| Binding migration | Official `llama-cpp-2` replaces the submodule-backed binding | Adapt upstream; prove Gemma 3 before removal |
| Gemma 4 E4B | Supported alongside Gemma 3 with correct templates and clean output | Port template fallback and thinking cleanup together |
| GPU loading resilience | Probe safe offload and reduce layers after load-time OOM | Keep CPU-only operation first-class |
| Inference observability | Report selected layers, GPU mode, and template fallbacks | Avoid server/request-specific logging |
| T480 model decision | Reproducible Gemma 3 4B versus Gemma 4 E4B benchmark selects the default | Compare quality, speed, RAM, and model size |
| Model coverage | Test and document other useful GGUF models | Record quality and memory tradeoffs |
| Benchmarks | Reproducible comparison with proprietary translators | Publish methodology and fixtures |
| CLI mode | Text, stdin, and `.txt` document modes shipped | Reuse translation core |
| Library mode | Translation core exposed as a Rust library; bindings evaluated | Stabilize Rust API before bindings |

Ordered model migration: TODO 21–24 in `docs/TODO.md`. Existing queue remains: 14 model evaluation; 15 comparative benchmarks; 16 CLI inference; 17 library mode and bindings.

Queued upstream ports: 25 GPU offload probing/OOM fallback; 26 portable CI PR triggers, concurrency, and maintained toolchain actions. Exclude upstream HTTP, browser, Docker, endpoint, and client-cancellation changes.

## NOT DONE v0.6 Release Confidence

| What | Result | Coding tips |
| --- | --- | --- |
| GPU CI | Hardware offloading verified where runners permit | Include upstream's generic CI improvements only |
| Release rehearsal | Test tag proves artifact workflow and smoke checks | Follow `docs/RELEASING.md` |
| Portable artifact | Ship an unpackable Linux CLI/GUI directory with documented model staging | Prove it on a clean offline host |

Queued: 18 GPU CI verification; 19 portable release dry run. Task 20 is dispositioned in `docs/ARCHIVE.md`.

<!-- Keep active detail in TODO.md. Never remove queued work without recording its disposition. -->
