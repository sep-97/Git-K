# Reviewed integration into main

The user explicitly requested review of all local branches and their integration into `main`. The source remains an execution/workflow prototype under `apps/command-prototype`; production implementation and parity gates remain open. Historical notes about keeping source outside main describe the earlier delivery approach and are superseded for source placement by this request.

## Pinned review

- Main base: `1c89536920f5c3fbd2dcba0df98d61acfae9d036`.
- Combined prototype head: `c108d4cd40d46441f07f3b892967b8ecd22d959c`.
- Review command: `git diff 1c89536920f5c3fbd2dcba0df98d61acfae9d036...c108d4cd40d46441f07f3b892967b8ecd22d959c`.
- Separate AI reviewers: `/root/standards_review` and `/root/spec_review`, using the code-review skill's independent axes. These are AI reviews, not human approval. Both reviewers also examined the corrective patch.
- Existing pull requests: #3 execution history, #4 staging and #5 commit creation. The combined integration includes their exact commits and all subsequent prototype branches.

## Every original local branch

| Branch | Reviewed tip | Disposition |
| --- | --- | --- |
| `bootstrap/repository` | `e07afb8` | Already an ancestor of main |
| `chore/project-foundation` | `cb15465` | Already integrated through PR #1 |
| `docs/implementation-plan` | `3ed78c4` | Already integrated through PR #2 |
| `main` | `1c89536` | Pinned base |
| `prototype/execution-history` | `afc3281` | Included in the combined prototype head |
| `prototype/file-staging` | `c914639` | Included in the combined prototype head |
| `prototype/commit-workflow` | `eb186cd` | Included in the combined prototype head |
| `prototype/amend-workflow` | `2c6e6d7` | Included in the combined prototype head |
| `prototype/branch-workflow` | `7e51be1` | Included in the combined prototype head |
| `prototype/reference-management` | `c108d4c` | Combined prototype head |
| `research/connected-parity` | `f415ad1` | Patch-identical to main's `30eec3d`; ancestry recorded |
| `research/core-parity` | `8dd67a5` | Patch-identical to main's `b0a1221`; ancestry recorded |
| `research/runtime-architecture` | `9f99ba4` | Patch-identical to main's `642952f`; ancestry recorded |

Both `git cherry` and stable patch IDs established research equivalence. Commit `da642a2` records all three original research heads as parents using a history-only merge. Its tree hash equals its first parent's tree hash, preserving the already-integrated reports and subsequent path generalization.

## Standards

Initial result: **0 documented-standard violations, 1 optional P3 duplication smell**. Branch creation/checkout repeated validation already available in `validate_ref_name`. The corrective patch reuses that helper. Follow-up Standards review found no remaining standards violations or merge-blocking concerns.

The repository's tracker states, unchecked production acceptance criteria, domain vocabulary and Tauri/Rust/React ADR are preserved. Prototype limitations remain recorded and are not reclassified as completed production work.

## Specification

Initial result: **1 P2 command-display finding; no other missing requirements or scope creep within the bounded prototype scopes**. Windows PowerShell could treat `@draft` as splatting, `--%` as stop-parsing, fail to invoke a quoted executable path, and lose embedded quotes or trailing backslashes in native arguments. Actual Rust argument-vector execution was correct.

The fix formats Windows replay through `Start-Process` with one CRT-escaped argument string and propagates the native exit code. Two real PowerShell/native-Git regressions replace assumptions about shell parsing with actual argument round trips. Expanded history now exposes the full recorded command, and the collapsed row has a full-command tooltip. Follow-up Specification review found no remaining concrete defect within this scope.

## Validation

- The two new replay tests first failed against the original display behavior; the four existing real-Git scenarios passed.
- After correction, **6 tests passed, 0 failed**. Native Git stored/read 13 literal cases including splatting-like text, stop-parsing text, apostrophes, embedded double quotes, trailing backslashes, multiline values, percent-variable text and an empty argument. A quoted full executable path and a genuine Git exit 128 were also checked.
- Frontend TypeScript/Vite production build passed.
- Windows native Tauri debug build passed with the isolated Rust/MSVC environment.
- Existing native walkthrough evidence remains in the prototype's validation notes. The full workflow walkthrough was not repeated for this formatting/helper-only correction.
- GitHub reported no configured check runs or commit statuses for the pinned prototype head. Local build and regression results are the validation evidence; they are not represented as GitHub CI.

## Integration method and remaining limits

One combined pull request carries all prototype commits, the research ancestry record, this review and the corrective patch into main. The earlier stacked pull requests are retargeted to main and accounted for by the same commit ancestry. No branch is force-pushed or deleted. Final remote ancestry and pull-request state are verified after merging.

This merge does not establish production readiness. External-process coordination, durable/redacted/bounded history, interactive prompts, cancellation, remote/provider workflows, cross-platform validation and complete GitKraken parity remain open. The Windows display is a runnable PowerShell representation of the original executable/argument vector; actual application execution continues to use Rust's original vector directly.
