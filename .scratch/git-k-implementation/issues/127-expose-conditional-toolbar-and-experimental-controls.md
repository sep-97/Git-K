# 127: Expose conditional toolbar and experimental controls

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: vertical-slice

**Phase:** I — Settings, activity and integration completion

**What to build:** Show the correct conditional toolbar actions and experimental feature controls, and execute their already implemented workflows.

Blocked by: [39: Manage LFS files and transfers](039-manage-lfs-files-and-transfers.md); [121: Select and verify the Git execution distribution](121-select-and-verify-the-git-execution-distribution.md); [95: Create and manage shareable patches](095-create-and-manage-shareable-patches.md); [109: Compose staged work into proposed commits](109-compose-staged-work-into-proposed-commits.md); [122: Apply panel, identity and status-bar presentation settings](122-apply-panel-identity-and-status-bar-presentation-settings.md); [123: Apply background Git and history-loading preferences](123-apply-background-git-and-history-loading-preferences.md); [124: Apply repository-specific Git and editor configuration](124-apply-repository-specific-git-and-editor-configuration.md); [125: Configure repository integrations and team or agent defaults](125-configure-repository-integrations-and-team-or-agent-defaults.md); [126: Configure and receive the reference desktop notifications](126-configure-and-receive-the-reference-desktop-notifications.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

## Acceptance criteria

- [ ] Verify enabled/disabled/absent states in the matching release; preserve the ordinary command and activity behavior for every toggle.
- [ ] **CORE-002:** Toolbar Git actions and conditional LFS button
- [ ] **CORE-238:** Enable experimental features in preferences
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-002, CORE-238.

Approved size target: one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
