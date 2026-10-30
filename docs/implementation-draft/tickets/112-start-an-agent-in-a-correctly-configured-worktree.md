# 112: Start an agent in a correctly configured worktree

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** H — AI and coding-agent workflows

**What to build:** Choose the branch/base/agent, validate names, create the worktree, inherit view state and open the new session in the selected tab.

**Blocked by:** [111: Configure coding-agent launch and repository setup](111-configure-coding-agent-launch-and-repository-setup.md); [35: Create and manage linked worktrees](035-create-and-manage-linked-worktrees.md); [44: Manage persistent per-worktree terminal tabs](044-manage-persistent-per-worktree-terminal-tabs.md); [24: Pin, hide, solo and search visible history](024-pin-hide-solo-and-search-visible-history.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support; G6 — Equivalent self-hosted service and provider contract.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify quick-start defaults, collisions and partial setup/worktree failures.
- [ ] **CON-AGENT-003:** Start sessions with branch/base/agent selection.
- [ ] **CON-AGENT-004:** Validate generated names; search bases; offer quick-start defaults.
- [ ] **CON-AGENT-005:** Create worktrees, run setup and launch agent terminals.
- [ ] **CON-AGENT-006:** Inherit hidden/soloed references and collapsed source sections.
- [ ] **CON-AGENT-010:** Open sessions/worktrees in current or new tabs.
- [ ] **CORE-005:** List/Agents presentation toggle
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CON-AGENT-003, CON-AGENT-004, CON-AGENT-005, CON-AGENT-006, CON-AGENT-010, CORE-005.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
