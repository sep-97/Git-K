# 35: Create and manage linked worktrees

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: vertical-slice

**Phase:** D — Remotes and extended Git

**What to build:** Create, open, find, lock/unlock and remove worktrees with correct branch/folder/path display and inherited view settings.

Blocked by: [13: Create, check out, rename and delete branches](013-create-check-out-rename-and-delete-branches.md); [11: Work in multiple repository tabs](011-work-in-multiple-repository-tabs.md); [24: Pin, hide, solo and search visible history](024-pin-hide-solo-and-search-visible-history.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

## Acceptance criteria

- [ ] Verify per-worktree and common repository state remain distinct; locking semantics must match actual Git behavior.
- [ ] **CORE-172:** Create worktree through branch context menu
- [ ] **CORE-173:** Worktree list shares underlying entries with Agents view
- [ ] **CORE-174:** Inherit hidden/solo/collapsed view settings on creation
- [ ] **CORE-175:** Open worktree replacing view
- [ ] **CORE-176:** Open worktree in separate repository tab
- [ ] **CORE-177:** Open worktree through graph WIP entry
- [ ] **CORE-178:** Remove worktree; optionally delete associated branch
- [ ] **CORE-179:** Worktree lock/unlock controls
- [ ] **CORE-180:** Display/search branch and differing folder name
- [ ] **CORE-181:** Full path available on worktree hover
- [ ] **RUNTIME-09:** Discover private and common worktree metadata through Git and coordinate operations with the approved shared-repository identity.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-172, CORE-173, CORE-174, CORE-175, CORE-176, CORE-177, CORE-178, CORE-179, CORE-180, CORE-181, RUNTIME-09.

Approved size target: one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
