use super::*;

fn pending_conflict() -> Fixture {
    let f = Fixture::new();
    f.git(&["switch", "-c", "incoming"]);
    fs::write(f.root.join("file.txt"), "incoming\n").unwrap();
    f.git(&["add", "--", "file.txt"]);
    f.git(&["commit", "-m", "Incoming"]);
    f.git(&["switch", "main"]);
    fs::write(f.root.join("file.txt"), "current\n").unwrap();
    f.git(&["add", "--", "file.txt"]);
    f.git(&["commit", "-m", "Current"]);
    assert!(f
        .runtime
        .git(&f.root, &["merge", "--no-edit", "incoming"])
        .is_err());
    f
}

#[test]
fn inspect_pending_conflict_through_the_workflow_seam() {
    let f = pending_conflict();
    let identity = identity(&f);
    let view = read_conflict(&f.runtime, &identity, "file.txt").unwrap();
    assert_eq!(view.file, "file.txt");
    assert_eq!(view.current.as_deref(), Some("current\n"));
    assert_eq!(view.incoming.as_deref(), Some("incoming\n"));
    assert!(view.output.unwrap().contains("<<<<<<< HEAD"));
}

fn identity(f: &Fixture) -> MergeIdentity {
    let state = snapshot(f.runtime.clone(), display_path(&f.root)).unwrap();
    MergeIdentity {
        path: state.path,
        head: state.head,
        branch: state.branch,
        merge: state.merge_head,
    }
}

fn apply(
    f: &Fixture,
    identity: &MergeIdentity,
    view: &ConflictView,
    action: &str,
    value: Option<&str>,
) -> Result<CommitOutcome, String> {
    resolve_conflict(
        f.runtime.clone(),
        identity.clone(),
        view.file.clone(),
        view.revision.clone(),
        action.into(),
        value.map(str::to_owned),
    )
}

#[test]
fn edited_output_stages_exact_bytes_and_continuation_preserves_both_parents() {
    let f = pending_conflict();
    let identity = identity(&f);
    let view = read_conflict(&f.runtime, &identity, "file.txt").unwrap();
    assert!(review_merge(&f.runtime, &identity)
        .err()
        .unwrap()
        .contains("all conflicted paths"));
    assert!(apply(&f, &identity, &view, "edit", view.output.as_deref())
        .err()
        .unwrap()
        .contains("remaining conflict markers"));
    let content = "Reviewed both versions — فارسی\n=======\nNo final newline";
    let result = apply(&f, &identity, &view, "edit", Some(content))
        .unwrap()
        .snapshot
        .unwrap();
    assert_eq!(result.head, identity.head);
    assert_eq!(result.merge_head, identity.merge);
    assert_eq!(
        fs::read(f.root.join("file.txt")).unwrap(),
        content.as_bytes()
    );
    assert_eq!(f.git(&["show", ":file.txt"]), content);
    assert!(f.git(&["ls-files", "-u"]).is_empty());
    let review = review_merge(&f.runtime, &identity).unwrap();
    assert!(review.summary.contains("file.txt"));
    fs::write(f.root.join("file.txt"), "unstaged follow-up\n").unwrap();
    let result = finish_merge(f.runtime.clone(), identity.clone(), review.revision)
        .unwrap()
        .snapshot
        .unwrap();
    assert!(result.merge_head.is_empty());
    assert_eq!(
        f.git(&["show", "-s", "--format=%P", "HEAD"]).trim(),
        format!("{} {}", identity.head, identity.merge)
    );
    assert_eq!(f.git(&["show", "HEAD:file.txt"]), content);
    assert_eq!(
        fs::read_to_string(f.root.join("file.txt")).unwrap(),
        "unstaged follow-up\n"
    );
    let events = f.runtime.events.lock().unwrap();
    assert!(events
        .iter()
        .any(|event| event.kind == "context" && event.text == identity.path));
    assert!(!events.iter().any(|event| event.kind == "stdout"
        && event.text.contains("bytes read for conflict preview")
        && event.text.contains(content)));
    assert!(events.iter().any(|event| event.kind == "command"
        && event.text.contains("commit")
        && event.text.contains("--no-edit")));
}

