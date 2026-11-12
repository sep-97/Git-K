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
        RefRequest { path: display_path(&self.root), expected_head: head, expected_branch: self.git(&["branch", "--show-current"]).trim().into(), action: action.into(), name: name.into(), original: original.map(String::from), expected_oid, start_point: Some(self.git(&["rev-parse", "HEAD"]).trim().into()), annotated: false, message: None }
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
    fs::write(f.root.join(".git/test-hooks/reference-transaction"), "#!/bin/sh\nif [ \"$1\" = committed ]; then\n git config --local core.worktree /git-k-reference-missing-worktree\nfi\n").unwrap();
    let result = f.apply(f.req("create-tag", "created-before-read-failed", None)).unwrap();
    assert!(result.snapshot.is_none()); assert!(result.refresh_error.is_some());
    let worktree = format!("core.worktree={}", display_path(&f.root));
    assert!(f.runtime.git(&f.root, &["-c", &worktree, "show-ref", "--verify", "refs/tags/created-before-read-failed"]).is_ok());
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
    fs::write(f.root.join(".git/test-hooks/post-checkout"), "#!/bin/sh\nexit 1\n").unwrap();
    assert!(f.apply(detach_request(&f, &base)).is_err());
    assert_eq!(snapshot(f.runtime.clone(), display_path(&f.root)).unwrap().branch, "Detached HEAD");
    fs::write(f.root.join(".git/test-hooks/post-checkout"), "#!/bin/sh\ngit config --local core.worktree /git-k-detached-missing-worktree\n").unwrap();
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
