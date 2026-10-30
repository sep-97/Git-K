# 127: Expose conditional toolbar and experimental controls

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** I — Settings, activity and integration completion

**What to build:** Show the correct conditional toolbar actions and experimental feature controls, and execute their already implemented workflows.

**Blocked by:** [39: Manage LFS files and transfers](039-manage-lfs-files-and-transfers.md); [121: Select and verify the Git execution distribution](121-select-and-verify-the-git-execution-distribution.md); [95: Create and manage shareable patches](095-create-and-manage-shareable-patches.md); [109: Compose staged work into proposed commits](109-compose-staged-work-into-proposed-commits.md); [122: Apply panel, identity and status-bar presentation settings](122-apply-panel-identity-and-status-bar-presentation-settings.md); [123: Apply background Git and history-loading preferences](123-apply-background-git-and-history-loading-preferences.md); [124: Apply repository-specific Git and editor configuration](124-apply-repository-specific-git-and-editor-configuration.md); [125: Configure repository integrations and team or agent defaults](125-configure-repository-integrations-and-team-or-agent-defaults.md); [126: Configure and receive the reference desktop notifications](126-configure-and-receive-the-reference-desktop-notifications.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify enabled/disabled/absent states in the matching release; preserve the ordinary command and activity behavior for every toggle.
- [ ] **CORE-002:** Toolbar Git actions and conditional LFS button
- [ ] **CORE-238:** Enable experimental features in preferences
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-002, CORE-238.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
