# Desktop and local Git parity audit

Research date: October 30, 2026 (Asia/Tehran). Reference release: GitKraken Desktop 12.6.1. Research branch: research/core-parity. Research ticket: [Audit desktop interface and local Git feature coverage](../../.scratch/gitkraken-parity/issues/01-core-parity-audit.md).

## Result and limits

This inventory identifies **252 documented behavior checkpoints** across the desktop interface, local Git workflows, terminal, preferences, and platform integration. It is a planning baseline, not proof of exhaustive feature discovery, pixel equivalence, implementation, or successful execution. All rows have implementation status **U — unimplemented**. Evidence is **D — described in official documentation** unless explicitly marked **G — evidence gap/contradiction**. The verification column proposes how to establish parity; it does not report tests performed.

The official release notes identify 12.6.1, dated October 20, 2026. Current feature pages are live documentation, commonly dated March–September 2026, and do not supply an immutable 12.6.1 build snapshot. Their instructions and screenshots therefore need comparison with the actual pinned release. [Release notes][release]

Connected features such as providers, issues, Launchpad, workspaces, organization administration, AI, agents, Cloud Patches and conflict prevention are audited separately. References to their entry points below preserve interface coverage; they do not establish connected-feature completeness.

## Verification vocabulary

These proposed verification methods are engineering recommendations, not claims made by the reference vendor:

- **V**: capture the same screen/state in the pinned reference and implementation at controlled window size, DPI, theme and language; compare layout, icons, labels, focus, menus, tooltips and disabled states.
- **G**: use disposable Git fixture repositories and compare working files, index, refs, history, config and operation state before/after; inspect the real command log.
- **K**: exercise mouse, keyboard, focus, shortcuts, context menus and drag/drop; record event availability by context.
- **P**: restart/switch profile/reopen repository to check scope and persistence.
- **T**: run actual shells/processes, including long output, failure, input, cancellation and background continuation.
- **X**: validate external tool, Git extension, filesystem or operating-system boundary with an appropriate installed prerequisite.
- **R**: verify the behavior against release-specific evidence rather than relying solely on the live feature page.

Every behavioral fixture should also verify action-history recording while the panel is collapsed, and repository refresh after a terminal or external-process mutation. That recording requirement is the user's addition; the reference terminal guide does not establish that GitKraken executes every GUI action in its interactive shell. [Terminal][terminal]

## Interface and navigation

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-001 | Repository layout: left panel, graph, commit panel | V | [Interface][interface] |
| CORE-002 | Toolbar Git actions and conditional LFS button | V,K | [Interface][interface] |
| CORE-003 | Toolbar pull variants and default choice | K,P | [Interface][interface] |
| CORE-004 | Left sections collapse, resize, hide and maximize | V,K,P | [Interface][interface] |
| CORE-005 | List/Agents presentation toggle | V,K | [Interface][interface] |
| CORE-006 | Graph commit, merge, stash and WIP nodes | V,G | [Interface][interface] |
| CORE-007 | Selecting a commit or WIP changes detail panel | V,K | [Interface][interface] |
| CORE-008 | Ghost branch appears for containing reference | V,K | [Interface][interface] |
| CORE-009 | Ghost branch double-click checks out its head | G,K | [Interface][interface] |
| CORE-010 | Branch hover highlights related history | V,K | [Interface][interface] |
| CORE-011 | Reorder repository tabs with drag/drop | K,P | [Interface][interface] |
| CORE-012 | New/close/switch tabs; dropdown and path tooltip | V,K | [Interface][interface] |
| CORE-013 | Per-profile tab sets and repository aliases | P,V | [Interface][interface] |
| CORE-014 | Toggle, resize and reorder graph metadata columns | V,K,P | [Interface][interface] |
| CORE-015 | Author/team filtering through author column | V,K | [Interface][interface] |
| CORE-016 | Status bar summary and preset zoom levels | V,K | [Interface][interface] |
| CORE-017 | Palette: live filtering and Enter execution | K,V | [Palette][palette] |
| CORE-018 | Palette command availability follows application context | K,G | [Palette][palette] |
| CORE-019 | Palette covers files, Git, views, logs and settings | K | [Palette][palette] |
| CORE-020 | Detect installed IDEs and expose open commands | X,K | [Palette][palette] |
| CORE-021 | Shortcut dialog and platform-specific bindings | K,V | [Shortcuts][shortcuts] |
| CORE-022 | Stage/unstage/commit and message-focus shortcuts | K,G | [Shortcuts][shortcuts] |
| CORE-023 | Graph and branch keyboard traversal | K,V | [Shortcuts][shortcuts] |
| CORE-024 | Panel toggles, close, fullscreen and zoom shortcuts | K,V | [Shortcuts][shortcuts] |
| CORE-025 | Tab cycling, numbered selection and search focus | K,V | [Shortcuts][shortcuts] |

