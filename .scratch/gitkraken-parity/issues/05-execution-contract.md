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
