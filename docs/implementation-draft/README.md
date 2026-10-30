# Git-K implementation plan for review

Status: draft; no application code or active implementation tickets are introduced by this package.

Start with the [proposed breakdown](breakdown.md), then use the [full specification](spec.md) and [coverage matrix](coverage.md) for detail. The draft contains 135 proposed work items across 11 streams, with one Markdown draft per item: 125 feature/delivery slices, nine gated envelopes requiring further decisions and child slices, and one final verification gate.

The user approved complete-workflow testing on October 30, 2026. Ticket size, dependencies and merge/split changes still require review under the selected to-tickets workflow. This approval is about the breakdown; architecture, reference and service choices remain in their existing decision tickets.

## First delivery sequence

1. Resolve the existing real-Git execution prototype and native support/tooling choices. Obtain matching reference access and settle connected-service choices where independent.
2. Build a running native app that opens a repository, reads real Git state and records actual commands while the activity panel is collapsed.
3. Inspect real diffs; stage and commit files; then support partial changes and the remaining frontier.

The existing [decision map](../../.scratch/gitkraken-parity/map.md) stays open. No proposal here closes it, narrows full parity to an MVP, or treats an unknown interface as implemented.

## How coverage is accounted for

- 252 desktop/core checkpoints and 133 connected entries come from the existing audits.
- 15 runtime findings inform the implementation contracts.
- 16 supplemental/user/discovery obligations retain favorites, detached HEAD, hooks, visibility requirements and areas that still need a matching-reference census or service decision.
- All 416 records have one primary owning draft. Some source records group provider/platform subcases; ownership is accountability, not automatic verification of every subcase when one ticket lands.
- No work is implemented or verified by its presence in this plan. The census is an explicitly unsized envelope that must be expanded into approved child slices after observation.

After breakdown approval, publish agreed tickets to the configured local tracker in dependency order. Preserve unresolved decision gates as blockers. Individual active tickets and their evidence then own implementation state; this draft remains the reviewed planning snapshot.
