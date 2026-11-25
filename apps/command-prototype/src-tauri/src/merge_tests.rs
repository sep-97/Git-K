use super::*;

fn request(f: &Fixture, source: &str, mode: &str) -> MergeRequest {
    let state = snapshot(f.runtime.clone(), display_path(&f.root)).unwrap();
    MergeRequest { path: state.path, expected_head: state.head, expected_branch: state.branch,
        target: f.git(&["rev-parse", &format!("refs/heads/{source}")]).trim().into(), source_branch: Some(source.into()),
        mode: mode.into(), message: "Merge \"literal\" & $(text) — فارسی".into(), abort: false }
}
fn topic(f: &Fixture, diverge: bool) {
    f.git(&["switch", "-c", "topic&literal"]);
    fs::write(f.root.join("topic.txt"), "incoming\n").unwrap();
    f.git(&["add", "--", "topic.txt"]); f.git(&["commit", "-m", "Topic work"]);
    f.git(&["switch", "main"]);
    if diverge {
        fs::write(f.root.join("main.txt"), "current\n").unwrap();
        f.git(&["add", "--", "main.txt"]); f.git(&["commit", "-m", "Main work"]);
    }
}

#[test]
fn fast_forward_and_diverged_merges_match_git_parent_and_tree_results() {
    for diverge in [false, true] {
        let f = Fixture::new(); topic(&f, diverge);
        let req = request(&f, "topic&literal", "normal");
        f.git(&["config", "merge.ff", "only"]);
        f.git(&["config", "branch.main.mergeOptions", "--squash --no-commit"]);
        let head = req.expected_head.clone(); let target = req.target.clone();
        let outcome = execute_merge(f.runtime.clone(), req).unwrap();
        assert!(outcome.refresh_error.is_none());
        assert_eq!(f.git(&["branch", "--show-current"]).trim(), "main");
        assert_eq!(f.git(&["show", "HEAD:topic.txt"]), "incoming\n");
        if diverge { assert_eq!(f.git(&["show", "--no-patch", "--format=%P", "HEAD"]).trim(), format!("{head} {target}")); }
        else { assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), target); }
        assert_eq!(f.git(&["rev-parse", "refs/heads/topic&literal"]).trim(), target);
        let events = f.runtime.events.lock().unwrap();
        let commands: Vec<_> = events.iter().filter(|event| event.kind == "command" && event.text.contains("--no-autostash")).collect();
        assert_eq!(commands.len(), 1);
        assert_eq!(events.iter().filter(|event| event.operation == commands[0].operation && event.kind == "completed" && event.text.starts_with("exit 0")).count(), 1);
    }
}

#[test]
fn explicit_merge_commit_keeps_both_parents_and_literal_message() {
    let f = Fixture::new(); topic(&f, false);
    let req = request(&f, "topic&literal", "merge-commit");
    let head = req.expected_head.clone(); let target = req.target.clone(); let message = req.message.clone();
    execute_merge(f.runtime.clone(), req).unwrap();
    assert_eq!(f.git(&["show", "--no-patch", "--format=%P", "HEAD"]).trim(), format!("{head} {target}"));
    assert_eq!(f.git(&["show", "--no-patch", "--format=%B", "HEAD"]).trim(), message);
    assert!(merge_head(&f.runtime, &f.root).unwrap().is_empty());
}

#[test]
fn squash_stages_combined_contents_for_the_existing_manual_commit_command() {
    let f = Fixture::new(); topic(&f, true);
    fs::write(f.root.join("untracked.txt"), "keep\n").unwrap();
    let req = request(&f, "topic&literal", "squash");
    let head = req.expected_head.clone(); let target = req.target.clone();
    let state = execute_merge(f.runtime.clone(), req).unwrap().snapshot.unwrap();
    assert_eq!(state.head, head); assert!(state.merge_head.is_empty());
    assert_eq!(f.git(&["show", ":topic.txt"]), "incoming\n");
    assert_eq!(f.git(&["show", ":main.txt"]), "current\n");
    assert_eq!(fs::read_to_string(f.root.join("topic.txt")).unwrap(), "incoming\n");
    execute_commit(f.runtime.clone(), display_path(&f.root), head.clone(), "main".into(), "Squashed work".into(), "".into(), false).unwrap();
    assert_eq!(f.git(&["show", "--no-patch", "--format=%P", "HEAD"]).trim(), head);
    assert_eq!(f.git(&["show", "HEAD:topic.txt"]), "incoming\n");
    assert_eq!(f.git(&["rev-parse", "refs/heads/topic&literal"]).trim(), target);
    assert_eq!(fs::read_to_string(f.root.join("untracked.txt")).unwrap(), "keep\n");
}

