use super::*;

static FIXTURE: AtomicU64 = AtomicU64::new(0);
struct Fixture { runtime: Runtime, root: PathBuf }
impl Fixture {
    fn new() -> Self {
        let parent = std::env::var_os("GIT_K_REF_TEST_ROOT").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join("git-k-reference-tests"));
        fs::create_dir_all(&parent).unwrap();
        let root = parent.join(format!("fixture-{}-{}-{}", std::process::id(), now(), FIXTURE.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&root).unwrap();
        let f = Self { runtime: Runtime::default(), root };
        f.git(&["init", "-b", "main"]);
        let hooks = f.root.join(".git/test-hooks"); fs::create_dir(&hooks).unwrap();
        for (key, value) in [("user.name", "Reference Fixture"), ("user.email", "reference@git-k.local"), ("commit.gpgsign", "false"), ("tag.gpgsign", "false"), ("core.autocrlf", "false"), ("core.fsmonitor", "false"), ("core.hooksPath", &display_path(&hooks))] { f.git(&["config", "--local", key, value]); }
        fs::write(f.root.join("file.txt"), "baseline\n").unwrap();
        f.git(&["add", "--", "file.txt"]); f.git(&["commit", "-m", "Baseline"]);
        f
    }
    fn git(&self, args: &[&str]) -> String { self.runtime.git(&self.root, args).unwrap() }
    fn req(&self, action: &str, name: &str, original: Option<&str>) -> RefRequest {
        let head = self.git(&["rev-parse", "HEAD"]).trim().into();
        let expected_oid = original.map(|name| self.git(&["show-ref", "--hash", "--verify", &format!("refs/{}/{name}", if action == "delete-tag" { "tags" } else { "heads" })]).trim().into());
        let branch = self.git(&["branch", "--show-current"]);
        RefRequest { path: display_path(&self.root), expected_head: head, expected_branch: if branch.trim().is_empty() { "Detached HEAD".into() } else { branch.trim().into() }, action: action.into(), name: name.into(), original: original.map(String::from), expected_oid, start_point: Some(self.git(&["rev-parse", "HEAD"]).trim().into()), annotated: false, message: None }
    }
    fn apply(&self, request: RefRequest) -> Result<CommitOutcome, String> { execute_ref_action(self.runtime.clone(), request) }
    fn exists(&self, name: &str) -> bool { self.runtime.git(&self.root, &["show-ref", "--verify", "--quiet", name]).is_ok() }
    fn dirty(&self) {
        fs::write(self.root.join("file.txt"), "staged\n").unwrap(); self.git(&["add", "--", "file.txt"]);
        fs::write(self.root.join("file.txt"), "working\n").unwrap(); fs::write(self.root.join("untracked.txt"), "keep\n").unwrap();
    }
    fn assert_dirty(&self) {
        assert_eq!(self.git(&["show", ":file.txt"]), "staged\n");
        assert_eq!(fs::read_to_string(self.root.join("file.txt")).unwrap(), "working\n");
        assert_eq!(fs::read_to_string(self.root.join("untracked.txt")).unwrap(), "keep\n");
    }
    fn hook(&self, name: &str, script: &str) {
        let path = self.root.join(".git/test-hooks").join(name);
        fs::write(&path, script).unwrap();
        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
}

#[test]
fn rename_and_merged_delete_preserve_work_and_config() {
    let f = Fixture::new(); f.git(&["branch", "spare"]); f.git(&["config", "branch.main.description", "preserve description"]); f.dirty();
    let request = f.req("rename-branch", "renamed&literal", Some("main"));
    let head = request.expected_head.clone();
    let outcome = f.apply(request).unwrap(); assert!(outcome.refresh_error.is_none());
    assert_eq!(f.git(&["branch", "--show-current"]).trim(), "renamed&literal");
    assert_eq!(f.git(&["config", "branch.renamed&literal.description"]).trim(), "preserve description");
    assert!(!f.exists("refs/heads/main")); assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), head);
    assert!(f.apply(f.req("rename-branch", "spare", Some("renamed&literal"))).is_err());
    assert!(f.apply(f.req("delete-branch", "", Some("renamed&literal"))).err().unwrap().contains("Switch away"));
    f.apply(f.req("delete-branch", "", Some("spare"))).unwrap(); assert!(!f.exists("refs/heads/spare")); f.assert_dirty();
    let events = f.runtime.events.lock().unwrap();
    assert!(events.iter().any(|event| event.kind == "command" && event.text.contains("branch") && event.text.contains("--delete")));
    assert!(events.iter().any(|event| event.kind == "completed"));
}