## Repository and file management

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-026 | Repository Management: browse, clone and initialize | V,K,G | [Open/clone/init][repository] |
| CORE-027 | Open repositories through New Tab and File menu | K,G | [Open/clone/init][repository] |
| CORE-028 | CLI opens current directory or explicit repository path | X,G | [Open/clone/init][repository] |
| CORE-029 | Clone from URL or connected-provider selection | G,X | [Open/clone/init][repository] |
| CORE-030 | Clone completion opens repository automatically | G,V | [Open/clone/init][repository] |
| CORE-031 | Shallow clone: branch, depth, date and custom flags | G,R | [Open/clone/init][repository] |
| CORE-032 | Open existing shallow repositories | G | [Open/clone/init][repository] |
| CORE-033 | Sparse clone paths and empty-path behavior | G,R | [Open/clone/init][repository] |
| CORE-034 | Initialize: path, ignore template and license options | G,V | [Open/clone/init][repository] |
| CORE-035 | Repository details/README and external-editor entry points | V,X | [Open/clone/init][repository] |
| CORE-036 | Create file through palette | G,K | [Files][files] |
| CORE-037 | Create file through commit-panel context menu | G,K | [Files][files] |
| CORE-038 | Slash-containing filename creates nested folders | G | [Files][files] |
| CORE-039 | Delete a listed file through context menu | G,K | [Files][files] |
| CORE-040 | View-all-files permits arbitrary repository-file selection | V,K | [Files][files] |
| CORE-041 | File filter appears in view-all-files mode | V,K | [Files][files] |

## Staging and committing

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-042 | WIP selects staged and unstaged file lists | V,G | [Staging][staging] |
| CORE-043 | Stage one file | G,K | [Staging][staging] |
| CORE-044 | Stage all changes | G,K | [Staging][staging] |
| CORE-045 | Stage selected lines or hunks | G,K | [Staging][staging] |
| CORE-046 | Unstage file, all, selected lines or hunks | G,K | [Staging][staging] |
| CORE-047 | Discard whole, multiple-file, line or hunk changes | G,K | [Staging][staging] |
| CORE-048 | Ignore by filename, extension or directory | G,K | [Staging][staging] |
| CORE-049 | Ignore tracked file versus ignore-and-stop-tracking | G | [Staging][staging] |
| CORE-050 | Documented root-only ignore handling needs build verification | G,R | [Staging][staging] |
| CORE-051 | Commit staged changes with entered message | G,K | [Commits][commits] |
| CORE-052 | Commit and push option | G,X | [Commits][commits] |
| CORE-053 | Co-author trailers and displayed attribution | G,V | [Commits][commits] |
| CORE-054 | Skip hooks for one commit | G,X | [Commits][commits] |
| CORE-055 | Commit template lookup: repository before global | G,P | [Commits][commits] |
| CORE-056 | Template editing creates/updates repository-local template | G,P | [Commits][commits] |
| CORE-057 | Apply template and remove-comment options | G,V | [Commits][commits] |
| CORE-058 | Amend previous commit with staged changes | G | [Commits][commits] |
| CORE-059 | Edit last commit message; update/cancel amend | G,K | [Commits][commits] |
| CORE-060 | Resize commit-message editor | V,K | [Commits][commits] |
| CORE-061 | Soft, mixed and hard reset variants | G | [Commits][commits] |
| CORE-062 | Revert commit creates reversing history entry | G | [Commits][commits] |
| CORE-063 | Restore one/multiple historical files as staged changes | G,R | [Commits][commits] |

