# Branch creation and checkout prototype

Status: implemented and validated; user design review pending
Assignee: /root
Source branch: `prototype/branch-workflow`
Base: `2c6e6d7`

## Scope

Add a New branch dialog using current HEAD or a selected history commit, with optional immediate checkout. Add explicit Checkout buttons to the local branch list. Validate literal branch names using Git, reject stale branch/HEAD views and unfinished/conflicted operations, and serialize application mutations. Use no-force, no-guess, no-track commands, preserve Git's worktree restrictions, refuse overwriting ignored files and avoid recursive submodule checkout. Require cancellation of amend before branch changes. Preserve normal commit drafts across branch changes.

Record actual commands even while Action history is collapsed. Refresh after failures without promising rollback: post-checkout hooks can report an error after switching branches. Distinguish successful mutation from subsequent read failure. Keep failed creation inputs available and require review after a changed source branch/HEAD.

This extends the existing throwaway prototype. Rename/delete, remote branches, detached-HEAD checkout, graph double-click/context menus and complete production issue 013 acceptance are not part of this increment. Production decision gates remain open.

## Validation

Observed on Windows x64, October 31, 2026. TypeScript/Vite production build and native Tauri debug build passed with the existing pinned dependencies and Rust/MSVC environment. Actual Windows UI actions ran against disposable Git repositories; independent Git commands and file reads checked outcomes.

| Check | Observed result |
| --- | --- |
| Create and checkout at HEAD | Created the literal name `feature/literal&$(value)` at the reviewed HEAD and switched to it. The staged blob, newer working contents and untracked sibling were preserved. |
| Invalid name and retry | `bad..name` produced the actual `check-ref-format` failure. The dialog retained the input and allowed correction to a valid name. |
| Duplicate name | Creating `main` failed with Git's existing-branch error. No branch was force-reset. |
| Conflicting dirty checkout | Switching to a branch with different tracked content was refused. Current branch, staged blob and working contents remained unchanged. |
| Create-and-checkout refusal | Creating at a selected incompatible commit was refused by `switch -c`. Independent `show-ref` confirmed that no new branch was left behind. |
| Create only at selected commit | Unchecking checkout and retrying created `from-selected` at the selected commit. Current branch, index and working contents stayed unchanged. |
| Branch/tag name collision | Local branch `compatible` remained listed with its real name despite a same-name tag. Checkout attached HEAD to the local branch and preserved partial staging. |
| Commit draft and amend coordination | The normal summary `Draft survives branch checkout` remained after compatible checkout. Entering amend disabled branch actions. Escape restored the normal draft and branch controls. |
| Stale source | External checkout at the same HEAD caused preflight refusal. The dialog kept the requested name, showed the changed-source explanation and disabled submission until cancelled/reopened. `must-not-create` did not exist afterward. |
| Ignored-file protection | A tracked path on the destination collided with an ignored local file on the source. Git refused checkout; `valuable ignored content` and the source branch remained intact. |
| Linked-worktree restriction | Checkout of a branch already used in another worktree failed with Git's real explanation. Both worktrees retained their branch assignments. |
| Post-checkout hook failure | A fixture hook exited 1 after switching to `compatible`. The UI showed the real error and refreshed to the actual new branch; it did not claim the operation rolled back. |
| Success then refresh failure | A fixture hook made later repository reads fail while returning success. The UI explicitly reported that the branch action succeeded but refresh failed and removed stale repository controls. Independent inspection confirmed the destination branch and files. |
| Unborn branch | New branch was disabled with no existing commit. First-commit creation remains the route to creating an initial branch tip. |
| Hidden history | Panel was collapsed at 6 commands and remained collapsed through the branch scenarios, reaching 251 before the last polling batch. Reopening at 254 exposed the successful `switch --no-guess --no-overwrite-ignore --no-recurse-submodules -- other` command and exit 0. |
| Dialog and final demo | Native modal showed a readable starting-point label, initially focused name field, checkout checkbox and visible Cancel/Create controls. Escape cancelled a stale dialog. The final demo successfully created/checked out `feature/branch-preview`, retaining four commits, the unstaged theme and untracked notes. |

No automated suite was added to this throwaway harness. The checks above exercise the public native workflow and inspect actual Git state.

Windows executable: `Git-K-branches.exe`. SHA-256: `3E7233809C1E3CAC23CFA06AED31A62505EFC1081E2F2B3D87F716D85E85BC8B`.

## Review and bounded verdict

Sequential standards and specification reviews by the implementing agent checked tracker/domain rules, literal argument handling, modal state/focus, mutation serialization, source preflight, file protection, real error reporting, draft retention and scope coverage. No outstanding defect was identified in this increment. This is not independent reviewer sign-off or production parity verification.

Native branch creation and checkout work within this scope, including ordinary dirty repositories and meaningful refusal/recovery states. Git handles index/ref locks; the app never removes them or bypasses worktree restrictions. Configured hooks and filters can still mutate repository state, including after Git has updated the branch. A reported error always triggers reconciliation, not a rollback promise.

External-process races, immutable reviewed state, interactive hooks/filters, submodule working-tree updates, full non-Unicode handling, cross-platform support, durable/redacted/bounded history and cancellation remain unresolved. Rename/delete, remote/tracking workflows, detached-HEAD checkout, complete graph/menu/keyboard entry points and production issue 013 remain ahead. Execution/native-support/reference decision gates remain unchanged.