#[test]
fn unmerged_worktree_stale_and_locked_refs_are_refused() {
    let f = Fixture::new();
    f.git(&["switch", "-c", "topic"]); fs::write(f.root.join("file.txt"), "topic\n").unwrap(); f.git(&["add", "--", "file.txt"]); f.git(&["commit", "-m", "Unmerged topic"]); f.git(&["switch", "main"]);
    assert!(f.apply(f.req("delete-branch", "", Some("topic"))).is_err()); assert!(f.exists("refs/heads/topic"));
    f.git(&["branch", "held"]); let linked = f.root.join("linked");
    f.git(&["worktree", "add", &display_path(&linked), "held"]);
    assert!(f.apply(f.req("rename-branch", "held-new", Some("held"))).err().unwrap().contains("another worktree"));
    assert!(f.apply(f.req("delete-branch", "", Some("held"))).is_err()); assert!(f.exists("refs/heads/held"));
    let duplicate_current = f.root.join("current-linked");
    f.git(&["worktree", "add", "--force", &display_path(&duplicate_current), "main"]);
    assert!(f.apply(f.req("rename-branch", "blocked-main", Some("main"))).err().unwrap().contains("another worktree"));
    let stale = f.req("delete-branch", "", Some("topic")); let main = f.git(&["rev-parse", "main"]);
    f.git(&["update-ref", "refs/heads/topic", main.trim()]);
    assert!(f.apply(stale).err().unwrap().contains("selected reference changed")); assert!(f.exists("refs/heads/topic"));
    let source_stale = f.req("rename-branch", "must-not-exist", Some("topic")); f.git(&["branch", "--move", "main", "external"]);
    assert!(f.apply(source_stale).err().unwrap().contains("branch or HEAD changed")); assert!(!f.exists("refs/heads/must-not-exist"));
    fs::write(f.root.join(".git/refs/heads/topic.lock"), "fixture lock").unwrap();
    assert!(f.apply(f.req("delete-branch", "", Some("topic"))).is_err()); assert!(f.exists("refs/heads/topic"));
    assert_eq!(fs::read_to_string(f.root.join(".git/refs/heads/topic.lock")).unwrap(), "fixture lock");
}

#[test]
fn tags_preserve_literal_annotations_and_reject_stale_deletion() {
    let f = Fixture::new(); f.dirty();
    f.git(&["config", "tag.gpgsign", "true"]); f.git(&["config", "gpg.program", "git-k-nonexistent-signer"]);
    f.apply(f.req("create-tag", "main", None)).unwrap(); // Lightweight ignores signing by definition.
    let mut signed = f.req("create-tag", "signed-failure", None); signed.annotated = true; signed.message = Some("Needs signing".into());
    assert!(f.apply(signed).is_err()); assert!(!f.exists("refs/tags/signed-failure"));
    f.git(&["config", "tag.gpgsign", "false"]);
    let mut annotated = f.req("create-tag", "v1&literal", None); annotated.annotated = true;
    let message = "Release \"quotes\" & $(literal) `tick` — فارسی\n\nSecond line\n";
    annotated.message = Some(message.into()); f.apply(annotated).unwrap();
    assert_eq!(f.git(&["cat-file", "-t", "refs/tags/v1&literal"]).trim(), "tag");
    assert!(f.git(&["cat-file", "-p", "refs/tags/v1&literal"]).ends_with(message));
    assert!(f.apply(f.req("create-tag", "v1&literal", None)).is_err());
    assert!(f.apply(f.req("create-tag", "bad..tag", None)).is_err());
    let snapshot = snapshot(f.runtime.clone(), display_path(&f.root)).unwrap();
    assert!(snapshot.branches.iter().any(|r| r.name == "main")); assert!(snapshot.tags.iter().any(|r| r.name == "main"));
    let stale = f.req("delete-tag", "", Some("v1&literal")); let head = f.git(&["rev-parse", "HEAD"]);
    f.git(&["update-ref", "refs/tags/v1&literal", head.trim()]);
    assert!(f.apply(stale).err().unwrap().contains("selected reference changed")); assert!(f.exists("refs/tags/v1&literal"));
    f.apply(f.req("delete-tag", "", Some("v1&literal"))).unwrap(); assert!(!f.exists("refs/tags/v1&literal"));
    assert!(f.exists("refs/tags/main")); assert!(f.exists("refs/heads/main")); f.assert_dirty();
}