## Diffs, history, blame and editing

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-064 | Diffs from WIP and selected historical commit | V,G | [Diffs][diff] |
| CORE-065 | Compare two commits | V,G | [Diffs][diff] |
| CORE-066 | Combined consecutive-commit changes, optionally including WIP | G,R | [Diffs][diff] |
| CORE-067 | Explain unsupported nonconsecutive multi-selection | V,R | [Diffs][diff] |
| CORE-068 | Hunk, inline and split diff modes | V,K | [Diffs][diff] |
| CORE-069 | Word differences, highlighting, mini-map and change navigation | V,K | [Diffs][diff] |
| CORE-070 | Wrap lines in file/diff/history/merge views | V,K | [Diffs][diff] |
| CORE-071 | Revert individual hunk | G | [Diffs][diff] |
| CORE-072 | Compare WIP against branch/commit | G,V | [Diffs][diff] |
| CORE-073 | File history and author-colored blame; file/diff toggle | V,G | [Diffs][diff] |
| CORE-074 | External diff tool or Git-configured default | X | [Diffs][diff] |
| CORE-075 | Create patch from selected files/commits; binary limitation | G,R | [Diffs][diff] |
| CORE-076 | Apply patch through palette | G,K | [Diffs][diff] |
| CORE-077 | Edit from palette, context menu or diff | K,G | [Editor][editor] |
| CORE-078 | New files automatically open in edit mode | V,G | [Editor][editor] |
| CORE-079 | Editable marker and unsaved-change indicator | V | [Editor][editor] |
| CORE-080 | Save, close/discard prompt and save-and-stage choices | K,G | [Editor][editor] |
| CORE-081 | Historical inspection edits current working-directory version | G | [Editor][editor] |
| CORE-082 | Encoding selection/guessing without encoding conversion on save | G,X | [Editor][editor] |
| CORE-083 | Markdown preview toggle for Markdown files | V | [Editor][editor] |

## Branches, conflicts and history rewriting

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-084 | Create branch at selected commit | G,K | [Branches][branches] |
| CORE-085 | Checkout via double-click or context menu | G,K | [Branches][branches] |
| CORE-086 | Rename branch via graph, left panel or palette | G,K | [Branches][branches] |
| CORE-087 | Delete branch after switching away | G,K | [Branches][branches] |
| CORE-088 | Multi-select and delete branches | G,K | [Branches][branches] |
| CORE-089 | Pin/unpin branch to graph's left | V,P | [Branches][branches] |
| CORE-090 | Smart visibility: current, target and upstream branches | V,G,R | [Branches][branches] |
| CORE-091 | Merge through context menu or drag/drop | G,K | [Branches][branches] |
| CORE-092 | Squash-merge preference stages result for manual commit | G,P,R | [Branches][branches] |
| CORE-093 | Conflicted files open merge tool | V,G | [Branches][branches] |
| CORE-094 | Merge viewer: current, incoming and output areas | V | [Branches][branches] |
| CORE-095 | Choose lines, edit output, navigate conflicts and search | G,K,V | [Branches][branches] |
| CORE-096 | Save and commit resolved merge output | G | [Branches][branches] |
| CORE-097 | Take current/incoming for a conflicted file | G,K | [Branches][branches] |
| CORE-098 | External merge-tool configuration/invocation | X | [Branches][branches] |
| CORE-099 | Rebase branch via drag/drop or context menu | G,K | [Branches][branches] |
| CORE-100 | Rebase selected range/pivot through branch head | G,R | [Branches][branches] |
| CORE-101 | Rebase onto a branchless selected commit | G,R | [Branches][branches] |
| CORE-102 | Interactive rebase from branch or parent commit | G,K | [Interactive rebase][interactive] |
| CORE-103 | Reject incompatible ancestry/source merges/initial-commit cases | G | [Interactive rebase][interactive] |
| CORE-104 | Interactive rebase pick action | G,K | [Interactive rebase][interactive] |
| CORE-105 | Interactive rebase reword dialog | G,V | [Interactive rebase][interactive] |
| CORE-106 | Interactive rebase squash into parent | G | [Interactive rebase][interactive] |
| CORE-107 | Interactive rebase drop action | G | [Interactive rebase][interactive] |
| CORE-108 | P/S/R/D action shortcuts and Reset | K,G | [Interactive rebase][interactive] |
| CORE-109 | App-started interactive rebase completion stays in app | G,T | [Interactive rebase][interactive] |
| CORE-110 | Cherry-pick selected commit onto current branch | G,K | [Cherry-pick][cherry] |
| CORE-111 | Cherry-pick branch HEAD through left-panel menu | G,K | [Cherry-pick][cherry] |
| CORE-112 | Multi-commit selection opens interactive cherry-pick | G,K | [Cherry-pick][cherry] |
| CORE-113 | Reorder cherry-picked commits with drag/drop | G,K | [Cherry-pick][cherry] |
| CORE-114 | Cherry-pick pick/reword/squash/drop choices | G,K | [Cherry-pick][cherry] |
| CORE-115 | Cherry-pick squash requires parent-child relationship | G | [Cherry-pick][cherry] |
| CORE-116 | Cherry-pick shortcuts and Reset without applying | K,G | [Cherry-pick][cherry] |
| CORE-117 | Squash selected consecutive linear commits | G,K | [Squash][squash] |
| CORE-118 | Oldest squash selection requires a parent | G | [Squash][squash] |
| CORE-119 | Squash unavailable for merge/nonlinear selections | V,G | [Squash][squash] |
| CORE-120 | Edit resulting squashed commit message | G,V | [Squash][squash] |
| CORE-121 | Diverged remote prompts force-push choice | G,X,V | [Squash][squash] |

