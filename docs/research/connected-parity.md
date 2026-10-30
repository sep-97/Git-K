# Connected feature parity research

Researched October 30, 2026. Reference release: GitKraken Desktop 12.6.1. This is a primary-documentation inventory, not proof of complete or implemented parity. No reference account, paid edition, provider credentials, production API calls, or live reference screenshots were used. Every feature below is **unimplemented** in this repository.

IDs are stable requirements handles, not implementation tickets. Group tables cite their supporting sources immediately above them. `P` means a third-party provider, `S` an equivalent self-hosted application service, `L` local application/Git state, `A` an AI endpoint, and `C` an installed coding-agent CLI. These dependency assignments are **engineering inferences**; they do not describe GitKraken's internal architecture. Documentation establishes observable requirements; exact layouts, disabled states, errors and permissions still require reference-build verification.

## Release evidence takes precedence over stale feature descriptions

The [12.x release notes](https://help.gitkraken.com/gitkraken-desktop/current/) explicitly record these baseline changes. The older GitHub integration and PR pages retain descriptions of in-app suggestions/review. For 12.6.1, the release notes establish a web-review handoff; verify residual controls in the actual build rather than recreating an obsolete review mode.

| ID | Documented release behavior | Dependency |
| --- | --- | --- |
| CON-REL-001 | Browser account sign-in/sign-up replaced the email/password form in 12.3.0. | S |
| CON-REL-002 | New account integration connections synchronize across devices; on-prem remains local. | S/P |
| CON-REL-003 | In-app PR review was replaced by gitkraken.dev Code Review links. | S/P |
| CON-REL-004 | Issue-launched agents receive issue context; unavailable when already branch-associated. | P/C/L |
| CON-REL-005 | Another agent may start in an occupied worktree using a new terminal. | C/L |
| CON-REL-006 | Jira project discovery supports 10,000 projects. | P |
| CON-REL-007 | Organization-configured AI keys remain hidden with an explanatory hover. | S/A |
| CON-REL-008 | Provider model lists update; PR dropdowns paginate and handle empty results. | P/A |

Validation: capture onboarding, web handoff, cross-device connection, paginated picker and existing-agent flows in 12.6.1. Preserve corrected behavior rather than historical bugs. Model names and account-plan appearance need a dated build capture.

## Hosting integrations

The [integration overview](https://help.gitkraken.com/gitkraken-desktop/integrations/) lists hosted and enterprise variants. Reference plan gates are evidence about GitKraken, not an approved monetization policy for this application. Full scope includes public/private and paid-reference capabilities.

GitHub.com behavior below comes from the [GitHub integration](https://help.gitkraken.com/gitkraken-desktop/github-gitkraken-desktop/). Multiple accounts use profiles; Community limits repository access to public repositories. The general source's review wording conflicts with CON-REL-003.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-001 | Connect GitHub through browser authentication or manual token. | P/S |
| CON-HOST-002 | Browse/clone repositories; create repositories with ignore/license options. | P/L |
| CON-HOST-003 | Generate/upload or select SSH keys; support SSH-agent overrides. | P/L |
| CON-HOST-004 | Discover forks, add remotes, create PRs and edit metadata. | P/L |

[GitHub Enterprise Server](https://help.gitkraken.com/gitkraken-desktop/github-enterprise/) requires a host/PAT, permits one account per profile, and documents Advanced-or-higher access plus a rolling server-version support window.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-005 | Connect a custom GitHub Enterprise host using a PAT. | P |
| CON-HOST-006 | Discover/create/clone repositories, fork, upload SSH keys, manage remotes/PRs. | P/L |

[GitLab.com](https://help.gitkraken.com/gitkraken-desktop/gitlab-gitkraken-desktop/) supports browser/manual OAuth and profile-isolated accounts; Community repository access is public-only.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-007 | Connect GitLab.com using browser or manual OAuth. | P/S |
| CON-HOST-008 | Discover/create/clone repos, upload SSH keys and manage fork remotes/MRs. | P/L |

[GitLab Self-Managed](https://help.gitkraken.com/gitkraken-desktop/gitlab-self-hosted/) uses a host/PAT with documented `api`/`read_user` scopes, Pro-or-higher access, profile isolation, and a rolling supported-version window. Provider requirements must be rechecked when implementing auth; the reference's token-expiration instructions are not a reusable provider policy.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-009 | Connect a GitLab Self-Managed host using a PAT. | P |
| CON-HOST-010 | Create/discover/clone repos, upload SSH keys, manage remotes and MRs. | P/L |

[Bitbucket Cloud](https://help.gitkraken.com/gitkraken-desktop/bitbucket/) supports browser/manual OAuth; reviewer visibility depends on repository permissions. Its SSH flow copies a key for manual upload.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-011 | Connect Bitbucket Cloud through browser/manual OAuth. | P/S |
| CON-HOST-012 | Discover/create/clone repos and fork remotes; copy generated SSH keys. | P/L |
| CON-HOST-013 | Create/view PRs with permission-dependent reviewer selection. | P |

[Bitbucket Data Center](https://help.gitkraken.com/gitkraken-desktop/bitbucket-data-center/) uses a host/PAT, profile-isolated accounts and Advanced-or-higher access; its supported-version window is rolling.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-014 | Connect a Bitbucket Data Center host using a PAT. | P |
| CON-HOST-015 | Discover/create/clone repos, copy SSH keys, manage fork remotes/PRs. | P/L |

[Azure DevOps](https://help.gitkraken.com/gitkraken-desktop/azure-devops/) documents OAuth/PAT, organization selection and Pro-or-higher access. PAT is the fallback when organization policy disables third-party OAuth. Server-hosted behavior additionally appears in the on-premise source below.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-016 | Connect through OAuth or a host/PAT; choose an organization. | P/S |
| CON-HOST-017 | Discover/create/clone repos; show remote avatars and fork options. | P/L |
| CON-HOST-018 | Copy generated SSH keys; create/view PRs and select reviewers. | P/L |

The [certificate guide](https://help.gitkraken.com/gitkraken-desktop/self-signed-certificates/) documents OS trust-store setup and Git Executable honoring global Git SSL settings. This is distinct from API-client TLS trust, which needs implementation validation.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-HOST-019 | Work with internally signed hosts after platform trust configuration. | P/L |
| CON-HOST-020 | Honor Git SSL configuration for Git-executable operations. | L |

Validation for each host: authorized/denied/revoked/expired account, missing scopes, empty/paginated repository and reviewer lists, public/private repos, forks, host aliases, separate profiles and API/Git authentication differences. Server fixtures need an explicit version matrix. Equivalent OAuth applications must use our registrations and callbacks; GitKraken account sessions are not ours.

## Issue trackers

[GitHub Issues](https://help.gitkraken.com/gitkraken-desktop/github-issues/) shares the GitHub connection, supports provider filter syntax and limits Community writes to public repositories.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ISS-001 | Show default issue filters, previews and editable detail views. | P |
| CON-ISS-002 | Create issues, custom filters and title-prefilled issue branches. | P/L |
| CON-ISS-003 | Open provider issue URLs or copy issue links. | P/L |

[GitHub Enterprise Issues](https://help.gitkraken.com/gitkraken-desktop/github-enterprise-issues/) shares the enterprise connection and inherits the GitHub issue workflow. Its minimum-version wording is older than the base integration's rolling window; test the selected server versions.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ISS-004 | Supply the issue workflow on the connected enterprise host. | P/L |

[GitLab Issues](https://help.gitkraken.com/gitkraken-desktop/gitlab-issues/) shares GitLab authentication and uses GitLab filters; Community access is public-repository-limited.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ISS-005 | Preview/edit/create issues, filter them and create title-prefilled branches. | P/L |
| CON-ISS-006 | Open GitLab issue URLs or copy links. | P/L |

[GitLab Self-Managed Issues](https://help.gitkraken.com/gitkraken-desktop/gitlab-self-managed-issues/) inherits those workflows and documents view-only Community access plus server requirements.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ISS-007 | Supply the issue workflow through the self-managed connection. | P/L |

[Jira](https://help.gitkraken.com/gitkraken-desktop/jira/) documents browser authorization, JQL, comments and synchronized edits. Community is view-only; writes/branching require a paid reference plan.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ISS-008 | Preview/create/edit issues, status, assignee and comments. | P |
| CON-ISS-009 | Filter with JQL, branch from issues and open/copy links. | P/L |

[Jira Data Center](https://help.gitkraken.com/gitkraken-desktop/jira-data-center/) uses host/PAT and the same issue functions; its guide specifies Advanced-or-higher, server 8.4+, and some unsupported SSO configurations.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ISS-010 | Connect Jira Data Center and supply the Jira issue workflow. | P/L |

[Trello](https://help.gitkraken.com/gitkraken-desktop/trello/) supplies card workflows in ISSUES. Community is view-only; writes and branching require a paid reference plan.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ISS-011 | Authorize Trello; preview/create/edit cards, lists, assignees and comments. | P |
| CON-ISS-012 | Use Trello filter syntax and title-prefilled card branches. | P/L |
| CON-ISS-013 | Synchronize card changes with the connected board. | P |

Validation: provider-specific fields, required-field errors, permissions, concurrent edits, filter syntax, branch-name collisions, and issue-to-branch linkage. Azure DevOps issues also appear in Launchpad documentation, but a dedicated Desktop issue-editor contract was not established by this audit; retain it as an evidence gap, not an exclusion.

## Pull requests and stacks

The [PR guide](https://help.gitkraken.com/gitkraken-desktop/pull-requests/) documents the following. Its broad review wording must be interpreted with CON-REL-003. Templates apply to GitHub/GitLab/Azure; stacks are GitHub.com-specific. Provider support varies rather than one universal PR implementation.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-PR-001 | Start PR creation from graph drag, context menu or sidebar. | P/L |
| CON-PR-002 | Require pushed branches; choose source/target repos and branches. | P/L |
| CON-PR-003 | Fill title/description and supported metadata/templates. | P |
| CON-PR-004 | View/filter PRs with CI/review indicators. | P |
| CON-PR-005 | Add GitHub PRs to an existing target-branch stack. | P/S |
| CON-PR-006 | Navigate stack positions and inspect graph stack hints. | P/S/L |
| CON-PR-007 | Offer GitHub merge commit, squash and rebase methods. | P |
| CON-PR-008 | Discover/add missing remotes for local PR inspection. | P/L |
| CON-PR-009 | Navigate build-status URLs and PR browser destinations. | P/L |
| CON-PR-010 | Preserve documented provider-specific MR/PR terminology and capabilities. | P |

The [GitHub integration](https://help.gitkraken.com/gitkraken-desktop/github-gitkraken-desktop/) also describes editing title, description, reviewers, assignees, milestone and labels, and applying/rejecting suggested changes. These residual controls need exact-build inspection because of the review-mode replacement. Keep these as subrequirements of CON-HOST-004/CON-PR-003, not assumed verified UI.

Validation: two/three-layer stacks, missing or merged targets, remote/fork PRs, protected-branch merge restrictions, draft/closed states, templates, missing remotes, metadata pagination and empty lists. The public PR APIs provide implementation primitives, not proof of GitKraken stack-storage semantics: [GitHub](https://docs.github.com/en/rest/pulls/pulls), [GitLab](https://docs.gitlab.com/api/merge_requests/), [Bitbucket](https://developer.atlassian.com/cloud/bitbucket/rest/api-group-pullrequests/), [Azure DevOps](https://learn.microsoft.com/en-us/rest/api/azure/devops/git/pull-requests?view=azure-devops-rest-7.1). GitHub metadata such as labels/assignees uses issue endpoints in addition to PR endpoints. The exact stack metadata contract remains unestablished.

## Workspaces and Insights entry points

The [workspace guide](https://help.gitkraken.com/gitkraken-desktop/workspaces/) establishes these behaviors. Cloud groups synchronize/share; local groups remain machine-local until conversion. Its cloud-dependent features are unavailable in GitKraken's own on-premise clients, which does not remove them from our requested equivalent.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-WS-001 | Create local groups from repositories, folders or VS Code workspaces. | L |
| CON-WS-002 | Synchronize local groups with a directory. | L |
| CON-WS-003 | Create cloud groups using connected hosting integrations. | P/S |
| CON-WS-004 | Share groups with selected teams/users across machines. | S |
| CON-WS-005 | Convert local groups into cloud groups. | P/S/L |
| CON-WS-006 | Select repositories for bulk clone/fetch/pull/open/locate/remove. | P/L |
| CON-WS-007 | Edit workspace name, icon, description, color and permissions. | S/L |
| CON-WS-008 | Hide/show groups and reorder by dragging. | S/L |
| CON-WS-009 | Open workspace PR activity in Launchpad. | P/S |
| CON-WS-010 | Launch workspace Insights in a browser. | P/S |

Validation: two users/two machines, revoked membership, local directory moves, mixed cloned/uncloned groups, partial bulk failures, conversion and ordering persistence. Workspace removal must be distinguished from deleting repository files.

The [Insights guide](https://help.gitkraken.com/gk-dev/gk-dev-insights/) describes an externally hosted add-on powered by GitClear, with access enabled by request. Desktop launch parity and full external analytics parity are distinct acceptance questions; neither is silently removed here.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-INS-001 | Connect GitHub/GitLab/Bitbucket/Azure/Jira data sources and import selected repositories. | P/S |
| CON-INS-002 | Track asynchronous import progress and historical activity. | P/S |
| CON-INS-003 | Configure personal role/timezone and additional Lead token connections. | P/S |
| CON-INS-004 | Connect Cursor/Copilot tokens for AI Impact metrics. | P/S |

The metric definitions, dashboard editor, GitClear processing algorithms and exact Desktop-accessible entitlement flows need a separate deep audit if equivalent external analytics is included. This report establishes the boundary and visible setup, not those algorithms.

## Launchpad

The [Launchpad guide](https://help.gitkraken.com/gitkraken-desktop/gitkraken-launchpad/) describes these surfaces. Team mode requires Advanced-or-higher in the reference; pin/snooze/saved views and cloud workspace contexts are unavailable in its on-premise variant.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-LP-001 | PR, Issues, WIPs, All and Snoozed tabs. | P/S/L |
| CON-LP-002 | Workspace/provider selection and integration connection prompts. | P/S |
| CON-LP-003 | Personal mode for author/assignee/reviewer/mentioned involvement. | P/S |
| CON-LP-004 | Team mode and collaborator filters. | P/S |
| CON-LP-005 | Pin and timed snooze/unsnooze items. | S |
| CON-LP-006 | Save/delete filtered views scoped to Personal/Team mode. | S |
| CON-LP-007 | PR columns show status, updates, authors, collaborators and repo/branch. | P/S |
| CON-LP-008 | Open/review/close PRs, inspect suggestions and check out branches/worktrees. | P/L/S |
| CON-LP-009 | Filter issues by source, ownership, mentions, labels, milestones/sprints. | P/S |
| CON-LP-010 | List local WIPs and open affected repositories. | L |
| CON-LP-011 | Status-bar PR summary opens matching activity groups. | P/S/L |

Validation: identity with no selected workspace, mixed-provider groups, empty/error/unauthenticated states, expiration of snoozes, mode-specific persistence and permissions. Exact status grouping/ranking rules are not fully defined by the guide.

## Shared patches

The [experimental feature guide](https://help.gitkraken.com/gitkraken-desktop/experimental-features/) documents Cloud Patches, their Git-executable prerequisite, and Desktop/GitLens/CLI sharing. The application needs its own equivalent links and interoperability decisions; claiming compatibility with vendor-private clients would exceed this evidence.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-PATCH-001 | Create shared patches from staged changes or existing commits. | L/S |
| CON-PATCH-002 | Copy links and find patches in the sidebar. | S/L |
| CON-PATCH-003 | Share by link, organization, or selected collaborators. | S |
| CON-PATCH-004 | Open patch links and apply to a selected branch. | S/L |
| CON-PATCH-005 | Delete patches through sidebar actions. | S |
| CON-PATCH-006 | Configure customer-owned S3 patch storage. | S |

Validation: recipient allowed/denied, repository permissions, unavailable/deleted patches, conflicting applies, rename/binary patches, storage outage, and link routing. Encryption and access policy are covered further in security controls below.

## Profiles, organizations and live collaboration

[Profiles](https://help.gitkraken.com/gitkraken-desktop/profiles/) isolate identities/preferences/integrations/tabs. Multiple profiles require a paid reference subscription. These are product behaviors; global Git identity changes require explicit UI intent.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-PROFILE-001 | Create/manage/switch profiles with author name/email. | L/S |
| CON-PROFILE-002 | Isolate integration accounts and general/UI preferences. | L/S |
| CON-PROFILE-003 | Keep profile-specific tabs; duplicate current tabs for new profiles. | L |
| CON-PROFILE-004 | Optionally synchronize profile identity to global Git config. | L |
| CON-PROFILE-005 | Show provider avatars or email-based Gravatar. | P/L |

[Organizations/teams](https://help.gitkraken.com/gitkraken-desktop/teams/) have plan-specific team/seat constraints and collaboration gates. The target's billing/licensing model is not yet chosen; reference billing screens remain part of the audit rather than authorization to charge users.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-ORG-001 | Invite organization members by identity/email. | S |
| CON-ORG-002 | Change member roles, billing contacts and remove members. | S |
| CON-ORG-003 | Create/rename/delete teams. | S |
| CON-ORG-004 | Add/remove team members. | S |
| CON-ORG-005 | Suggest repository contributors as prospective members. | S/L |
| CON-ORG-006 | Show seat availability and exhausted-seat purchase prompts. | S |

[Team View](https://help.gitkraken.com/gitkraken-desktop/team-view/) is separate from Launchpad Team mode. It shows live checked-out branches/local edits/activity, requires membership and Advanced-or-higher, and supports WIP-sharing opt-out.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-TEAM-001 | Choose repository/default team. | S/L |
| CON-TEAM-002 | Show teammates' current branches and uncommitted files. | S/L |
| CON-TEAM-003 | Show added/removed lines and potential overlap warnings. | S/L |
| CON-TEAM-004 | Set Active/Away presence. | S |
| CON-TEAM-005 | Disable sharing of personal work-in-progress. | S/L |

Validation: two desktops with simultaneous edits, disconnect/reconnect, stale presence, opt-out, removed membership and profile switching. Exact conflict-prediction algorithm, synchronization protocol and retention are undocumented in these sources.

## AI workflows

The [AI guide](https://help.gitkraken.com/gitkraken-desktop/gkd-gitkraken-ai/) documents paid-reference AI features; Composer/conflict assistance have preview behavior. Supported configurable providers include OpenAI, Azure, Anthropic, Google and a custom URL; on-premise uses the custom endpoint. This is workflow parity, not a promise of identical nondeterministic model output.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-AI-001 | Generate/edit commit summary and description from changes. | A/L |
| CON-AI-002 | Regenerate an amended latest-commit message. | A/L |
| CON-AI-003 | Explain selected commits and branch changes. | A/L |
| CON-AI-004 | Generate editable stash messages. | A/L |
| CON-AI-005 | Generate PR title/description using selected templates. | A/P/L |
| CON-AI-006 | Propose conflict resolution with explanations/confidence and accept/discard. | A/L |
| CON-AI-007 | Compose changes into proposed commits before applying. | A/L |
| CON-AI-008 | Recompose contiguous commits; reorder/squash/edit proposals. | A/L |
| CON-AI-009 | Preserve authors/coauthors; report real Composer progress. | A/L |
| CON-AI-010 | Configure provider/model/key/custom endpoint. | A/L/S |
| CON-AI-011 | Customize global and workflow-specific instructions. | A/L/S |

Validation: deterministic mock responses plus live selected providers, cancellation before application, invalid grouping, large/binary/generated-file changes, attribution preservation and edits before accepting output. Exact prompt templates, model inventory, token budgeting, credit calculation and failure messages need build/provider evidence. Hosted default AI requires an equivalent service or explicit provider configuration decision, not the GitKraken subscription backend.

## Coding agents

The [agent guide](https://help.gitkraken.com/gitkraken-desktop/agents/) documents Claude Code, Codex CLI, Copilot CLI, Cursor CLI, Gemini CLI and OpenCode. Live status integrations cover Claude/Codex/Copilot/OpenCode; inline permission responses cover Claude/Codex/Copilot. Other agents can run manually. The following is distinct from built-in AI assistance.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-AGENT-001 | Detect/configure installed CLIs and custom arguments. | C/L |
| CON-AGENT-002 | Configure repository-specific sequential setup commands. | L |
| CON-AGENT-003 | Start sessions with branch/base/agent selection. | C/L |
| CON-AGENT-004 | Validate generated names; search bases; offer quick-start defaults. | C/L |
| CON-AGENT-005 | Create worktrees, run setup and launch agent terminals. | C/L |
| CON-AGENT-006 | Inherit hidden/soloed references and collapsed source sections. | L |
| CON-AGENT-007 | Show WIP, ahead/behind, Running and Waiting status cards. | C/L |
| CON-AGENT-008 | Filter/sort sessions by status and branch. | C/L |
| CON-AGENT-009 | Allow/deny supported agent requests; other prompts use terminals. | C/L |
| CON-AGENT-010 | Open sessions/worktrees in current or new tabs. | C/L |
| CON-AGENT-011 | Lock/unlock/remove worktrees, optionally deleting branches. | L |
| CON-AGENT-012 | Install/reinstall/uninstall status plugins only on request. | C/L |
| CON-AGENT-013 | Display plugin availability and update installed integrations. | C/L |
| CON-AGENT-014 | Report metadata-only status without sending prompts/output/code. | C/L |
| CON-AGENT-015 | Indicate merged GitHub PRs on session cards. | P/L |

Validation: each supported CLI/version, setup failure, concurrent sessions, exit/crash/reconnect, waiting prompts, denied/dismissed permissions, plugin installation reversal and worktree cleanup. This audit did not establish the vendor `gk` event wire protocol or a compatible public integration SDK; equivalent adapters must be designed and verified against agent-owned interfaces. See CON-REL-004/005 for release-specific session behavior.

## Organization security and deployment variants

[Security controls](https://help.gitkraken.com/gk-dev/gk-dev-security-controls/) documents role-restricted AI access/provider policy, including enforced keys/URLs for supported providers, and encrypted patch storage. These are cloud organization settings; on-premise settings are local. Customer-owned S3 patch storage in the reference requires Business/Enterprise.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-SEC-001 | Organization roles can enable/disable AI features. | S/A |
| CON-SEC-002 | Restrict providers and enforce supported keys/endpoints. | S/A |
| CON-SEC-003 | Configure/test S3 bucket, region and credentials. | S |
| CON-SEC-004 | Encrypt patches and enforce link/repository access. | S |

Validation: member/admin boundaries, policy changes during a session, secret masking, denied providers, inaccessible storage and recipient authorization. Encryption format, key management, revocation and retention are not established here; design them explicitly for the equivalent service.

[Self-Hosted Server](https://help.gitkraken.com/gitkraken-desktop/self-hosted/) describes internal authentication/license management, email/LDAP and version control. Its [Serverless variant](https://help.gitkraken.com/gitkraken-desktop/serverless/) has no account/server requirement and loads/updates a local license file. Their Desktop feature sets are documented as equal, with GitKraken-cloud-dependent functions unavailable.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-DEP-001 | Internal server authentication, including email/LDAP. | S |
| CON-DEP-002 | Central license/client-version administration. | S |
| CON-DEP-003 | Offline standalone startup and local license update. | L |
| CON-DEP-004 | On-premise Launchpad uses enterprise provider connections. | P/L |

**Product decision required:** self-hosted-first for our service is not the same as copying GitKraken's cloud-disabled edition. The full requested feature union needs equivalent shared-state services, including cloud-group/Team Launchpad behavior. Licensing/billing/SSO/LDAP semantics and whether standalone deployment is a separately exposed product mode need explicit decisions and deeper reference access.

## Current CI management behavior

The [GitHub Actions guide](https://help.gitkraken.com/gitkraken-desktop/github-actions/) says its Left Panel workflow manager was removed in 11.10. Baseline behavior is editing, staging and committing workflow YAML, not restoring that obsolete tab.

| ID | Documented behavior | Dependency |
| --- | --- | --- |
| CON-CI-001 | Create/edit/delete Actions YAML under .github/workflows and commit changes. | L/P |

## Proposed equivalent service boundaries

These are proposals inferred from the observable requirements, not descriptions of GitKraken internals or settled implementation choices:

1. **Identity and organization service:** browser authentication, devices, invitations, roles, teams, organization policies and optional enterprise identity integration. Keep a local profile distinct from a shared account.
2. **Provider connection adapters:** explicit host/account identity, provider credentials, capabilities, pagination/rate limits, error states and reconnect flows. Local Git credentials and API permissions are separate capabilities. Use documented provider APIs rather than pretending these actions are Git subprocesses.
3. **Shared workspace and activity service:** workspace permissions/order/settings, Launchpad personal preferences, shared activity and optional live WIP/presence. Decide which provider data is queried live, cached or indexed.
4. **Patch service:** metadata, encrypted object storage, recipient authorization, revocation and application links. Customer-owned storage alone does not replace identity/access services.
5. **AI gateway/adapters:** configurable endpoints/providers, organization policy, request progress and optional usage accounting. Model generations are suggestions; applying them invokes ordinary recorded repository operations.
6. **Local agent adapter host:** CLI launch, worktree lifecycle, event normalization and permission responses. Remote shared services should not be required merely to run a local terminal session.
7. **Web destinations:** Code Review and Insights need a product boundary decision: reproduce their Desktop entry points with equivalent hosted views, and define any full external-product parity scope explicitly. No public drop-in GitKraken cloud API contract was established in this investigation.

The user's always-record requirement is additional to the reference: application requests need safe, understandable activity records, including provider target/action/outcome and redacted parameters. A collapse/expand preference must not stop those records. API calls should not be labeled invented `git` commands. Exact payload logging, retention and export policy remain design work.

## Open evidence and human decisions

- A real 12.6.1 build with suitable entitlements is needed to establish final tabs, exact fields, menus, layouts, interaction states, accessibility, shortcuts and service/account errors. Documentation dates and release entries conflict in places.
- The user's full-feature requirement retains paid-reference capabilities. Establish adequate reference entitlement access and record the feature union without substituting the reduced Community or on-premise set.
- Define the scope of browser destinations: Desktop handoff parity, equivalent Code Review, and equivalent Insights analytics are different deliverables; none has been discarded.
- Select self-hosted identity/deployment requirements, account/profile synchronization and organization structure. Decide whether vendor-like plan/seat/billing flows are functional product requirements or reference affordances to redesign.
- Select AI defaults and provider test accounts, organization usage policy, supported agent versions and adapter/event contracts. No provider accounts were accessed or secrets requested by this research.
- Deep-audit provider-specific issue/PR fields, custom filters, Azure work items, Git Integration for Jira links, SSO/account administration, Insights dashboards, web review/suggestions, deep links and notification preferences before claiming the inventory is exhaustive.
- The one-year server support windows and current provider model catalogs are moving documentation claims; freeze an actual compatibility matrix for the pinned release's tests.

## Research completion criteria

This investigation establishes a broad, source-linked connected-feature inventory and concrete service/acceptance questions. It does not settle the human decision tickets, prove vendor-backend compatibility, or claim exhaustive field-level enumeration. Implementation status is uniformly unimplemented. Follow up by capturing the reference UI, expanding each grouped requirement into stateful acceptance cases, and assigning every verified action a runtime command/API activity representation.
