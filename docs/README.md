---
summary: 'Index of project docs with recommended read order by audience.'
read_when:
  - Onboarding to this repository.
  - Looking for what to read first.
---

# Documentation Index

## By Audience

### Users (End Users)

Start here to understand and run LTEngine:

1. `README.md` - Project overview, quick start, API examples
2. `docs/PORTABLE_APP.md` - Runtime contract, API surface, model configuration
3. `docs/ROADMAP.md` - Current features and planned improvements
4. `docs/linux-dev-setup.md` - Development/build setup

### Contributors (Developers)

Start here to contribute code:

1. `README.md` - Project overview
2. `CONTRIBUTING.md` - Contribution guidelines and PR workflow
3. `docs/linux-dev-setup.md` - Development environment setup
4. `.kilocode/rules/ARCHITECTURE.md` - Architecture contract and API details
5. `docs/STYLE.md` - Rust coding conventions
6. `docs/PORTABLE_APP.md` - Runtime behavior reference

### Maintainers (Core Team)

Internal process documentation:

1. `docs/WORKFLOW.md` - Pickup/handoff process for agent continuity
2. `docs/RELEASING.md` - Release checklist and tagging workflow
3. `docs/HANDOFF.md` - Current session state (ephemeral)
4. `docs/TODO.md` - Active task execution checklist
5. `docs/PRIMARY_TODO.md` - Large multi-step workstream tracking
6. `docs/CODE_OF_CONDUCT.md` - Community guidelines

## Doc Responsibilities

| Document | Owned By | Purpose |
|----------|----------|---------|
| `docs/PORTABLE_APP.md` | Repository | Source of truth for runtime contract and API behavior |
| `.kilocode/rules/ARCHITECTURE.md` | Agent system | Architecture contract for AI agents |
| `README.md` | Repository | User-facing overview and quick start |
| `CONTRIBUTING.md` | Repository | Contributor guidelines |

## Placeholder Docs (Not Applicable)

These docs exist as placeholders but are not relevant to LTEngine:

- `docs/elevation-setup.md` - Elevation data not applicable
- `docs/mislabel-inventory.md` - Transit route mislabels not applicable
- `docs/scripting.md` - Lua scripting not applicable

## Internal Process Docs (Maintainer-Only)

These are ephemeral/temporary docs for active development tracking:

- `docs/HANDOFF.md` - Cross-agent session handoff state
- `docs/TODO.md` - Current task execution checklist
- `docs/PRIMARY_TODO.md` - Multi-step workstream template