## Stashes, recovery, search and visibility

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-122 | Toolbar creates stash and graph entry | G,V | [Stashes][stash] |
| CORE-123 | Apply stash retains entry | G | [Stashes][stash] |
| CORE-124 | Pop stash applies and removes entry | G | [Stashes][stash] |
| CORE-125 | Delete stash through graph or left panel | G,K | [Stashes][stash] |
| CORE-126 | Hide/show one or all stashes without deletion | V,G | [Stashes][stash] |
| CORE-127 | Name stash before creation | G,V | [Stashes][stash] |
| CORE-128 | Edit existing stash message | G,K | [Stashes][stash] |
| CORE-129 | Create partial stash from selected files | G,K | [Stashes][stash] |
| CORE-130 | Restore selected stash files while retaining entry | G | [Stashes][stash] |
| CORE-131 | Commit-panel stash tab and summary/description | V,G | [Stashes][stash] |
| CORE-132 | Undo latest supported operation; contextual availability | G,V | [Undo/redo][undo] |
| CORE-133 | Undo checkout, commit, discard, branch/remote removal, reset | G | [Undo/redo][undo] |
| CORE-134 | Undo rebase, interactive cherry-pick and history edits | G | [Undo/redo][undo] |
| CORE-135 | Redo undone action through toolbar/shortcuts | G,K | [Undo/redo][undo] |
| CORE-136 | Live commit search by message, SHA or author | V,G | [Search][search] |
| CORE-137 | Highlight matching commit in graph | V,K | [Search][search] |
| CORE-138 | Search limited by loaded history; graph-depth setting | G,P | [Search][search] |
| CORE-139 | Start search through global palette entry | K | [Search][search] |
| CORE-140 | Hide reference without repository mutation | V,G | [Hide/solo][hide] |
| CORE-141 | Restore hidden reference through icon | V,K | [Hide/solo][hide] |
| CORE-142 | Solo one or multiple references | V,K | [Hide/solo][hide] |
| CORE-143 | Hide/solo entire remotes | V,K | [Hide/solo][hide] |
| CORE-144 | Bulk show/hide by section header | V,K | [Hide/solo][hide] |

