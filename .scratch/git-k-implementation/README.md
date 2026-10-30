# Git-K implementation tracker

Published: October 30, 2026, following explicit user approval of the breakdown.

## Scope and sources

[Specification](spec.md), [coverage ownership](coverage.md), [approved breakdown](../../docs/implementation-draft/breakdown.md), and [existing decision map](../gitkraken-parity/map.md).

125 feature/delivery slices and one final verification gate are published under issues/. Nine envelopes are retained under pending/ until their decisions and child slices are defined. All 416 planning records retain an owner; none is marked implemented or verified by publication.

## Readiness

- The ready-for-agent label means the defined breakdown is approved. It does not override blocked state or dependencies.
- Before claiming an issue, require every Blocked by item to be done, every Decision gate to be resolved, and no unresolved behavior that makes the requested implementation unspecified.
- Verification gates remain required before recording full parity verification or completing the final acceptance gate.
- Pending envelopes cannot be claimed as implementation slices. Resolve their questions, approve child slices, link those children, and complete them before the envelope can become done for dependent work.
- Update Status in the authoritative work file when readiness changes; use blocked, ready, claimed and done for defined work. Preserve Label separately from execution state.

## Next actionable work

The existing [execution and repository coordination prototype](../gitkraken-parity/issues/05-execution-contract.md) is the next actionable map item. Native support decisions and matching-reference access can proceed independently where their prerequisites allow. Production slices remain blocked by the explicit contracts, including G5 and G8; publication does not resolve them.

## Published work and pending scope

