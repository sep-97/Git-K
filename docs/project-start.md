# Git manager project start

Status: wayfinding map charted and research in progress; application implementation has not started.
Recorded: October 30, 2026 (Asia/Tehran).

## Confirmed requirements

- Build a desktop Git manager using Tauri, Rust, and React/TypeScript.
- Target every feature, tab, and function of GitKraken Desktop 12.6.1, with a closely matching user experience. Track later releases separately.
- Show the real commands and results behind graphical Git actions so users can understand what happens.
- Track planning and implementation work in local Markdown files.
- Use the default triage labels: needs-triage, needs-info, ready-for-agent, ready-for-human, wontfix.
- Validate Windows first, followed by macOS and Linux; all three platforms remain in scope.
- Build equivalent connected services for self-hosting first.
- Allow the terminal/activity panel to collapse while always recording application actions. Git actions show real commands; provider actions show API activity.

## Version baseline

The official release notes currently list **12.6.1**, released **October 20, 2026**. The user confirmed this fixed reference release; exact reference edition and visual-comparison access still need to be established. The official download page still advertises 12.5.0, so it is not authoritative for this baseline. [Release notes](https://help.gitkraken.com/gitkraken-desktop/current/), [download page](https://gitkraken.com/download).

Version 12.6.1 includes terminal tab navigation and reordering, worktree display improvements, additional agents in an existing worktree, and rename handling fixes. Its preceding 12.6.0 release introduced stacked GitHub pull requests and multiple terminal tabs. These are part of the baseline to investigate. [Release notes](https://help.gitkraken.com/gitkraken-desktop/current/).

## Initial coverage inventory

This is an index for the detailed audit, not an exhaustive specification. Every area below is unimplemented. Each must later expand into individual controls, actions, state transitions, shortcuts, settings, error handling, provider variations, and acceptance scenarios. No area is excluded by its position in this table.

| Area | Coverage to audit | Primary reference |
| --- | --- | --- |
| Repository interface | Tabs, toolbar, left panel, graph, commit panel, status bar, columns, selection | [Interface](https://help.gitkraken.com/gitkraken-desktop/interface/) |
| Repository management | Opening, cloning, initialization, recents, favorites | [Documentation index](https://help.gitkraken.com/gitkraken-desktop/interface/) |
| File and commit workflows | Staging, commits, diffs, history, blame, editing, stashes, search | [Documentation index](https://help.gitkraken.com/gitkraken-desktop/interface/) |
| History and references | Branches, tags, merge, rebase, cherry-pick, squash, undo/redo, hide/solo | [Documentation index](https://help.gitkraken.com/gitkraken-desktop/interface/) |
| Extended Git workflows | Worktrees, submodules, Gitflow, LFS, hooks, signing, WSL | [Documentation index](https://help.gitkraken.com/gitkraken-desktop/interface/) |
| Terminal | Repository/worktree sessions, multiple tabs, autocomplete, shell selection, appearance, minimizing and termination | [Terminal](https://help.gitkraken.com/gitkraken-desktop/terminal/) |
| Preferences | General settings, profiles, SSH, external tools, UI customization, signing, repository settings, experimental features | [Preferences](https://help.gitkraken.com/gitkraken-desktop/preferences/) |
| Git hosting | GitHub, GitLab, Bitbucket, Azure DevOps and documented self-hosted variants | [Integrations](https://help.gitkraken.com/gitkraken-desktop/integrations/) |
| Issue tracking | GitHub/GitLab issues, Jira and Trello, including documented hosted/server variants | [Integrations](https://help.gitkraken.com/gitkraken-desktop/integrations/) |
| Workspaces | Local/cloud groups, bulk actions, sharing, conversion, Insights entry points | [Workspaces](https://help.gitkraken.com/gitkraken-desktop/workspaces/) |
| Launchpad | Individual/team workflows and connected-provider behavior | [Launchpad](https://help.gitkraken.com/gitkraken-desktop/gitkraken-launchpad/) |
| AI | Supported assistance, provider settings, organization behavior | [AI](https://help.gitkraken.com/gitkraken-desktop/gkd-gitkraken-ai/) |
| Agent workflows | Sessions, worktrees, terminal integration, status reporting and controls | [Agents](https://help.gitkraken.com/gitkraken-desktop/agents/) |
| Deployment variants | Hosted versus on-premise behavior and related settings | [Documentation index](https://help.gitkraken.com/gitkraken-desktop/interface/) |

Cloud workspaces involve sharing and cross-machine state. Reproducing that behavior will require a service design in addition to the desktop application; this is an engineering inference from the documented behavior, not a decision to remove it. [Workspaces](https://help.gitkraken.com/gitkraken-desktop/workspaces/).

## Proposed execution architecture

These are design proposals to validate during wayfinding:

- The Rust application owns repository state and execution. The React interface invokes typed application operations through Tauri.
- An operation can contain several Git commands, a provider request, or a local application change. User intent and individual execution steps have separate identities.
- Git subprocess steps retain executable, arguments, working directory, start/end times, stdout/stderr, and exit status. Display strings derive from executed arguments, with credential values redacted.
- A dedicated command history panel shows application-driven execution. Interactive shell sessions use PTYs and remain distinct from the application's command runner.
- Provider actions such as creating a pull request show an understandable API activity entry. They must not be represented as invented Git commands.
- Graphical and terminal-driven changes trigger repository reconciliation. Reads and writes need explicit concurrency rules, especially across worktrees sharing repository metadata.
- Operations needing user input, authentication, conflict resolution, or recovery expose real states in the interface. Cancellation behavior must account for Git operations that have already changed state.

## Environment observations

- This workspace initially contained only work/ and outputs/; no application, Git repository, remote, or agent configuration was present.
- Git, Node and npm are on PATH.
- rustc, cargo and gh are not on the current PATH. This does not prove they are absent from the computer. Rust toolchain and Windows native build prerequisites must be verified before attempting a desktop build.
- No builds or application tests have run because application code has not been created.

## Decisions to settle

1. Reference edition and evidence for validating screens and interactions.
2. Architecture-specific and platform-specific acceptance requirements within the confirmed platform order.
3. Equivalent service and provider boundaries for account, organization, cloud, AI and agent capabilities.
4. Truthful execution, coordination and recovery semantics behind action history and the interactive terminal.
5. Acceptance evidence for feature behavior, visual fidelity, accessibility, performance and recovery.

## Workflow

The invoked ask-matt router directs a project of this size to setup, then wayfinder, followed by a specification, dependency-linked tickets, and implementation. The user approved the setup draft, selected AGENTS.md, and confirmed local Markdown tickets and default triage labels. Configuration has been applied in this repository. The feature inventory remains preliminary until the decision map and detailed audit resolve the unknowns.

The user confirmed the first decision round on October 30, 2026: pin 12.6.1, Windows first, self-hosted services first, and a collapsible panel with continuous recording. Continue through the [wayfinding map](../.scratch/gitkraken-parity/map.md). Recommendations in later research remain proposals until their decision tickets are resolved.
