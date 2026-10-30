# Git-K full-parity implementation specification

Status: approved-plan — user confirmed the breakdown and complete-workflow testing on October 30, 2026.
Label: ready-for-agent

Publication preserves unresolved architecture, reference and service decision gates. No application implementation is claimed.

## Problem Statement

The user wants every feature, tab, function and observable desktop workflow of GitKraken Desktop 12.6.1, with a closely matching experience. They also want users to understand graphical actions through actual Git commands and results. A terminal panel alone does not establish truthful execution history.

## Solution

Build Git-K with Tauri, a Rust application core and React/TypeScript. Retain the full documented and subsequently observed parity scope, including paid-reference connected capabilities. Validate Windows first, then macOS and Linux. Supply equivalent connected services for self-hosting first. Users may collapse the terminal/activity panel while recording continues. Git actions show real commands; provider actions show API activity.

The existing audits index 252 desktop/Git checkpoints, 133 connected entries and 15 runtime findings. These are evidence records, not a complete observed menu census or implemented features. This plan assigns each indexed record a primary owner and explicitly carries supplemental and undiscovered behavior obligations.

## User Stories

1. As a Git-K user, I want to launch the native Tauri app, open a local repository, inspect real branch/status/history data and see the exact read commands and results in a collapsible action panel, so that I can complete this reference workflow with understandable, verifiable results.

2. As a Git-K user, I want to select a changed or historical file and inspect real hunk, inline and split diffs with change navigation, so that I can complete this reference workflow with understandable, verifiable results.

3. As a Git-K user, I want to stage one or all changed files, write a message and create a real commit while watching the actual Git commands, so that I can complete this reference workflow with understandable, verifiable results.

4. As a Git-K user, I want to choose complete files, individual lines or hunks to stage, unstage or discard, and inspect the resulting staged and unstaged diffs, so that I can complete this reference workflow with understandable, verifiable results.

5. As a Git-K user, I want to use file actions to ignore a filename, extension or directory, or deliberately stop tracking an already tracked file, so that I can complete this reference workflow with understandable, verifiable results.

6. As a Git-K user, I want to create files and nested directories, browse/filter the tree, edit content, save or discard pending edits, and stage saved work including workflow YAML, so that I can complete this reference workflow with understandable, verifiable results.

7. As a Git-K user, I want to inspect historical content while editing the current file, choose supported encodings and preview Markdown without silently converting bytes, so that I can complete this reference workflow with understandable, verifiable results.

8. As a Git-K user, I want to configure and apply the correct repository/global commit template, edit message fields, add co-authors and resize the message editor, so that I can complete this reference workflow with understandable, verifiable results.

9. As a Git-K user, I want to amend staged changes or the latest message, with explicit update/cancel behavior and refreshed history, so that I can complete this reference workflow with understandable, verifiable results.

10. As a Git-K user, I want to inspect commit, merge, stash and WIP nodes, select details, highlight related history and customize/filter graph columns, so that I can complete this reference workflow with understandable, verifiable results.

11. As a Git-K user, I want to open, close, reorder and switch repositories through tabs, path tooltips, dropdowns and keyboard navigation, so that I can complete this reference workflow with understandable, verifiable results.

12. As a Git-K user, I want to search and execute context-appropriate actions through the command palette and matching menu/shortcut routes, so that I can complete this reference workflow with understandable, verifiable results.

13. As a Git-K user, I want to manage branches from graph/sidebar actions, including ghost-branch checkout and multi-selection deletion, so that I can complete this reference workflow with understandable, verifiable results.

14. As a Git-K user, I want to check out a commit, see detached-HEAD status, create commits and preserve them on a new branch or return to a branch, so that I can complete this reference workflow with understandable, verifiable results.

15. As a Git-K user, I want to merge using drag/drop or context actions, including the reference squash-merge preference and staged result, so that I can complete this reference workflow with understandable, verifiable results.

16. As a Git-K user, I want to inspect current/incoming/output panes, choose or edit resolutions, take one side, use an external tool and complete the merge, so that I can complete this reference workflow with understandable, verifiable results.

17. As a Git-K user, I want to rebase a branch, selected range or branchless target and resolve or recover from a conflicted operation, so that I can complete this reference workflow with understandable, verifiable results.

18. As a Git-K user, I want to pick, reword, squash or drop supported commits through the application, reset the plan and complete the operation without an external editor, so that I can complete this reference workflow with understandable, verifiable results.

