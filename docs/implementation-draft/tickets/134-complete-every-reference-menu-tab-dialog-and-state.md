# 134: Complete every reference menu, tab, dialog and state

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** gated-envelope.

**Phase:** K — Reference census and final acceptance

**What to build:** Compare the full observed reference census with Git-K and implement remaining native menus, onboarding, dialogs, tooltips, shortcut/context states and screen-specific differences.

**Blocked by:** [127: Expose conditional toolbar and experimental controls](127-expose-conditional-toolbar-and-experimental-controls.md); [54: Select themes and follow system appearance](054-select-themes-and-follow-system-appearance.md); [55: Inspect scoped activity and diagnostic output](055-inspect-scoped-activity-and-diagnostic-output.md); [84: Start offline and manage local entitlements](084-start-offline-and-manage-local-entitlements.md); [93: Open the equivalent connected Code Review workflow](093-open-the-equivalent-connected-code-review-workflow.md); [102: Implement the agreed Insights dashboards and metrics](102-implement-the-agreed-insights-dashboards-and-metrics.md); [115: Respond to supported agent permission requests](115-respond-to-supported-agent-permission-requests.md); [133: Use the correct Windows or WSL application context](133-use-the-correct-windows-or-wsl-application-context.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support; G4 — Complete parity acceptance; G7 — Matching reference build and feature access.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Expand every newly discovered behavior into a stable checkpoint and owning ticket; split this work into child slices after the matching-build census.
- [ ] This is a discovery/integration envelope, not a claim that its unknown work already fits one implementation session.
- [ ] **CORE-001:** Repository layout: left panel, graph, commit panel
- [ ] **USER-004:** Match the reference desktop experience across every observed screen, tab, function and interaction state.
- [ ] **GAP-001:** Census startup, onboarding, native menus, dialogs, settings suboptions, account states and experimental controls.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-001, USER-004, GAP-001.

This is an explicitly unsized decision/discovery envelope. Resolve its reference/service/protocol questions, then obtain approval of newly defined child slices before scheduling implementation. Approval of this envelope is approval to retain and refine the scope, not to mark it ready-for-agent.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
