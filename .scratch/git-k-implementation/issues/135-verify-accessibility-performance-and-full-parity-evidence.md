# 135: Verify accessibility, performance and full parity evidence

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: verification-gate

**Phase:** K — Reference census and final acceptance

**What to build:** Run the agreed visual/behavioral/platform/recovery/accessibility/performance checks and close every observed difference with evidence.

Blocked by: [5: Ignore files and stop tracking explicitly](005-ignore-files-and-stop-tracking-explicitly.md); [23: Stash or restore selected changes](023-stash-or-restore-selected-changes.md); [31: Clone and manage sparse checkout selection](031-clone-and-manage-sparse-checkout-selection.md); [41: Sign and verify with SSH keys](041-sign-and-verify-with-ssh-keys.md); [45: Customize terminal input and completion](045-customize-terminal-input-and-completion.md); [47: Undo and redo history rewriting](047-undo-and-redo-history-rewriting.md); [48: Search commits and navigate matches](048-search-commits-and-navigate-matches.md); [53: Use configured editors and external tools](053-use-configured-editors-and-external-tools.md); [59: Create and navigate stacked GitHub pull requests](059-create-and-navigate-stacked-github-pull-requests.md); [71: Use Enterprise issues](071-use-enterprise-issues.md); [73: Use Self-Managed GitLab issues](073-use-self-managed-gitlab-issues.md); [75: Use Jira Data Center issues](075-use-jira-data-center-issues.md); [77: Complete the Azure work-item reference audit and workflow](../pending/077-complete-the-azure-work-item-reference-audit-and-workflow.md); [78: Complete the Git Integration for Jira workflow](../pending/078-complete-the-git-integration-for-jira-workflow.md); [85: Authenticate through the supported internal identity options](085-authenticate-through-the-supported-internal-identity-options.md); [88: Run actions across selected workspace repositories](088-run-actions-across-selected-workspace-repositories.md); [92: Use team and enterprise-provider Launchpad contexts](092-use-team-and-enterprise-provider-launchpad-contexts.md); [94: Act on Launchpad pull requests](094-act-on-launchpad-pull-requests.md); [97: Use customer-controlled patch storage](097-use-customer-controlled-patch-storage.md); [99: Show overlap and reference conflict-prevention behavior](../pending/099-show-overlap-and-reference-conflict-prevention-behavior.md); [105: Explain commits and branch changes](105-explain-commits-and-branch-changes.md); [106: Generate editable stash messages](106-generate-editable-stash-messages.md); [107: Generate pull-request descriptions with templates](107-generate-pull-request-descriptions-with-templates.md); [108: Review AI conflict-resolution suggestions](108-review-ai-conflict-resolution-suggestions.md); [110: Recompose existing contiguous commits](110-recompose-existing-contiguous-commits.md); [116: Launch agents from linked issues](116-launch-agents-from-linked-issues.md); [117: Run another agent in an occupied worktree](117-run-another-agent-in-an-occupied-worktree.md); [118: Clean up agent worktrees with PR context](118-clean-up-agent-worktrees-with-pr-context.md); [120: Apply editor and wrapping preferences across views](120-apply-editor-and-wrapping-preferences-across-views.md); [128: Handle provider pagination and changing model catalogs](128-handle-provider-pagination-and-changing-model-catalogs.md); [129: Replay and export truthful activity under load](129-replay-and-export-truthful-activity-under-load.md); [131: Run and distribute the native macOS workflows](131-run-and-distribute-the-native-macos-workflows.md); [134: Complete every reference menu, tab, dialog and state](../pending/134-complete-every-reference-menu-tab-dialog-and-state.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md); [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

## Acceptance criteria

- [ ] Compare on named reference fixtures/hardware and approved budgets; verify every in-scope checkpoint, including all newly discovered IDs.
- [ ] Publish a release candidate only after no unresolved parity exception is hidden by aggregate counts.
- [ ] **GAP-008:** Agree and verify visual, accessibility, responsiveness, performance and recovery acceptance on named fixtures.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: GAP-008.

This verifies the completed streams and agreed evidence. Any failures produce corrective slices that must land before acceptance; it does not hide an unlimited implementation task inside final QA.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
