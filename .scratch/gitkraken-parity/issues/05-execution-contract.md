# Define the execution history and repository coordination contract

Parent: ../map.md
Type: prototype
Label: wayfinder:prototype
Status: claimed
Assignee: /root
Blocked by: 01-core-parity-audit.md, 03-runtime-investigation.md

## Question

What operation lifecycle and interface keep the command history truthful and understandable while handling interactive input, external commands, concurrency, conflicts and recovery?

## Decision inputs

Runtime investigation and terminal/interface requirements. A narrowly scoped prototype should validate execution/recording behavior with real Git, expose its limitations, and be reviewed with the user before the production contract is settled. Collapsing the panel must retain logging. Provider actions must have truthful API activity entries. This is a design prototype, not a substitute for the eventual application.

## Work in progress

The user asked to move to the next level after confirming that no application existed. A runnable Tauri + Rust + React prototype was built and launched on `prototype/execution-history`, focused on repository inspection, real command activity, hidden-panel recording and disposable scenarios. This claim does not resolve the production execution contract or native support decision.

## Prototype evidence

[Validation and bounded verdict](../../../apps/command-prototype/VALIDATION.md): the Windows native walkthrough passed; eight real Git reads remained recorded while the panel was collapsed, and an actual Git failure retained exit 128 and stderr. The throwaway source is captured on `prototype/execution-history`, outside main. User review and the remaining interactive, coordination, recovery and retention decisions are still required before resolution.

The user requested the next feature. The `prototype/file-staging` extension adds whole-file index changes with separate staged/unstaged views; [staging findings](../../../apps/command-prototype/STAGING.md) record successful native workflows, unchanged working-file hashes, and truthful index-lock failure/retry. This remains prototype evidence and does not resolve the broader contract.

The subsequent `prototype/commit-workflow` extension adds a summary/description composer and real commits from the index. [Commit findings](../../../apps/command-prototype/COMMITS.md) distinguish Git failures from commits that succeed before a refresh fails, and cover first commits, literal messages, hook/identity/signing failures and stale-branch rejection. Application mutations share a lock, but external-process coordination and the broader execution contract remain unresolved. This evidence does not complete production issue 003 or resolve this decision.

The user requested continued feature implementation. The `prototype/amend-workflow` extension adds latest-message editing and staged amendments with explicit update/cancel behavior. [Amend findings](../../../apps/command-prototype/AMEND.md) cover original-author and parent preservation, message-only updates, merge commits, draft restoration, hook rejection, stale branch rejection, unfinished-operation refusal and successful amendments followed by failed reads. Production issue 009 and this execution decision remain gated; prototype progress does not close them.

The next requested increment, `prototype/branch-workflow`, adds local branch creation at HEAD/a selected commit and checkout controls. [Branch findings](../../../apps/command-prototype/BRANCHES.md) cover literal names, preserved index/working contents, dirty and ignored collisions, worktree restrictions, stale sources, post-checkout hook errors after a real switch and successful checkout followed by read failure. Production issue 013 remains incomplete and gated; rename/delete and complete reference entry points are still ahead.

The `prototype/reference-management` extension adds local branch rename/merged deletion and local tag create/inspect/delete. [Reference findings](../../../apps/command-prototype/REFERENCES.md) combine native rename/tag/cancellation checks with real-Git operation-level regressions for actual deletions, unmerged/worktree refusals, stale objects, signing failure and completed writes followed by failed reads. Branch mutations still have external-process races; production issues 013/034 and this decision remain gated.

## Reviewed integration

The user explicitly requested review and merge of all local branches into `main`. The [integration review](../../../docs/reviews/main-integration.md) records branch coverage and separate Standards/Spec results, including the corrected Windows command-display defect. This authorizes prototype source placement in main, not production acceptance; this decision remains claimed and production gates remain unchanged.
