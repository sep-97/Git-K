# 44: Manage persistent per-worktree terminal tabs

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: vertical-slice

**Phase:** E — Terminals, recovery and desktop integration

**What to build:** Open, select, close and reorder terminal tabs with the reference keyboard/focus/scroll/label behavior while processes continue across worktree changes.

Blocked by: [43: Use a real embedded interactive terminal](043-use-a-real-embedded-interactive-terminal.md); [35: Create and manage linked worktrees](035-create-and-manage-linked-worktrees.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

## Acceptance criteria

- [ ] Verify closing the active tab selects the correct neighbor and keeps other sessions running.
- [ ] **CORE-209:** Create, select and close multiple terminal tabs
- [ ] **CORE-210:** Terminal sets follow repository/worktree selection
- [ ] **CORE-211:** Processes continue when switching worktrees
- [ ] **CORE-245:** 12.6.1 terminal tab shortcuts and Terminal Tabs submenu
- [ ] **CORE-246:** 12.6.1 selected terminal tab auto-scrolls into view
- [ ] **CORE-247:** 12.6.1 terminal tab drag order retained until app close
- [ ] **CORE-248:** 12.6.1 closing active terminal selects neighboring tab
- [ ] **CORE-249:** 12.6.1 selecting terminal tab focuses terminal input
- [ ] **CORE-250:** 12.6.1 Windows terminal labels follow running process
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-209, CORE-210, CORE-211, CORE-245, CORE-246, CORE-247, CORE-248, CORE-249, CORE-250.

Approved size target: one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