19. As a Git-K user, I want to cherry-pick from a commit or branch head and edit a multi-commit plan with ordering, reword, squash and drop actions, so that I can complete this reference workflow with understandable, verifiable results.

20. As a Git-K user, I want to select a valid linear range, edit the resulting message and create the squashed history, so that I can complete this reference workflow with understandable, verifiable results.

21. As a Git-K user, I want to choose soft/mixed/hard reset, revert a commit, or restore selected historical files with truthful resulting state, so that I can complete this reference workflow with understandable, verifiable results.

22. As a Git-K user, I want to name and manage stashes through toolbar, graph, sidebar and commit-panel routes, so that I can complete this reference workflow with understandable, verifiable results.

23. As a Git-K user, I want to create a partial stash or restore selected files from an existing stash while preserving the intended remaining changes, so that I can complete this reference workflow with understandable, verifiable results.

24. As a Git-K user, I want to control reference/stash visibility through individual and bulk actions without mutating Git history, so that I can complete this reference workflow with understandable, verifiable results.

25. As a Git-K user, I want to add, edit, remove and inspect remotes with provider identity and supported shallow-remote options, so that I can complete this reference workflow with understandable, verifiable results.

26. As a Git-K user, I want to fetch one or all remotes and configure periodic background fetching while seeing accurate status and history updates, so that I can complete this reference workflow with understandable, verifiable results.

27. As a Git-K user, I want to complete helper/askpass/SSH/signing-related interaction requests for a remote operation without invisible prompts or credential output, so that I can complete this reference workflow with understandable, verifiable results.

28. As a Git-K user, I want to push from toolbar, menus or drag/drop, create a missing upstream branch and expose the reference divergence choices, so that I can complete this reference workflow with understandable, verifiable results.

29. As a Git-K user, I want to choose and remember fast-forward, merge-fallback, fast-forward-only or rebase pulling and set the upstream, so that I can complete this reference workflow with understandable, verifiable results.

30. As a Git-K user, I want to clone by URL or a connected-provider selection, configure supported shallow options and open the result, so that I can complete this reference workflow with understandable, verifiable results.

31. As a Git-K user, I want to choose sparse clone paths and get the reference behavior when the path selection is empty, so that I can complete this reference workflow with understandable, verifiable results.

32. As a Git-K user, I want to initialize at a chosen path with ignore/license options and open the resulting repository, so that I can complete this reference workflow with understandable, verifiable results.

33. As a Git-K user, I want to browse, locate, open, clone and initialize through repository management, with details/README, recent/favorite navigation and favorite shortcuts, so that I can complete this reference workflow with understandable, verifiable results.

34. As a Git-K user, I want to create lightweight/annotated tags, inspect annotations, navigate/filter/solo them and perform the observed push/move/rename/delete flows, so that I can complete this reference workflow with understandable, verifiable results.

35. As a Git-K user, I want to create, open, find, lock/unlock and remove worktrees with correct branch/folder/path display and inherited view settings, so that I can complete this reference workflow with understandable, verifiable results.

36. As a Git-K user, I want to run operations across worktrees, observe external changes and recover from lock conflicts or interrupted child processes with truthful state, so that I can complete this reference workflow with understandable, verifiable results.

37. As a Git-K user, I want to add/initialize/update submodules, inspect their state, switch into them and commit updated pointers with the configured automatic-update policy, so that I can complete this reference workflow with understandable, verifiable results.

38. As a Git-K user, I want to configure Gitflow names/prefixes and start or finish feature, release and hotfix workflows through real multi-step Git operations, so that I can complete this reference workflow with understandable, verifiable results.

39. As a Git-K user, I want to initialize LFS, track patterns, inspect pointer metadata and run supported transfer/prune actions from the interface, so that I can complete this reference workflow with understandable, verifiable results.

40. As a Git-K user, I want to select/generate a GPG key, configure the signing program and defaults, and inspect verification states on actual signed history, so that I can complete this reference workflow with understandable, verifiable results.

41. As a Git-K user, I want to configure SSH signing and allowed signers, sign real changes and copy the public key for provider setup, so that I can complete this reference workflow with understandable, verifiable results.

42. As a Git-K user, I want to configure hook locations, run normal hook-dependent operations, inspect hook output and bypass hooks for one commit when requested, so that I can complete this reference workflow with understandable, verifiable results.

