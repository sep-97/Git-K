# 6: Create, edit, save and delete repository files

**Status:** draft — awaiting breakdown approval; not a published implementation ticket.

**Kind:** vertical-slice.

**Phase:** B — Repository files, graph and navigation

**What to build:** Create files and nested directories, browse/filter the tree, edit content, save or discard pending edits, and stage saved work including workflow YAML.

**Blocked by:** [2: Inspect changed and historical files](002-inspect-changed-and-historical-files.md); [3: Stage files and commit real changes](003-stage-files-and-commit-real-changes.md)

**Decision gates:** G5 — Execution and repository coordination contract; G8 — Git distribution and native platform support.

**Reference verification gates:** G7 (matching build/access) and G4 (agreed parity acceptance) apply to final verification even when behavior can be developed from known fixtures earlier.

**Spec:** the full-parity specification in this draft package. **Source context:** use the linked research and coverage records; confirmed requirements and the existing decision map take precedence over proposed details.

## Acceptance criteria

- [ ] Verify unsaved-close prompts and save-and-stage against actual file and index contents.
- [ ] **CORE-036:** Create file through palette
- [ ] **CORE-037:** Create file through commit-panel context menu
- [ ] **CORE-038:** Slash-containing filename creates nested folders
- [ ] **CORE-039:** Delete a listed file through context menu
- [ ] **CORE-040:** View-all-files permits arbitrary repository-file selection
- [ ] **CORE-041:** File filter appears in view-all-files mode
- [ ] **CORE-077:** Edit from palette, context menu or diff
- [ ] **CORE-078:** New files automatically open in edit mode
- [ ] **CORE-079:** Editable marker and unsaved-change indicator
- [ ] **CORE-080:** Save, close/discard prompt and save-and-stage choices
- [ ] **CON-CI-001:** Create/edit/delete Actions YAML under .github/workflows and commit changes.
- [ ] Exercise a successful workflow and a material failure or unavailable state through the approved public workflow seam; compare against independently known results.
- [ ] Show real Git execution or provider activity once, with correct origin, repository/account context, result and redacted secrets; collapse the panel while the action runs and retain recording.
- [ ] Match the observed reference entry points, focus/keyboard behavior, disabled/loading/error states and preference scope for this workflow. Record unresolved reference evidence separately from implementation progress.
- [ ] Register the action in the applicable menus/palette/settings, update the coverage evidence and run the checks relevant to the changed behavior.

## Test and review approach

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. A draft acceptance checkbox does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-036, CORE-037, CORE-038, CORE-039, CORE-040, CORE-041, CORE-077, CORE-078, CORE-079, CORE-080, CON-CI-001.

Target one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary ticket owns complete coverage accounting, but its initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../research/core-parity.md), [connected audit](../../research/connected-parity.md), [runtime investigation](../../research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).
