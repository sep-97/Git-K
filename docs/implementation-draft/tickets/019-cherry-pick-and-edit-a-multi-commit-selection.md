# 19: Cherry-pick and edit a multi-commit selection

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** C — History editing and conflicts

**What to build:** Cherry-pick from a commit or branch head and edit a multi-commit plan with ordering, reword, squash and drop actions.

**Blocked by:** [13: Create, check out, rename and delete branches](013-create-check-out-rename-and-delete-branches.md); [16: Resolve a conflicted merge](016-resolve-a-conflicted-merge.md); [8: Use commit templates and co-authors](008-use-commit-templates-and-co-authors.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify parent-child squash restrictions, shortcuts, reset and conflict recovery.
- [ ] **CORE-110:** Cherry-pick selected commit onto current branch
- [ ] **CORE-111:** Cherry-pick branch HEAD through left-panel menu
- [ ] **CORE-112:** Multi-commit selection opens interactive cherry-pick
- [ ] **CORE-113:** Reorder cherry-picked commits with drag/drop
- [ ] **CORE-114:** Cherry-pick pick/reword/squash/drop choices
- [ ] **CORE-115:** Cherry-pick squash requires parent-child relationship
- [ ] **CORE-116:** Cherry-pick shortcuts and Reset without applying
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-110, CORE-111, CORE-112, CORE-113, CORE-114, CORE-115, CORE-116.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
