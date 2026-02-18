---
summary: 'Execution checklist for documentation review remediation.'
read_when:
  - Updating project documentation after a review.
  - Tracking documentation correctness and structure tasks.
---

> **MAINTAINER-ONLY DOCUMENT**
> This is an ephemeral internal document for tracking active tasks.
> It is temporary state and will be updated as work progresses.
> End users and contributors should refer to ROADMAP.md for planned features.

# TODO

### DONE 1. Define Doc Review Scope

# Task: Define and document the documentation review scope.
#
# Scope:
# - Review first-party docs in README.md, docs/, and .kilocode/rules/ARCHITECTURE.md
# - Compare docs against current implementation behavior
# - Identify stale, inaccurate, and ambiguous documentation
# - Produce prioritized remediation list (P0/P1/P2)
#
# Done when:
# - [x] Coverage summary is documented
# - [x] Mismatches are documented with concrete file targets

### DONE 2. Apply P0 Fixes

# Task: Fix high-priority documentation inaccuracies and drift.
#
# Scope:
# - Update default model references to gemma3-12b where needed
# - Mark /translate_file as shipped where docs still say pending
# - Fix release docs branch command from master to main
# - Align API contract notes with implementation (download expiry, API key behavior, file format notation)
#
# Done when:
# - [x] README.md is corrected
# - [x] docs/PORTABLE_APP.md is corrected
# - [x] docs/ROADMAP.md is corrected
# - [x] docs/RELEASING.md is corrected

### DONE 3. Apply P1 Improvements

# Task: Improve contributor and setup documentation usability.
#
# Scope:
# - Add CONTRIBUTING.md with local quality gates and PR expectations
# - Make setup docs copy-pasteable (replace bracketed pseudo-commands)
# - Add expected first-run behavior and practical troubleshooting notes
#
# Done when:
# - [x] CONTRIBUTING.md exists and is linked from README.md
# - [x] docs/linux-dev-setup.md commands are copy-pasteable
# - [x] Setup flow includes expected outputs and common failure cases

### DONE 4. Apply P2 Improvements

# Task: Reduce drift risk and improve docs discoverability.
#
# Scope:
# - Choose a single source of truth for architecture/API docs
# - Rework docs/README.md by audience (user/developer/releaser)
# - Mark internal process docs as maintainer-only
# - Clarify placeholder docs so they are not confused with complete docs
#
# Done when:
# - [x] docs/PORTABLE_APP.md and .kilocode/rules/ARCHITECTURE.md responsibilities are explicit
# - [x] docs/README.md read order is audience-oriented
# - [x] Internal docs (HANDOFF/TODO/PRIMARY_TODO) are clearly labeled
