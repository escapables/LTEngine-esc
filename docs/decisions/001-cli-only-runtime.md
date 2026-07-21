---
summary: 'Decision to replace the inherited HTTP runtime and browser UI with a direct portable CLI.'
read_when:
  - Changing runtime interfaces or HTTP dependencies.
  - Planning CLI extraction or server removal.
---

# ADR-001: Use a CLI-Only Runtime

## Status

Superseded by ADR-002

## Date

2026-07-21

## Context

This fork exists for portable, offline document translation on Linux, primarily Swedish to English. The inherited Actix server requires a loopback listener, retains web-specific dependencies, and makes local use subject to firewall or endpoint-policy restrictions. Neither browser access nor LibreTranslate API compatibility is required for the intended deployment.

Removing the server immediately would also remove the only shipped translation interface before a replacement exists.

## Decision

Build a direct CLI for text, stdin, and document translation. Extract shared translation behavior from HTTP handlers first, reach CLI parity, then remove Actix, multipart/download state, LibreTranslate endpoints, and bundled browser resources.

The finished product will not run or maintain a web server. Swedish-to-English documents are the primary acceptance path; language selection remains general.

## Alternatives Considered

### Keep CLI and HTTP

Rejected. Maintaining two interfaces preserves loopback-facing code and dependencies that the fork does not use.

### Remove HTTP Immediately

Rejected. It would leave no working user interface and risks losing reusable validation and document behavior during migration.

## Consequences

- CLI tasks and portable offline verification take priority over endpoint hardening.
- Existing HTTP-specific tasks remain archived with explicit dispositions.
- Actix and static frontend dependencies stay temporarily, then become removal targets.
- LibreTranslate API compatibility is no longer a product requirement.
- Release acceptance must prove translation without a listener or network connection.
