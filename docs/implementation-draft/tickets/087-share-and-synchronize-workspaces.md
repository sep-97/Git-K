# 87: Share and synchronize workspaces

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** G — Self-hosted accounts, collaboration and connected views

**What to build:** Create/convert/edit/reorder shared workspaces with colors/icons/details and team/user permissions across two desktops.

**Blocked by:** [86: Organize local workspaces](086-organize-local-workspaces.md); [82: Manage teams and membership](082-manage-teams-and-membership.md); [80: Synchronize account integration connections across devices](080-synchronize-account-integration-connections-across-devices.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support; G6 — Equivalent self-hosted service and provider contract.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify conversion, revoked membership, concurrent edits and persistence.
- [ ] **CON-WS-003:** Create cloud groups using connected hosting integrations.
- [ ] **CON-WS-004:** Share groups with selected teams/users across machines.
- [ ] **CON-WS-005:** Convert local groups into cloud groups.
- [ ] **CON-WS-007:** Edit workspace name, icon, description, color and permissions.
- [ ] **CON-WS-008:** Hide/show groups and reorder by dragging.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-WS-003, CON-WS-004, CON-WS-005, CON-WS-007, CON-WS-008.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
