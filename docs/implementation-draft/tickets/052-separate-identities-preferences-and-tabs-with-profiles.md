# 52: Separate identities, preferences and tabs with profiles

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** E — Terminals, recovery and desktop integration

**What to build:** Create/switch profiles with isolated identities, settings, accounts and tab sets; update global Git identity only through the explicit option.

**Blocked by:** [11: Work in multiple repository tabs](011-work-in-multiple-repository-tabs.md); [3: Stage files and commit real changes](003-stage-files-and-commit-real-changes.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify switching profiles neither changes another profile nor silently rewrites global Git configuration.
- [ ] **CORE-013:** Per-profile tab sets and repository aliases
- [ ] **CORE-219:** Profile-scoped settings, tabs and integrations
- [ ] **CON-PROFILE-001:** Create/manage/switch profiles with author name/email.
- [ ] **CON-PROFILE-002:** Isolate integration accounts and general/UI preferences.
- [ ] **CON-PROFILE-003:** Keep profile-specific tabs; duplicate current tabs for new profiles.
- [ ] **CON-PROFILE-004:** Optionally synchronize profile identity to global Git config.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-013, CORE-219, CON-PROFILE-001, CON-PROFILE-002, CON-PROFILE-003, CON-PROFILE-004.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
