# Detached HEAD prototype

Status: implemented and validated within the scope below; user design review pending
Assignee: /root
Source branch: `prototype/detached-head`
Base: `58e8b49`

## Scope

Select a history commit and choose **Checkout commit…**. Review its full object ID and current tip before entering detached HEAD. Cancel/Escape uses the existing dialog cancellation path without invoking Git. The native runner uses a validated object ID, the shared mutation lock and branch/HEAD/conflict/unfinished-operation preflight. Checkout never forces, stashes, overwrites ignored files or recurses into submodules.

A persistent detached banner shows the current tip, branch-preservation guidance and reflog recovery context. **Preserve on new branch** opens the existing branch dialog at HEAD. Existing staging, commit and amend workflows work while detached. History now includes HEAD explicitly alongside all references, so detached commits appear within the existing 60-commit limit. Returning to a branch includes the old detached tip in the success notice. Leaving a tip does not immediately delete its objects; reflog expiry and garbage collection still limit eventual recovery.

This increment continues the claimed execution prototype decision. Production issue 014 and all decision gates remain blocked; full graph menus, reflog browsing, arbitrary SHA entry, complete cross-platform/reference parity and production coordination remain ahead.

## Validation — November 12, 2026

Windows TypeScript/Vite build and native Tauri debug build passed. The real-Git regression runner passed **11 tests, 0 failures**, including all six previous reference/PowerShell regressions and five detached scenarios against the exact operation function invoked by `edit_reference`:

- Detached checkout preserves staged contents, newer working contents and an untracked sibling. A subsequent real Git commit stays detached, leaves main unchanged and appears in the loaded snapshot. Creating a branch preserves the tip.
- Dirty checkout collisions, invalid/missing objects, stale source branches and unfinished merge state refuse checkout and preserve state.
- A post-checkout hook failure is reported even though HEAD has detached. A successful checkout followed by read failure returns mutation success separately from refresh failure.
- Untracked and ignored-file checkout collisions both refuse the switch, preserving the source branch, HEAD, staged contents, working contents and untracked siblings.
- A valid blob object ID is refused as a checkout target. Moving HEAD after review also refuses checkout while retaining the externally created commit and local files.

Native UI walkthrough on a disposable demo checked the actual checkout review, detached banner and full tip, staged one file, created `Preserve detached UI work` through the composer and preserved it through **Preserve on new branch → Create and checkout**. Independent Git inspection confirmed the new branch pointed to `241827a1b9be893d0f5faa1cd2003b069ccffe52`, main remained at `1435817b1ef1e7a287019d748c9055c8dc654c90`, and the untracked notes file remained. The graph loaded five commits including the detached tip. Returning to main succeeded while the preserved branch retained the new commit. The panel stayed collapsed while its count advanced from 23 to 112 during checkout, staging, commit, branch preservation and return.

The checkout dialog and banner fit the default window size. Tests use direct Git commits/branch preservation to arrange and inspect fixture state; the native walkthrough covers the application composer and branch commands.

### Completion review and repeat checks — November 12, 2026

The original nine tests passed again before adding the two refusal regressions above; the expanded eleven-test run passed. `npm run build`, the native Tauri debug build and `git diff --check` passed.

A fresh native demo (`demo-1794481487589`) supplied the missing cancellation and stale-dialog checks. Selecting its tip recorded command 24. Opening checkout review and cancelling with Escape, then reopening and clicking Cancel, left that count at 24; independent Git inspection showed branch `main`, HEAD `afbc7802c57d9a7bca77ed53e7cb66c9d3c76b51`, modified `src/theme.css` and untracked `NOTES.md` unchanged. Escape returned focus to **Checkout commit…**; the reopened dialog focused Cancel.

While checkout review was open, an independent Git command renamed the disposable source branch to `external-review`. Submitting the stale review showed the actual source-preflight error, refreshed the branch, retained the dialog with a stale-source warning and disabled its checkout button. Escape still closed it. Git inspection confirmed the same HEAD and working changes, and the reflog showed only the arranged branch rename, with no checkout. The collapsed panel recorded the ten preflight/refresh reads, advancing from 24 to 34.

Re-selecting the tip and opening a fresh review then completed detached checkout. The banner displayed the full tip and preservation control, independent Git inspection confirmed detached HEAD at the unchanged `external-review` tip with both local changes preserved, and collapsed activity advanced to 54 commands. The native app was left on this disposable demo for inspection.

## Review and limits

Separate AI Standards and Spec reviewers (`/root/detached_standards` and `/root/detached_spec`) reviewed `git diff HEAD` against base `58e8b49`, including this untracked scope note and surrounding source. Standards: zero findings. Spec: zero actionable findings within the bounded prototype scope. This is AI review, not human design approval. The additional tests and completion notes received a follow-up review before commit. No production acceptance criterion was marked complete.

External-process races, interactive hooks/filters/signing, non-Unicode paths, cancellation, durable/redacted/bounded history and other platforms remain unvalidated. A hook may change state before reporting failure; errors trigger refreshed state, not a rollback promise. Recovery guidance is contextual text and an object ID, not a reflog recovery UI.

Windows preview: `Git-K-detached.exe`. SHA-256: `4BC37536B1A6C663BC60EDF4D7B8D9DBC8F3F36D8A5914A7FBC697F1BB4662C9`.

The repeat native build produced `work/native-target/debug/git-k-prototype.exe`, SHA-256 `ACA4869AC151E4222AF37D11F38C6F97921F597C6C3B3C4C2A2209F23495334E`. The retained preview above was not replaced.
