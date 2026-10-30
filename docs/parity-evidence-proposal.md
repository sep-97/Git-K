# Proposal: evidence required for feature parity

Status: proposed input to the acceptance decision; not approved acceptance criteria.

## Coverage record

Each discovered feature should retain a stable identifier and a source. Track these dimensions independently:

| Dimension | What it establishes |
| --- | --- |
| Documentation evidence | A specific first-party page describes the behavior |
| Release evidence | The behavior belongs to the pinned reference release |
| Reference observation | The matching reference application exposes the behavior in the recorded state |
| Implementation | Our application performs the behavior |
| Verification | Recorded checks compare outcomes and interactions against the reference |

Documentation evidence alone does not establish exact visual parity or prove the absence of undocumented controls. A visible control with placeholder behavior is not a completed feature.

## Reference observations

For each screen or action being compared, retain reference version, operating system, edition/account capabilities, window size, scaling, theme, relevant preferences, repository fixture and starting state. Compare the corresponding resulting state in our application.

Include menus, context menus, toolbar dropdowns, tooltips, shortcuts, selection, dragging, disabled controls, empty/loading/error states and persisted settings. Record undocumented behavior and documentation conflicts as open evidence gaps instead of guessing silently.

A matching reference build and adequate account access are prerequisites for verifying all observable functionality. An older installed version may help with navigation research but cannot close 12.6.1 parity checks.

## Proposed repository fixtures

Use isolated fixture repositories for behavior verification, including:

- An unborn branch, a normal linear history, branching and merge histories, detached HEAD, tags and stashes.
- Tracked/untracked/ignored files; staged and unstaged portions of the same file; renames, binary files and unusual filenames.
- Clean and conflicted merges, rebases and cherry-picks, including interrupted operations and recovery.
- Linked worktrees sharing repository metadata, submodules, shallow history and a large history suitable for profiling.
- Local test remotes for fetch/pull/push and failures; separately verified authenticated provider workflows.
- Hooks and signing configurations, including failure paths that must remain visible to the user.

These are proposed validation inputs, not a claim that every scenario has been audited or implemented. Architecture-specific fixtures and quantitative performance budgets remain decisions for the acceptance ticket.

## Execution transparency checks

For an application-driven Git action, verify the actual executable, arguments and working directory against the recorded command, with secrets redacted. The action must execute once; recording must not execute it again. Verify input requests, stdout, stderr, exit status and partial outcomes for multi-step operations.

Collapse and reopen the panel during an operation: execution and recording must continue. Background refreshes may be grouped for readability while their execution remains inspectable. Check cancellation, authentication failures, conflicts and application restart behavior according to the eventual execution contract.

Provider operations need truthful activity entries describing API actions. They must not display invented Git CLI commands. Interactive shell output and application execution records should have distinguishable origins.

## Connected-service verification

Use deterministic provider simulations for error and pagination cases, plus live end-to-end evidence for supported providers when credentials and appropriate test repositories are available. Validate multi-user sharing, authorization boundaries and cross-machine persistence against the chosen service contract.

Track unavailable credentials, paid capabilities, provider restrictions and missing reference access as unmet verification prerequisites. Their associated features remain in scope.

## Completion accounting

Report audited, implemented and verified feature counts separately. New discoveries extend the inventory. A parity claim requires every in-scope feature to have satisfactory evidence and no unresolved exceptions hidden by aggregate percentages.
