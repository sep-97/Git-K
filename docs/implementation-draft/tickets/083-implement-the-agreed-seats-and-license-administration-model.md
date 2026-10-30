# 83: Implement the agreed seats and license administration model

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** gated-envelope.

**Phase:** G — Self-hosted accounts, collaboration and connected views

**What to build:** Expose the agreed equivalent account/seat/license/version administration behavior, including exhausted-seat states and applicable billing handoffs.

**Blocked by:** [81: Manage organization members and roles](081-manage-organization-members-and-roles.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support; G6 — Equivalent self-hosted service and provider contract.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Settle functional licensing/billing expectations in G6 before implementing screens or payment dependencies.
- [ ] Use controlled test entitlements and billing simulations; a reference purchase prompt is not purchase authorization.
- [ ] **CON-ORG-006:** Show seat availability and exhausted-seat purchase prompts.
- [ ] **CON-DEP-002:** Central license/client-version administration.
- [ ] **GAP-007:** Settle reference account, plan, seats, licensing, billing and offline-mode behavior before implementing equivalents.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-ORG-006, CON-DEP-002, GAP-007.

This is an explicitly unsized decision/discovery envelope. Resolve its reference/service/protocol questions, then obtain approval of newly defined child slices before scheduling implementation. Approval of this envelope is approval to retain and refine the scope, not to mark it ready-for-agent.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
