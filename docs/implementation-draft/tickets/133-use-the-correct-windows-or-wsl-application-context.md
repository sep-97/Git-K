# 133: Use the correct Windows or WSL application context

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** J — Native delivery and platform validation

**What to build:** Run the Linux app under supported WSLg, explain cross-filesystem choices and route links/files/agents through the correct context.

**Blocked by:** [132: Run and distribute the native Linux workflows](132-run-and-distribute-the-native-linux-workflows.md); [43: Use a real embedded interactive terminal](043-use-a-real-embedded-interactive-terminal.md); [112: Start an agent in a correctly configured worktree](112-start-an-agent-in-a-correctly-configured-worktree.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify the supported WSL2 boundary rather than assuming Windows paths and executables work inside Linux.
- [ ] **CORE-241:** Linux application runs inside WSL2 with WSLg
- [ ] **CORE-242:** Cross-filesystem warning and open-correct-instance choices
- [ ] **CORE-243:** WSL preferences for URL/file opening
- [ ] **CORE-244:** WSL versus Windows coding-agent executable context
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-241, CORE-242, CORE-243, CORE-244.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
