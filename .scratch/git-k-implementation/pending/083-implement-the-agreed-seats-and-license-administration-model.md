# 83: Implement the agreed seats and license administration model

Status: pending-decomposition
Label: needs-info
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: gated-envelope

**Phase:** G — Self-hosted accounts, collaboration and connected views

**What to build:** Expose the agreed equivalent account/seat/license/version administration behavior, including exhausted-seat states and applicable billing handoffs.

Blocked by: [81: Manage organization members and roles](../issues/081-manage-organization-members-and-roles.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md); [G6: Equivalent self-hosted service and provider contract](../../gitkraken-parity/issues/06-service-boundaries.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

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

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-ORG-006, CON-DEP-002, GAP-007.

This is an explicitly unsized decision/discovery envelope. Resolve its reference/service/protocol questions, then obtain approval of newly defined child slices before scheduling implementation. Approval of this envelope is approval to retain and refine the scope, not to mark it ready-for-agent.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
