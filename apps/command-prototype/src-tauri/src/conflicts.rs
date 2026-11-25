use super::*;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::{Read, Write},
    path::Component,
};

const TEXT_LIMIT: usize = 256 * 1024;

#[derive(Clone, serde::Deserialize)]
pub(super) struct MergeIdentity {
    pub path: String,
    pub head: String,
    pub branch: String,
    pub merge: String,
}

#[derive(Serialize)]
pub(super) struct ConflictView {
    pub file: String,
    pub revision: String,
    pub current_exists: bool,
    pub incoming_exists: bool,
    pub current: Option<String>,
    pub incoming: Option<String>,
    pub output: Option<String>,
    pub marker_size: usize,
    pub external_tool: Option<String>,
    pub can_take_side: bool,
    #[serde(skip)]
    current_entry: Option<String>,
    #[serde(skip)]
    incoming_entry: Option<String>,
}

#[derive(Serialize)]
pub(super) struct MergeReview {
    pub revision: String,
    pub summary: String,
    pub message: String,
}

fn pending_target(runtime: &Runtime, identity: &MergeIdentity) -> Result<PathBuf, String> {
    let root = operation_target(
        runtime,
        &identity.path,
        &identity.head,
        &identity.branch,
        true,
    )?;
    if identity.merge.is_empty() || merge_head(runtime, &root)? != identity.merge {
        return Err("The pending merge changed. Refresh and review it again.".into());
    }
    Ok(root)
}

fn revision(value: impl Hash) -> String {
    let mut hash = DefaultHasher::new();
    value.hash(&mut hash);
    format!("{:016x}", hash.finish())
}

fn checked_file(root: &Path, file: &str) -> Result<PathBuf, String> {
    if file.contains(['\0', '\u{fffd}']) || Path::new(file).components().any(|component| {
        !matches!(component, Component::Normal(name) if !name.to_string_lossy().eq_ignore_ascii_case(".git"))
    }) {
        return Err("This repository path cannot be edited by the prototype.".into());
    }
    let target = root.join(file);
    let mut path = root.to_path_buf();
    for component in Path::new(file).components() {
        path.push(component);
        match fs::symlink_metadata(&path) {
            Ok(meta) if (path == target && meta.is_file()) || (path != target && meta.is_dir()) => {},
            Ok(_) => return Err("This conflict has a special file or parent path. Use your Git tools to resolve it.".into()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && path == target => {},
            Err(error) => return Err(format!("Could not inspect the working path: {error}")),
        }
    }
    Ok(target)
}

// Record size, timestamps and the complete bytes of small working files. Binary
// and oversized output cannot enter the text editor.
fn working_state(path: &Path) -> Result<(String, Option<String>), String> {
    let metadata = match fs::metadata(path) {
        Ok(meta) => meta,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(("missing".into(), None))
        }
        Err(error) => return Err(error.to_string()),
    };
    let mut bytes = Vec::new();
    if metadata.len() <= TEXT_LIMIT as u64 {
        fs::File::open(path)
            .and_then(|file| file.take(TEXT_LIMIT as u64 + 1).read_to_end(&mut bytes))
            .map_err(|e| e.to_string())?;
    }
    let token = revision((metadata.len(), metadata.modified().ok(), &bytes));
    let text = if metadata.len() > TEXT_LIMIT as u64 {
        None
    } else {
        editable_text(bytes)
    };
    Ok((token, text))
}

fn editable_text(bytes: Vec<u8>) -> Option<String> {
    if bytes.len() > TEXT_LIMIT || bytes.contains(&0) {
        return None;
    }
    String::from_utf8(bytes).ok()
}

fn blob_text(runtime: &Runtime, root: &Path, oid: &str) -> Result<Option<String>, String> {
    let size = runtime
        .git(root, &["cat-file", "-s", oid])?
        .trim()
        .parse::<usize>()
        .map_err(|e| e.to_string())?;
    if size > TEXT_LIMIT {
        return Ok(None);
    }
    let output = runtime.git_output_with_capture(root, &["cat-file", "blob", oid], false)?;
    if !output.status.success() {
        return Err(output.error());
    }
    Ok(editable_text(output.stdout_bytes))
}

fn configured_tool(runtime: &Runtime, root: &Path) -> Result<Option<String>, String> {
    let value = runtime.git_output(root, &["config", "--get", "merge.tool"])?;
    if value.status.code() == Some(1) {
        return Ok(None);
    }
    if !value.status.success() {
        return Err(value.error());
    }
    let tool = value.stdout.trim();
    Ok((!tool.is_empty()
        && tool
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c)))
    .then(|| tool.to_owned()))
}

