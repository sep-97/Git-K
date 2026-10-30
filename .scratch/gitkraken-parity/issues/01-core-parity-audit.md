# Audit desktop interface and local Git feature coverage

Parent: ../map.md
Type: research
Label: wayfinder:research
Status: resolved
Assignee: /root/core_audit
Blocked by: none

## Question

What are the reference release's desktop surfaces and local Git behaviors, how do they vary by state, and what primary evidence is available for parity verification?

## Research contract

Read the installed research skill. Audit official reference documentation, release notes and available local reference installations. Separate verified behavior, inference, and gaps. Record an indexed feature inventory with stable identifiers, source links, expected behavior, verification needs, and implementation status. Cover the graph, tabs, menus, staging and commit flows, diffs/history/blame, branches and history changes, conflict workflows, terminal behavior, preferences, Git extensions, worktrees, keyboard interaction and platform-specific features. Read at least the direct feature pages; navigation headings alone do not establish detailed behavior. Keep per-source paraphrase limits in mind.

Deliver docs/research/core-parity.md on research/core-parity. Do not modify shared tracker files or resolve this ticket in the research worktree. Report the commit and any important remaining gaps to the parent agent.

## Answer

Resolved October 30, 2026. [Desktop and local Git parity audit](../../../docs/research/core-parity.md) contains 252 stable documented checkpoints and 35 directly reviewed official sources. Research commit: 8dd67a524139ca8b560cf7462b801a409e7552d7 on research/core-parity; integrated into main.

All checkpoints remain unimplemented. Documentation provides a broad planning baseline, not an exhaustive observed feature census. The report records conflicting shortcuts and custom-theme descriptions, release-provenance concerns, and missing visual/modal/error-state evidence. The reference-access and acceptance tickets retain these gaps. This closes the bounded documentation investigation, not full parity discovery or verification.