#[test]
fn completed_ref_change_survives_a_refresh_failure() {
    let f = Fixture::new();
    f.hook("reference-transaction", "#!/bin/sh\nif [ \"$1\" = committed ]; then\n git config --local core.worktree /git-k-reference-missing-worktree\nfi\n");
    let result = f.apply(f.req("create-tag", "created-before-read-failed", None)).unwrap();
    assert!(result.snapshot.is_none()); assert!(result.refresh_error.is_some());
    let worktree = format!("core.worktree={}", display_path(&f.root));
    assert!(f.runtime.git(&f.root, &["-c", &worktree, "show-ref", "--verify", "refs/tags/created-before-read-failed"]).is_ok());
}

#[test]
fn checkout_commit_detaches_without_changing_branches_or_local_files() {
    let f = Fixture::new(); f.dirty();
    let request = f.req("checkout-commit", "", None);
    let head = request.expected_head.clone();
    let outcome = f.apply(request).unwrap();
    let state = outcome.snapshot.unwrap();
    assert_eq!(state.branch, "Detached HEAD"); assert_eq!(state.head, head);
    assert_eq!(f.git(&["branch", "--show-current"]), "");
    assert_eq!(f.git(&["rev-parse", "refs/heads/main"]).trim(), head);
    f.assert_dirty();
    let events = f.runtime.events.lock().unwrap();
    assert!(events.iter().any(|event| event.kind == "command" && event.text.contains("switch") && event.text.contains("--detach")));
}

#[test]
fn detached_commits_remain_visible_in_the_history_snapshot() {
    let f = Fixture::new(); let baseline = f.git(&["rev-parse", "HEAD"]).trim().to_string();
    f.apply(f.req("checkout-commit", "", None)).unwrap();
    fs::write(f.root.join("detached.txt"), "detached work\n").unwrap();
    fs::write(f.root.join("untracked.txt"), "keep\n").unwrap();
    f.git(&["add", "--", "detached.txt"]);
    let outcome = execute_commit(f.runtime.clone(), display_path(&f.root), baseline.clone(), "Detached HEAD".into(), "Detached work".into(), "Description".into(), false).unwrap();
    let state = outcome.snapshot.unwrap(); let head = state.head;
    assert!(state.commits.iter().any(|commit| commit.sha == head), "Detached HEAD must remain visible even when no branch contains it");
    assert_eq!(f.git(&["show", "HEAD:detached.txt"]), "detached work\n");
    assert_eq!(f.git(&["rev-parse", "refs/heads/main"]).trim(), baseline);
    let saved = execute_branch(f.runtime.clone(), display_path(&f.root), head.clone(), "Detached HEAD".into(), "saved&literal".into(), Some(head.clone()), true).unwrap();
    assert_eq!(saved.snapshot.unwrap().branch, "saved&literal");
    let returned = execute_branch(f.runtime.clone(), display_path(&f.root), head.clone(), "saved&literal".into(), "main".into(), None, true).unwrap();
    assert_eq!(returned.snapshot.unwrap().branch, "main");
    assert_eq!(f.git(&["rev-parse", "refs/heads/saved&literal"]).trim(), head);
    assert_eq!(f.git(&["show", "refs/heads/saved&literal:detached.txt"]), "detached work\n");
    assert_eq!(fs::read_to_string(f.root.join("untracked.txt")).unwrap(), "keep\n");
}

