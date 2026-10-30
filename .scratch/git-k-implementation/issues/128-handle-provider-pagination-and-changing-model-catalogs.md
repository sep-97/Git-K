# 128: Handle provider pagination and changing model catalogs

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: vertical-slice

**Phase:** I — Settings, activity and integration completion

**What to build:** Use complete empty/paginated metadata pickers and refreshed provider model catalogs across the supported workflows.

Blocked by: [58: Create, inspect and merge GitHub pull requests](058-create-inspect-and-merge-github-pull-requests.md); [61: Use Enterprise repositories and pull requests](061-use-enterprise-repositories-and-pull-requests.md); [64: Use GitLab Self-Managed workflows](064-use-gitlab-self-managed-workflows.md); [67: Use Bitbucket Data Center workflows](067-use-bitbucket-data-center-workflows.md); [69: Use Azure DevOps pull requests and reviewers](069-use-azure-devops-pull-requests-and-reviewers.md); [103: Configure AI providers and organization policy](103-configure-ai-providers-and-organization-policy.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md); [G6: Equivalent self-hosted service and provider contract](../../gitkraken-parity/issues/06-service-boundaries.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

## Acceptance criteria

- [ ] Verify rate limits, revoked scopes, page boundaries and model unavailability without silent truncation.
- [ ] **CON-REL-008:** Provider model lists update; PR dropdowns paginate and handle empty results.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-REL-008.

Approved size target: one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