fn inspect_path(
    runtime: &Runtime,
    root: &Path,
    file: &str,
    preview: bool,
) -> Result<ConflictView, String> {
    let index = runtime.git(
        root,
        &[
            "--literal-pathspecs",
            "ls-files",
            "--unmerged",
            "-z",
            "--",
            file,
        ],
    )?;
    let mut sides = [None; 3];
    let mut regular = true;
    for entry in index.split_terminator('\0') {
        let (fields, path) = entry
            .split_once('\t')
            .ok_or("Git returned invalid conflict entries.")?;
        if path != file {
            continue;
        }
        let fields: Vec<_> = fields.split_whitespace().collect();
        if fields.len() != 3 || !valid_oid(fields[1]) {
            return Err("Git returned invalid conflict entries.".into());
        }
        let stage = match fields[2] {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            _ => return Err("Git returned an invalid stage.".into()),
        };
        regular &= matches!(fields[0], "100644" | "100755");
        sides[stage] = Some((fields[0], fields[1]));
    }
    if sides.iter().all(Option::is_none) {
        return Err("This path is no longer conflicted. Refresh to see the current index.".into());
    }
    let target = checked_file(root, file)?;
    let (working_token, mut output) = working_state(&target)?;
    let [_, current_side, incoming_side] = sides;
    let current_oid = current_side.map(|(_, oid)| oid);
    let incoming_oid = incoming_side.map(|(_, oid)| oid);
    let attr = runtime.git(
        root,
        &["check-attr", "-z", "conflict-marker-size", "--", file],
    )?;
    let marker_size = attr
        .split('\0')
        .nth(2)
        .and_then(|size| size.parse::<usize>().ok())
        .filter(|size| *size > 0)
        .unwrap_or(7);
    let read_side = |oid| match oid {
        Some(oid) if regular && preview => blob_text(runtime, root, oid),
        _ => Ok(None),
    };
    let current = read_side(current_oid)?;
    let incoming = read_side(incoming_oid)?;
    if !regular
        || (preview
            && ((current_oid.is_some() && current.is_none())
                || (incoming_oid.is_some() && incoming.is_none())))
    {
        output = None;
    }
    Ok(ConflictView {
        file: file.into(),
        revision: revision((&index, working_token, marker_size)),
        current_exists: current_oid.is_some(),
        incoming_exists: incoming_oid.is_some(),
        current,
        incoming,
        output,
        marker_size,
        external_tool: configured_tool(runtime, root)?,
        can_take_side: regular,
        current_entry: current_side.map(|(mode, oid)| format!("{mode},{oid},{file}")),
        incoming_entry: incoming_side.map(|(mode, oid)| format!("{mode},{oid},{file}")),
    })
}

pub(super) fn read_conflict(
    runtime: &Runtime,
    identity: &MergeIdentity,
    file: &str,
) -> Result<ConflictView, String> {
    let _guard = runtime
        .mutation
        .try_lock()
        .map_err(|_| "Another repository change is still running.")?;
    inspect_path(runtime, &pending_target(runtime, identity)?, file, true)
}

fn has_markers(contents: &str, marker_size: usize) -> bool {
    contents.lines().any(|line| {
        let first = line.as_bytes().first().copied().unwrap_or(0);
        // A bare separator can also be a valid underline in a text file.
        if !b"<>|".contains(&first) {
            return false;
        }
        let count = line.bytes().take_while(|byte| *byte == first).count();
        (count == marker_size || count == 7)
            && matches!(line.as_bytes().get(count), None | Some(b' ' | b'\t'))
    })
}