## Remotes, tags, submodules and worktrees

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-145 | Add remote using URL or integration fork selection | G,X | [Remotes][remotes] |
| CORE-146 | Additional shallow-remote depth/date/custom options | G,R | [Remotes][remotes] |
| CORE-147 | Remote provider icons | V | [Remotes][remotes] |
| CORE-148 | Push via toolbar, menu or drag/drop | G,K,X | [Remotes][remotes] |
| CORE-149 | Create/name missing remote branch during push | G,X,V | [Remotes][remotes] |
| CORE-150 | Force-push offer after divergence | G,X,V | [Remotes][remotes] |
| CORE-151 | Fetch/fetch-all and ahead/behind indicators | G,V | [Remotes][remotes] |
| CORE-152 | Configurable periodic fetch | G,P | [Remotes][remotes] |
| CORE-153 | Pull fast-forward/fallback-merge/only/rebase options | G | [Remotes][remotes] |
| CORE-154 | Set upstream and remember preferred pull method | G,P | [Remotes][remotes] |
| CORE-155 | Create lightweight tag at selected commit | G | [Tags][tags] |
| CORE-156 | Create annotated tag or annotate existing tag | G | [Tags][tags] |
| CORE-157 | Annotation hover in graph and left panel | V | [Tags][tags] |
| CORE-158 | Push tag to remote | G,X | [Tags][tags] |
| CORE-159 | Jump to tag; hide/solo tag | V,K | [Tags][tags] |
| CORE-160 | Create branch at tag | G,K | [Tags][tags] |
| CORE-161 | Tag move/fast-forward or recreate workflow | G,R | [Tags][tags] |
| CORE-162 | Delete tag locally/remotely; rename via recreate | G,X | [Tags][tags] |
| CORE-163 | Tag filtering and section visibility | V,K | [Tags][tags] |
| CORE-164 | Toggle submodules section | V,K | [Submodules][submodules] |
| CORE-165 | Add submodule URL/path and update .gitmodules | G,X | [Submodules][submodules] |
| CORE-166 | Initialize submodules when cloning parent | G,X | [Submodules][submodules] |
| CORE-167 | Update through submodule context menu | G,X | [Submodules][submodules] |
| CORE-168 | Open submodule, change checked-out target, return | G,V | [Submodules][submodules] |
| CORE-169 | Prompt to preserve updated submodule pointer | G,V | [Submodules][submodules] |
| CORE-170 | Uninitialized/out-of-sync/uncommitted states | G,V | [Submodules][submodules] |
| CORE-171 | Global/repository automatic submodule-update settings | G,P | [Submodules][submodules] |
| CORE-172 | Create worktree through branch context menu | G,K | [Worktrees][worktrees] |
| CORE-173 | Worktree list shares underlying entries with Agents view | G,V | [Worktrees][worktrees] |
| CORE-174 | Inherit hidden/solo/collapsed view settings on creation | P,V | [Worktrees][worktrees] |
| CORE-175 | Open worktree replacing view | G,K | [Worktrees][worktrees] |
| CORE-176 | Open worktree in separate repository tab | G,K | [Worktrees][worktrees] |
| CORE-177 | Open worktree through graph WIP entry | V,K | [Worktrees][worktrees] |
| CORE-178 | Remove worktree; optionally delete associated branch | G | [Worktrees][worktrees] |
| CORE-179 | Worktree lock/unlock controls | G,R | [Worktrees][worktrees] |
| CORE-180 | Display/search branch and differing folder name | V,K,R | [Worktrees][worktrees] |
| CORE-181 | Full path available on worktree hover | V | [Worktrees][worktrees] |

## Git extensions and signing

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-182 | Configure Gitflow branch names and prefixes | G,P | [Gitflow][gitflow] |
| CORE-183 | Initialize missing main/develop branches | G | [Gitflow][gitflow] |
| CORE-184 | Start feature/release/hotfix through Gitflow panel | G,K | [Gitflow][gitflow] |
| CORE-185 | Finish feature into develop and remove branch | G | [Gitflow][gitflow] |
| CORE-186 | Finish release into both base branches with tag | G | [Gitflow][gitflow] |
| CORE-187 | Finish hotfix into main and develop | G | [Gitflow][gitflow] |
| CORE-188 | Configure release-tag prefix and message | G,P | [Gitflow][gitflow] |
| CORE-189 | Initialize LFS for new/existing repositories | G,X | [LFS][lfs] |
| CORE-190 | Track file patterns through preferences/context menu | G,X | [LFS][lfs] |
| CORE-191 | LFS file label and pointer metadata display | V,G | [LFS][lfs] |
| CORE-192 | LFS pull after clone/submodule initialization | G,X | [LFS][lfs] |
| CORE-193 | LFS toolbar operations, including pull and prune | G,X | [LFS][lfs] |
| CORE-194 | Select/generate GPG key and program path | X,V | [Signing][signing] |
| CORE-195 | Default commit/tag signing toggles | G,P,X | [Signing][signing] |
| CORE-196 | Signed badge and detailed verification states | G,V,X | [Signing][signing] |
| CORE-197 | SSH signing format, public key and allowed-signers path | G,X | [Signing][signing] |
| CORE-198 | SSH signing requires Git executable mode | G,X,R | [Signing][signing] |
| CORE-199 | Copy public key for provider setup | X,K | [Signing][signing] |