fn conflict(f: &Fixture) {
    f.git(&["switch", "-c", "topic&literal"]);
    fs::write(f.root.join("file.txt"), "incoming\n").unwrap();
    f.git(&["add", "--", "file.txt"]); f.git(&["commit", "-m", "Incoming conflict"]);
    f.git(&["switch", "main"]);
    fs::write(f.root.join("file.txt"), "current\n").unwrap();
    f.git(&["add", "--", "file.txt"]); f.git(&["commit", "-m", "Current conflict"]);
}
fn abort_request(f: &Fixture) -> MergeRequest {
    let mut req = request(f, "topic&literal", "normal");
    req.abort = true; req.source_branch = None; req.target = merge_head(&f.runtime, &f.root).unwrap(); req
}

#[test]
fn conflicts_retain_real_state_and_reviewed_abort_restores_tracked_files() {
    let f = Fixture::new(); conflict(&f);
    fs::write(f.root.join("untracked.txt"), "keep\n").unwrap();
    let req = request(&f, "topic&literal", "normal"); let head = req.expected_head.clone(); let target = req.target.clone();
    assert!(execute_merge(f.runtime.clone(), req).is_err());
    let state = snapshot(f.runtime.clone(), display_path(&f.root)).unwrap();
    assert_eq!(state.head, head); assert_eq!(state.merge_head, target);
    assert!(state.files.iter().any(|file| file.state == "UU"));
    assert!(!f.git(&["ls-files", "-u"]).is_empty());
    let mut stale = abort_request(&f); stale.target = head.clone();
    assert!(execute_merge(f.runtime.clone(), stale).is_err());
    assert_eq!(merge_head(&f.runtime, &f.root).unwrap(), target);
    let state = execute_merge(f.runtime.clone(), abort_request(&f)).unwrap().snapshot.unwrap();
    assert!(state.merge_head.is_empty()); assert_eq!(state.head, head);
    assert_eq!(fs::read_to_string(f.root.join("file.txt")).unwrap(), "current\n");
    assert!(f.git(&["diff", "--cached"]).is_empty()); assert!(f.git(&["diff"]).is_empty());
    assert_eq!(fs::read_to_string(f.root.join("untracked.txt")).unwrap(), "keep\n");
}

#[test]
fn conflicting_squash_is_refused_before_index_and_worktree_changes() {
    let f = Fixture::new(); conflict(&f);
    let req = request(&f, "topic&literal", "squash"); let head = req.expected_head.clone();
    assert!(execute_merge(f.runtime.clone(), req).err().unwrap().contains("Squash preview"));
    assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), head);
    assert!(merge_head(&f.runtime, &f.root).unwrap().is_empty());
    assert_eq!(fs::read_to_string(f.root.join("file.txt")).unwrap(), "current\n");
    assert!(f.git(&["diff", "--cached"]).is_empty()); assert!(f.git(&["diff"]).is_empty());
}

#[test]
fn dirty_stale_detached_and_invalid_merges_are_refused() {
    let f = Fixture::new(); topic(&f, false); let original = request(&f, "topic&literal", "normal");
    for (mode, target) in [("unsupported", original.target.as_str()), ("normal", "HEAD"), ("normal", "--force"), ("normal", &"f".repeat(40)), ("normal", f.git(&["rev-parse", "HEAD:file.txt"]).trim())] {
        let mut req = original.clone(); req.mode = mode.into(); req.target = target.into(); req.source_branch = None;
        assert!(execute_merge(f.runtime.clone(), req).is_err());
    }
    f.git(&["branch", "--force", "topic&literal", &original.expected_head]);
    assert!(execute_merge(f.runtime.clone(), original).err().unwrap().contains("incoming branch changed"));
    let stale = request(&f, "topic&literal", "normal");
    f.git(&["commit", "--allow-empty", "-m", "External HEAD change"]);
    assert!(execute_merge(f.runtime.clone(), stale).err().unwrap().contains("branch or HEAD changed"));
    f.dirty();
    assert!(execute_merge(f.runtime.clone(), request(&f, "topic&literal", "normal")).err().unwrap().contains("tracked changes")); f.assert_dirty();
    f.git(&["switch", "--detach", "HEAD"]);
    assert!(execute_merge(f.runtime.clone(), request(&f, "topic&literal", "normal")).err().unwrap().contains("Check out a branch")); f.assert_dirty();
}

