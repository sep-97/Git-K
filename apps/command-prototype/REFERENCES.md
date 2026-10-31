# Local branch and tag management prototype

Status: implemented and validated within the scope below; user design review pending
Assignee: /root
Source branch: `prototype/reference-management`
Base: `7e51be1`

## Scope

Rename local branches, including the current branch, without overwriting existing names. Delete an inactive local branch only when Git's normal merged-branch check permits it. Refuse operations on branches used by another worktree. Add explicit review/cancel dialogs for reference changes and deletion. Preserve normal commit drafts and index/working contents.

List local tags; create unsigned lightweight or annotated tags at HEAD/a selected commit; inspect tag objects/annotations; delete a selected local tag. Annotated tags honor configured identity and signing. Tag deletion uses Git's expected-object comparison. Branch rename/delete preflight compares the reviewed object, but external-process coordination remains incomplete.

Record actual commands with Action history collapsed, reject stale source state/references, and distinguish mutation success from failed refresh. Add real-Git regression checks against the same operation function used by the native command. Validate non-destructive UI workflows and deletion review/cancel through the native app; validate actual deletions through the regression runner in disposable repositories.

Force deletion, bulk deletion, remote tags, tag move/rename, full graph/menu/palette entry points, signing setup and complete production issues 013/034 remain ahead. Existing decision gates are unchanged.

## Validation

Observed on Windows x64, October 31, 2026. TypeScript/Vite production build and the native Tauri debug build passed. Four real-Git regression scenarios passed against the exact synchronous operation function called by `edit_reference`; no Git commands were mocked. The final native build was then launched and checked.

| Real-Git regression scenario | Observed result |
| --- | --- |
| Rename and merged deletion | Current-branch rename preserved the commit, branch description/configuration, staged blob, newer working contents and untracked sibling. Duplicate rename and current-branch deletion were refused. Inactive merged-branch deletion succeeded, and its actual command/completion were recorded. |
| Unmerged, worktree, stale and lock refusals | Unmerged deletion failed without removing the branch. Rename/delete of a branch checked out in another worktree were refused. A current branch deliberately checked out in two worktrees was also protected. Moved target objects and changed source branches were rejected. A genuine ref-lock collision preserved both the branch and fixture lock file. |
| Local tags and signing | Lightweight creation remained unsigned even with tag signing enabled. Annotated creation honored a configured nonexistent signer and failed without creating a tag. With signing disabled, annotation text containing quotes, shell punctuation, Unicode and paragraphs was preserved. Duplicate/invalid tag creation failed. Same-name branches/tags remained distinct. Stale tag deletion failed; fresh expected-object deletion succeeded without changing the index/working contents. |
| Successful write then failed refresh | A fixture reference-transaction hook made subsequent repository reads fail after tag creation. The returned outcome preserved mutation success and reported a refresh error. Independent Git inspection with the fixture worktree override confirmed the tag existed. |

The regression command was `cargo test --manifest-path src-tauri/Cargo.toml -- --test-threads=1` in the established Rust/MSVC build environment. Final result: **4 passed, 0 failed**. Fixtures were retained under the task's `work/reference-test-fixtures` directory.

| Native UI workflow | Observed result |
| --- | --- |
| Rename checked-out branch | Renamed `main` to `renamed-ui`; independent checks confirmed identical HEAD, staged contents, working contents and untracked sibling. A same-name `main` tag was preserved. |
| Branch deletion review/cancel | The current branch's deletion control was disabled. An inactive branch opened a separate review with its name/object and merged-branch explanation. Cancel preserved the branch. Actual deletion was exercised by the real-Git runner, not submitted through Windows UI automation. |
| Tag deletion review/cancel | Local-only deletion review displayed the selected name and object. Escape cancelled it; independent inspection confirmed the tag remained. Actual deletion was exercised by the real-Git runner. |
| Lightweight tag at selected commit | `ui-light` was created at the selected topic commit rather than HEAD. Independent `cat-file` identified a commit object; current branch and working state were unchanged. |
| Annotated tag at HEAD | Created `v1-ui` with a literal multiline message containing quotes, `$(literal)`, backticks and Persian text. Independent tag-object inspection confirmed the exact annotation and peeled commit. Empty annotation disabled Create. |
| Tag inspection | Selecting `v1-ui` displayed its actual tagger, date, annotation and commit. Reads used its reviewed object ID, not a potentially changed symbolic name. |
| Stale native rename | After opening the spare branch dialog, an external update moved that branch to another object. Rename was refused, input retained, refreshed state shown, and submission disabled until cancellation/review. No destination branch was created. |
| Final executable | Rebuilt after strengthening duplicate-worktree protection. Renamed `renamed-ui` to `reviewed-ui` successfully while retaining HEAD, partial staging, working contents and annotated tags. The current-branch deletion explanation was visible. |
| Hidden-panel recording | Final build's counter advanced from 7 to 29 while hidden. Reopening exposed the real `branch --move -- renamed-ui reviewed-ui` command with exit 0 and the worktree/preflight reads. |
| Layout | Native tag dialog showed name, annotation checkbox/message, signing explanation and Cancel/Create controls without clipping at the default window size. Sidebar scrolling exposed local tags; the inspector rendered the annotation. |

Windows executable: `Git-K-references.exe`. SHA-256: `417E4039EF1E51E0D4BA17EADAA0C8BF2E98AC6DED90E94A0CD38CE6593C9D58`.

## Review and bounded verdict

Sequential standards and specification reviews by the implementing agent checked local tracker rules, structured Git arguments, reviewed-object comparisons, branch/worktree protection, merged-only deletion, annotation/signing behavior, dialog cancellation, failure reconciliation and scope evidence. Review identified the duplicate-current-branch worktree case; the implementation and regression scenario were expanded, and checks passed again. No outstanding defect was identified within this bounded scope. This is not independent reviewer sign-off.

Reference changes preserve the ordinary index/working contents in the validated cases. Git hooks can change state independently, so failures trigger reconciliation rather than rollback promises. Branch rename/delete still have a gap between object preflight and the Git command. Tag deletion uses `update-ref -d` with the reviewed object for an atomic comparison, but object IDs do not detect deletion/recreation at the same object. Unmerged branch deletion is intentionally unavailable; force deletion is not exposed.

This remains prototype evidence. Production issues 013/034, remote tags, tag move/rename, bulk operations, full reference menus/graph/palette behavior, durable/redacted/bounded history, process cancellation, interactive signing/hooks, non-Unicode handling, complete repository coordination and cross-platform/reference validation remain ahead. No production decision gate is closed by this work.