43. As a Git-K user, I want to open a repository-bound shell, type commands, resize or minimize it and terminate the chosen session, so that I can complete this reference workflow with understandable, verifiable results.

44. As a Git-K user, I want to open, select, close and reorder terminal tabs with the reference keyboard/focus/scroll/label behavior while processes continue across worktree changes, so that I can complete this reference workflow with understandable, verifiable results.

45. As a Git-K user, I want to use command/flag completion, configure terminal typography/cursor behavior and insert dragged files or text as input, so that I can complete this reference workflow with understandable, verifiable results.

46. As a Git-K user, I want to undo and redo the supported checkout/commit/discard/reference/reset actions, with contextual availability and truthful state, so that I can complete this reference workflow with understandable, verifiable results.

47. As a Git-K user, I want to undo and redo the supported history-editing operations, including the reference detached-HEAD reset/rebase recovery behavior, so that I can complete this reference workflow with understandable, verifiable results.

48. As a Git-K user, I want to search by message, SHA or author, navigate highlighted matches and understand the loaded-history limit, so that I can complete this reference workflow with understandable, verifiable results.

49. As a Git-K user, I want to compare commits or WIP, combine consecutive selections, inspect file history and blame, and explain unsupported selections, so that I can complete this reference workflow with understandable, verifiable results.

50. As a Git-K user, I want to create patches from supported selected files/commits and apply them through the palette, so that I can complete this reference workflow with understandable, verifiable results.

51. As a Git-K user, I want to open a repository through CLI/deep links, navigate to a reference, offer a prefilled clone and choose among multiple local copies, so that I can complete this reference workflow with understandable, verifiable results.

52. As a Git-K user, I want to create/switch profiles with isolated identities, settings, accounts and tab sets; update global Git identity only through the explicit option, so that I can complete this reference workflow with understandable, verifiable results.

53. As a Git-K user, I want to detect/select editors and diff/merge/terminal tools, run them for the selected repository context and expose custom commands, so that I can complete this reference workflow with understandable, verifiable results.

54. As a Git-K user, I want to choose supported built-in/system themes and expose only the custom-theme behavior actually present in 12.6.1, so that I can complete this reference workflow with understandable, verifiable results.

55. As a Git-K user, I want to inspect application/repository activity with timestamps/durations, follow hook-output links, enable diagnostic detail and forget stored credentials, so that I can complete this reference workflow with understandable, verifiable results.

56. As a Git-K user, I want to perform Git and API workflows against an internally signed host with the approved platform/Git trust configuration, so that I can complete this reference workflow with understandable, verifiable results.

57. As a Git-K user, I want to connect GitHub through supported authentication, browse/create/clone repositories and configure/upload SSH keys, so that I can complete this reference workflow with understandable, verifiable results.

58. As a Git-K user, I want to create a PR from reference entry points, select forks/branches and metadata/templates, inspect status/remotes and use supported merge/browser actions, so that I can complete this reference workflow with understandable, verifiable results.

59. As a Git-K user, I want to add a PR to a target-branch stack and navigate stack positions and graph hints, so that I can complete this reference workflow with understandable, verifiable results.

60. As a Git-K user, I want to connect a chosen Enterprise Server with the required token scopes and an isolated profile account, so that I can complete this reference workflow with understandable, verifiable results.

61. As a Git-K user, I want to create/discover/clone/fork Enterprise repositories, upload SSH keys and complete the supported PR flows on that host, so that I can complete this reference workflow with understandable, verifiable results.

62. As a Git-K user, I want to connect GitLab through browser/manual OAuth and browse or clone with the correct account context, so that I can complete this reference workflow with understandable, verifiable results.

63. As a Git-K user, I want to create repositories, upload SSH keys, manage fork remotes and create/view the supported GitLab merge-request workflow, so that I can complete this reference workflow with understandable, verifiable results.

64. As a Git-K user, I want to connect a self-managed host by PAT and perform its repository, SSH, remote and merge-request workflows, so that I can complete this reference workflow with understandable, verifiable results.

65. As a Git-K user, I want to authorize Bitbucket Cloud, discover/create/clone repositories, manage fork remotes and copy SSH keys for setup, so that I can complete this reference workflow with understandable, verifiable results.

66. As a Git-K user, I want to create/view Bitbucket PRs and select reviewers according to repository permissions, so that I can complete this reference workflow with understandable, verifiable results.

