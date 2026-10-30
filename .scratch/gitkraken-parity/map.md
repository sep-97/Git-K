# Find the route to GitKraken Desktop 12.6.1 parity

Label: wayfinder:map
Status: open

## Destination

Establish the reference coverage, acceptance approach, and architecture decisions needed to produce a buildable specification and dependency-linked implementation tickets for full GitKraken Desktop 12.6.1 parity in a Tauri + Rust + React application.

## Notes

- Full feature parity is the user's requirement, including every feature, tab and function. Implementation ordering does not remove scope.
- Reference release: 12.6.1. Subsequent releases are tracked separately.
- Validate Windows first, then macOS and Linux, retaining cross-platform architecture.
- Connected services are self-hosted first.
- The terminal/activity panel may collapse; recording application actions continues. Git actions expose real commands; provider actions expose API activity.
- The current repository has no application implementation. Research and decision assets precede the specification; this map does not authorize replacing full parity with an MVP.
- Read AGENTS.md and docs/agents/issue-tracker.md before modifying work state. Consult the installed wayfinder skill for this map, research for external facts, and grilling plus domain-modeling for human decisions. The subsequent route is to-spec, to-tickets, implement.
- Research ticket outputs live in docs/research/. Each research ticket is worked in its own worktree on a research/<name> branch. Integrate findings into main before recording their resolutions here.
- Human decision tickets require the user's answers; recommendations do not resolve them.

## Decisions so far

- [Audit desktop interface and local Git feature coverage](issues/01-core-parity-audit.md): 252 documented checkpoints establish the first coverage baseline; exact-build and visual evidence gaps remain open.
- [Audit connected services, AI and agent feature coverage](issues/02-connected-parity-audit.md): 133 documented entries expose provider/service boundaries and release-specific browser handoffs; full connected scope is retained.
- [Investigate Rust command execution and terminal architecture](issues/03-runtime-investigation.md): suitable mechanisms identified; exact application execution recording and interactive shell sessions need separate contracts, with prototype and build prerequisites still open.

## Not yet specified

- The detailed delivery sequence depends on the completed coverage audit and architecture decisions.
- Provider-specific failure and recovery cases may expose additional decisions once the service and execution contracts are chosen.
- Long-lived action history and shared-data evolution may expose additional decisions after those contracts are defined.

## Out of scope

- Changes introduced after the pinned 12.6.1 baseline belong to separately tracked future-release work, per the user's answer on October 30, 2026.
