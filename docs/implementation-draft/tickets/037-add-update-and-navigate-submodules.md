# 37: Add, update and navigate submodules

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** D — Remotes and extended Git

**What to build:** Add/initialize/update submodules, inspect their state, switch into them and commit updated pointers with the configured automatic-update policy.

**Blocked by:** [30: Clone and open repositories](030-clone-and-open-repositories.md); [28: Push commits and handle divergence](028-push-commits-and-handle-divergence.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify nested and uninitialized submodules plus dirty and out-of-sync states.
- [ ] **CORE-164:** Toggle submodules section
- [ ] **CORE-165:** Add submodule URL/path and update .gitmodules
- [ ] **CORE-166:** Initialize submodules when cloning parent
- [ ] **CORE-167:** Update through submodule context menu
- [ ] **CORE-168:** Open submodule, change checked-out target, return
- [ ] **CORE-169:** Prompt to preserve updated submodule pointer
- [ ] **CORE-170:** Uninitialized/out-of-sync/uncommitted states
- [ ] **CORE-171:** Global/repository automatic submodule-update settings
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-164, CORE-165, CORE-166, CORE-167, CORE-168, CORE-169, CORE-170, CORE-171.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
