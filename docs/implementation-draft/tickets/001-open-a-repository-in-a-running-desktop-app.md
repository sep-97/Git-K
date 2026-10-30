# 1: Open a repository in a running desktop app

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** A — First complete desktop Git workflows

**What to build:** Launch the native Tauri app, open a local repository, inspect real branch/status/history data and see the exact read commands and results in a collapsible action panel.

**Blocked by:** No implementation-ticket prerequisites. Open decision gates below still apply.

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Build and launch a Windows app using pinned dependencies and verified native prerequisites; report invalid/non-repository paths in the UI.
- [ ] Use structured executable arguments and native path identity; preserve stdout/stderr and exit status without running display strings.
- [ ] Collapse and reopen the panel during an operation: recording and process ownership continue; the approved execution contract defines replay and shutdown.
- [ ] Verify with unborn HEAD, a normal history and unusual literal filenames.
- [ ] **CORE-027:** Open repositories through New Tab and File menu
- [ ] **RUNTIME-01:** Stream typed operation progress through the desktop workflow interface and implement the replay behavior chosen by the execution prototype.
- [ ] **RUNTIME-02:** Execute literal program arguments and native paths, derive a shell-aware display and never execute the display string.
- [ ] **RUNTIME-03:** Drain both output streams while handling stdin and exit, including a fixture that would deadlock if pipes were handled sequentially.
- [ ] **RUNTIME-07:** Keep process ownership independent of the panel and report truthful shutdown/cancellation outcomes while awaiting children.
- [ ] **RUNTIME-12:** Round-trip native filename identity independently from its display label through selection and command execution.
- [ ] **RUNTIME-15:** Constrain frontend command access and validate repository handles/inputs while treating repository text as data.
- [ ] **USER-001:** Record actual application Git commands and results while the panel is collapsed; execute each step once.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-027, RUNTIME-01, RUNTIME-02, RUNTIME-03, RUNTIME-07, RUNTIME-12, RUNTIME-15, USER-001.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
