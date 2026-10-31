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
    assert!(events.iter().any(|event| event.kind == "command" && event.text.contains("branch --delete")));
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