## Terminal and preferences

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-200 | Open embedded terminal from toolbar/palette | T,K | [Terminal][terminal] |
| CORE-201 | Shell starts in active repository/worktree directory | T,G | [Terminal][terminal] |
| CORE-202 | Command/flag autocomplete | T,K | [Terminal][terminal] |
| CORE-203 | Windows PowerShell/Bash selection | T,P,X | [Terminal][terminal] |
| CORE-204 | macOS/Linux default-shell behavior | T,X | [Terminal][terminal] |
| CORE-205 | Font, size, line-height and cursor settings | V,P | [Terminal][terminal] |
| CORE-206 | Autocomplete configuration | T,P | [Terminal][terminal] |
| CORE-207 | Resizing and minimizing retains terminal header | V,T | [Terminal][terminal] |
| CORE-208 | Kill selected terminal session | T,K | [Terminal][terminal] |
| CORE-209 | Create, select and close multiple terminal tabs | T,K | [Terminal][terminal] |
| CORE-210 | Terminal sets follow repository/worktree selection | T,G | [Terminal][terminal] |
| CORE-211 | Processes continue when switching worktrees | T | [Terminal][terminal] |
| CORE-212 | Additional agent opens separate terminal tab | T,R | [Terminal][terminal] |
| CORE-213 | Drag files/text to insert terminal input | T,K | [Terminal][terminal] |
| CORE-214 | General fetch/prune/conflict/submodule settings | P,G | [Preferences][preferences] |
| CORE-215 | Default branch, merge-backup-file and graph-depth settings | P,G | [Preferences][preferences] |
| CORE-216 | Lazy history loading and remembered tabs | P,G | [Preferences][preferences] |
| CORE-217 | Windows longpaths/AutoCRLF settings | G,X,P | [Preferences][preferences] |
| CORE-218 | Extended logging and forget-credentials controls | P,X | [Preferences][preferences] |
| CORE-219 | Profile-scoped settings, tabs and integrations | P | [Preferences][preferences] |
| CORE-220 | SSH and hosting/issue integration settings entry points | V,X | [Preferences][preferences] |
| CORE-221 | External editor/diff/merge/terminal/custom-command settings | P,X | [Preferences][preferences] |
| CORE-222 | Coding-agent commands and status-plugin settings | P,X | [Preferences][preferences] |
| CORE-223 | Desktop and marketing notification settings | P,V | [Preferences][preferences] |
| CORE-224 | Locale/date, avatar and graph metadata customization | P,V | [Preferences][preferences] |
| CORE-225 | Hide Launchpad and toggle Agents presentation | P,V | [Preferences][preferences] |
| CORE-226 | Editor font/tab/EOL/highlight/line-number/wrap settings | P,V | [Preferences][preferences] |
| CORE-227 | Repository encoding/Gitflow/hooks/LFS/commit settings | P,G | [Preferences][preferences] |
| CORE-228 | Repository issue/Team View/submodule/agent setup settings | P,V | [Preferences][preferences] |
| CORE-229 | Built-in theme selection and system-theme following | P,V,X | [Themes][themes] |
| CORE-230 | G: custom theme support is contradicted by disablement notice | R | [Themes][themes] |

## Diagnostics and platform behavior

| ID | Documented checkpoint | Verify | Source |
| --- | --- | --- | --- |
| CORE-231 | Activity Logs footer panel | V,K | [Activity][activity] |
| CORE-232 | Application versus active-repository log scope | G,V | [Activity][activity] |
| CORE-233 | Entries show timestamp, action and duration | V,G | [Activity][activity] |
| CORE-234 | Extended logging and hook-output notification links | G,X,V | [Activity][activity] |
| CORE-235 | Copy deep link to repository/commit/branch/tag | X,K | [Deep links][links] |
| CORE-236 | Link navigates/open repository or offers prefilled clone | X,G | [Deep links][links] |
| CORE-237 | Multiple local copies trigger choice | X,V | [Deep links][links] |
| CORE-238 | Enable experimental features in preferences | P,V | [Experimental][experimental] |
| CORE-239 | Git executable mode and bundled/system binary selection | G,X,R | [Experimental][experimental] |
| CORE-240 | Cloud Patch create/share/apply entry points | V,X | [Experimental][experimental] |
| CORE-241 | Linux application runs inside WSL2 with WSLg | X,V | [WSL][wsl] |
| CORE-242 | Cross-filesystem warning and open-correct-instance choices | X,V | [WSL][wsl] |
| CORE-243 | WSL preferences for URL/file opening | X,P | [WSL][wsl] |
| CORE-244 | WSL versus Windows coding-agent executable context | X,T | [WSL][wsl] |
| CORE-245 | 12.6.1 terminal tab shortcuts and Terminal Tabs submenu | K,R | [Release notes][release] |
| CORE-246 | 12.6.1 selected terminal tab auto-scrolls into view | V,R | [Release notes][release] |
| CORE-247 | 12.6.1 terminal tab drag order retained until app close | K,R | [Release notes][release] |
| CORE-248 | 12.6.1 closing active terminal selects neighboring tab | K,T,R | [Release notes][release] |
| CORE-249 | 12.6.1 selecting terminal tab focuses terminal input | K,T,R | [Release notes][release] |
| CORE-250 | 12.6.1 Windows terminal labels follow running process | T,X,R | [Release notes][release] |
| CORE-251 | 12.6.1 rename staging preserves rename representation | G,R | [Release notes][release] |
| CORE-252 | 12.6.1 detached-HEAD reset/rebase undo preserves redo | G,R | [Release notes][release] |

