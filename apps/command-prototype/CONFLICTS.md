# Conflict resolution prototype

Open **Working changes** during a pending merge and select an unresolved file. The editor shows the current and incoming versions from Git's index alongside the working output. Navigate unresolved files and conflict blocks, search the output, choose either version of a block, or type a combined resolution. **Save and stage output** writes and stages the edited file. The merge stays pending until you finish or abort it.

For a whole-file decision, choose **Take current file** or **Take incoming file** and review the change. A missing version is shown as a deletion. Whole-file choices preserve the selected Git blob and mode, including for binary files. For an external merge tool, set Git's `merge.tool` to your preferred tool, for example with `git config --global merge.tool <tool-name>`. The app uses the effective Git configuration for the open repository and runs the reviewed selection with `git mergetool --no-prompt`. Configure external tools to return an exit status or otherwise work without terminal input. Review any backup files and unresolved paths after the tool exits.

When every conflict is resolved, select **Finish merge…**. Review the staged summary and pending merge message, then choose **Create merge commit**. Git uses its configured hooks and signing and commits the staged version. The app checks the pending merge identity, staged entries and pending message again before committing. **Abort merge…** remains available through its existing review.

## Validation

On Linux on November 25, 2026, the frontend built and all 31 headless real-Git tests passed. The five new workflows exercised Git's conflict stages, edited and staged bytes, both merge parents, preservation of unstaged edits, stale file and stage checks, index lock refusal, hook rejection, binary choice, deletion and a configured external tool. The first test reproduced the missing inspection behavior before implementation.

The Chromium check exercised navigation, search, a diff3 block choice, edited output, retry after a failed write, a binary whole-file choice, merge review cancellation, and completion with the activity panel collapsed. It used a stubbed Tauri bridge. The headless Git tests separately verified that Git recorded the commands, context and results. Conflict preview contents are omitted from Action history.

Native Linux validation stopped at the missing `glib-2.0` development package. Native Windows validation and matching reference observation are pending. The text editor accepts UTF-8 files up to 256 KiB and retains uniform CRLF. Binary and oversized regular files use whole-file choices. Special paths and non-UTF-8 names may require Git outside the app. Application actions share a lock. External Git or filesystem changes between the last check and a write remain possible. Conflict resolution for squash, rebase and cherry-pick is outside this prototype workflow.

Run from the app directory:

```sh
npm ci
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --locked -- --test-threads=1
GIT_K_BROWSER=/usr/bin/chromium npm run check:merge-ui
```

The code review checked path handling, the lock around app mutations, exact staging for whole-file choices, and recovery after a failed write or refresh. The workflow review checked the ticket's entry points and outcomes against the implemented controls and tests. The work stays within the pending merge prototype while the production dependencies and reference checks remain open.