#[test]
fn commit_checkout_refuses_stale_sources_invalid_targets_and_unfinished_operations() {
    let f = Fixture::new(); let stale = f.req("checkout-commit", "", None);
    f.git(&["switch", "-c", "external"]);
    assert!(f.apply(stale).err().unwrap().contains("branch or HEAD changed"));
    for target in ["HEAD", "--force", &"f".repeat(40), f.git(&["rev-parse", "HEAD:file.txt"]).trim()] {
        let mut request = f.req("checkout-commit", "", None); request.start_point = Some(target.into());
        assert!(f.apply(request).is_err());
        assert_eq!(f.git(&["branch", "--show-current"]).trim(), "external");
    }
    fs::write(f.root.join(".git/MERGE_HEAD"), f.git(&["rev-parse", "HEAD"])).unwrap();
    assert!(f.apply(f.req("checkout-commit", "", None)).err().unwrap().contains("unfinished merge"));
    assert_eq!(f.git(&["branch", "--show-current"]).trim(), "external");
}

#[test]
fn commit_checkout_preserves_tracked_untracked_and_ignored_collisions() {
    let f = Fixture::new(); let baseline = f.git(&["rev-parse", "HEAD"]);
    fs::write(f.root.join("file.txt"), "different history\n").unwrap();
    f.git(&["add", "--", "file.txt"]); f.git(&["commit", "-m", "Different history"]); f.dirty();
    let mut request = f.req("checkout-commit", "", None); request.start_point = Some(baseline.trim().into());
    assert!(f.apply(request).is_err()); f.assert_dirty();
    assert_eq!(f.git(&["branch", "--show-current"]).trim(), "main");
    for ignored in [false, true] {
        let f = Fixture::new(); let baseline = f.git(&["rev-parse", "HEAD"]);
        fs::write(f.root.join("collision.txt"), "committed\n").unwrap();
        f.git(&["add", "--", "collision.txt"]); f.git(&["commit", "-m", "Track collision"]);
        let target = f.git(&["rev-parse", "HEAD"]);
        f.git(&["switch", "--detach", baseline.trim()]);
        if ignored { fs::write(f.root.join(".git/info/exclude"), "collision.txt\n").unwrap(); }
        fs::write(f.root.join("collision.txt"), "local work\n").unwrap();
        let mut request = f.req("checkout-commit", "", None); request.start_point = Some(target.trim().into());
        assert!(f.apply(request).is_err());
        assert_eq!(f.git(&["rev-parse", "HEAD"]), baseline);
        assert_eq!(fs::read_to_string(f.root.join("collision.txt")).unwrap(), "local work\n");
    }
}

#[test]
fn leaving_detached_head_retains_reflog_recovery_context() {
    let f = Fixture::new(); f.apply(f.req("checkout-commit", "", None)).unwrap();
    f.git(&["commit", "--allow-empty", "-m", "Unattached work"]);
    let head = f.git(&["rev-parse", "HEAD"]).trim().to_string();
    execute_branch(f.runtime.clone(), display_path(&f.root), head.clone(), "Detached HEAD".into(), "main".into(), None, true).unwrap();
    assert_eq!(f.git(&["rev-parse", "HEAD@{1}"]).trim(), head);
    let mut request = f.req("checkout-commit", "", None); request.start_point = Some(head.clone());
    assert_eq!(f.apply(request).unwrap().snapshot.unwrap().head, head);
}

#[test]
fn detached_checkout_reports_hook_errors_and_successful_writes_before_failed_reads() {
    let f = Fixture::new();
    f.hook("post-checkout", "#!/bin/sh\necho 'fixture hook rejected after switch' >&2\nexit 1\n");
    assert!(f.apply(f.req("checkout-commit", "", None)).err().unwrap().contains("fixture hook rejected"));
    assert_eq!(f.git(&["branch", "--show-current"]), "");
    assert!(f.runtime.events.lock().unwrap().iter().any(|event| event.kind == "failed" && event.text.contains("exit 1")));
    let f = Fixture::new(); let head = f.git(&["rev-parse", "HEAD"]);
    f.hook("post-checkout", "#!/bin/sh\ngit config --local core.worktree /git-k-detached-missing-worktree\n");
    let outcome = f.apply(f.req("checkout-commit", "", None)).unwrap();
    assert!(outcome.snapshot.is_none()); assert!(outcome.refresh_error.is_some());
    let worktree = format!("core.worktree={}", display_path(&f.root));
    assert_eq!(f.git(&["-c", &worktree, "branch", "--show-current"]), "");
    assert_eq!(f.git(&["-c", &worktree, "rev-parse", "HEAD"]), head);
}