67. As a Git-K user, I want to connect a Data Center host by PAT and use its repository, remote, SSH and pull-request workflows, so that I can complete this reference workflow with understandable, verifiable results.

68. As a Git-K user, I want to use OAuth or PAT, choose the organization and discover/create/clone repositories with the observed fork/avatar behavior, so that I can complete this reference workflow with understandable, verifiable results.

69. As a Git-K user, I want to copy SSH keys and create/view Azure PRs with the correct supported metadata and reviewers, so that I can complete this reference workflow with understandable, verifiable results.

70. As a Git-K user, I want to filter/preview/edit/create issues, copy/open links and create title-prefilled branches through GitHub issue actions, so that I can complete this reference workflow with understandable, verifiable results.

71. As a Git-K user, I want to use the same issue and branch workflow against the selected Enterprise host, so that I can complete this reference workflow with understandable, verifiable results.

72. As a Git-K user, I want to preview/create/edit/filter GitLab issues and open/copy links or create issue-prefilled branches, so that I can complete this reference workflow with understandable, verifiable results.

73. As a Git-K user, I want to use the issue workflow through the self-managed connection and correct host/account permissions, so that I can complete this reference workflow with understandable, verifiable results.

74. As a Git-K user, I want to authorize Jira, browse projects, edit/create issues and comments, filter with JQL and create linked branches, so that I can complete this reference workflow with understandable, verifiable results.

75. As a Git-K user, I want to connect Jira Data Center by host/PAT and perform the supported issue and branch workflows, so that I can complete this reference workflow with understandable, verifiable results.

76. As a Git-K user, I want to authorize Trello, preview/create/edit cards and comments, filter and create card-prefilled branches with synchronized changes, so that I can complete this reference workflow with understandable, verifiable results.

77. As a Git-K user, I want to observe the reference Azure work-item entry points, add their subrequirements and implement all confirmed desktop-accessible actions, so that I can complete this reference workflow with understandable, verifiable results.

78. As a Git-K user, I want to audit the reference Jira integration/deep-link flows and implement the confirmed connected workflow, so that I can complete this reference workflow with understandable, verifiable results.

79. As a Git-K user or self-hosted administrator, I want to deploy an equivalent account endpoint, complete browser-based sign-in/sign-up and return to the correct desktop session, so that I can complete this reference workflow with understandable, verifiable results.

80. As a Git-K user or self-hosted administrator, I want to connect an integration on one desktop and recover the expected account connection on another with the reference profile/host distinctions, so that I can complete this reference workflow with understandable, verifiable results.

81. As a Git-K user or self-hosted administrator, I want to invite/remove members, manage roles and billing contacts and use contributor suggestions through the self-hosted organization workflow, so that I can complete this reference workflow with understandable, verifiable results.

82. As a Git-K user or self-hosted administrator, I want to create, rename and delete teams and add/remove members with persistent shared state, so that I can complete this reference workflow with understandable, verifiable results.

83. As a Git-K user or self-hosted administrator, I want to expose the agreed equivalent account/seat/license/version administration behavior, including exhausted-seat states and applicable billing handoffs, so that I can complete this reference workflow with understandable, verifiable results.

84. As a Git-K user or self-hosted administrator, I want to start the agreed standalone/offline mode and apply a local entitlement/license update with usable repository workflows, so that I can complete this reference workflow with understandable, verifiable results.

85. As a Git-K user or self-hosted administrator, I want to configure and complete supported internal email/LDAP or equivalent agreed enterprise identity flows, so that I can complete this reference workflow with understandable, verifiable results.

86. As a Git-K user or self-hosted administrator, I want to group repositories from selections, folders or VS Code workspaces and synchronize the group with a local directory, so that I can complete this reference workflow with understandable, verifiable results.

87. As a Git-K user or self-hosted administrator, I want to create/convert/edit/reorder shared workspaces with colors/icons/details and team/user permissions across two desktops, so that I can complete this reference workflow with understandable, verifiable results.

88. As a Git-K user or self-hosted administrator, I want to select repositories and bulk clone/fetch/pull/open/locate/remove them while inspecting each result, so that I can complete this reference workflow with understandable, verifiable results.

89. As a Git-K user or self-hosted administrator, I want to open provider/workspace activity, inspect personal PR involvement and status columns and navigate status-bar summaries, so that I can complete this reference workflow with understandable, verifiable results.

90. As a Git-K user or self-hosted administrator, I want to filter connected issues by their supported fields and open repositories with local work in progress, so that I can complete this reference workflow with understandable, verifiable results.

