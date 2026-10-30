# 58: Create, inspect and merge GitHub pull requests

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** F — Hosting providers, PRs and issue trackers

**What to build:** Create a PR from reference entry points, select forks/branches and metadata/templates, inspect status/remotes and use supported merge/browser actions.

**Blocked by:** [57: Connect GitHub and manage hosted repositories](057-connect-github-and-manage-hosted-repositories.md); [28: Push commits and handle divergence](028-push-commits-and-handle-divergence.md); [13: Create, check out, rename and delete branches](013-create-check-out-rename-and-delete-branches.md); [10: Navigate the commit graph](010-navigate-the-commit-graph.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support; G6 — Equivalent self-hosted service and provider contract.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify permission-dependent controls, protected branches, drafts, metadata pagination and accurate API activity.
- [ ] Do not restore the obsolete full in-app review mode.
- [ ] **CON-HOST-004:** Discover forks, add remotes, create PRs and edit metadata.
- [ ] **CON-PR-001:** Start PR creation from graph drag, context menu or sidebar.
- [ ] **CON-PR-002:** Require pushed branches; choose source/target repos and branches.
- [ ] **CON-PR-003:** Fill title/description and supported metadata/templates.
- [ ] **CON-PR-004:** View/filter PRs with CI/review indicators.
- [ ] **CON-PR-007:** Offer GitHub merge commit, squash and rebase methods.
- [ ] **CON-PR-008:** Discover/add missing remotes for local PR inspection.
- [ ] **CON-PR-009:** Navigate build-status URLs and PR browser destinations.
- [ ] **CON-PR-010:** Preserve documented provider-specific MR/PR terminology and capabilities.
- [ ] **USER-002:** Record provider operations as truthful API activity, with no fabricated Git commands.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-HOST-004, CON-PR-001, CON-PR-002, CON-PR-003, CON-PR-004, CON-PR-007, CON-PR-008, CON-PR-009, CON-PR-010, USER-002.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
