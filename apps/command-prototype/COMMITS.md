# Commit workflow prototype

Status: implemented and validated; user design review pending
Assignee: /root
Source branch: `prototype/commit-workflow`
Base: `c914639`

## Question

Can a user commit staged changes with a summary and optional description, inspect the real result, and recover truthfully from a Git failure while action history keeps recording?

## Scope

- Add a commit composer to Working changes. Require a nonblank single-line summary and staged changes; disable committing while busy or conflicted.
- Execute `git commit --cleanup=verbatim --message <message>` as structured Rust arguments. Include staged contents only. Honor configured identity, hooks and signing in existing repositories.
- Record the command, output and exit status even with the activity panel collapsed. This records the parent Git process, not every subprocess launched by a hook.
- Refresh history and working/index state after success. Clear the message only after Git succeeds. Distinguish a successful commit followed by a failed refresh from a failed commit.
- Retain the message after failure, report the error and reconcile current repository state. Reset the draft when switching repositories; retain it on refresh of the same repository.
- Serialize application mutations. Reject a changed branch/HEAD, unresolved conflicts and unfinished merge/rebase/cherry-pick/revert workflows before committing. External-process coordination remains a production gate.
- Support first commits on unborn branches. Configure identity and unsigned commits only inside newly created disposable demos so their commit workflow can be tried immediately.
- Validate normal and partially staged commits, literal multiline messages, first commits, hook rejection and missing identity through the native UI and independent Git checks.

Amend, merge continuation, empty commits, signing configuration, interactive hook/editor prompts, hunk selection, remote actions and production execution policy are outside this step. Production issue 003 and the execution decision remain gated. This extends the throwaway native harness; it is not production parity acceptance.

## Validation

Observed on Windows x64, October 31, 2026. Frontend typecheck/Vite build and native Tauri debug build passed with the pinned dependencies and existing toolchain. The native UI was driven against disposable repositories and checked independently with Git:

| Check | Observed result |
| --- | --- |
| Partially staged commit | HEAD contains `staged content`; the working file still contains `working content remains`. The untracked sibling stays untracked and the cached diff becomes empty. History advances from one to two commits. |
| Literal message | Quotes, `&`, `$(value)`, backticks and Unicode remained literal. Git's commit object contains the summary, blank separator and two description lines as entered. |
| Empty message / no staged files | Submit is disabled for a blank summary. After success both fields clear and submit is disabled because the index is empty. Rust also validates summary and fresh staged contents. |
| First commit | An unborn branch with staged content becomes a one-commit history. Newer working contents and an untracked file remain intact. |
| Hook rejection | A fixture pre-commit hook prints a rejection and exits 1. UI shows its error and retains the message and staged entry; HEAD and the staged blob remain unchanged. Git can refresh index metadata even after a failed commit, so byte-for-byte index preservation is not claimed. |
| Missing identity | Git rejects the fixture's empty author identity. No fallback author is injected, no commit is created, and the message remains editable. |
| Signing failure | An intentionally nonexistent configured signer produces Git exit 128. The real stderr is shown, HEAD stays unchanged, and staged contents and draft remain. The app does not bypass signing. |
| Stale branch | An external switch to a different branch at the same HEAD is rejected before any commit runs. UI reconciles the new branch and retains the draft. |
| In-progress operation | A fixture cherry-pick marker is rejected with an unavailable-continuation explanation. No commit is created. |
| Success then read failure | A fixture post-commit hook makes subsequent repository reads fail. UI explicitly reports that the commit succeeded but refresh failed, shows actual commit output, and removes stale repository controls. Independent Git inspection confirms exactly one new commit. |
| Draft scope | Refresh of the same repository retains the draft. Opening another repository clears it. |
| Hidden history | Checks ran with the panel collapsed; the counter advanced from 6 to 165 actual commands. Reopening exposes the signing failure's actual commit command and exit 128. The session reports four failed operations, including hook/identity/signing failures and the deliberate post-commit read failure. |

No automated suite is added to this throwaway harness.

Separate Standards and Spec reviews of source checkpoint `7564b2e` both passed with zero findings. The final executable was rebuilt after adding local records for preflight refusals. Its native demo confirmed that the refusal is retained in Action history, followed by a successful commit on the refreshed main branch. History advanced from four to five commits, the new commit contained the staged theme, and `NOTES.md` remained untracked. The final session recorded 64 real Git commands with zero failed Git operations; the rejected preflight was recorded as local activity rather than a fabricated failed command.

Windows executable: `Git-K-commits.exe`, built from `7564b2e` on October 31, 2026. SHA-256: `FEB44CBC0B29CC0AFF0C02131A73453737F7E81E77F84FFD41E195B9FAAE87EA`.

## Bounded verdict

The native workflow can commit the index with a literal message, preserve normal working changes, refresh the graph, and distinguish command failure from post-success read failure. Existing repositories keep their configured identity, hooks and signing. Hooks themselves may change repository state; the application reports Git's outcome and reconciles rather than promising rollback.

This is prototype evidence, not production parity acceptance. External changes between preflight and commit remain possible; checking branch/HEAD does not establish atomic coordination or an immutable reviewed index. Durable history, bounded output, secret redaction, process-tree cancellation, interactive signing/hook prompts, cross-platform behavior and the remaining reference interaction evidence remain open. Amend and continuation workflows are not implemented.