91. As a Git-K user or self-hosted administrator, I want to use PR/Issues/WIPs/All/Snoozed tabs, pin items, schedule snoozes and create/delete saved views, so that I can complete this reference workflow with understandable, verifiable results.

92. As a Git-K user or self-hosted administrator, I want to choose a team/collaborator view and inspect activity from the supported internal and hosted provider contexts, so that I can complete this reference workflow with understandable, verifiable results.

93. As a Git-K user or self-hosted administrator, I want to follow the desktop browser-review handoff into the agreed equivalent connected review destination, so that I can complete this reference workflow with understandable, verifiable results.

94. As a Git-K user or self-hosted administrator, I want to open/review/close PRs, inspect supported suggestions and check out their branch or a worktree from Launchpad, so that I can complete this reference workflow with understandable, verifiable results.

95. As a Git-K user or self-hosted administrator, I want to create shared patches from staged changes or commits, list/copy their links and delete selected patches, so that I can complete this reference workflow with understandable, verifiable results.

96. As a Git-K user or self-hosted administrator, I want to share encrypted patch content with allowed collaborators or links, open it and apply it to the chosen branch, so that I can complete this reference workflow with understandable, verifiable results.

97. As a Git-K user or self-hosted administrator, I want to configure/test an S3-compatible customer storage target and complete the same authorized share/apply workflow, so that I can complete this reference workflow with understandable, verifiable results.

98. As a Git-K user or self-hosted administrator, I want to choose a repository team, inspect teammate branches/WIP, set presence and opt out of WIP sharing, so that I can complete this reference workflow with understandable, verifiable results.

99. As a Git-K user or self-hosted administrator, I want to inspect changed-line overlap and the fully audited reference conflict-prevention workflow, so that I can complete this reference workflow with understandable, verifiable results.

100. As a Git-K user or self-hosted administrator, I want to open the equivalent workspace Insights destination, connect the supported sources and follow repository import progress, so that I can complete this reference workflow with understandable, verifiable results.

101. As a Git-K user or self-hosted administrator, I want to configure the reference-supported role/timezone and additional source/token connections in the equivalent Insights workflow, so that I can complete this reference workflow with understandable, verifiable results.

102. As a Git-K user or self-hosted administrator, I want to complete the deeper analytics audit, define observable metrics and provide the agreed dashboard/drilldown experience, so that I can complete this reference workflow with understandable, verifiable results.

103. As a Git-K user, I want to choose supported provider/model/key/custom endpoint and workflow instructions, with enforced organization policy and masked admin-managed values, so that I can complete this reference workflow with understandable, verifiable results.

104. As a Git-K user, I want to generate or regenerate an editable commit message and amend message from actual selected changes, so that I can complete this reference workflow with understandable, verifiable results.

105. As a Git-K user, I want to request and inspect an explanation for selected commits or branch changes, so that I can complete this reference workflow with understandable, verifiable results.

106. As a Git-K user, I want to generate a stash message, edit or discard it and create the stash through the ordinary recorded operation, so that I can complete this reference workflow with understandable, verifiable results.

107. As a Git-K user, I want to generate/edit PR titles and descriptions using the selected template and supported provider flow, so that I can complete this reference workflow with understandable, verifiable results.

108. As a Git-K user, I want to inspect suggested resolutions with explanations/confidence, accept or discard them and complete the normal conflict workflow, so that I can complete this reference workflow with understandable, verifiable results.

109. As a Git-K user, I want to review a proposed grouping of changes into commits, edit it and apply it with real progress and preserved authors/co-authors, so that I can complete this reference workflow with understandable, verifiable results.

110. As a Git-K user, I want to reorder/squash/edit AI-proposed compositions of supported existing commits and apply the approved history plan, so that I can complete this reference workflow with understandable, verifiable results.

111. As a Git-K user, I want to detect/configure supported local CLIs, custom arguments and sequential repository setup commands, so that I can complete this reference workflow with understandable, verifiable results.

112. As a Git-K user, I want to choose the branch/base/agent, validate names, create the worktree, inherit view state and open the new session in the selected tab, so that I can complete this reference workflow with understandable, verifiable results.

113. As a Git-K user, I want to inspect/install/reinstall/uninstall supported status adapters and report metadata-only events, so that I can complete this reference workflow with understandable, verifiable results.

