# Git-K execution prototype

Throwaway Tauri + Rust + React prototype. `prototype/execution-history` captures the original read-only build; `prototype/file-staging` adds index mutations; `prototype/commit-workflow` adds commit creation; `prototype/amend-workflow` adds latest-commit updates; `prototype/branch-workflow` adds local branch creation and checkout. It explores whether the actual Git execution record can remain independent of a visible or collapsed interface. It is not the production application or proof of GitKraken feature parity.

## What you can do

- Open a local repository through a folder picker or path field.
- Inspect actual branches, a graph of up to 60 loaded commits, working changes, commit details and file diffs.
- Inspect Staged and Unstaged files separately, including both sides of partially staged files.
- Stage or unstage one file or all available files. These actions update the actual repository index and preserve working files. Conflict resolution is unavailable in this prototype.
- Enter a commit summary and optional description, then commit staged contents with the repository's configured identity, hooks and signing. A failed commit retains the draft and refreshes repository state.
- Choose **Amend latest** to load the latest message. Edit the message with or without staged changes, review the target and rewrite warning, then choose **Update latest commit**. **Cancel amend** (or Escape inside the composer) restores your previous new-commit draft and leaves Git state in place.
- Choose **New branch** to create at current HEAD, or select a history commit and choose **Create branch here** in the inspector. Enter a literal name, optionally switch to it, or cancel without a Git write.
- Use **Checkout** beside a local branch to switch. Compatible changes can follow; Git refuses overwriting tracked/untracked/ignored local files or checking out a branch used by another worktree. Branch changes are disabled during amend.
- Choose **⋯** beside a branch to rename it or review its deletion. Deletion uses Git's normal merged-branch requirement and does not force removal. Switch away from the current branch before deleting it; branches used by another worktree are protected.
- Choose **New tag** at HEAD or **Create tag here** on a selected history commit. Create an unsigned lightweight tag or add an annotation using configured identity/signing. Select a tag to inspect its actual object and annotation. The **×** beside a tag opens an explicit local deletion review dialog; Cancel/Escape leaves it intact.
- Select a history commit and choose **Checkout commit…** to review and enter detached HEAD. Its banner shows the current tip and **Preserve on new branch**. Commit staged work while detached, then preserve it or return using a branch's **Checkout** button.
- Expand an action-history entry to inspect its command, working directory, captured output and exit status.
- Run eight real status checks one second apart, collapse the activity panel, then reopen it to inspect the captured operations.
- Stop scheduling remaining checks, or run a deliberately missing-reference lookup to inspect an actual Git failure.
- Create a new disposable demo with a merge history, a modified file and an untracked file. New demos have a local demo identity and unsigned commits. Staging modifies the selected repository's index; committing writes a real commit.

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
7. Stage the demo theme, enter a summary and description, then choose **Commit staged**. Inspect the new history entry and recorded commit command. The untracked notes file remains outside the commit. Commit controls perform real writes in whichever repository is open.
8. Choose **Working changes → Amend latest**. Edit the latest summary, then update it with no staged files. The commit is replaced and history refreshes. Stage a file first to include its index contents in the amendment. Cancel restores the new-commit draft; it does not undo staging performed while editing.
9. Choose **New branch**, enter `feature/my-change`, and create it with checkout selected. The sidebar and current branch refresh. Select a different commit and choose **Create branch here**, unchecking **Switch to the new branch** to create a reference without changing the working tree. Existing branches have explicit **Checkout** buttons.
10. Choose **⋯** beside a branch, enter a different name and rename it. For an inactive branch, **Delete branch…** opens a separate review before **Delete local branch**. Git refuses unmerged branches and branches used by another worktree.
11. Choose **New tag**, enter a name and optionally enable **Annotated tag** with a message. Select the created tag to inspect its object. Its **×** opens local deletion review; remote tags are not changed.

## Deliberate limits

- History is in memory for the current process. Restart recovery, durable storage, retention, output bounds, comprehensive credential redaction and export policy are not settled here.
- The activity panel displays the latest 150 command entries; the process retains the session event stream. The inspector displays at most 600 lines of a selected output.
- The command display is formatted for PowerShell on Windows and POSIX shells elsewhere; Rust executes the original argument vector directly. Windows displays use `Start-Process` with one CRT-escaped argument string and preserve `LASTEXITCODE`, avoiding Windows PowerShell 5.1 native-argument loss for quotes, empty values and trailing backslashes. Expand an entry to inspect the full recorded command.
- This is an application command record, not an interactive terminal. Arbitrary commands, PTY sessions, merge/rebase/cherry-pick/revert continuation, remotes and provider APIs are not implemented.
- Staging uses system Git, so configured attributes and clean filters apply. App index mutations are serialized, but coordination with external Git processes, sparse checkout and submodule staging remain outside validation. Git's index-lock errors are shown without removing its lock files.
- Stopping the walkthrough stops future scheduled reads; it is not a claim of full child-process-tree cancellation.
- Native filename handling in this prototype uses UTF-8 text conversion; complete non-Unicode round-tripping, repository concurrency, hook/editor/auth interactions and cross-platform behavior remain unvalidated.
- Commit commands use literal argument values. Normal noninteractive hook rejection and signing failure are covered; interactive prompts are not. The history records the parent Git process and its output, not every hook subprocess. Branch/HEAD preflight checks do not prevent external index changes between review and commit.
- Amend rewrites the latest commit and includes all staged contents. It preserves Git's default original-author behavior. Publication state is not detected: the rewrite warning also applies to already-pushed commits. Empty-commit amendments, unusual message whitespace/encoding, interactive prompts and external-process coordination remain outside this validation.
- Branch creation requires an existing commit. Force/bulk deletion, tracking/remote branches, graph context/double-click entry points remain ahead. Detached checkout is covered in DETACHED.md. Checkout never forces or automatically stashes changes and does not recurse into submodules. Git's configured hooks/filters can still change state; post-checkout failure does not imply rollback. External changes between preflight and mutation remain possible.
- Tags are local only. Tag push, remote deletion, move/rename, signing setup, tag filters and full reference interaction parity remain ahead. Annotation signing honors existing configuration and can fail if signing needs unavailable interaction. Branch rename/delete compares the selected object before Git runs; tag deletion also uses an atomic expected-object comparison. These checks do not establish complete external-process coordination.
- User review is still required before the production execution contract can be resolved. The first production slice remains gated.

The local reference extension includes nine real-Git regression scenarios and two Windows PowerShell replay regressions in `src-tauri/src/ref_tests.rs`. Run `cargo test --manifest-path src-tauri/Cargo.toml -- --test-threads=1` with the same Rust/MSVC environment as the native build. Fixtures remain under the temporary `git-k-reference-tests` directory, or `GIT_K_REF_TEST_ROOT` when set. Earlier prototype workflows still rely on frontend/native builds and the UI walkthroughs recorded in their notes; a comprehensive automated suite remains ahead.

See [validation results](VALIDATION.md) for the Windows build and observed workflow evidence.
See [file staging scope and validation](STAGING.md) for the next prototype feature.
See [commit workflow scope and validation](COMMITS.md) for commit creation and failure handling.
See [amend workflow scope and validation](AMEND.md) for latest-message updates, staged amendments and cancellation.
See [branch workflow scope and validation](BRANCHES.md) for creation, checkout and preservation/failure behavior.
See [local reference scope and validation](REFERENCES.md) for branch rename/delete, tags and the real-Git regression checks.

See [detached HEAD scope and validation](DETACHED.md) for checkout, committing, preservation and recovery context.
