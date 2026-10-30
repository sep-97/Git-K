# 123: Apply background Git and history-loading preferences

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** I — Settings, activity and integration completion

**What to build:** Configure fetch/prune/conflict/submodule behavior, branch/backup/graph limits, lazy history/tab persistence and Windows Git preferences.

**Blocked by:** [26: Fetch remotes and show ahead/behind state](026-fetch-remotes-and-show-ahead-behind-state.md); [29: Pull with the selected integration strategy](029-pull-with-the-selected-integration-strategy.md); [37: Add, update and navigate submodules](037-add-update-and-navigate-submodules.md); [10: Navigate the commit graph](010-navigate-the-commit-graph.md); [11: Work in multiple repository tabs](011-work-in-multiple-repository-tabs.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify actual background commands, loaded-history bounds and Git configuration effects rather than only saved switch values.
- [ ] **CORE-214:** General fetch/prune/conflict/submodule settings
- [ ] **CORE-215:** Default branch, merge-backup-file and graph-depth settings
- [ ] **CORE-216:** Lazy history loading and remembered tabs
- [ ] **CORE-217:** Windows longpaths/AutoCRLF settings
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-214, CORE-215, CORE-216, CORE-217.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
