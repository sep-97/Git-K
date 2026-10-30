# Git-K execution prototype

Throwaway Tauri + Rust + React prototype. `prototype/execution-history` captures the original read-only build; `prototype/file-staging` extends it with index mutations. It explores whether the actual Git execution record can remain independent of a visible or collapsed interface. It is not the production application or proof of GitKraken feature parity.

## What you can do

- Open a local repository through a folder picker or path field.
- Inspect actual branches, a graph of up to 60 loaded commits, working changes, commit details and file diffs.
- Inspect Staged and Unstaged files separately, including both sides of partially staged files.
- Stage or unstage one file or all available files. These actions update the actual repository index and preserve working files. Conflict resolution is unavailable in this prototype.
- Expand an action-history entry to inspect its command, working directory, captured output and exit status.
- Run eight real status checks one second apart, collapse the activity panel, then reopen it to inspect the captured operations.
- Stop scheduling remaining checks, or run a deliberately missing-reference lookup to inspect an actual Git failure.
- Create a new disposable demo with a merge history, a modified file and an untracked file. Repository inspection is read-only; the staging controls explicitly modify the selected repository's index.

## Run from source

With Node, Git, a Rust MSVC toolchain and the native Tauri prerequisites available:

```powershell
npm ci
npm run tauri -- dev
```

To build an executable with embedded frontend assets:

```powershell
npm run tauri -- build --debug --no-bundle
```

The executable needs Git and the WebView2 Runtime. Rust and Node are build prerequisites, not requirements for launching the built executable. Native support and distribution decisions remain open for production.

## Guided walkthrough

1. Choose **Create demo workspace**. Inspect its two local branches, four commits, merge and two working changes.
2. Select a changed file and then a commit. Compare the inspector with the corresponding recorded commands.
3. Choose **Run 8 live checks**, collapse **Action history**, wait until the checks finish and reopen it. All eight real status commands should remain inspectable.
4. Choose **Show a real failure**. The missing-reference command must report its real nonzero exit and error text; it must not appear successful.
5. Open another repository or an empty initialized repository and inspect its real data. Folder-picker cancellation should leave the current view unchanged.
6. In **Working changes**, stage a file and inspect its Staged diff. Unstage it and verify its working contents remain. Try **Stage all** and **Unstage all**, including with the activity panel collapsed.

## Deliberate limits

- History is in memory for the current process. Restart recovery, durable storage, retention, output bounds, comprehensive credential redaction and export policy are not settled here.
- The activity panel displays the latest 150 command entries; the process retains the session event stream. The inspector displays at most 600 lines of a selected output.
- The command display is formatted for PowerShell on Windows and POSIX shells elsewhere; Rust executes the original argument vector directly.
- This is an application command record, not an interactive terminal. Arbitrary commands, PTY sessions, committing user repositories, remotes and provider APIs are not implemented.
- Staging uses system Git, so configured attributes and clean filters apply. App index mutations are serialized, but coordination with external Git processes, sparse checkout and submodule staging remain outside validation. Git's index-lock errors are shown without removing its lock files.
- Stopping the walkthrough stops future scheduled reads; it is not a claim of full child-process-tree cancellation.
- Native filename handling in this prototype uses UTF-8 text conversion; complete non-Unicode round-tripping, repository concurrency, hook/editor/auth interactions and cross-platform behavior remain unvalidated.
- User review is still required before the production execution contract can be resolved. The first production slice remains gated.

No automated test suite is added to this throwaway prototype. Verification consists of frontend/native builds and the actual UI walkthroughs recorded in the prototype notes.

See [validation results](VALIDATION.md) for the Windows build and observed workflow evidence.
See [file staging scope and validation](STAGING.md) for the next prototype feature.
