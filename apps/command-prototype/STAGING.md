# File staging prototype

Status: implemented; review in progress
Assignee: /root
Source branch: `prototype/file-staging`
Base: `afc328113091f814d53c420ebfec2c4aaae3c6d2`

## Question

Can a user move whole-file changes between the working tree and index, inspect each side separately, and see every actual Git command and outcome?

## Scope

- Separate Staged and Unstaged lists. A file changed on both sides appears in both.
- Stage or unstage one file, or all available files, without discarding working files.
- Show the selected side's actual diff; untracked file content is labeled as a local read.
- Execute structured Git arguments in Rust; record commands, stdout/stderr and exit status while the panel is collapsed.
- Refresh from Git after mutations. Do not report success after an error; show the error and reconcile the current state.
- Validate the selected file against fresh Git status and serialize application index mutations. Treat unresolved conflicts as unavailable in this prototype.
- Exercise modified, untracked, deleted, renamed, partially staged and unborn-HEAD states using disposable repositories. Verify an index-lock failure leaves the UI and repository truthful.

Commit creation, hunk staging, conflict resolution and production execution/coordination policy are outside this step. The published production issue 003 stays blocked and incomplete. This native harness extends the earlier execution prototype under the same throwaway policy.

## Validation

Observed on Windows x64, October 30, 2026. Frontend typecheck/Vite build and the native Tauri debug build passed with the existing lockfiles and toolchain.

The native UI was driven against disposable Git repositories and checked independently through Git CLI and file hashes:

| Check | Observed result |
| --- | --- |
| Partially staged file | `partial.txt` appeared in both groups. Staged diff showed base → staged version; Unstaged diff showed staged version → working version. |
| Literal single-file staging | Staging `literal[1].txt` left the modified sibling `literal1.txt` unstaged. |
| Rename unstaging | Restored both rename paths in the index; the destination remained in the working tree as untracked and the source remained deleted. Every existing working-file hash stayed unchanged. |
| Stage all | Staged a rename, deletion, two modified literal filenames, a new filename containing a space, and the latest version of a partially staged file. Independent Git status showed six staged entries and no unstaged entries. |
| Index-lock failure | A fixture-owned `.git/index.lock` caused the actual unstage command to fail. UI showed the Git error, retained six staged entries and cleared the prior success notice. The index SHA-256 was unchanged. The fixture lock was removed manually outside the app; the app does not remove lock files. |
| Unstage all | Retry after lock removal cleared the index diff. All working-file hashes still matched the originals. |
| Unborn HEAD | Staged a first file before any commit, then modified it externally. Unstage removed only the index entry and retained the latest working contents. |
| Unresolved merge | Conflict notice appeared and both group actions were disabled. Conflict resolution remains a later feature. |
| Collapsed history | All staging/unstaging checks ran with the panel collapsed. The counter advanced from 10 to 58 through the normal-repository checks, including the failure and retry; recording continued. |

The final source also avoids treating a normal unborn-HEAD probe as a failed command and disables the path input while an operation is busy. Final rebuilt-binary smoke check and separate code reviews are pending.

## Bounded verdict

The whole-file UI can truthfully separate index and working-tree state, execute real stage/unstage commands and reconcile after a Git error. This prototype does not establish production coordination or parity acceptance. Git's own index lock protects individual writes; external changes can still occur between the status read and mutation. Sparse checkout, submodule staging, non-Unicode paths and configured filter interactions remain outside this walkthrough. No automated tests were added to this throwaway harness.
