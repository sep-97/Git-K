# 128: Handle provider pagination and changing model catalogs

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** I — Settings, activity and integration completion

**What to build:** Use complete empty/paginated metadata pickers and refreshed provider model catalogs across the supported workflows.

**Blocked by:** [58: Create, inspect and merge GitHub pull requests](058-create-inspect-and-merge-github-pull-requests.md); [61: Use Enterprise repositories and pull requests](061-use-enterprise-repositories-and-pull-requests.md); [64: Use GitLab Self-Managed workflows](064-use-gitlab-self-managed-workflows.md); [67: Use Bitbucket Data Center workflows](067-use-bitbucket-data-center-workflows.md); [69: Use Azure DevOps pull requests and reviewers](069-use-azure-devops-pull-requests-and-reviewers.md); [103: Configure AI providers and organization policy](103-configure-ai-providers-and-organization-policy.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support; G6 — Equivalent self-hosted service and provider contract.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify rate limits, revoked scopes, page boundaries and model unavailability without silent truncation.
- [ ] **CON-REL-008:** Provider model lists update; PR dropdowns paginate and handle empty results.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-REL-008.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