#[test]
fn stale_edits_and_locked_index_preserve_work_then_hook_failure_retains_merge() {
    let f = pending_conflict();
    let identity = identity(&f);
    let stale = read_conflict(&f.runtime, &identity, "file.txt").unwrap();
    fs::write(f.root.join("file.txt"), "external edit\n").unwrap();
    assert!(apply(&f, &identity, &stale, "incoming", None)
        .err()
        .unwrap()
        .contains("working file changed"));
    let current = read_conflict(&f.runtime, &identity, "file.txt").unwrap();
    let lock = f.root.join(".git/index.lock");
    fs::write(&lock, "existing lock").unwrap();
    assert!(
        apply(&f, &identity, &current, "edit", Some("replacement\n"))
            .err()
            .unwrap()
            .contains("locked")
    );
    assert_eq!(
        fs::read_to_string(f.root.join("file.txt")).unwrap(),
        "external edit\n"
    );
    assert_eq!(fs::read_to_string(&lock).unwrap(), "existing lock");
    fs::remove_file(lock).unwrap();
    apply(&f, &identity, &current, "incoming", None).unwrap();
    assert_eq!(f.git(&["show", ":file.txt"]), "incoming\n");
    let stale = review_merge(&f.runtime, &identity).unwrap();
    fs::write(f.root.join("added.txt"), "external staged file\n").unwrap();
    f.git(&["add", "--", "added.txt"]);
    assert!(
        finish_merge(f.runtime.clone(), identity.clone(), stale.revision)
            .err()
            .unwrap()
            .contains("staged contents")
    );
    let review = review_merge(&f.runtime, &identity).unwrap();
    f.hook(
        "pre-commit",
        "#!/bin/sh\necho 'fixture rejects continuation' >&2\nexit 1\n",
    );
    assert!(
        finish_merge(f.runtime.clone(), identity.clone(), review.revision)
            .err()
            .unwrap()
            .contains("fixture rejects continuation")
    );
    assert_eq!(f.git(&["rev-parse", "HEAD"]).trim(), identity.head);
    assert_eq!(merge_head(&f.runtime, &f.root).unwrap(), identity.merge);
    assert_eq!(f.git(&["show", ":added.txt"]), "external staged file\n");
}

#[test]
fn binary_and_delete_conflicts_use_git_without_decoding_the_selected_side() {
    let f = Fixture::new();
    let file = "literal [1].bin";
    let gone = "remove.txt";
    fs::write(f.root.join(file), [0, 1, 2]).unwrap();
    fs::write(f.root.join(gone), "base\n").unwrap();
    f.git(&["--literal-pathspecs", "add", "--", file, gone]);
    f.git(&["commit", "-m", "Binary baseline"]);
    f.git(&["switch", "-c", "incoming"]);
    fs::write(f.root.join(file), [0, 8, 9, 255]).unwrap();
    f.git(&["rm", "--", gone]);
    f.git(&["--literal-pathspecs", "add", "--", file]);
    f.git(&["commit", "-m", "Incoming"]);
    f.git(&["switch", "main"]);
    fs::write(f.root.join(file), [0, 4, 5]).unwrap();
    fs::write(f.root.join(gone), "keep\n").unwrap();
    f.git(&["--literal-pathspecs", "add", "--", file, gone]);
    f.git(&["commit", "-m", "Current"]);
    assert!(f
        .runtime
        .git(&f.root, &["merge", "--no-edit", "incoming"])
        .is_err());
    let identity = identity(&f);
    let view = read_conflict(&f.runtime, &identity, file).unwrap();
    assert!(view.output.is_none());
    assert!(view.current_exists && view.incoming_exists);
    assert!(apply(&f, &identity, &view, "edit", Some("replacement")).is_err());
    apply(&f, &identity, &view, "incoming", None).unwrap();
    assert_eq!(fs::read(f.root.join(file)).unwrap(), [0, 8, 9, 255]);
    assert_eq!(
        f.git(&["rev-parse", &format!(":{file}")]),
        f.git(&["rev-parse", &format!("incoming:{file}")])
    );
    let view = read_conflict(&f.runtime, &identity, gone).unwrap();
    assert!(!view.incoming_exists);
    apply(&f, &identity, &view, "incoming", None).unwrap();
    assert!(!f.root.join(gone).exists());
    assert!(f.git(&["ls-files", "-u"]).is_empty());
}

#[test]
fn configured_external_tool_resolves_and_stages_only_the_selected_file() {
    let f = pending_conflict();
    let identity = identity(&f);
    let view = read_conflict(&f.runtime, &identity, "file.txt").unwrap();
    assert!(apply(&f, &identity, &view, "external", Some("missing-fixture")).is_err());
    f.git(&["config", "merge.tool", "git-k-fixture"]);
    f.git(&[
        "config",
        "mergetool.git-k-fixture.cmd",
        "cp -- \"$REMOTE\" \"$MERGED\"",
    ]);
    f.git(&["config", "mergetool.git-k-fixture.trustExitCode", "true"]);
    let view = read_conflict(&f.runtime, &identity, "file.txt").unwrap();
    assert_eq!(view.external_tool.as_deref(), Some("git-k-fixture"));
    apply(
        &f,
        &identity,
        &view,
        "external",
        view.external_tool.as_deref(),
    )
    .unwrap();
    assert_eq!(f.git(&["show", ":file.txt"]), "incoming\n");
    assert!(f.git(&["ls-files", "-u"]).is_empty());
    assert_eq!(merge_head(&f.runtime, &f.root).unwrap(), identity.merge);
}