114. As a Git-K user, I want to inspect/filter/sort session cards with WIP, ahead/behind and running/waiting states, so that I can complete this reference workflow with understandable, verifiable results.

115. As a Git-K user, I want to allow or deny supported requests inline and direct unsupported requests to the correct terminal, so that I can complete this reference workflow with understandable, verifiable results.

116. As a Git-K user, I want to start an agent with the selected issue context and preserve the reference branch-association constraints, so that I can complete this reference workflow with understandable, verifiable results.

117. As a Git-K user, I want to start an additional agent in its own terminal without disturbing an existing session, so that I can complete this reference workflow with understandable, verifiable results.

118. As a Git-K user, I want to show merged-PR context and perform explicit lock/unlock/remove/optional-branch-delete actions from agent cards, so that I can complete this reference workflow with understandable, verifiable results.

119. As a Git-K user, I want to show the correct provider or email-based identity avatar in profile and history contexts, so that I can complete this reference workflow with understandable, verifiable results.

120. As a Git-K user, I want to configure editor typography/tab/EOL/highlighting/line-number/wrapping and observe the same intended settings in file, diff, history and merge views, so that I can complete this reference workflow with understandable, verifiable results.

121. As a Git-K user, I want to select the supported bundled/system Git configuration, inspect compatibility and perform a real operation with the chosen executable, so that I can complete this reference workflow with understandable, verifiable results.

122. As a Git-K user, I want to resize/collapse/hide panels, configure date/avatar/graph presentation, choose zoom and show or hide supported Launchpad/Agents surfaces, so that I can complete this reference workflow with understandable, verifiable results.

123. As a Git-K user, I want to configure fetch/prune/conflict/submodule behavior, branch/backup/graph limits, lazy history/tab persistence and Windows Git preferences, so that I can complete this reference workflow with understandable, verifiable results.

124. As a Git-K user, I want to configure repository encoding, Gitflow, hooks, LFS and commit preferences through the completed workflows, so that I can complete this reference workflow with understandable, verifiable results.

125. As a Git-K user, I want to navigate SSH/hosting/issue settings and apply repository issue, Team View, submodule and agent setup defaults, so that I can complete this reference workflow with understandable, verifiable results.

126. As a Git-K user, I want to configure supported desktop/marketing notification preferences and receive the reference-supported notifications for actual events, so that I can complete this reference workflow with understandable, verifiable results.

127. As a Git-K user, I want to show the correct conditional toolbar actions and experimental feature controls, and execute their already implemented workflows, so that I can complete this reference workflow with understandable, verifiable results.

128. As a Git-K user, I want to use complete empty/paginated metadata pickers and refreshed provider model catalogs across the supported workflows, so that I can complete this reference workflow with understandable, verifiable results.

129. As a Git-K user, I want to exercise the approved history retention/replay/export policy while operations stream output, the view is slow or hidden, and the app restarts, so that I can complete this reference workflow with understandable, verifiable results.

130. As a Git-K user, I want to install a packaged Windows app, open a repository, update it and recover the previous supported state when an update fails, so that I can complete this reference workflow with understandable, verifiable results.

131. As a Git-K user, I want to install/run the macOS app with the correct default shell, native menus, key bindings, process control and supported CPU architectures, so that I can complete this reference workflow with understandable, verifiable results.

132. As a Git-K user, I want to install/run the Linux app with the selected package formats, desktop integration, shell/process handling and supported distribution/CPU matrix, so that I can complete this reference workflow with understandable, verifiable results.

133. As a Git-K user, I want to run the Linux app under supported WSLg, explain cross-filesystem choices and route links/files/agents through the correct context, so that I can complete this reference workflow with understandable, verifiable results.

134. As a Git-K user, I want to compare the full observed reference census with Git-K and implement remaining native menus, onboarding, dialogs, tooltips, shortcut/context states and screen-specific differences, so that I can complete this reference workflow with understandable, verifiable results.

135. As a Git-K user, I want to run the agreed visual/behavioral/platform/recovery/accessibility/performance checks and close every observed difference with evidence, so that I can complete this reference workflow with understandable, verifiable results.

## Implementation Decisions

### Confirmed

- Use Tauri, Rust and React/TypeScript; preserve the pinned 12.6.1 reference and track later releases separately.
- Retain all three desktop platforms, Windows-first validation, self-hosted equivalent connected services and continuous recording with a collapsible panel.
- Use the configured local Markdown tracker. The presence of a GitHub remote does not change that choice.
- Test complete public workflows: desktop intent, Rust execution, independently observable repository/service result and truthful action history. Provider simulations cover failure cases and live checks cover supported integrations. The user approved this testing seam in the current conversation.

