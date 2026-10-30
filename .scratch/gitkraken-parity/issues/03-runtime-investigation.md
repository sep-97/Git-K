# Investigate Rust command execution and terminal architecture

Parent: ../map.md
Type: research
Label: wayfinder:research
Status: resolved
Assignee: /root/runtime_research
Blocked by: none

## Question

Which Tauri/Rust mechanisms support faithful command execution history, interactive terminals, repository synchronization and cross-platform delivery, and what local prerequisites are present?

## Research contract

Read the installed research skill. Investigate primary Tauri, Rust, Git and terminal-library sources. Record subprocess versus PTY semantics, streaming/backpressure, IPC typing, argument/display separation, credential redaction, concurrent operations across shared worktrees, cancellation/recovery, external shell changes, auth/hooks/signing and non-UTF-8 paths. Recommend a narrow runtime boundary and identify where prototypes or human decisions are still required. Inspect this Windows environment read-only for Rust/Cargo outside PATH, MSVC/build tools, WebView2 and a local reference installation; do not install or change system tools. Record presence as evidence rather than treating a PATH miss as absence. Include platform and packaging considerations, library/license facts only when verified, and proposed validation fixtures.

Deliver docs/research/runtime-architecture.md on research/runtime-architecture. Do not modify shared tracker files or resolve this ticket in the research worktree. Report the commit and important remaining gaps to the parent agent.

## Answer

Resolved October 30, 2026. [Runtime architecture research](../../../docs/research/runtime-architecture.md) records 15 primary-source mechanism findings, a proposed boundary between execution, journaling, repository coordination and PTY sessions, and six concrete prototype questions. Research commit: 9f99ba48be90e7541b959312945175c14331a3c5 on research/runtime-architecture; integrated into main.

Read-only tooling evidence establishes Git/Node, native Build Tools, a Windows SDK and WebView2 presence; Rust/Cargo were not available on PATH or in the standard paths inspected. The local reference client reports 9.13.0, leaving the matching-reference-access task open. No build was run and proposed architecture remains unaccepted until its dependent decision/prototype tickets are worked. This closes the bounded investigation, not the production runtime design or implementation.
