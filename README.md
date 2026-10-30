# Git-K

A planned Tauri + Rust + React/TypeScript desktop Git manager targeting GitKraken Desktop feature parity, with visible execution history for graphical actions.

## Current state

This branch contains a [Tauri + Rust + React execution prototype](apps/command-prototype/README.md) for real repository inspection and command recording. Native build and UI walkthrough evidence is being collected; this is throwaway code for the open execution-contract decision, not the production Git manager.

The [wayfinding map](.scratch/gitkraken-parity/map.md) tracks remaining investigations and decisions. The [implementation tracker](.scratch/git-k-implementation/README.md) contains the approved specification, published slices and pending envelopes. Production slices remain gated; the full parity target is unchanged.

The [project baseline](docs/project-start.md) records confirmed requirements, cited reference material, an initial feature inventory, environment observations, and unresolved decisions.

The [planning status](docs/planning-status.md) links the completed research, remaining decision frontier and next work.

The [approved breakdown](docs/implementation-draft/breakdown.md) records the reviewed plan. Authoritative work state and acceptance criteria now live in the local implementation tracker; existing decision gates remain open.

## Workflow

1. Audit reference features and resolve the map's architecture and acceptance questions, retaining all requested parity areas.
2. Produce a buildable specification and implementation tickets.
3. Implement and validate complete workflows incrementally, with progress measured against the feature inventory.

Read [AGENTS.md](AGENTS.md) for local tracker and domain documentation conventions.
