---
summary: 'Execution checklist for /translate_file implementation.'
read_when:
  - Implementing file translation feature.
  - Tracking current implementation progress.
---

# TODO

### 1. Define Work

Task: Define and document the work scope for implementing the translate_file endpoint.

Scope:
- Implement /translate_file endpoint for document translation
- Create file store for translated files
- Add download endpoint for serving translated files
- Update frontend settings to enable file translation

Done when:
- [ ] Scope is documented in PRIMARY_TODO.md
- [ ] Verification commands are defined

### 2. Execute Work

Task: Implement and verify the translate_file endpoint implementation.

Scope:
- Add uuid dependency to Cargo.toml
- Create FileStore module with in-memory storage
- Implement translate_file handler with multipart parsing
- Add GET /download/{id} endpoint
- Update frontend settings

Done when:
- [ ] Code compiles without errors
- [ ] cargo test passes
- [ ] curl test works end-to-end

### 3. Session Handoff

Task: Document session outcomes for next agent pickup.

Scope:
- Update HANDOFF.md with completed work
- Document open risks and blockers
- List immediate next actions

Done when:
- [ ] HANDOFF.md is current
- [ ] Next actions are actionable and specific