## Additional directly reviewed workflows requiring child checkpoints

These paragraphs preserve findings without pretending the numbered inventory already covers every modal option or state. They must expand into additional stable IDs during specification. Existing IDs must not be renumbered.

- **Favorites:** star/unstar repositories in Repository Management and switch among up to nine favorites using platform-specific shortcuts. [Favorites][favorites]
- **Detached HEAD:** checkout a commit, show HEAD/warning, permit commits, preserve them by creating a branch, exit through branch checkout and offer Undo/reflog recovery context. Disappearing from the normal graph must not be described as immediate Git object deletion. [Detached HEAD][detached]
- **Hooks:** default/custom/global hook paths; executable-bit requirement on Unix; nonzero exits block operations; per-commit bypass; documented action-specific hooks include pre-commit, prepare-commit-msg, commit-msg, post-commit, pre-rebase, post-checkout, post-merge, post-rewrite and pre-push. Hook output must remain visible. [Hooks][hooks]

## Evidence gaps and contradictions to resolve

1. **Reference execution evidence:** this investigation read official documentation; it did not launch or interact with a 12.6.1 reference installation. The separate runtime investigation owns the read-only local installation check. Exact edition, build distribution, enabled features and connected accounts remain part of acceptance planning.
2. **Visual parity:** screenshots and animated demonstrations exist on many pages, but this report did not measure their typography, dimensions, colors or animation. Native menus, every modal, context-menu ordering, loading/empty/error states, responsive layout and accessibility tree need an actual reference capture ledger.
3. **Shortcut conflict:** the interface guide assigns Cmd/Ctrl+K to toggling the left panel, while the dedicated shortcut reference assigns Cmd/Ctrl+J to that panel and Cmd/Ctrl+K to Commit Detail. The diff guide alternates Cmd/Ctrl+P and Cmd/Ctrl+Shift+P for the palette. Bindings must be observed in the pinned build. [Interface][interface], [Shortcuts][shortcuts], [Diffs][diff]
4. **Theme conflict:** the same page documents custom theme creation/live reload and says custom themes were disabled starting in 11.8.0. CORE-230 is an unresolved evidence item, not a claim that custom themes are active in 12.6.1. [Themes][themes]
5. **Documentation semantics need scrutiny:** the staging page claims root-only ignore handling; tag instructions describe rename/move workarounds and overstate Git tag immutability; worktree documentation overstates what locking protects; detached-HEAD prose calls unbranched commits lost/removed. Observe the application and consult canonical Git semantics before adopting these explanations. [Staging][staging], [Tags][tags], [Worktrees][worktrees], [Detached HEAD][detached]
6. **Settings location conflicts:** merge/diff tool guides mention General, while current preferences lists External Tools. Exact current routing requires reference verification. [Branches][branches], [Diffs][diff], [Preferences][preferences]
7. **Release drift:** sparse checkout, restore-file, smart visibility, selected-range rebase and some current feature descriptions need release provenance checks. A live page can describe behavior introduced or altered outside the pinned release. Such rows are marked R where especially material, but R is recommended across the inventory.
8. **Unsupported/error behavior:** invalid names, locked refs, permission failures, dirty checkout, conflicts during stash/cherry-pick/rebase, missing tools, network/auth errors, binary/large/non-UTF-8 files, symlinks, submodule deletion and external concurrent mutations need a reference-state audit. Documentation does not fully establish exact recovery controls or behavior.
9. **Whole-product coverage:** native File/Edit/View/Help menus, startup/onboarding, updates, diagnostics, settings suboptions, account/edition UI and all experimental states still need an observed menu-to-action census. The palette is a discovery aid, not proof every action is listed there. No omitted area is ruled out of scope.

