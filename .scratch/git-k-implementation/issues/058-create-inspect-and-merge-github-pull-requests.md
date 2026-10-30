# 58: Create, inspect and merge GitHub pull requests

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: vertical-slice

**Phase:** F — Hosting providers, PRs and issue trackers

**What to build:** Create a PR from reference entry points, select forks/branches and metadata/templates, inspect status/remotes and use supported merge/browser actions.

Blocked by: [57: Connect GitHub and manage hosted repositories](057-connect-github-and-manage-hosted-repositories.md); [28: Push commits and handle divergence](028-push-commits-and-handle-divergence.md); [13: Create, check out, rename and delete branches](013-create-check-out-rename-and-delete-branches.md); [10: Navigate the commit graph](010-navigate-the-commit-graph.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md); [G6: Equivalent self-hosted service and provider contract](../../gitkraken-parity/issues/06-service-boundaries.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

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

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-HOST-004, CON-PR-001, CON-PR-002, CON-PR-003, CON-PR-004, CON-PR-007, CON-PR-008, CON-PR-009, CON-PR-010, USER-002.

Approved size target: one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