### Proposed modules and contracts to validate

- A desktop workflow module provides repository/account selection, domain actions, result observation and context-specific presentation. Its callers do not build shell strings.
- A repository/operation module owns Git processes, native path identity, operation steps, interaction requests and repository reconciliation. Executable/arguments/working directory form the authoritative execution input; display formatting never becomes a second execution.
- An action-history module retains operation origin, ordered observed events and truthful outcomes independently of panel mounting. The execution prototype must settle retention, restart, spool/truncation, redaction, cancellation and UI-only activity semantics.
- An interactive-terminal module owns PTY sessions, input/output/resize/lifecycle and flow control. PTY bytes are not represented as an exact audit of arbitrary shell commands or descendant processes.
- Provider modules supply distinct host/account/capability behavior through replaceable adapters. Local Git credentials, provider API credentials and shared account identity remain distinct concepts.
- Self-hosted account, workspace/team/presence/patch, policy and connected-view modules supply the observable shared workflows. Their persistence, identity, credential placement, deployment interfaces, licensing and browser-destination scope require the existing service decision. No database, auth vendor, billing vendor or proprietary protocol has been silently selected.
- AI suggestions remain editable and applying them uses ordinary recorded operations. Agent integrations require explicitly supported versions/protocols and metadata origins.

These are proposals synthesized from research, not closed architecture decisions. Resolve the corresponding map gates before dependent production work; validate uncertain behavior with the existing prototype task and actual reference observation.

## Testing Decisions

- The highest public workflow seam is the primary test surface, approved by the user. Observe files, index, refs/history, public service results and rendered/action-history state through public interfaces; avoid private helpers, internal schemas and implementation-shaped mocks.
- Use independently constructed fixture outcomes and Git's public interface as an oracle. Verify successes plus meaningful errors, input requests, conflicts, cancellation, external changes, slow output and hidden-panel continuation.
- Use deterministic provider/AI/agent adapters for repeatable error and protocol cases, then verify required supported services/agent versions with live controlled accounts and actual native hosts. Mock success alone cannot verify live integration parity.
- There is no prior application test suite in this repository. Start small at the agreed seam, one failing behavior and one implementation at a time. Add lower seams only when a demonstrated hard case needs them and the user confirms the change in test surface.
- Every feature slice includes UI routes, execution/service effects and relevant checks. Visual/reference-state comparison happens per slice when access exists; the final census catches omissions and is not a deferred styling phase.
- Keep documentation evidence, release evidence, reference observation, implementation and verification separate. A completed ticket or a passing mock does not automatically verify every grouped platform/provider subcase.
- Quantitative performance budgets, reference screenshots, required accessibility behavior and supported native/hosted version matrices remain acceptance decisions. Do not invent green results or fixed performance promises before measuring the agreed fixtures.

## Out of Scope

- Behavior introduced after the pinned 12.6.1 baseline; track it separately as the user requested.
- Recreating behavior proven removed from the pinned release solely because an older client or stale page describes it. Record the release evidence for removal.

No documented, paid-reference, connected, platform-specific or newly observed in-baseline capability is dropped by milestone ordering. The scope of external Code Review/Insights beyond desktop-required workflows, and equivalent billing/licensing behavior, remains explicitly gated rather than silently included, excluded or treated as implemented.

## Further Notes

This is the published implementation plan over an open decision map. Existing gates are retained: complete parity acceptance (G4), execution/coordination prototype (G5), equivalent service contract (G6), matching reference access (G7), and native/Git support (G8). The current installed reference is older, and Rust readiness is not build-verified. Those facts do not authorize downgrading the parity target.

The next actionable work remains the existing execution-contract prototype, accompanied by native support/tooling decisions and reference access. Resolve the applicable decision gates before starting dependent production work.

[Approved breakdown](../../docs/implementation-draft/breakdown.md), [coverage records](coverage.md), [decision map](../gitkraken-parity/map.md), [baseline](../../docs/project-start.md), [core audit](../../docs/research/core-parity.md), [connected audit](../../docs/research/connected-parity.md), [runtime research](../../docs/research/runtime-architecture.md), and [accepted direction](../../docs/adr/0001-desktop-and-service-direction.md).