fn replace_output(target: &Path, output: &str) -> Result<(), String> {
    let metadata = fs::symlink_metadata(target).map_err(|e| e.to_string())?;
    if !metadata.is_file() {
        return Err("The output path changed. Reopen the conflict to review it.".into());
    }
    let temporary = target.with_file_name(format!(
        ".git-k-resolution-{}-{}",
        std::process::id(),
        now()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| e.to_string())?;
    let written = file
        .write_all(output.as_bytes())
        .and_then(|_| file.set_permissions(metadata.permissions()));
    drop(file);
    let result = written.and_then(|_| fs::rename(&temporary, target));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.map_err(|e| e.to_string())
}

pub(super) fn resolve_conflict(
    runtime: Runtime,
    identity: MergeIdentity,
    file: String,
    expected: String,
    action: String,
    value: Option<String>,
) -> Result<CommitOutcome, String> {
    let _guard = runtime
        .mutation
        .try_lock()
        .map_err(|_| "Another repository change is still running.")?;
    let root = pending_target(&runtime, &identity)?;
    let view = inspect_path(&runtime, &root, &file, action == "edit")?;
    if view.revision != expected {
        return Err("The conflict or working file changed. Reopen it to review before applying a resolution.".into());
    }
    let index_lock = runtime.git(&root, &["rev-parse", "--git-path", "index.lock"])?;
    if root
        .join(index_lock.trim_end_matches(['\r', '\n']))
        .exists()
    {
        return Err("Git's index is locked. Wait for the other Git operation and retry.".into());
    }
    let stage = |args: &[&str]| {
        runtime.git(&root, args).map_err(|error| {
            format!("The working file was updated, but Git could not stage it: {error}. Review the working file before retrying.")
        })
    };
    let output = match action.as_str() {
        "edit" => {
            let original = view.output.ok_or("This output cannot be edited as text.")?;
            let contents = value
                .filter(|s| !s.contains('\0') && s.len() <= TEXT_LIMIT)
                .ok_or("Enter UTF-8 output within the size limit.")?;
            if has_markers(&contents, view.marker_size) {
                return Err("Resolve the remaining conflict markers before saving.".into());
            }
            let contents = if original.contains("\r\n")
                && !original.replace("\r\n", "").contains(['\n', '\r'])
            {
                contents.replace("\r\n", "\n").replace('\n', "\r\n")
            } else {
                contents
            };
            replace_output(&checked_file(&root, &file)?, &contents)?;
            runtime.event(
                0,
                "local",
                format!("Saved conflict output in {}: {file}", display_path(&root)),
            );
            stage(&["--literal-pathspecs", "add", "--", &file])?
        }
        "current" | "incoming" if view.can_take_side => {
            let current = action == "current";
            let entry = if current {
                view.current_entry
            } else {
                view.incoming_entry
            };
            if let Some(entry) = entry {
                runtime.git(
                    &root,
                    &[
                        "--literal-pathspecs",
                        "checkout",
                        if current { "--ours" } else { "--theirs" },
                        "--",
                        &file,
                    ],
                )?;
                // Keep the exact chosen blob and mode even when worktree filters run.
                stage(&["update-index", "--add", "--cacheinfo", &entry])?
            } else {
                runtime.git(&root, &["--literal-pathspecs", "rm", "--", &file])?
            }
        }
        "external" => {
            let tool = view
                .external_tool
                .filter(|tool| Some(tool.as_str()) == value.as_deref())
                .ok_or(
                    "The configured merge tool changed or is unavailable. Reopen the conflict.",
                )?;
            runtime.git(
                &root,
                &[
                    "--literal-pathspecs",
                    "mergetool",
                    "--no-prompt",
                    &format!("--tool={tool}"),
                    "--",
                    &file,
                ],
            )?
        }
        _ => return Err("This resolution is unavailable for the selected path.".into()),
    };
    Ok(after_write(runtime.clone(), &root, output))
}

fn after_write(runtime: Runtime, root: &Path, output: String) -> CommitOutcome {
    match snapshot(runtime, display_path(root)) {
        Ok(snapshot) => CommitOutcome {
            output,
            snapshot: Some(snapshot),
            refresh_error: None,
        },
        Err(error) => CommitOutcome {
            output,
            snapshot: None,
            refresh_error: Some(error),
        },
    }
}

fn merge_review_at(runtime: &Runtime, root: &Path) -> Result<MergeReview, String> {
    if !runtime
        .git(root, &["ls-files", "--unmerged", "-z"])?
        .is_empty()
    {
        return Err("Resolve all conflicted paths before finishing the merge.".into());
    }
    let index = runtime.git(root, &["ls-files", "--stage", "-z"])?;
    let summary = runtime.git(
        root,
        &[
            "diff",
            "--cached",
            "--no-ext-diff",
            "--no-textconv",
            "--no-renames",
            "--stat",
            "--",
        ],
    )?;
    let path = runtime.git(root, &["rev-parse", "--git-path", "MERGE_MSG"])?;
    let message = fs::read_to_string(root.join(path.trim_end_matches(['\r', '\n'])))
        .map_err(|e| format!("Could not read the pending merge message: {e}"))?;
    Ok(MergeReview {
        revision: revision((index, &message)),
        summary,
        message,
    })
}

pub(super) fn review_merge(
    runtime: &Runtime,
    identity: &MergeIdentity,
) -> Result<MergeReview, String> {
    let _guard = runtime
        .mutation
        .try_lock()
        .map_err(|_| "Another repository change is still running.")?;
    merge_review_at(runtime, &pending_target(runtime, identity)?)
}

pub(super) fn finish_merge(
    runtime: Runtime,
    identity: MergeIdentity,
    expected: String,
) -> Result<CommitOutcome, String> {
    let _guard = runtime
        .mutation
        .try_lock()
        .map_err(|_| "Another repository change is still running.")?;
    let root = pending_target(&runtime, &identity)?;
    if merge_review_at(&runtime, &root)?.revision != expected {
        return Err(
            "The staged contents or merge message changed. Open a new merge review.".into(),
        );
    }
    let output = runtime.git(&root, &["commit", "--no-edit"])?;
    runtime.event(
        0,
        "local",
        "Git created the merge commit. Refreshing repository state.",
    );
    Ok(after_write(runtime.clone(), &root, output))
}
