# 93: Open the equivalent connected Code Review workflow

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** gated-envelope.

**Phase:** G — Self-hosted accounts, collaboration and connected views

**What to build:** Follow the desktop browser-review handoff into the agreed equivalent connected review destination.

**Blocked by:** [58: Create, inspect and merge GitHub pull requests](058-create-inspect-and-merge-github-pull-requests.md); [79: Sign in to a self-hosted account from the desktop](079-sign-in-to-a-self-hosted-account-from-the-desktop.md); [87: Share and synchronize workspaces](087-share-and-synchronize-workspaces.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support; G4 — Complete parity acceptance; G6 — Equivalent self-hosted service and provider contract.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Before implementation, settle the required review/comment/suggestion scope and expand the observed reference cases.
- [ ] Preserve the 12.6.1 handoff instead of recreating an obsolete desktop review mode.
- [ ] **CON-REL-003:** In-app PR review was replaced by gitkraken.dev Code Review links.
- [ ] **GAP-005:** Settle the equivalent web Code Review scope and implement the connected workflow required by the desktop.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-REL-003, GAP-005.

This is an explicitly unsized decision/discovery envelope. Resolve its reference/service/protocol questions, then obtain approval of newly defined child slices before scheduling implementation. Approval of this envelope is approval to retain and refine the scope, not to mark it ready-for-agent.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