#[cfg(windows)]
fn powershell(script: &str, root: &Path) -> std::process::Output {
    let mut command = Command::new("powershell.exe");
    let script = format!("{script}; exit $LASTEXITCODE");
    command.args(["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", &script]).current_dir(root);
    use std::os::windows::process::CommandExt;
    command.creation_flags(0x08000000);
    command.output().unwrap()
}

#[cfg(windows)]
#[test]
fn displayed_arguments_round_trip_through_powershell() {
    let f = Fixture::new();
    let values = ["@draft", "--%", "name@example.com", "literal'quote", "$(value)", "with space", "literal\"quote", "space and \"quote\"", "with space\\", "with space\\\\", "line1\nline2", "percent%PATH%", ""];
    for value in values {
        let display = command_display("git", &["--no-pager", "config", "--local", "review.roundtrip", value]);
        let script = format!("$draft = @('wrong', 'expanded'); {display}");
        let output = powershell(&script, &f.root);
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        assert_eq!(f.git(&["config", "--get", "review.roundtrip"]), format!("{value}\n"));
    }
}

#[cfg(windows)]
#[test]
fn displayed_quoted_executable_runs_in_powershell() {
    let f = Fixture::new();
    let located = Command::new("where.exe").arg("git").output().unwrap();
    assert!(located.status.success());
    let executable = String::from_utf8(located.stdout).unwrap().lines().next().unwrap().to_string();
    let display = command_display(&executable, &["--no-pager", "check-ref-format", "--branch", "@draft"]);
    let script = format!("$draft = @('wrong', 'expanded'); {display}");
    let output = powershell(&script, &f.root);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "@draft");
    let failed = command_display(&executable, &["rev-parse", "--verify", "refs/heads/missing-review-reference"]);
    let output = powershell(&failed, &f.root);
    assert_eq!(output.status.code(), Some(128));
}

fn detach_request(f: &Fixture, target: &str) -> RefRequest {
    let mut request = f.req("checkout-commit", "", None);
    request.expected_branch = snapshot(f.runtime.clone(), display_path(&f.root)).unwrap().branch;
    request.start_point = Some(target.into());
    request
}

#[test]
fn detached_checkout_preserves_work_and_loads_unreferenced_head() {
    let f = Fixture::new(); let base = f.git(&["rev-parse", "HEAD"]).trim().to_string(); f.dirty();
    let outcome = f.apply(detach_request(&f, &base)).unwrap();
    assert_eq!(outcome.snapshot.unwrap().branch, "Detached HEAD"); f.assert_dirty();
    f.git(&["commit", "-m", "Detached work"]);
    let tip = f.git(&["rev-parse", "HEAD"]).trim().to_string();
    assert_ne!(tip, base); assert_eq!(f.git(&["rev-parse", "main"]).trim(), base);
    let view = snapshot(f.runtime.clone(), display_path(&f.root)).unwrap();
    assert_eq!(view.head, tip); assert!(view.commits.iter().any(|c| c.sha == tip));
    // Existing branch-preservation Git workflow, with an independent object check.
    f.git(&["switch", "--no-track", "-c", "preserved", "--", &tip]);
    assert_eq!(f.git(&["rev-parse", "refs/heads/preserved"]).trim(), tip);
    assert_eq!(fs::read_to_string(f.root.join("file.txt")).unwrap(), "working\n");
    let events = f.runtime.events.lock().unwrap();
    assert!(events.iter().any(|e| e.kind == "command" && e.text.contains("--detach")));
    assert!(events.iter().any(|e| e.kind == "completed"));
}

