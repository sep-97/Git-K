# 46: Undo and redo supported local actions

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** E — Terminals, recovery and desktop integration

**What to build:** Undo and redo the supported checkout/commit/discard/reference/reset actions, with contextual availability and truthful state.

**Blocked by:** [9: Amend the latest commit](009-amend-the-latest-commit.md); [4: Stage, unstage and discard lines or hunks](004-stage-unstage-and-discard-lines-or-hunks.md); [13: Create, check out, rename and delete branches](013-create-check-out-rename-and-delete-branches.md); [21: Reset, revert and restore historical files](021-reset-revert-and-restore-historical-files.md); [25: Configure remotes and inspect their identity](025-configure-remotes-and-inspect-their-identity.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Compare the resulting files/index/refs with the saved operation state; unsupported reversals must not appear enabled.
- [ ] **CORE-132:** Undo latest supported operation; contextual availability
- [ ] **CORE-133:** Undo checkout, commit, discard, branch/remote removal, reset
- [ ] **CORE-135:** Redo undone action through toolbar/shortcuts
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-132, CORE-133, CORE-135.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
