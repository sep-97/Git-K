# 38: Start and finish Gitflow work

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** D — Remotes and extended Git

**What to build:** Configure Gitflow names/prefixes and start or finish feature, release and hotfix workflows through real multi-step Git operations.

**Blocked by:** [13: Create, check out, rename and delete branches](013-create-check-out-rename-and-delete-branches.md); [15: Merge branches through the graph](015-merge-branches-through-the-graph.md); [34: Create and manage local and remote tags](034-create-and-manage-local-and-remote-tags.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify both base branches, tags, messages, branch deletion and partially failed operations.
- [ ] **CORE-182:** Configure Gitflow branch names and prefixes
- [ ] **CORE-183:** Initialize missing main/develop branches
- [ ] **CORE-184:** Start feature/release/hotfix through Gitflow panel
- [ ] **CORE-185:** Finish feature into develop and remove branch
- [ ] **CORE-186:** Finish release into both base branches with tag
- [ ] **CORE-187:** Finish hotfix into main and develop
- [ ] **CORE-188:** Configure release-tag prefix and message
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-182, CORE-183, CORE-184, CORE-185, CORE-186, CORE-187, CORE-188.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
