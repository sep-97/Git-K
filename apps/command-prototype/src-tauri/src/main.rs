#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use serde::Serialize;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{atomic::{AtomicBool, AtomicU64, Ordering}, Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Serialize)]
struct Event { seq: u64, operation: u64, timestamp: u64, kind: String, text: String }
#[derive(Clone, Default)]
struct Runtime {
    events: Arc<Mutex<Vec<Event>>>,
    next: Arc<AtomicU64>,
    watch: Arc<AtomicBool>,
    mutation: Arc<Mutex<()>>,
}
impl Runtime {
    fn event(&self, operation: u64, kind: &str, text: impl Into<String>) {
        let mut events = self.events.lock().unwrap();
        let seq = events.len() as u64 + 1;
        events.push(Event { seq, operation, timestamp: now(), kind: kind.into(), text: text.into() });
    }
    fn git(&self, cwd: &Path, args: &[&str]) -> Result<String, String> {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let executable = std::env::var_os("GIT_K_GIT").unwrap_or_else(|| "git".into());
        let mut effective_args = vec!["--no-pager"];
        effective_args.extend_from_slice(args);
        let args = effective_args.as_slice();
        let display = format!("{} {}", quote(&executable.to_string_lossy()), args.iter().map(|s| quote(s)).collect::<Vec<_>>().join(" "));
        self.event(id, "command", display);
        self.event(id, "context", cwd.display().to_string());
        let mut command = Command::new(executable);
        command.args(args).current_dir(cwd).env("GIT_TERMINAL_PROMPT", "0").env("GIT_OPTIONAL_LOCKS", "0")
            .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        for (key, _) in std::env::vars_os() {
            let name = key.to_string_lossy().to_ascii_uppercase();
            if ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR", "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_NAMESPACE", "GIT_PREFIX", "GIT_QUARANTINE_PATH", "GIT_CONFIG_COUNT", "GIT_CONFIG_PARAMETERS", "GIT_GLOB_PATHSPECS", "GIT_NOGLOB_PATHSPECS", "GIT_LITERAL_PATHSPECS", "GIT_ICASE_PATHSPECS"].contains(&name.as_str()) || name.starts_with("GIT_CONFIG_KEY_") || name.starts_with("GIT_CONFIG_VALUE_") {
                command.env_remove(key);
            }
        }
        #[cfg(windows)] {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
        }
        let started = now();
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => { self.event(id, "failed", format!("Could not start: {error}")); return Err(error.to_string()); }
        };
        let out = child.stdout.take().unwrap();
        let err = child.stderr.take().unwrap();
        let first = self.clone();
        let stdout = thread::spawn(move || {
            let mut collected = Vec::new();
            let mut reader = BufReader::new(out);
            loop {
                let mut bytes = Vec::new();
                match reader.read_until(b'\n', &mut bytes) {
                    Ok(0) => break,
                    Ok(_) => { first.event(id, "stdout", String::from_utf8_lossy(&bytes).into_owned()); collected.extend(bytes); },
                    Err(error) => { first.event(id, "stderr", format!("Output read error: {error}")); break; }
                }
            }
            collected
        });
        let second = self.clone();
        let stderr = thread::spawn(move || {
            let mut collected = Vec::new();
            for line in BufReader::new(err).split(b'\n') {
                match line {
                    Ok(bytes) => { second.event(id, "stderr", String::from_utf8_lossy(&bytes).into_owned()); collected.extend(bytes); collected.push(b'\n'); },
                    Err(error) => { second.event(id, "stderr", format!("Error-stream read error: {error}")); break; }
                }
            }
            collected
        });
        let result = child.wait().map_err(|e| e.to_string());
        let out = stdout.join().map_err(|_| "Output reader stopped".to_string())?;
        let err = stderr.join().map_err(|_| "Error reader stopped".to_string())?;
        let result = match result {
            Ok(status) => status,
            Err(error) => { self.event(id, "failed", error.clone()); return Err(error); }
        };
        self.event(id, if result.success() { "completed" } else { "failed" }, format!("exit {} · {} ms", result.code().map(|v| v.to_string()).unwrap_or_else(|| "interrupted".into()), now() - started));
        if !result.success() {
            let error = String::from_utf8_lossy(&err).trim().to_string();
            return Err(if error.is_empty() { format!("Git exited with {}. Inspect the recorded output in Action history.", result.code().map(|v| v.to_string()).unwrap_or_else(|| "an interruption".into())) } else { error });
        }
        Ok(String::from_utf8_lossy(&out).into_owned())
    }
}
fn now() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 }
fn quote(value: &str) -> String {
    if !value.is_empty() && value.chars().all(|c| c.is_ascii_alphanumeric() || "-_/.:=@%".contains(c)) { value.into() }
    else {
        #[cfg(windows)] { format!("'{}'", value.replace('\'', "''")) }
        #[cfg(not(windows))] { format!("'{}'", value.replace('\'', "'\\''")) }
    }
}
fn repository(input: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(input);
    if !path.is_dir() { return Err("Choose an existing repository folder.".into()); }
    path.canonicalize().map_err(|e| e.to_string())
}
fn display_path(path: &Path) -> String { path.display().to_string().trim_start_matches("\\\\?\\").to_string() }
#[derive(Serialize)]
struct FileChange { path: String, state: String, previous: Option<String> }
#[derive(Serialize)]
struct Commit { sha: String, parents: Vec<String>, subject: String, author: String, age: String, refs: String }
#[derive(Serialize)]
struct LocalRef { name: String, oid: String, kind: String }
#[derive(Serialize)]
struct Snapshot { path: String, branch: String, head: String, files: Vec<FileChange>, commits: Vec<Commit>, branches: Vec<LocalRef>, tags: Vec<LocalRef> }
#[derive(Serialize)]
struct CommitOutcome { output: String, snapshot: Option<Snapshot>, refresh_error: Option<String> }
fn changes(runtime: &Runtime, path: &Path) -> Result<Vec<FileChange>, String> {
    let status = runtime.git(path, &["status", "--porcelain=v1", "--untracked-files=all", "-z"])?;
    let mut entries = status.split('\0').filter(|s| !s.is_empty());
    let mut files = Vec::new();
    while let Some(entry) = entries.next() {
        if entry.len() < 3 { continue; }
        let state = entry[..2].to_string();
        let previous = if state.contains('R') || state.contains('C') { entries.next().map(String::from) } else { None };
        files.push(FileChange { path: entry[3..].into(), state, previous });
    }
    Ok(files)
}
fn conflicted(file: &FileChange) -> bool { ["DD", "AU", "UD", "UA", "DU", "AA", "UU"].contains(&file.state.as_str()) }
fn pending(file: &FileChange, stage: bool) -> bool {
    if conflicted(file) { return false; }
    if stage { file.state == "??" || file.state.as_bytes()[1] != b' ' }
    else { file.state != "??" && file.state.as_bytes()[0] != b' ' }
}
fn local_refs(runtime: &Runtime, path: &Path, prefix: &str) -> Result<Vec<LocalRef>, String> {
    let refs = runtime.git(path, &["for-each-ref", "--format=%(refname)%1f%(objectname)%1f%(objecttype)", prefix])?;
    Ok(refs.lines().filter_map(|line| {
        let fields: Vec<_> = line.split('\x1f').collect();
        if fields.len() != 3 { return None; }
        Some(LocalRef { name: fields[0].strip_prefix(prefix)?.trim_start_matches('/').into(), oid: fields[1].into(), kind: fields[2].into() })
    }).collect())
}
fn snapshot(runtime: Runtime, input: String) -> Result<Snapshot, String> {
    runtime.event(0, "local", format!("Open / refresh repository: {input}"));
    let requested = repository(&input)?;
    let root = runtime.git(&requested, &["rev-parse", "--show-toplevel"])?;
    let path = PathBuf::from(root.trim());
    let branch = runtime.git(&path, &["branch", "--show-current"])?;
    let files = changes(&runtime, &path)?;
    let branches = local_refs(&runtime, &path, "refs/heads/")?;
    let tags = local_refs(&runtime, &path, "refs/tags/")?;
    let head = runtime.git(&path, &["rev-parse", "--revs-only", "HEAD"])?;
    let commits = if !head.trim().is_empty() {
        let history = runtime.git(&path, &["log", "--all", "--date-order", "-n", "60", "--format=%H%x1f%P%x1f%s%x1f%an%x1f%ar%x1f%D"])?;
        history.lines().filter_map(|line| {
            let fields: Vec<_> = line.split('\x1f').collect();
            if fields.len() != 6 { return None; }
            Some(Commit { sha: fields[0].into(), parents: fields[1].split_whitespace().map(String::from).collect(), subject: fields[2].into(), author: fields[3].into(), age: fields[4].into(), refs: fields[5].into() })
        }).collect()
    } else { Vec::new() };
    Ok(Snapshot { path: display_path(&path), branch: if branch.trim().is_empty() { "Detached HEAD".into() } else { branch.trim().into() }, head: head.trim().into(), files, commits, branches, tags })
}
#[tauri::command]
fn initial_path() -> String {
    if let Some(path) = std::env::args_os().nth(1) { return display_path(Path::new(&path)); }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    cwd.ancestors().find(|path| path.join(".git").exists()).map(display_path).unwrap_or_default()
}
#[tauri::command]
fn record_interface(runtime: tauri::State<'_, Runtime>, action: String) -> Result<(), String> {
    match action.as_str() {
        "Activity panel collapsed" | "Activity panel opened" => { runtime.event(0, "local", action); Ok(()) },
        _ => Err("Unsupported interface activity.".into())
    }
}
#[tauri::command]
fn journal(runtime: tauri::State<'_, Runtime>, after: u64) -> Vec<Event> {
    runtime.events.lock().unwrap().iter().filter(|e| e.seq > after).take(1000).cloned().collect()
}
#[tauri::command]
async fn open_repository(runtime: tauri::State<'_, Runtime>, path: String) -> Result<Snapshot, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || snapshot(runtime, path)).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn inspect_file(runtime: tauri::State<'_, Runtime>, path: String, file: String, side: String) -> Result<String, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let root = repository(&path)?;
        let relative = Path::new(&file);
        if relative.is_absolute() || relative.components().any(|c| matches!(c, std::path::Component::ParentDir | std::path::Component::Prefix(_))) { return Err("File must belong to the selected repository.".into()); }
        if side == "untracked" {
            let selected = root.join(relative).canonicalize().map_err(|e| e.to_string())?;
            if !selected.starts_with(&root) { return Err("Selected file resolves outside this repository.".into()); }
            runtime.event(0, "local", format!("Read untracked file: {file}"));
            let bytes = fs::read(selected).map_err(|e| e.to_string())?;
            return Ok(String::from_utf8_lossy(&bytes).into_owned());
        }
        let files = changes(&runtime, &root)?;
        let change = files.iter().find(|entry| entry.path == file);
        let mut args = vec!["--literal-pathspecs", "diff", "--no-ext-diff", "--no-textconv", "--no-color"];
        match side.as_str() { "staged" => args.push("--cached"), "unstaged" => {}, _ => return Err("Choose staged or unstaged changes.".into()) }
        args.extend(["--", &file]);
        if side == "staged" { if let Some(previous) = change.and_then(|entry| entry.previous.as_deref()) { args.push(previous); } }
        runtime.git(&root, &args)
    }).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn change_index(runtime: tauri::State<'_, Runtime>, path: String, file: Option<String>, stage: bool) -> Result<Snapshot, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = runtime.mutation.try_lock().map_err(|_| "Another repository change is still running.".to_string())?;
        let requested = repository(&path)?;
        let root = PathBuf::from(runtime.git(&requested, &["rev-parse", "--show-toplevel"])?.trim());
        let files = changes(&runtime, &root)?;
        if files.iter().any(conflicted) { return Err("Resolve repository conflicts before using this prototype's staging controls.".into()); }
        let candidates: Vec<_> = files.iter().filter(|entry| pending(entry, stage)).collect();
        let mut paths = Vec::new();
        if let Some(selected) = &file {
            let entry = candidates.iter().find(|entry| entry.path == *selected).ok_or("The selected change is no longer available. Refresh and try again.")?;
            paths.push(entry.path.as_str());
            if (!stage && entry.state.starts_with('R')) || (stage && entry.state.as_bytes()[1] == b'R') {
                if let Some(previous) = entry.previous.as_deref() { paths.push(previous); }
            }
        } else {
            if candidates.is_empty() { return Err("No changes are available for this action.".into()); }
            paths.push(".");
        }
        runtime.event(0, "local", format!("{}: {}", if stage { "Stage changes" } else { "Unstage changes" }, file.as_deref().unwrap_or("all files")));
        if stage {
            let mut args = vec!["--literal-pathspecs", "add", "--all", "--"];
            args.extend(paths);
            runtime.git(&root, &args)?;
        } else {
            // --revs-only returns empty output for an unborn branch, without a failing lookup.
            let head = runtime.git(&root, &["rev-parse", "--revs-only", "HEAD"])?;
            let mut args = if !head.trim().is_empty() { vec!["--literal-pathspecs", "restore", "--staged", "--source=HEAD", "--"] }
                else { vec!["--literal-pathspecs", "rm", "--cached", "--force", "-r", "--ignore-unmatch", "--"] };
            args.extend(paths);
            runtime.git(&root, &args)?;
        }
        snapshot(runtime.clone(), display_path(&root))
    }).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn latest_message(runtime: tauri::State<'_, Runtime>, path: String, expected_head: String, expected_branch: String) -> Result<String, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = runtime.mutation.try_lock().map_err(|_| "Another repository change is still running.".to_string())?;
        let root = commit_target(&runtime, &path, &expected_head, &expected_branch)?;
        if expected_head.is_empty() { return Err("There is no commit to amend on this branch.".into()); }
        runtime.event(0, "local", format!("Read amend message for {expected_head}. No repository mutation."));
        runtime.git(&root, &["show", "--no-patch", "--format=format:%B", &expected_head, "--"])
    }).await.map_err(|e| e.to_string())?
}
fn commit_target(runtime: &Runtime, path: &str, expected_head: &str, expected_branch: &str) -> Result<PathBuf, String> {
    let requested = repository(path)?;
    let root = PathBuf::from(runtime.git(&requested, &["rev-parse", "--show-toplevel"])?.trim());
    let head = runtime.git(&root, &["rev-parse", "--revs-only", "HEAD"])?;
    let branch = runtime.git(&root, &["branch", "--show-current"])?;
    let branch = if branch.trim().is_empty() { "Detached HEAD" } else { branch.trim() };
    if head.trim() != expected_head || branch != expected_branch { return Err("The current branch or HEAD changed since this view was loaded. Review the refreshed repository and reopen any amend or branch dialog before trying again.".into()); }
    for marker in ["MERGE_HEAD", "CHERRY_PICK_HEAD", "REVERT_HEAD", "rebase-merge", "rebase-apply", "sequencer"] {
        let marker_path = runtime.git(&root, &["rev-parse", "--git-path", marker])?;
        if root.join(marker_path.trim()).exists() { return Err("An unfinished merge, rebase, cherry-pick or revert needs its own continuation workflow. Finish it outside this prototype before changing branches or committing.".into()); }
    }
    if changes(runtime, &root)?.iter().any(conflicted) { return Err("Resolve repository conflicts before changing branches or committing.".into()); }
    Ok(root)
}
#[tauri::command]
async fn create_commit(runtime: tauri::State<'_, Runtime>, path: String, expected_head: String, expected_branch: String, summary: String, description: String, amend: bool) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    let journal = runtime.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        runtime.event(0, "local", if amend { "Requested update of the latest commit, including any staged changes." } else { "Requested commit of staged changes." });
        let _guard = runtime.mutation.try_lock().map_err(|_| "Another repository change is still running.".to_string())?;
        let summary = summary.trim();
        if summary.is_empty() || summary.contains(['\n', '\r', '\0']) || description.contains('\0') { return Err("Enter a single-line commit summary. Messages cannot contain NUL characters.".into()); }
        let root = commit_target(&runtime, &path, &expected_head, &expected_branch)?;
        if amend && expected_head.is_empty() { return Err("There is no commit to amend on this branch.".into()); }
        if !amend {
            let staged = runtime.git(&root, &["diff", "--cached", "--name-only", "-z", "--"])?;
            if staged.is_empty() { return Err("No staged changes to commit. Stage a file first.".into()); }
        }
        let message = if description.trim().is_empty() { summary.into() } else { format!("{summary}\n\n{}", description.trim()) };
        let mut args = vec!["commit", "--cleanup=verbatim", "--message", &message];
        if amend { args.push("--amend"); }
        let output = runtime.git(&root, &args)?;
        runtime.event(0, "local", if amend { "Git amended the commit. Refreshing repository state." } else { "Git created the commit. Refreshing repository state." });
        // Once Git succeeds, a later read failure must not invite a duplicate commit.
        Ok(match snapshot(runtime.clone(), display_path(&root)) {
            Ok(snapshot) => CommitOutcome { output, snapshot: Some(snapshot), refresh_error: None },
            Err(error) => CommitOutcome { output, snapshot: None, refresh_error: Some(error) },
        })
    }).await.map_err(|e| e.to_string()).and_then(|result| result);
    if let Err(error) = &result { journal.event(0, "local", format!("Commit did not complete: {error}")); }
    result
}
#[tauri::command]
async fn change_branch(runtime: tauri::State<'_, Runtime>, path: String, expected_head: String, expected_branch: String, name: String, start_point: Option<String>, checkout: bool) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    let journal = runtime.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        runtime.event(0, "local", format!("Requested {} branch: {name}", if start_point.is_some() { "creation of" } else { "checkout of" }));
        let _guard = runtime.mutation.try_lock().map_err(|_| "Another repository change is still running.".to_string())?;
        let root = commit_target(&runtime, &path, &expected_head, &expected_branch)?;
        if name.is_empty() || name.starts_with('-') || name.contains(['\0', '\n', '\r']) { return Err("Enter a literal local branch name that does not begin with a dash.".into()); }
        // --branch can expand @{-n}; accept only the exact literal spelling supplied.
        let validated = runtime.git(&root, &["check-ref-format", "--branch", &name])?;
        if validated.trim_end_matches(['\n', '\r']) != name { return Err("Use a literal branch name, not a previous-checkout expression.".into()); }
        let output = if let Some(start) = start_point {
            if ![40, 64].contains(&start.len()) || !start.chars().all(|c| c.is_ascii_hexdigit()) { return Err("Select an existing commit as the branch starting point.".into()); }
            runtime.git(&root, &["cat-file", "-e", &format!("{start}^{{commit}}")])?;
            if checkout {
                // One Git command creates and switches; no separate partially completed create step.
                runtime.git(&root, &["switch", "--no-guess", "--no-overwrite-ignore", "--no-recurse-submodules", "--no-track", "-c", &name, "--", &start])?
            } else {
                runtime.git(&root, &["branch", "--no-track", "--", &name, &start])?
            }
        } else {
            if !checkout { return Err("Choose a branch action.".into()); }
            let full_ref = format!("refs/heads/{name}");
            runtime.git(&root, &["show-ref", "--verify", &full_ref])?;
            if name == expected_branch { return Err("This branch is already checked out.".into()); }
            runtime.git(&root, &["switch", "--no-guess", "--no-overwrite-ignore", "--no-recurse-submodules", "--", &name])?
        };
        runtime.event(0, "local", "Git completed the branch action. Refreshing repository state.");
        Ok(match snapshot(runtime.clone(), display_path(&root)) {
            Ok(snapshot) => CommitOutcome { output, snapshot: Some(snapshot), refresh_error: None },
            Err(error) => CommitOutcome { output, snapshot: None, refresh_error: Some(error) },
        })
    }).await.map_err(|e| e.to_string()).and_then(|result| result);
    if let Err(error) = &result { journal.event(0, "local", format!("Branch action reported an error; repository state may have changed: {error}")); }
    result
}
#[tauri::command]
async fn inspect_tag(runtime: tauri::State<'_, Runtime>, path: String, oid: String) -> Result<String, String> {
    if !valid_oid(&oid) { return Err("Invalid tag object identifier.".into()); }
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || runtime.git(&repository(&path)?, &["show", "--no-patch", "--no-ext-diff", "--no-textconv", "--no-color", "--format=fuller", &oid, "--"])).await.map_err(|e| e.to_string())?
}
fn valid_oid(oid: &str) -> bool { [40, 64].contains(&oid.len()) && oid.chars().all(|c| c.is_ascii_hexdigit()) }
#[derive(serde::Deserialize)]
struct RefRequest {
    path: String, expected_head: String, expected_branch: String, action: String,
    name: String, original: Option<String>, expected_oid: Option<String>,
    start_point: Option<String>, annotated: bool, message: Option<String>,
}
fn validate_ref_name(runtime: &Runtime, root: &Path, name: &str, branch: bool) -> Result<(), String> {
    if name.is_empty() || name.starts_with('-') || name.contains(['\0', '\n', '\r']) { return Err("Enter a literal reference name that does not begin with a dash.".into()); }
    if branch {
        let validated = runtime.git(root, &["check-ref-format", "--branch", name])?;
        if validated.trim_end_matches(['\n', '\r']) != name { return Err("Use a literal branch name, not a previous-checkout expression.".into()); }
    } else { runtime.git(root, &["check-ref-format", &format!("refs/tags/{name}")])?; }
    Ok(())
}
fn execute_ref_action(runtime: Runtime, request: RefRequest) -> Result<CommitOutcome, String> {
    let result = (|| {
        let _guard = runtime.mutation.try_lock().map_err(|_| "Another repository change is still running.".to_string())?;
        runtime.event(0, "local", format!("Requested {}: {}", request.action, request.original.as_deref().unwrap_or(&request.name)));
        let root = commit_target(&runtime, &request.path, &request.expected_head, &request.expected_branch)?;
        let output = match request.action.as_str() {
            "rename-branch" | "delete-branch" | "delete-tag" => {
                let original = request.original.as_deref().ok_or("Select a reference first.")?;
                let is_branch = request.action != "delete-tag";
                validate_ref_name(&runtime, &root, original, is_branch)?;
                let full_ref = format!("refs/{}/{original}", if is_branch { "heads" } else { "tags" });
                let expected = request.expected_oid.as_deref().filter(|oid| valid_oid(oid)).ok_or("Review the selected reference object first.")?;
                let actual = runtime.git(&root, &["show-ref", "--hash", "--verify", &full_ref])?;
                if actual.trim() != expected { return Err("The selected reference changed since this dialog was opened. Cancel and review its refreshed state.".into()); }
                if is_branch {
                    if request.action == "delete-branch" && original == request.expected_branch { return Err("Switch away from the current branch before deleting it.".into()); }
                    let worktrees = runtime.git(&root, &["worktree", "list", "--porcelain", "-z"])?;
                    let checked_out = worktrees.split('\0').filter(|field| *field == format!("branch {full_ref}")).count();
                    let own_checkout = usize::from(original == request.expected_branch);
                    if checked_out > own_checkout { return Err("This branch is checked out in another worktree. Open that worktree before changing its branch.".into()); }
                    if request.action == "rename-branch" {
                        validate_ref_name(&runtime, &root, &request.name, true)?;
                        runtime.git(&root, &["branch", "--move", "--", original, &request.name])?
                    } else { runtime.git(&root, &["branch", "--delete", "--", original])? }
                } else {
                    // Atomically refuse deletion if another process moved this tag after review.
                    runtime.git(&root, &["update-ref", "-d", &full_ref, expected])?
                }
            }
            "create-tag" => {
                validate_ref_name(&runtime, &root, &request.name, false)?;
                let start = request.start_point.as_deref().filter(|oid| valid_oid(oid)).ok_or("Select a commit for the tag.")?;
                runtime.git(&root, &["cat-file", "-e", &format!("{start}^{{commit}}")])?;
                if request.annotated {
                    let message = request.message.as_deref().filter(|text| !text.trim().is_empty() && !text.contains('\0')).ok_or("Enter an annotation message without NUL characters.")?;
                    runtime.git(&root, &["tag", "--annotate", "--cleanup=verbatim", "--message", message, "--", &request.name, start])?
                } else {
                    // A lightweight tag has no tag object or signature, even if tag.gpgSign is enabled.
                    runtime.git(&root, &["tag", "--no-sign", "--", &request.name, start])?
                }
            }
            _ => return Err("Unsupported local reference action.".into()),
        };
        runtime.event(0, "local", "Git completed the reference action. Refreshing repository state.");
        Ok(match snapshot(runtime.clone(), display_path(&root)) {
            Ok(snapshot) => CommitOutcome { output, snapshot: Some(snapshot), refresh_error: None },
            Err(error) => CommitOutcome { output, snapshot: None, refresh_error: Some(error) },
        })
    })();
    if let Err(error) = &result { runtime.event(0, "local", format!("Reference action reported an error; review refreshed state: {error}")); }
    result
}
#[tauri::command]
async fn edit_reference(runtime: tauri::State<'_, Runtime>, request: RefRequest) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || execute_ref_action(runtime, request)).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn inspect_commit(runtime: tauri::State<'_, Runtime>, path: String, sha: String) -> Result<String, String> {
    if ![40, 64].contains(&sha.len()) || !sha.chars().all(|c| c.is_ascii_hexdigit()) { return Err("Invalid commit identifier.".into()); }
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || runtime.git(&repository(&path)?, &["show", "--no-ext-diff", "--no-textconv", "--no-color", "--format=fuller", &sha, "--"])).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn watch_repository(runtime: tauri::State<'_, Runtime>, path: String) -> Result<(), String> {
    if runtime.watch.swap(true, Ordering::SeqCst) { return Err("A refresh walkthrough is already running.".into()); }
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let result: Result<(), String> = (|| {
            let root = repository(&path)?;
            runtime.event(0, "local", "Walkthrough: eight real status checks, one per second.");
            for _ in 0..8 {
                if !runtime.watch.load(Ordering::SeqCst) { break; }
                runtime.git(&root, &["status", "--short", "--branch"])?;
                thread::sleep(Duration::from_secs(1));
            }
            runtime.event(0, "local", "Refresh walkthrough ended.");
            Ok(())
        })();
        runtime.watch.store(false, Ordering::SeqCst);
        result
    }).await.map_err(|e| e.to_string())?
}
#[tauri::command]
fn stop_watch(runtime: tauri::State<'_, Runtime>) { runtime.watch.store(false, Ordering::SeqCst); runtime.event(0, "local", "Requested stop: remaining refresh checks will not be scheduled."); }
#[tauri::command]
async fn demonstrate_failure(runtime: tauri::State<'_, Runtime>, path: String) -> Result<(), String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let root = repository(&path)?;
        let missing = format!("refs/git-k-prototype/missing-{}", now());
        runtime.git(&root, &["rev-parse", "--verify", &missing]).map(|_| ())
    }).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn create_demo(runtime: tauri::State<'_, Runtime>) -> Result<String, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let parent = std::env::var_os("GIT_K_PROTOTYPE_WORKSPACE").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join("git-k-prototype"));
        let root = parent.join(format!("demo-{}", now()));
        fs::create_dir_all(&parent).map_err(|e| e.to_string())?;
        fs::create_dir(&root).map_err(|e| e.to_string())?;
        fs::create_dir(root.join("src")).map_err(|e| e.to_string())?;
        runtime.event(0, "local", format!("Created disposable demo: {}", display_path(&root)));
        runtime.git(&root, &["init", "-b", "main"])?;
        let root_text = display_path(&root);
        runtime.git(&root, &["config", "--local", "core.worktree", &root_text])?;
        runtime.git(&root, &["config", "--local", "core.fsmonitor", "false"])?;
        runtime.git(&root, &["config", "--local", "user.name", "Git-K Demo"])?;
        runtime.git(&root, &["config", "--local", "user.email", "demo@git-k.local"])?;
        runtime.git(&root, &["config", "--local", "commit.gpgsign", "false"])?;
        let empty_hooks = root.join(".git").join("prototype-empty-hooks");
        fs::create_dir_all(&empty_hooks).map_err(|e| e.to_string())?;
        let empty_hooks_text = display_path(&empty_hooks);
        runtime.git(&root, &["config", "--local", "core.hooksPath", &empty_hooks_text])?;
        fs::write(root.join("README.md"), "# Aurora\n\nA disposable repository for exploring Git-K.\n").map_err(|e| e.to_string())?;
        fs::write(root.join("src/theme.css"), ":root {\n  --accent: #8b7aff;\n  --surface: #171d29;\n}\n").map_err(|e| e.to_string())?;
        runtime.event(0, "local", "Wrote demo README and theme files.");
        runtime.git(&root, &["add", "--", "."])?;
        let commit = |message: &str| runtime.git(&root, &["-c", "user.name=Git-K Demo", "-c", "user.email=demo@git-k.local", "-c", "commit.gpgsign=false", "commit", "-m", message]);
        commit("Initialize the Aurora workspace")?;
        runtime.git(&root, &["switch", "-c", "feature/theme"])?;
        fs::write(root.join("src/theme.css"), ":root {\n  --accent: #9e8cff;\n  --surface: #11151d;\n}\n").map_err(|e| e.to_string())?;
        runtime.event(0, "local", "Updated demo theme colors.");
        runtime.git(&root, &["add", "--", "src/theme.css"])?;
        commit("Refine the dark theme palette")?;
        runtime.git(&root, &["switch", "main"])?;
        fs::write(root.join("README.md"), "# Aurora\n\nA disposable repository for exploring Git-K.\n\n## Getting started\nOpen the workspace and inspect its history.\n").map_err(|e| e.to_string())?;
        runtime.event(0, "local", "Added demo getting-started documentation.");
        runtime.git(&root, &["add", "--", "README.md"])?;
        commit("Document the workspace setup")?;
        runtime.git(&root, &["-c", "user.name=Git-K Demo", "-c", "user.email=demo@git-k.local", "-c", "commit.gpgsign=false", "merge", "--no-ff", "feature/theme", "-m", "Merge the theme improvements"])?;
        fs::write(root.join("src/theme.css"), ":root {\n  --accent: #a99aff;\n  --surface: #11151d;\n  --success: #7bdba9;\n}\n").map_err(|e| e.to_string())?;
        fs::write(root.join("NOTES.md"), "# Next steps\n\nInspect this untracked file, then explore the commit graph.\n").map_err(|e| e.to_string())?;
        runtime.event(0, "local", "Left a modified theme and an untracked notes file for inspection.");
        Ok(display_path(&root))
    }).await.map_err(|e| e.to_string())?
}
fn main() {
    tauri::Builder::default().plugin(tauri_plugin_dialog::init()).manage(Runtime::default())
        .invoke_handler(tauri::generate_handler![initial_path, record_interface, journal, open_repository, inspect_file, change_index, latest_message, create_commit, change_branch, edit_reference, inspect_tag, inspect_commit, watch_repository, stop_watch, demonstrate_failure, create_demo])
        .run(tauri::generate_context!()).expect("Could not launch Git-K prototype");
}

#[cfg(test)]
mod ref_tests;
