# 125: Configure repository integrations and team or agent defaults

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: vertical-slice

**Phase:** I — Settings, activity and integration completion

**What to build:** Navigate SSH/hosting/issue settings and apply repository issue, Team View, submodule and agent setup defaults.

Blocked by: [57: Connect GitHub and manage hosted repositories](057-connect-github-and-manage-hosted-repositories.md); [61: Use Enterprise repositories and pull requests](061-use-enterprise-repositories-and-pull-requests.md); [64: Use GitLab Self-Managed workflows](064-use-gitlab-self-managed-workflows.md); [67: Use Bitbucket Data Center workflows](067-use-bitbucket-data-center-workflows.md); [69: Use Azure DevOps pull requests and reviewers](069-use-azure-devops-pull-requests-and-reviewers.md); [70: Work with GitHub issues and issue branches](070-work-with-github-issues-and-issue-branches.md); [74: Connect Jira and work with issues](074-connect-jira-and-work-with-issues.md); [76: Use Trello cards as development work](076-use-trello-cards-as-development-work.md); [98: Share team branch and WIP presence](098-share-team-branch-and-wip-presence.md); [112: Start an agent in a correctly configured worktree](112-start-an-agent-in-a-correctly-configured-worktree.md); [37: Add, update and navigate submodules](037-add-update-and-navigate-submodules.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

## Acceptance criteria

- [ ] Verify profile/repository scope, account switching and that each settings route reaches a functional integration.
- [ ] **CORE-220:** SSH and hosting/issue integration settings entry points
- [ ] **CORE-228:** Repository issue/Team View/submodule/agent setup settings
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-220, CORE-228.

Approved size target: one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