| Work item | Kind | Location |
| --- | --- | --- |
| 1: Open a repository in a running desktop app | vertical-slice | [Published issue](issues/001-open-a-repository-in-a-running-desktop-app.md) |
| 2: Inspect changed and historical files | vertical-slice | [Published issue](issues/002-inspect-changed-and-historical-files.md) |
| 3: Stage files and commit real changes | vertical-slice | [Published issue](issues/003-stage-files-and-commit-real-changes.md) |
| 4: Stage, unstage and discard lines or hunks | vertical-slice | [Published issue](issues/004-stage-unstage-and-discard-lines-or-hunks.md) |
| 5: Ignore files and stop tracking explicitly | vertical-slice | [Published issue](issues/005-ignore-files-and-stop-tracking-explicitly.md) |
| 6: Create, edit, save and delete repository files | vertical-slice | [Published issue](issues/006-create-edit-save-and-delete-repository-files.md) |
| 7: Inspect historical files and encoded documents | vertical-slice | [Published issue](issues/007-inspect-historical-files-and-encoded-documents.md) |
| 8: Use commit templates and co-authors | vertical-slice | [Published issue](issues/008-use-commit-templates-and-co-authors.md) |
| 9: Amend the latest commit | vertical-slice | [Published issue](issues/009-amend-the-latest-commit.md) |
| 10: Navigate the commit graph | vertical-slice | [Published issue](issues/010-navigate-the-commit-graph.md) |
| 11: Work in multiple repository tabs | vertical-slice | [Published issue](issues/011-work-in-multiple-repository-tabs.md) |
| 12: Execute actions through menus and keyboard navigation | vertical-slice | [Published issue](issues/012-execute-actions-through-menus-and-keyboard-navigation.md) |
| 13: Create, check out, rename and delete branches | vertical-slice | [Published issue](issues/013-create-check-out-rename-and-delete-branches.md) |
| 14: Work in detached HEAD and preserve commits | vertical-slice | [Published issue](issues/014-work-in-detached-head-and-preserve-commits.md) |
| 15: Merge branches through the graph | vertical-slice | [Published issue](issues/015-merge-branches-through-the-graph.md) |
| 16: Resolve a conflicted merge | vertical-slice | [Published issue](issues/016-resolve-a-conflicted-merge.md) |
| 17: Rebase a branch or selected range | vertical-slice | [Published issue](issues/017-rebase-a-branch-or-selected-range.md) |
| 18: Edit an interactive rebase plan | vertical-slice | [Published issue](issues/018-edit-an-interactive-rebase-plan.md) |
| 19: Cherry-pick and edit a multi-commit selection | vertical-slice | [Published issue](issues/019-cherry-pick-and-edit-a-multi-commit-selection.md) |
| 20: Squash a consecutive history selection | vertical-slice | [Published issue](issues/020-squash-a-consecutive-history-selection.md) |
| 21: Reset, revert and restore historical files | vertical-slice | [Published issue](issues/021-reset-revert-and-restore-historical-files.md) |
| 22: Create, inspect, apply, pop and delete stashes | vertical-slice | [Published issue](issues/022-create-inspect-apply-pop-and-delete-stashes.md) |
| 23: Stash or restore selected changes | vertical-slice | [Published issue](issues/023-stash-or-restore-selected-changes.md) |
| 24: Pin, hide, solo and search visible history | vertical-slice | [Published issue](issues/024-pin-hide-solo-and-search-visible-history.md) |
| 25: Configure remotes and inspect their identity | vertical-slice | [Published issue](issues/025-configure-remotes-and-inspect-their-identity.md) |
| 26: Fetch remotes and show ahead/behind state | vertical-slice | [Published issue](issues/026-fetch-remotes-and-show-ahead-behind-state.md) |
| 27: Complete remote authentication and input requests | vertical-slice | [Published issue](issues/027-complete-remote-authentication-and-input-requests.md) |
| 28: Push commits and handle divergence | vertical-slice | [Published issue](issues/028-push-commits-and-handle-divergence.md) |
| 29: Pull with the selected integration strategy | vertical-slice | [Published issue](issues/029-pull-with-the-selected-integration-strategy.md) |
| 30: Clone and open repositories | vertical-slice | [Published issue](issues/030-clone-and-open-repositories.md) |
| 31: Clone and manage sparse checkout selection | vertical-slice | [Published issue](issues/031-clone-and-manage-sparse-checkout-selection.md) |
| 32: Initialize a repository with templates | vertical-slice | [Published issue](issues/032-initialize-a-repository-with-templates.md) |
| 33: Organize repositories through the repository hub | vertical-slice | [Published issue](issues/033-organize-repositories-through-the-repository-hub.md) |
| 34: Create and manage local and remote tags | vertical-slice | [Published issue](issues/034-create-and-manage-local-and-remote-tags.md) |
| 35: Create and manage linked worktrees | vertical-slice | [Published issue](issues/035-create-and-manage-linked-worktrees.md) |
| 36: Reconcile concurrent and interrupted repository operations | vertical-slice | [Published issue](issues/036-reconcile-concurrent-and-interrupted-repository-operations.md) |
| 37: Add, update and navigate submodules | vertical-slice | [Published issue](issues/037-add-update-and-navigate-submodules.md) |
| 38: Start and finish Gitflow work | vertical-slice | [Published issue](issues/038-start-and-finish-gitflow-work.md) |
| 39: Manage LFS files and transfers | vertical-slice | [Published issue](issues/039-manage-lfs-files-and-transfers.md) |
| 40: Sign commits and tags with GPG | vertical-slice | [Published issue](issues/040-sign-commits-and-tags-with-gpg.md) |
| 41: Sign and verify with SSH keys | vertical-slice | [Published issue](issues/041-sign-and-verify-with-ssh-keys.md) |
| 42: Configure and observe Git hooks | vertical-slice | [Published issue](issues/042-configure-and-observe-git-hooks.md) |
| 43: Use a real embedded interactive terminal | vertical-slice | [Published issue](issues/043-use-a-real-embedded-interactive-terminal.md) |
| 44: Manage persistent per-worktree terminal tabs | vertical-slice | [Published issue](issues/044-manage-persistent-per-worktree-terminal-tabs.md) |
| 45: Customize terminal input and completion | vertical-slice | [Published issue](issues/045-customize-terminal-input-and-completion.md) |
| 46: Undo and redo supported local actions | vertical-slice | [Published issue](issues/046-undo-and-redo-supported-local-actions.md) |
| 47: Undo and redo history rewriting | vertical-slice | [Published issue](issues/047-undo-and-redo-history-rewriting.md) |
| 48: Search commits and navigate matches | vertical-slice | [Published issue](issues/048-search-commits-and-navigate-matches.md) |
| 49: Compare selections and inspect file history or blame | vertical-slice | [Published issue](issues/049-compare-selections-and-inspect-file-history-or-blame.md) |
| 50: Create and apply local patches | vertical-slice | [Published issue](issues/050-create-and-apply-local-patches.md) |
| 51: Open repositories and references from external links | vertical-slice | [Published issue](issues/051-open-repositories-and-references-from-external-links.md) |
| 52: Separate identities, preferences and tabs with profiles | vertical-slice | [Published issue](issues/052-separate-identities-preferences-and-tabs-with-profiles.md) |
| 53: Use configured editors and external tools | vertical-slice | [Published issue](issues/053-use-configured-editors-and-external-tools.md) |
| 54: Select themes and follow system appearance | vertical-slice | [Published issue](issues/054-select-themes-and-follow-system-appearance.md) |
| 55: Inspect scoped activity and diagnostic output | vertical-slice | [Published issue](issues/055-inspect-scoped-activity-and-diagnostic-output.md) |
| 56: Connect to internally trusted Git hosts | vertical-slice | [Published issue](issues/056-connect-to-internally-trusted-git-hosts.md) |
| 57: Connect GitHub and manage hosted repositories | vertical-slice | [Published issue](issues/057-connect-github-and-manage-hosted-repositories.md) |
| 58: Create, inspect and merge GitHub pull requests | vertical-slice | [Published issue](issues/058-create-inspect-and-merge-github-pull-requests.md) |
| 59: Create and navigate stacked GitHub pull requests | vertical-slice | [Published issue](issues/059-create-and-navigate-stacked-github-pull-requests.md) |
| 60: Connect a GitHub Enterprise host | vertical-slice | [Published issue](issues/060-connect-a-github-enterprise-host.md) |
| 61: Use Enterprise repositories and pull requests | vertical-slice | [Published issue](issues/061-use-enterprise-repositories-and-pull-requests.md) |
| 62: Connect GitLab and discover repositories | vertical-slice | [Published issue](issues/062-connect-gitlab-and-discover-repositories.md) |
| 63: Create GitLab repositories and merge requests | vertical-slice | [Published issue](issues/063-create-gitlab-repositories-and-merge-requests.md) |
| 64: Use GitLab Self-Managed workflows | vertical-slice | [Published issue](issues/064-use-gitlab-self-managed-workflows.md) |
| 65: Connect Bitbucket Cloud and manage repositories | vertical-slice | [Published issue](issues/065-connect-bitbucket-cloud-and-manage-repositories.md) |
| 66: Create and inspect Bitbucket pull requests | vertical-slice | [Published issue](issues/066-create-and-inspect-bitbucket-pull-requests.md) |
| 67: Use Bitbucket Data Center workflows | vertical-slice | [Published issue](issues/067-use-bitbucket-data-center-workflows.md) |
| 68: Connect Azure DevOps and manage repositories | vertical-slice | [Published issue](issues/068-connect-azure-devops-and-manage-repositories.md) |
| 69: Use Azure DevOps pull requests and reviewers | vertical-slice | [Published issue](issues/069-use-azure-devops-pull-requests-and-reviewers.md) |
| 70: Work with GitHub issues and issue branches | vertical-slice | [Published issue](issues/070-work-with-github-issues-and-issue-branches.md) |
| 71: Use Enterprise issues | vertical-slice | [Published issue](issues/071-use-enterprise-issues.md) |
| 72: Work with GitLab issues and issue branches | vertical-slice | [Published issue](issues/072-work-with-gitlab-issues-and-issue-branches.md) |
| 73: Use Self-Managed GitLab issues | vertical-slice | [Published issue](issues/073-use-self-managed-gitlab-issues.md) |
| 74: Connect Jira and work with issues | vertical-slice | [Published issue](issues/074-connect-jira-and-work-with-issues.md) |
| 75: Use Jira Data Center issues | vertical-slice | [Published issue](issues/075-use-jira-data-center-issues.md) |
| 76: Use Trello cards as development work | vertical-slice | [Published issue](issues/076-use-trello-cards-as-development-work.md) |
| 77: Complete the Azure work-item reference audit and workflow | gated-envelope | [Pending envelope](pending/077-complete-the-azure-work-item-reference-audit-and-workflow.md) |
| 78: Complete the Git Integration for Jira workflow | gated-envelope | [Pending envelope](pending/078-complete-the-git-integration-for-jira-workflow.md) |
| 79: Sign in to a self-hosted account from the desktop | vertical-slice | [Published issue](issues/079-sign-in-to-a-self-hosted-account-from-the-desktop.md) |
| 80: Synchronize account integration connections across devices | vertical-slice | [Published issue](issues/080-synchronize-account-integration-connections-across-devices.md) |
| 81: Manage organization members and roles | vertical-slice | [Published issue](issues/081-manage-organization-members-and-roles.md) |
| 82: Manage teams and membership | vertical-slice | [Published issue](issues/082-manage-teams-and-membership.md) |
| 83: Implement the agreed seats and license administration model | gated-envelope | [Pending envelope](pending/083-implement-the-agreed-seats-and-license-administration-model.md) |
| 84: Start offline and manage local entitlements | vertical-slice | [Published issue](issues/084-start-offline-and-manage-local-entitlements.md) |
| 85: Authenticate through the supported internal identity options | vertical-slice | [Published issue](issues/085-authenticate-through-the-supported-internal-identity-options.md) |
| 86: Organize local workspaces | vertical-slice | [Published issue](issues/086-organize-local-workspaces.md) |
| 87: Share and synchronize workspaces | vertical-slice | [Published issue](issues/087-share-and-synchronize-workspaces.md) |
| 88: Run actions across selected workspace repositories | vertical-slice | [Published issue](issues/088-run-actions-across-selected-workspace-repositories.md) |
| 89: Inspect personal pull-request activity in Launchpad | vertical-slice | [Published issue](issues/089-inspect-personal-pull-request-activity-in-launchpad.md) |
| 90: Inspect issues and local work in Launchpad | vertical-slice | [Published issue](issues/090-inspect-issues-and-local-work-in-launchpad.md) |
| 91: Pin, snooze and save Launchpad views | vertical-slice | [Published issue](issues/091-pin-snooze-and-save-launchpad-views.md) |
| 92: Use team and enterprise-provider Launchpad contexts | vertical-slice | [Published issue](issues/092-use-team-and-enterprise-provider-launchpad-contexts.md) |
| 93: Open the equivalent connected Code Review workflow | gated-envelope | [Pending envelope](pending/093-open-the-equivalent-connected-code-review-workflow.md) |
| 94: Act on Launchpad pull requests | vertical-slice | [Published issue](issues/094-act-on-launchpad-pull-requests.md) |
| 95: Create and manage shareable patches | vertical-slice | [Published issue](issues/095-create-and-manage-shareable-patches.md) |
| 96: Authorize recipients and apply shared patches | vertical-slice | [Published issue](issues/096-authorize-recipients-and-apply-shared-patches.md) |
| 97: Use customer-controlled patch storage | vertical-slice | [Published issue](issues/097-use-customer-controlled-patch-storage.md) |
| 98: Share team branch and WIP presence | vertical-slice | [Published issue](issues/098-share-team-branch-and-wip-presence.md) |
| 99: Show overlap and reference conflict-prevention behavior | gated-envelope | [Pending envelope](pending/099-show-overlap-and-reference-conflict-prevention-behavior.md) |
| 100: Connect workspace Insights and import data | vertical-slice | [Published issue](issues/100-connect-workspace-insights-and-import-data.md) |
| 101: Configure Insights role and AI-impact sources | vertical-slice | [Published issue](issues/101-configure-insights-role-and-ai-impact-sources.md) |
| 102: Implement the agreed Insights dashboards and metrics | gated-envelope | [Pending envelope](pending/102-implement-the-agreed-insights-dashboards-and-metrics.md) |
| 103: Configure AI providers and organization policy | vertical-slice | [Published issue](issues/103-configure-ai-providers-and-organization-policy.md) |
| 104: Generate and edit commit messages with AI | vertical-slice | [Published issue](issues/104-generate-and-edit-commit-messages-with-ai.md) |
| 105: Explain commits and branch changes | vertical-slice | [Published issue](issues/105-explain-commits-and-branch-changes.md) |
| 106: Generate editable stash messages | vertical-slice | [Published issue](issues/106-generate-editable-stash-messages.md) |
| 107: Generate pull-request descriptions with templates | vertical-slice | [Published issue](issues/107-generate-pull-request-descriptions-with-templates.md) |
| 108: Review AI conflict-resolution suggestions | vertical-slice | [Published issue](issues/108-review-ai-conflict-resolution-suggestions.md) |
| 109: Compose staged work into proposed commits | vertical-slice | [Published issue](issues/109-compose-staged-work-into-proposed-commits.md) |
| 110: Recompose existing contiguous commits | vertical-slice | [Published issue](issues/110-recompose-existing-contiguous-commits.md) |
| 111: Configure coding-agent launch and repository setup | vertical-slice | [Published issue](issues/111-configure-coding-agent-launch-and-repository-setup.md) |
| 112: Start an agent in a correctly configured worktree | vertical-slice | [Published issue](issues/112-start-an-agent-in-a-correctly-configured-worktree.md) |
| 113: Install and manage explicit status integrations | gated-envelope | [Pending envelope](pending/113-install-and-manage-explicit-status-integrations.md) |
| 114: Inspect running and waiting agent sessions | vertical-slice | [Published issue](issues/114-inspect-running-and-waiting-agent-sessions.md) |
| 115: Respond to supported agent permission requests | gated-envelope | [Pending envelope](pending/115-respond-to-supported-agent-permission-requests.md) |
| 116: Launch agents from linked issues | vertical-slice | [Published issue](issues/116-launch-agents-from-linked-issues.md) |
| 117: Run another agent in an occupied worktree | vertical-slice | [Published issue](issues/117-run-another-agent-in-an-occupied-worktree.md) |
| 118: Clean up agent worktrees with PR context | vertical-slice | [Published issue](issues/118-clean-up-agent-worktrees-with-pr-context.md) |
| 119: Display configured identities and avatars | vertical-slice | [Published issue](issues/119-display-configured-identities-and-avatars.md) |
| 120: Apply editor and wrapping preferences across views | vertical-slice | [Published issue](issues/120-apply-editor-and-wrapping-preferences-across-views.md) |
| 121: Select and verify the Git execution distribution | vertical-slice | [Published issue](issues/121-select-and-verify-the-git-execution-distribution.md) |
| 122: Apply panel, identity and status-bar presentation settings | vertical-slice | [Published issue](issues/122-apply-panel-identity-and-status-bar-presentation-settings.md) |
| 123: Apply background Git and history-loading preferences | vertical-slice | [Published issue](issues/123-apply-background-git-and-history-loading-preferences.md) |
| 124: Apply repository-specific Git and editor configuration | vertical-slice | [Published issue](issues/124-apply-repository-specific-git-and-editor-configuration.md) |
| 125: Configure repository integrations and team or agent defaults | vertical-slice | [Published issue](issues/125-configure-repository-integrations-and-team-or-agent-defaults.md) |
| 126: Configure and receive the reference desktop notifications | vertical-slice | [Published issue](issues/126-configure-and-receive-the-reference-desktop-notifications.md) |
| 127: Expose conditional toolbar and experimental controls | vertical-slice | [Published issue](issues/127-expose-conditional-toolbar-and-experimental-controls.md) |
| 128: Handle provider pagination and changing model catalogs | vertical-slice | [Published issue](issues/128-handle-provider-pagination-and-changing-model-catalogs.md) |
| 129: Replay and export truthful activity under load | vertical-slice | [Published issue](issues/129-replay-and-export-truthful-activity-under-load.md) |
| 130: Install, update and recover a Windows build | vertical-slice | [Published issue](issues/130-install-update-and-recover-a-windows-build.md) |
| 131: Run and distribute the native macOS workflows | vertical-slice | [Published issue](issues/131-run-and-distribute-the-native-macos-workflows.md) |
| 132: Run and distribute the native Linux workflows | vertical-slice | [Published issue](issues/132-run-and-distribute-the-native-linux-workflows.md) |
| 133: Use the correct Windows or WSL application context | vertical-slice | [Published issue](issues/133-use-the-correct-windows-or-wsl-application-context.md) |
| 134: Complete every reference menu, tab, dialog and state | gated-envelope | [Pending envelope](pending/134-complete-every-reference-menu-tab-dialog-and-state.md) |
| 135: Verify accessibility, performance and full parity evidence | verification-gate | [Published issue](issues/135-verify-accessibility-performance-and-full-parity-evidence.md) |