## Proposed acceptance assets

Create an evidence ledger keyed by stable ID, with reference build/edition/platform, prerequisite fixture, entry point, expected result, screenshots or recording, implementation test, result and unresolved differences. Add IDs for every newly observed control/state; mark disabled/removed reference behaviors explicitly rather than building old documentation accidentally.

Suggested disposable fixtures: empty/unborn repository; linear/merge/octopus history; detached HEAD; dirty/staged/partially staged files; renames and large change sets; reversible discard; local bare remotes with divergence; shallow/sparse clones; linked worktrees; nested submodules; LFS with a local test endpoint; hooks that pass/fail/produce output; GPG/SSH test keys; invalid encodings and unusual paths. These are proposed tests, not tests executed in this research.

Graph acceptance should include deterministic topology, selection ranges, ref highlighting, visibility combinations and large-history behavior. Measure performance on named fixtures and hardware before setting budgets; no performance target or exact graph layout algorithm is established by this report.

## Primary sources reviewed

All sources below are first-party GitKraken Desktop documentation, read October 30, 2026. Claims are concise paraphrases, with fewer than 200 derived words per page in this document. Sources establish documented behavior, not successful reference execution.

[interface]: https://help.gitkraken.com/gitkraken-desktop/interface/
[release]: https://help.gitkraken.com/gitkraken-desktop/current/
[palette]: https://help.gitkraken.com/gitkraken-desktop/command-palette/
[shortcuts]: https://help.gitkraken.com/gitkraken-desktop/keyboard-shortcuts/
[repository]: https://help.gitkraken.com/gitkraken-desktop/open-clone-init/
[files]: https://help.gitkraken.com/gitkraken-desktop/adding-and-removing/
[staging]: https://help.gitkraken.com/gitkraken-desktop/staging/
[commits]: https://help.gitkraken.com/gitkraken-desktop/commits/
[diff]: https://help.gitkraken.com/gitkraken-desktop/diff/
[editor]: https://help.gitkraken.com/gitkraken-desktop/editing-files/
[branches]: https://help.gitkraken.com/gitkraken-desktop/branching-and-merging/
[interactive]: https://help.gitkraken.com/gitkraken-desktop/interactive-rebase/
[cherry]: https://help.gitkraken.com/gitkraken-desktop/cherrypick/
[squash]: https://help.gitkraken.com/gitkraken-desktop/squash/
[stash]: https://help.gitkraken.com/gitkraken-desktop/stashing/
[undo]: https://help.gitkraken.com/gitkraken-desktop/undo-and-redo/
[search]: https://help.gitkraken.com/gitkraken-desktop/search/
[hide]: https://help.gitkraken.com/gitkraken-desktop/hiding-and-soloing/
[remotes]: https://help.gitkraken.com/gitkraken-desktop/pushing-and-pulling/
[tags]: https://help.gitkraken.com/gitkraken-desktop/tags/
[submodules]: https://help.gitkraken.com/gitkraken-desktop/submodules/
[worktrees]: https://help.gitkraken.com/gitkraken-desktop/worktrees/
[gitflow]: https://help.gitkraken.com/gitkraken-desktop/git-flow/
[lfs]: https://help.gitkraken.com/gitkraken-desktop/git-lfs/
[signing]: https://help.gitkraken.com/gitkraken-desktop/commit-signing-with-gpg/
[terminal]: https://help.gitkraken.com/gitkraken-desktop/terminal/
[preferences]: https://help.gitkraken.com/gitkraken-desktop/preferences/
[themes]: https://help.gitkraken.com/gitkraken-desktop/themes/
[activity]: https://help.gitkraken.com/gitkraken-desktop/activity-logs/
[links]: https://help.gitkraken.com/gitkraken-desktop/linking/
[experimental]: https://help.gitkraken.com/gitkraken-desktop/experimental-features/
[wsl]: https://help.gitkraken.com/gitkraken-desktop/windows-subsystem-for-linux/
[favorites]: https://help.gitkraken.com/gitkraken-desktop/favorites/
[detached]: https://help.gitkraken.com/gitkraken-desktop/detached-head-state/
[hooks]: https://help.gitkraken.com/gitkraken-desktop/githooks/
