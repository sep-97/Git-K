# Issue tracker: Local Markdown

Specs and issues live in .scratch/<effort>/.

- Specifications: spec.md within the effort directory.
- Implementation tickets: issues/NN-title.md, one file per ticket.
- Incoming request triage: Status records the role from triage-labels.md.
- Comments: append under a Comments heading.

## Wayfinding operations

- Map: .scratch/<effort>/map.md, with Label: wayfinder:map.
- Decision tickets: issues/NN-title.md, with Parent: ../map.md and Type: research, prototype, grilling or task.
- Decision state: Status: open, claimed or resolved; Assignee records the claim.
- Blocking: Blocked by lists ticket filenames. A ticket is unblocked when all listed blockers are resolved.
- Frontier: open, unclaimed, unblocked tickets in numeric order.
- Claim before work: set Status: claimed and Assignee.
- Resolve: append an Answer, set Status: resolved and add a named link with a one-line result to the map.
- Implementation dependencies: Blocked by lists implementation ticket filenames. Finished implementation tickets use Status: done.
- Refer to tickets by their linked titles in user-facing discussion.
