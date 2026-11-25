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

## Remote reconciliation — November 25, 2026

Fetched `prototype/detached-head` at `2375eea` and combined its five detached regression scenarios, explicit HEAD history query, no-guess checkout and source/recovery evidence with the local headless test support and application-command workflow checks. All 15 Linux real-Git tests and the frontend build passed after reconciliation. The retained remote hook tests now use the Unix-executable fixture helper. Native evidence below remains scoped to its original source.

## Earlier Windows evidence retained from the remote branch

The following records belong to remote checkpoint `2375eea` and its predecessor `774447b`, fetched from `prototype/detached-head`. They predate this workspace’s UI and headless-test changes. They remain evidence for that earlier source; current native validation is still pending.

## Detached HEAD prototype

Status: implemented and validated within the scope below; user design review pending
Assignee: /root
Source branch: `prototype/detached-head`
Base: `58e8b49`

### Scope

Select a history commit and choose **Checkout commit…**. Review its full object ID and current tip before entering detached HEAD. Cancel/Escape uses the existing dialog cancellation path without invoking Git. The native runner uses a validated object ID, the shared mutation lock and branch/HEAD/conflict/unfinished-operation preflight. Checkout never forces, stashes, overwrites ignored files or recurses into submodules.

A persistent detached banner shows the current tip, branch-preservation guidance and reflog recovery context. **Preserve on new branch** opens the existing branch dialog at HEAD. Existing staging, commit and amend workflows work while detached. History now includes HEAD explicitly alongside all references, so detached commits appear within the existing 60-commit limit. Returning to a branch includes the old detached tip in the success notice. Leaving a tip does not immediately delete its objects; reflog expiry and garbage collection still limit eventual recovery.

This increment continues the claimed execution prototype decision. Production issue 014 and all decision gates remain blocked; full graph menus, reflog browsing, arbitrary SHA entry, complete cross-platform/reference parity and production coordination remain ahead.

### Validation — November 12, 2026

Windows TypeScript/Vite build and native Tauri debug build passed. The real-Git regression runner passed **11 tests, 0 failures**, including all six previous reference/PowerShell regressions and five detached scenarios against the exact operation function invoked by `edit_reference`:

- Detached checkout preserves staged contents, newer working contents and an untracked sibling. A subsequent real Git commit stays detached, leaves main unchanged and appears in the loaded snapshot. Creating a branch preserves the tip.
- Dirty checkout collisions, invalid/missing objects, stale source branches and unfinished merge state refuse checkout and preserve state.
- A post-checkout hook failure is reported even though HEAD has detached. A successful checkout followed by read failure returns mutation success separately from refresh failure.
- Untracked and ignored-file checkout collisions both refuse the switch, preserving the source branch, HEAD, staged contents, working contents and untracked siblings.
- A valid blob object ID is refused as a checkout target. Moving HEAD after review also refuses checkout while retaining the externally created commit and local files.

Native UI walkthrough on a disposable demo checked the actual checkout review, detached banner and full tip, staged one file, created `Preserve detached UI work` through the composer and preserved it through **Preserve on new branch → Create and checkout**. Independent Git inspection confirmed the new branch pointed to `241827a1b9be893d0f5faa1cd2003b069ccffe52`, main remained at `1435817b1ef1e7a287019d748c9055c8dc654c90`, and the untracked notes file remained. The graph loaded five commits including the detached tip. Returning to main succeeded while the preserved branch retained the new commit. The panel stayed collapsed while its count advanced from 23 to 112 during checkout, staging, commit, branch preservation and return.

The checkout dialog and banner fit the default window size. Tests use direct Git commits/branch preservation to arrange and inspect fixture state; the native walkthrough covers the application composer and branch commands.

#### Completion review and repeat checks — November 12, 2026

The original nine tests passed again before adding the two refusal regressions above; the expanded eleven-test run passed. `npm run build`, the native Tauri debug build and `git diff --check` passed.

A fresh native demo (`demo-1794481487589`) supplied the missing cancellation and stale-dialog checks. Selecting its tip recorded command 24. Opening checkout review and cancelling with Escape, then reopening and clicking Cancel, left that count at 24; independent Git inspection showed branch `main`, HEAD `afbc7802c57d9a7bca77ed53e7cb66c9d3c76b51`, modified `src/theme.css` and untracked `NOTES.md` unchanged. Escape returned focus to **Checkout commit…**; the reopened dialog focused Cancel.

While checkout review was open, an independent Git command renamed the disposable source branch to `external-review`. Submitting the stale review showed the actual source-preflight error, refreshed the branch, retained the dialog with a stale-source warning and disabled its checkout button. Escape still closed it. Git inspection confirmed the same HEAD and working changes, and the reflog showed only the arranged branch rename, with no checkout. The collapsed panel recorded the ten preflight/refresh reads, advancing from 24 to 34.

Re-selecting the tip and opening a fresh review then completed detached checkout. The banner displayed the full tip and preservation control, independent Git inspection confirmed detached HEAD at the unchanged `external-review` tip with both local changes preserved, and collapsed activity advanced to 54 commands. The native app was left on this disposable demo for inspection.

#### Continuation audit — November 12, 2026

The requested continuation began at committed checkpoint `774447b` with a clean working tree. Independent Standards (`/root/detached_standards_recheck`) and Spec (`/root/detached_spec_recheck`) reviews of `git diff 58e8b49...774447b` found zero actionable findings within the bounded prototype scope. No source correction was needed.

The real-Git/Windows PowerShell suite passed again: **11 passed, 0 failed**. `npm run build` passed. The native rebuild initially failed because the existing `work/native-target/debug/git-k-prototype.exe` was running and Windows refused to replace it. Rebuilding with a separate target directory passed, preserving the running application and its session. The new executable is `work/native-recheck-target/debug/git-k-prototype.exe`, SHA-256 `8EFEC21B81837E2CDD5D2EE4C1E21A234B9D92342942CFF7949DE6A602B22A85`. This continuation rechecked the source and builds; native interaction evidence remains the walkthroughs above.

A fresh audit read the authoritative dependency and decision-gate links for all 126 defined issues: no production ticket is eligible, all 126 require unresolved G5/G8, and all nine envelopes remain pending decomposition. Issue 001 is the first production frontier after those contracts resolve. No production status, acceptance checkbox or decision gate was advanced. Implementation stops at the missing execution/coordination/retention and Git-distribution/native-support decisions.

### Review and limits

Separate AI Standards and Spec reviewers (`/root/detached_standards` and `/root/detached_spec`) reviewed `git diff HEAD` against base `58e8b49`, including this untracked scope note and surrounding source. Standards: zero findings. Spec: zero actionable findings within the bounded prototype scope. This is AI review, not human design approval. The additional tests and completion notes received a follow-up review before commit. No production acceptance criterion was marked complete.

External-process races, interactive hooks/filters/signing, non-Unicode paths, cancellation, durable/redacted/bounded history and other platforms remain unvalidated. A hook may change state before reporting failure; errors trigger refreshed state, not a rollback promise. Recovery guidance is contextual text and an object ID, not a reflog recovery UI.

Windows preview: `Git-K-detached.exe`. SHA-256: `4BC37536B1A6C663BC60EDF4D7B8D9DBC8F3F36D8A5914A7FBC697F1BB4662C9`.

The repeat native build produced `work/native-target/debug/git-k-prototype.exe`, SHA-256 `ACA4869AC151E4222AF37D11F38C6F97921F597C6C3B3C4C2A2209F23495334E`. The retained preview above was not replaced.
