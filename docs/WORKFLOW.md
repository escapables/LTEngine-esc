---
summary: 'Unified pickup/handoff workflow and execution checklist for cross-agent continuity.'
read_when:
  - Starting a session.
  - Handing off work.
  - Reviewing cross-agent continuity expectations.
---

# Workflow

`HANDOFF.md` is a short continuity snapshot for cross-agent pickup.

Shared instructions already live elsewhere and are not duplicated here:
- `/pickup` prompt + `~/projects/agent-scripts/kimi/pickup/SKILL.md` (session start flow)
- local/global `AGENTS.md` (gate/CI/docs behavior)

## Handoff Contract (Repo-Specific)

- Keep `docs/HANDOFF.md` under `60` total lines (including front matter).
- Replace stale content; do not accumulate long historical logs.
- Keep section shape: `Session`, `Completed`, `Verification Run`, `Open Risks / Blockers`, `Next Actions`.
- Keep `Completed` to short session deltas only (max `4` bullets).
- Keep `Verification Run` concrete (command + result).
- Keep `Next Actions` to `2-3` bullets with concrete commands or checks.
