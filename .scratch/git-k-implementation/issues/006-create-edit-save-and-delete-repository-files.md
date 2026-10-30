# 6: Create, edit, save and delete repository files

Status: blocked
Label: ready-for-agent
Assignee: unassigned
Approved: 2026-10-30 — breakdown and dependency structure; open contracts remain gated.

Kind: vertical-slice

**Phase:** B — Repository files, graph and navigation

**What to build:** Create files and nested directories, browse/filter the tree, edit content, save or discard pending edits, and stage saved work including workflow YAML.

Blocked by: [2: Inspect changed and historical files](002-inspect-changed-and-historical-files.md); [3: Stage files and commit real changes](003-stage-files-and-commit-real-changes.md)

Decision gates: [G5: Execution and repository coordination contract](../../gitkraken-parity/issues/05-execution-contract.md); [G8: Git distribution and native platform support](../../gitkraken-parity/issues/08-native-support.md)

Verification gates: [G7: Matching reference build and feature access](../../gitkraken-parity/issues/07-reference-access.md); [G4: Complete parity acceptance](../../gitkraken-parity/issues/04-parity-acceptance.md). These gate final parity verification; unresolved behavior also holds implementation where it cannot be specified honestly.

Spec: [Full-parity implementation specification](../spec.md). Confirmed requirements and unresolved decision gates take precedence over proposed details.

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

Use the user-approved complete-workflow seam. Drive one failing behavior check, implement its narrow path, then repeat. Use real disposable Git repositories for Git state, deterministic provider adapters for failure cases, and live account checks when required. Test public outcomes rather than private implementation structure. Run appropriate type/build/workflow checks, then separate Standards and Spec reviews before committing. An unchecked acceptance criterion does not mean a test has run.

## Coverage and sizing

Primary requirement IDs: CORE-036, CORE-037, CORE-038, CORE-039, CORE-040, CORE-041, CORE-077, CORE-078, CORE-079, CORE-080, CON-CI-001.

Approved size target: one fresh implementation context. If reference observation or integration complexity reveals a larger task, split it into independently demoable child slices, preserve the listed coverage and update blocking edges before starting.

A source checkpoint can contain multiple subcases. The primary owner keeps complete coverage accounting, but an initial delivery must not be used to mark future platform/provider subcases verified. Dependent workflow tickets supply those subcases; the final acceptance gate checks every record independently.

## Evidence inputs

[Core audit](../../../docs/research/core-parity.md), [connected audit](../../../docs/research/connected-parity.md), [runtime investigation](../../../docs/research/runtime-architecture.md), [coverage](../coverage.md), and [specification](../spec.md).

## Publication

Published from the user-approved breakdown on October 30, 2026. The reviewed planning snapshot is commit 54a029b8beaad7203adcca2c44631c96b1c6b798. This file now owns the work state and acceptance criteria.
