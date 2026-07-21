---
summary: 'Session workflow and TODO lifecycle.'
read_when:
  - Starting or wrapping up a session.
  - Updating TODO or handoff state.
---

# Workflow

## Session Flow

1. Pickup: read `docs/HANDOFF.md`, `docs/TODO.md`, `docs/PRIMARY_TODO.md`, `docs/PORTABLE_APP.md`.
2. Confirm runtime facts against code before changing stale docs.
3. Spec and plan new features or significant behavior changes.
4. Implement in small batches; add regression tests when they fit.
5. Verify with `bin/verify-fast`; use `bin/test-gate` before larger handoffs.
6. Handoff: replace `docs/HANDOFF.md` with current state and explicit changed files.

## HANDOFF.md Contract

`HANDOFF.md` is a short continuity snapshot, not a history log.

- Keep under 60 lines.
- Replace stale content.
- Sections: `Session`, `Completed`, `Changed Files`, `Verification Run`, `Open Risks / Blockers`, `Next Actions`.
- `Completed`: current session deltas only.
- `Changed Files`: every file added, modified, or deleted this session.
- `Verification Run`: exact command and result.
- `Next Actions`: 2–3 concrete bullets.

## TODO Lifecycle

Finished item:

```text
// DONE 5 — short completion summary.
```

Rules:

- Remove the finished Task/Scope/Done-when block.
- Never renumber existing items.
- Add new items with the next unused number.
- Mark DONE only after verification.
- Keep `docs/TODO.md` for ready work; keep future detail in `docs/PRIMARY_TODO.md`.

## Milestone Completion

When every task for a milestone is done:

- Verify milestone deliverables.
- Change its `docs/PRIMARY_TODO.md` heading from NOT DONE to DONE.
- Move older completed milestones to `docs/ARCHIVE.md` when space is tight.
- Promote the next 3–6 ready tasks into `docs/TODO.md`.
- Update `docs/HANDOFF.md`.