#[test]
fn untracked_and_ignored_collisions_are_preserved_for_each_merge_mode() {
    for mode in ["normal", "merge-commit", "squash"] {
        for ignored in [false, true] {
            let f = Fixture::new(); topic(&f, false);
            if ignored { fs::write(f.root.join(".git/info/exclude"), "topic.txt\n").unwrap(); }
            fs::write(f.root.join("topic.txt"), "local work\n").unwrap();
            let req = request(&f, "topic&literal", mode); let head = req.expected_head.clone();
            assert!(execute_merge(f.runtime.clone(), req).is_err(), "mode={mode}, ignored={ignored} must refuse an overwrite");
            assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), head);
            assert_eq!(fs::read_to_string(f.root.join("topic.txt")).unwrap(), "local work\n");
            assert!(f.git(&["diff", "--cached"]).is_empty());
        }
    }
}

#[test]
fn hook_and_signing_failures_keep_pending_state_and_can_be_aborted() {
    for signing in [false, true] {
        let f = Fixture::new(); topic(&f, true);
        if signing { f.git(&["config", "commit.gpgsign", "true"]); f.git(&["config", "gpg.program", "git-k-missing-signer"]); }
        else { f.hook("pre-merge-commit", "#!/bin/sh\necho 'fixture merge rejected' >&2\nexit 1\n"); }
        let req = request(&f, "topic&literal", "merge-commit"); let head = req.expected_head.clone();
        assert!(execute_merge(f.runtime.clone(), req).is_err());
        assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), head);
        assert!(!merge_head(&f.runtime, &f.root).unwrap().is_empty());
        execute_merge(f.runtime.clone(), abort_request(&f)).unwrap();
        assert!(merge_head(&f.runtime, &f.root).unwrap().is_empty());
    }
}

#[test]
fn successful_merge_before_failed_refresh_is_not_reported_as_a_failed_write() {
    let f = Fixture::new(); topic(&f, false);
    let req = request(&f, "topic&literal", "normal"); let target = req.target.clone();
    f.hook("post-merge", "#!/bin/sh\ngit config --local core.worktree /git-k-merge-missing-worktree\n");
    let outcome = execute_merge(f.runtime.clone(), req).unwrap();
    assert!(outcome.snapshot.is_none()); assert!(outcome.refresh_error.is_some());
    let worktree = format!("core.worktree={}", display_path(&f.root));
    assert_eq!(f.git(&["-c", &worktree, "rev-parse", "HEAD"]).trim(), target);
}

#[test]
fn real_index_lock_failure_keeps_the_lock_and_repository_contents() {
    let f = Fixture::new(); topic(&f, false); let req = request(&f, "topic&literal", "normal"); let head = req.expected_head.clone();
    fs::write(f.root.join(".git/index.lock"), "fixture lock").unwrap();
    assert!(execute_merge(f.runtime.clone(), req).is_err());
    assert_eq!(fs::read_to_string(f.root.join(".git/index.lock")).unwrap(), "fixture lock");
    assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), head);
    assert!(!f.root.join("topic.txt").exists());
}

#[test]
fn renamed_directory_collisions_preserve_ignored_work() {
    let f = Fixture::new();
    fs::create_dir(f.root.join("old")).unwrap(); fs::write(f.root.join("old/base.txt"), "baseline\n").unwrap();
    f.git(&["add", "--", "old/base.txt"]); f.git(&["commit", "-m", "Directory baseline"]);
    f.git(&["switch", "-c", "topic&literal"]);
    fs::write(f.root.join("old/local.txt"), "incoming\n").unwrap();
    f.git(&["add", "--", "old/local.txt"]); f.git(&["commit", "-m", "Incoming file"]);
    f.git(&["switch", "main"]); f.git(&["mv", "old", "new"]); f.git(&["commit", "-m", "Rename directory"]);
    fs::write(f.root.join(".git/info/exclude"), "new/local.txt\n").unwrap();
    fs::write(f.root.join("new/local.txt"), "local work\n").unwrap();
    let _ = execute_merge(f.runtime.clone(), request(&f, "topic&literal", "normal"));
    assert_eq!(fs::read_to_string(f.root.join("new/local.txt")).unwrap(), "local work\n");
}
