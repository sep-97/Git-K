# Amend workflow prototype

Status: implemented and validated; user design review pending
Assignee: /root
Source branch: `prototype/amend-workflow`
Base: `eb186cd`

## Scope

Extend the existing commit prototype with an explicit amend mode. Load the latest commit message, allow message-only updates or include staged contents, and restore the previous new-commit draft on cancel without changing Git state. Show the target commit and a history-rewrite warning before update. Keep the target branch/HEAD fixed while editing, reject stale targets and unfinished operations, honor repository hooks/signing and record actual commands while the panel is hidden. Preserve the amend draft after failure and distinguish successful mutation from refresh failure.

Validate through the native UI against disposable repositories, with independent Git inspection of message, parent/tree identity, staged/working contents, cancellation and failures. Production issue 009 and the execution/native-support decisions remain gated; this is throwaway prototype evidence, not production parity acceptance.

## Validation

Observed on Windows x64, October 31, 2026. TypeScript/Vite production build and native Tauri debug build passed with the existing pinned dependencies and isolated Rust/MSVC toolchain. The built executable was driven through the actual Windows UI using disposable repositories; independent Git commands checked the resulting objects and working state.

| Check | Observed result |
| --- | --- |
| Load message | Latest summary and multiline description loaded into the composer. The original target SHA/branch and history-rewrite warning remained visible. The summary received focus; Escape from the composer cancelled amend. |
| Cancel | The prior `Saved new commit draft` summary was restored. HEAD, staged blob and working contents were unchanged. Cancel itself performs no Git mutation; staging while editing is not undone. |
| Staged amendment | A two-commit fixture retained its parent, commit count and original author/date. HEAD was replaced, the new tree contained `staged amendment`, and the working file still contained `working content remains`. The untracked sibling stayed untracked and the cached diff became empty. |
| Literal message | Quotes, ampersand, `$(literal)`, backticks and Persian text were preserved exactly in Git's message, with the original two-line description. |
| Message-only amendment | With no staged files, update remained available. The commit ID/message changed while the tree, count and working file stayed intact. |
| Hook rejection | A rejecting pre-commit hook returned exit 1 and `Amend fixture rejected`. The UI kept amend mode and its message. Independent inspection confirmed unchanged HEAD, staged blob and working content. |
| Stale branch at the same HEAD | An external switch to `external-branch` caused refusal before any commit command. The new branch was refreshed, the original amend target/draft stayed pinned, and Update became disabled until cancellation. Both branch tips and staged contents were unchanged. |
| Unborn branch / normal commit regression | Amend was disabled when no latest commit existed. Normal Commit staged still created the first commit from the index and preserved the newer working contents. |
| Success then refresh failure | A fixture post-commit hook made subsequent reads fail. The UI explicitly reported that amend succeeded but refresh failed, showed Git's successful output and removed stale repository controls. Independent inspection confirmed a single replacement commit with the staged contents and unchanged count. |
| Repository switch | Opening another repository cleared amend mode and the message. |
| Unfinished operation | A fixture cherry-pick marker was rejected before a commit ran, with the draft retained. HEAD stayed unchanged. After removing the fixture marker, retry succeeded. |
| Merge amendment | Updating the demo merge message retained both parents, its tree, four reachable commits and the existing unstaged theme/untracked notes. |
| Hidden history | The panel was collapsed at 28 commands and remained collapsed through the staged/message-only amendments and failure scenarios. The count reached 186. Reopening exposed the actual `git commit ... --amend` command with exit 0. Preflight refusals were local activity, not fabricated Git failures. |
| Native layout | At the default 1400 × 850 window size, the target warning, message fields, Cancel and Update controls were visible with Action history expanded. The changes list and composer retain separate scrolling when space is limited. |

No automated test suite was added to this throwaway harness. Validation uses the complete native workflow and independent repository observations, rather than mocked Git responses.

Windows executable: `Git-K-amend.exe`. SHA-256: `F996E609F304EEF9560065C9FB291F03058782CE8F89E236C6B36CDE2CEC8B5D`.

## Review and bounded verdict

The standards review checked local tracker/domain rules, command argument handling, mutation serialization, failure reporting and scope documentation. The specification review checked update/cancel, staged versus working contents, fixed targets, author/parents, genuine failures and hidden-panel recording. Both reviews found no outstanding defect in this prototype scope. These were sequential reviews by the implementing agent, not independent reviewer sign-off.

The workflow can amend the latest message or include staged changes, with an explicit rewrite warning and cancellation that restores the new-commit draft. The shared commit preflight still rejects changed branch/HEAD, conflicts and unfinished operations. Hooks and signing are not bypassed.

This does not establish production feature parity. Published-history detection, immutable reviewed-index coordination, interactive prompts, empty-commit amendments, exact preservation of unusual message whitespace/encoding, non-Unicode paths, durable/redacted/bounded history, process-tree cancellation and cross-platform/reference interaction evidence remain unresolved. The existing commit composer trims summary and description edges; ordinary multiline messages are covered here. Production issue 009 and the execution/native-support decisions remain gated.