#[test]
fn detached_checkout_refuses_dirty_stale_invalid_and_unfinished_states() {
    let f = Fixture::new(); let base = f.git(&["rev-parse", "HEAD"]).trim().to_string();
    fs::write(f.root.join("file.txt"), "new commit\n").unwrap(); f.git(&["add", "file.txt"]); f.git(&["commit", "-m", "New tip"]);
    let tip = f.git(&["rev-parse", "HEAD"]).trim().to_string(); f.dirty();
    assert!(f.apply(detach_request(&f, &base)).is_err()); f.assert_dirty();
    assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), tip);
    assert_eq!(f.git(&["branch", "--show-current"]).trim(), "main");
    assert!(f.apply(detach_request(&f, "--force")).is_err());
    assert!(f.apply(detach_request(&f, &"0".repeat(40))).is_err());
    let stale = detach_request(&f, &base); f.git(&["branch", "--move", "renamed"]);
    assert!(f.apply(stale).err().unwrap().contains("branch or HEAD changed"));
    fs::write(f.root.join(".git/MERGE_HEAD"), &base).unwrap();
    assert!(f.apply(detach_request(&f, &base)).err().unwrap().contains("unfinished"));
    assert_eq!(f.git(&["branch", "--show-current"]).trim(), "renamed"); f.assert_dirty();
}

#[test]
fn detached_checkout_reconciles_hook_error_and_refresh_failure() {
    let f = Fixture::new(); let base = f.git(&["rev-parse", "HEAD"]).trim().to_string();
    f.hook("post-checkout", "#!/bin/sh\nexit 1\n");
    assert!(f.apply(detach_request(&f, &base)).is_err());
    assert_eq!(snapshot(f.runtime.clone(), display_path(&f.root)).unwrap().branch, "Detached HEAD");
    f.hook("post-checkout", "#!/bin/sh\ngit config --local core.worktree /git-k-detached-missing-worktree\n");
    let outcome = f.apply(detach_request(&f, &base)).unwrap();
    assert!(outcome.snapshot.is_none()); assert!(outcome.refresh_error.is_some());
    let worktree = format!("core.worktree={}", display_path(&f.root));
    assert_eq!(f.git(&["-c", &worktree, "rev-parse", "HEAD"]).trim(), base);
    assert!(f.git(&["-c", &worktree, "branch", "--show-current"]).trim().is_empty());
}

#[test]
fn detached_checkout_protects_untracked_and_ignored_collisions() {
    for ignored in [false, true] {
        let f = Fixture::new();
        let base = f.git(&["rev-parse", "HEAD"]).trim().to_string();
        fs::write(f.root.join("collision.txt"), "committed target\n").unwrap();
        f.git(&["add", "--", "collision.txt"]); f.git(&["commit", "-m", "Target with collision"]);
        let target = f.git(&["rev-parse", "HEAD"]).trim().to_string();
        f.git(&["switch", "-c", "source", &base]);
        if ignored { fs::write(f.root.join(".git/info/exclude"), "collision.txt\n").unwrap(); }
        fs::write(f.root.join("collision.txt"), "local contents to keep\n").unwrap(); f.dirty();

        assert!(f.apply(detach_request(&f, &target)).is_err(), "ignored={ignored}");
        assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), base);
        assert_eq!(f.git(&["branch", "--show-current"]).trim(), "source");
        assert_eq!(fs::read_to_string(f.root.join("collision.txt")).unwrap(), "local contents to keep\n");
        f.assert_dirty();
    }
}

#[test]
fn detached_checkout_rejects_noncommit_objects_and_moved_head() {
    let f = Fixture::new();
    let base = f.git(&["rev-parse", "HEAD"]).trim().to_string();
    let blob = f.git(&["rev-parse", "HEAD:file.txt"]).trim().to_string();
    assert!(f.apply(detach_request(&f, &blob)).is_err());
    assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), base);
    assert_eq!(f.git(&["branch", "--show-current"]).trim(), "main");

    let stale = detach_request(&f, &base);
    f.dirty(); f.git(&["commit", "-m", "External commit after review"]);
    let tip = f.git(&["rev-parse", "HEAD"]).trim().to_string();
    assert_ne!(tip, base);
    assert!(f.apply(stale).err().unwrap().contains("branch or HEAD changed"));
    assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), tip);
    assert_eq!(f.git(&["branch", "--show-current"]).trim(), "main");
    assert_eq!(fs::read_to_string(f.root.join("file.txt")).unwrap(), "working\n");
    assert_eq!(fs::read_to_string(f.root.join("untracked.txt")).unwrap(), "keep\n");
}
