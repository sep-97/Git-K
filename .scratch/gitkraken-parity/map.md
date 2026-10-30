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

No decision tickets resolved yet. The standing user choices above were confirmed before charting this map.

## Not yet specified

- The detailed delivery sequence depends on the completed coverage audit and architecture decisions.
- Performance targets, test fixtures and platform-specific support requirements need evidence from the coverage and runtime investigations.
- Cloud data ownership, deployment operations, service compatibility, and AI/agent provider arrangements need investigation before their dependent decisions can be made concrete.

## Out of scope

- Changes introduced after the pinned 12.6.1 baseline belong to separately tracked future-release work, per the user's answer on October 30, 2026.
