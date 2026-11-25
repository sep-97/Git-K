# Merge workflow prototype

Status: implemented; real-Git and browser checks passed; native validation and user design review pending
Assignee: /root
Source branch: `prototype/merge-workflow`

## Scope

Select a graph commit and choose **Merge into current branch…** in the inspector, or open a local branch's **⋯** management dialog and choose **Merge into current branch…**. The review shows the exact incoming object ID, current branch and HEAD. Cancel/Escape requests no write. The three modes are:

- **Merge (fast-forward when possible)**: advances to the incoming tip when possible, otherwise creates a merge commit.
- **Always create a merge commit**: keeps both parents even when a fast-forward is possible. The editable message is passed literally; configured identity, hooks and signing apply.
- **Squash into staged changes**: keeps HEAD in place and stages the combined result. Review it in Working changes and use the existing commit composer. The new-commit draft is preserved.

Start requires an attached branch with an existing commit, a clean tracked index/worktree and no unfinished operation. The runner compares the reviewed current branch/HEAD and, for a branch source, its exact reference object. The command receives the reviewed object ID. No automatic stash, forced overwrite or unrelated-history override is used. Per-branch merge flags are cleared for this invocation and mode flags are explicit; other configured Git behavior still applies.

All three modes require **Git 2.38+** for `merge-tree --write-tree` preview. The computed result tree includes directory-rename paths; its paths are compared with local untracked and ignored paths before the actual merge runs. This conservative guard can refuse a merge when a local path overlaps the preview. A preview can write Git objects without changing the index/worktree. Configured merge drivers may run during both preview and merge. Their own side effects are outside the runner's control.

The command runner now retains stdout, stderr and exit status internally before applying its usual success/error result. This allows a real conflict preview (exit 1 with a result tree) to be inspected without relabeling its activity as success. Each actual process still has one command/result record, independent of panel visibility. Successful merge/abort writes followed by failed reads return success and refresh failure separately, close the dialog and disable further mutation until reopening the repository.

Normal/explicit-commit conflicts or pre-commit signing/hook failures retain actual index/worktree state and expose a **Merge in progress** banner. **Abort merge…** reviews the current merge identity and discarding resolution edits; the runner compares it again and delegates to `git merge --abort`. An abort refusal retains the actual state. Conflicting squashes are refused at preview, since Git leaves no `MERGE_HEAD` for an abort. Conflict editing, merge continuation, drag/drop, complete context menus, persistent squash preferences and full reference parity remain ahead. Production issue 015 and the execution/native-support gates remain blocked.

## Validation

Observed on Linux, November 25, 2026 (Asia/Tehran):

- `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --locked -- --test-threads=1`: **26 passed, 0 failed**. Eleven new real-Git scenarios cover fast-forward/diverged/explicit-commit parents and trees, literal messages, branch-local configuration overrides, squash plus manual application commit, conflict state and reviewed abort, conflicting squash refusal before index/worktree changes, dirty/stale/detached/invalid inputs, untracked/ignored collisions across all modes, directory-rename collisions, hook/signing failure and abort, successful writes followed by failed reads, and a genuine index lock retained after refusal. Fifteen existing reference/detached scenarios also pass. Two Windows-only PowerShell cases were not run.
- `npm run build`: TypeScript/Vite passed.
- `GIT_K_BROWSER=/usr/bin/chromium npm run check:merge-ui`: passed. The committed browser check uses a stubbed Tauri IPC bridge for graph/branch entry points, Cancel/Escape, reviewed source/target/message, collapsed-panel behavior, squash and commit-draft preservation, conflict/abort, stale dialog, success followed by refresh failure, and dirty/detached disabled states. It is browser contract evidence, not a native complete workflow.
- `git diff --check`: passed.

The first workflow test failed with `Merge workflow is not implemented` before implementation. Preservation tests then exposed that `--no-overwrite-ignore` alone did not protect an ignored file in an explicit merge commit. Checking only the incoming tree also missed a directory-rename result and failed its regression. The final guard checks the computed result tree; both regressions now pass.

The source branch also integrates published detached-HEAD history at `2375eea` with the local Linux/headless additions; that integration passed 15 tests before merge work began. Earlier Windows evidence is retained in [DETACHED.md](DETACHED.md) with its original source scope.

## Reviews and limits

Standards self-review checked literal argument handling, immutable source selection, shared mutation serialization, conservative file-collision refusal, authentic process/result recording, hook/signing failures and refresh-after-success handling. Its ignored-file and renamed-directory findings were fixed and covered by failing-then-passing real-Git tests. Spec self-review compared this bounded increment with item 015: merge and staged-squash behavior are covered; full reference entry points/preferences, native workflow evidence and production gates remain open. These self-reviews do not represent independent review or human design approval.

Native Linux validation cannot run in this environment because `webkit2gtk-4.1` and `rsvg2` are missing; Windows native validation is also pending for this increment. External-process changes between preview/preflight and mutation, arbitrary custom-driver/filter/hook effects, non-Unicode paths, platform-specific path aliases/case behavior, interactive signing, cancellation and persistent history remain outside validation. No completed production slice or reference parity is claimed.

## Reproduce and continue

From this app directory, run `npm ci`, then the build and Rust command above. For browser checks, install a Playwright Chromium build with `npx playwright install chromium` and run `npm run check:merge-ui`, or set `GIT_K_BROWSER` to an existing Chromium executable. The check starts and stops its own localhost Vite server on port 1421.

Run native Windows walkthroughs for all three modes, with a collapsed activity panel, a genuine conflict/abort, cancellation and an overwrite refusal. The next planned prototype workflow is item 016, conflict inspection, resolution and merge continuation. Production issue status remains governed by its original dependency and decision gates.
