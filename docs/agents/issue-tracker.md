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

## Published implementation work

The approved Git-K specification, coverage and work index live in `.scratch/git-k-implementation/`. Defined work is one file per item under `issues/`; unsized envelopes remain under `pending/`.

- `Status` is execution state: `blocked`, `ready`, `claimed` or `done`. `Label: ready-for-agent` records approval of the defined breakdown; the label alone never permits starting work.
- Before claiming an issue, every linked `Blocked by` item must be `done`, and every linked `Decision gate` must be `resolved`. Also check that no unresolved behavior makes the requested implementation unspecified.
- `Verification gates` must be satisfied before recording complete parity verification. A completed initial slice does not automatically verify all grouped provider/platform subcases.
- A pending envelope has `Status: pending-decomposition` and `Label: needs-info`. It becomes executable only through approved child slices after its scope is settled. Dependent issues remain blocked until those children are completed and the envelope is explicitly marked `done`.
- An approved plan does not resolve, close or otherwise change parent wayfinding decisions. Read those decisions through the linked gate files.
