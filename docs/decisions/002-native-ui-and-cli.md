---
summary: 'Decision to provide native drag-and-drop document translation alongside the direct CLI.'
read_when:
  - Designing or implementing the user interface.
  - Selecting a Linux GUI toolkit or changing local interfaces.
---

# ADR-002: Provide Native UI and CLI

## Status

Accepted; supersedes ADR-001

## Date

2026-07-21

## Context

The portable translator needs a basic interface for people who do not want to operate through a terminal. Users should be able to drop a text document into the application, follow translation progress, inspect the result, and save it. The no-webserver decision remains: loopback networking and a browser-hosted UI are outside the product contract.

## Decision

Provide two local interfaces over the same translation core:

- A direct CLI for automation, stdin, and scripted document translation.
- A native Linux GUI for drag-and-drop document translation, language selection, progress, output preview, and save-as.

Before choosing the GUI layout or toolkit, run `$visual-companion` with the project directory and compare two to four focused mockups. Persist the selected design rationale. Health-check any proposed GUI dependency before adoption. The shipped GUI must call the Rust core directly and must not start a web server or depend on a browser bridge.

## Consequences

- ADR-001's HTTP-removal decision remains in force; only its CLI-only interface choice changes.
- CLI/core work lands before the GUI so both interfaces share document behavior and errors.
- GUI progress consumes per-slice translation events from the long-document pipeline.
- Portable release checks cover both CLI translation and native GUI startup without a listener.
