# Detached HEAD prototype

Status: implemented; real-Git and browser checks passed; native walkthrough and user design review pending
Assignee: /root
Base: `58e8b49`

## Scope

Select a history commit and choose **Check out commit…**. Review its full object ID and the detached-HEAD warning, then confirm **Check out detached HEAD**, Cancel or press Escape. The command uses the reviewed immutable object ID and Git's normal file-protection rules. Amend, conflicts, unfinished operations and stale branch/HEAD state prevent checkout.

A persistent banner shows the detached commit ID and **Create branch at HEAD**. Existing staged-commit controls also work while detached. New commits remain visible through the existing `git log --all` query, which already includes HEAD; the new regression confirms this behavior. Creating and switching to a branch preserves those commits. Returning to an existing branch shows the previous detached ID and a recovery command. Leaving detached HEAD does not immediately delete Git objects; recovery depends on retained objects and reflog expiration.

The operation continues to record real commands independently of the activity panel. Git refusal refreshes the actual state and retains the dialog error. A post-checkout hook can fail after HEAD moves; the refreshed view reflects this state. A successful checkout followed by failed reads reports success separately, closes its dialog and prevents more mutations until the repository is reopened.

Production issue 014 remains blocked by its existing decision and implementation gates. This increment does not resolve the production execution contract or claim reference interaction parity.

## Validation

Observed on Linux, November 25, 2026 (Asia/Tehran):

- `npm ci --ignore-scripts` and `npm run build` passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --locked -- --test-threads=1` passed: **10 tests, 0 failures**. Six new real-Git regressions cover checkout with partially staged/working/untracked contents preserved; detached commit creation, graph visibility, branch preservation and return; stale source/invalid target/unfinished-operation refusal; tracked, untracked and ignored collisions; reflog recovery; post-checkout hook errors and completed writes followed by failed reads. Four existing reference regressions also pass. Windows-only PowerShell tests were not run.
- Headless Chromium smoke checks passed for Cancel/Escape with no mutation request, collapsed-panel checkout, exact reviewed target/source values, banner visibility, branch creation at detached HEAD, checkout failure and post-success refresh failure. This check used a stubbed Tauri IPC bridge and is UI evidence only; it does not establish a native complete workflow. The temporary check is `/tmp/git-k-detached-ui.cjs`; screenshot `/tmp/git-k-detached-ui.png` shows fixture data.
- `git diff --check` passed. Standards review checked literal arguments, existing mutation serialization, read/write outcome reporting and default desktop feature behavior. Spec review checked the bounded prototype scope against production issue 014; full reference entry points, native behavior and production acceptance remain unverified.

The first new checkout regression failed with `Unsupported local reference action` before the operation was added. The existing refresh-failure fixture initially failed on Linux because its hook lacked execute permission; the fixture helper now sets that permission on Unix so the hook actually runs.

The optional `desktop` Cargo feature remains enabled by default. Disabling default features runs the same synchronous Git operation functions used by the native command wrappers without requiring a webview. Native build settings and normal Tauri commands retain the desktop dependencies. Commit and branch commands were extracted into these shared synchronous functions for real-Git workflow testing.

Rust was installed in the task user's directory for these checks. This environment lacks `webkit2gtk-4.1` and `rsvg2`, so a native Linux build/walkthrough could not be completed. Windows native validation also remains pending for this increment. Previous Windows results apply only to the earlier prototype versions.

## Next work

Run the native walkthrough on Windows: checkout a historical commit with the activity panel collapsed, commit staged work while detached, create a branch at the new HEAD, then return to the original branch and confirm both histories and local files. Check Cancel/Escape and a genuine overwrite refusal. Keep the production gates open until their own decisions and evidence are complete. The following planned feature area is merge and conflict handling (items 015–016).
