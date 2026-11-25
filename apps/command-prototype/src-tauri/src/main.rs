#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use serde::Serialize;
#[cfg(any(feature = "desktop", test))]
mod conflicts;
#[cfg(any(feature = "desktop", test))]
use conflicts::*;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{atomic::{AtomicU64, Ordering}, Arc, Mutex},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
#[cfg(feature = "desktop")]
use std::{sync::atomic::AtomicBool, time::Duration};

#[derive(Clone, Serialize)]
struct Event { seq: u64, operation: u64, timestamp: u64, kind: String, text: String }
#[derive(Clone, Default)]
struct Runtime {
    events: Arc<Mutex<Vec<Event>>>,
    next: Arc<AtomicU64>,
    #[cfg(feature = "desktop")]
    watch: Arc<AtomicBool>,
    mutation: Arc<Mutex<()>>,
}
struct GitOutput { status: std::process::ExitStatus, stdout: String, stderr: String, #[cfg(any(feature = "desktop", test))] stdout_bytes: Vec<u8> }
impl GitOutput {
    fn error(&self) -> String {
        let error = self.stderr.trim();
        if error.is_empty() { format!("Git exited with {}. Inspect the recorded output in Action history.", self.status.code().map(|v| v.to_string()).unwrap_or_else(|| "an interruption".into())) }
        else { error.into() }
    }
}
impl Runtime {
    fn event(&self, operation: u64, kind: &str, text: impl Into<String>) {
        let mut events = self.events.lock().unwrap();
        let seq = events.len() as u64 + 1;
        events.push(Event { seq, operation, timestamp: now(), kind: kind.into(), text: text.into() });
    }
    fn git(&self, cwd: &Path, args: &[&str]) -> Result<String, String> {
        let output = self.git_output(cwd, args)?;
        if output.status.success() { Ok(output.stdout) } else { Err(output.error()) }
    }
    fn git_output(&self, cwd: &Path, args: &[&str]) -> Result<GitOutput, String> {
        self.git_output_with_capture(cwd, args, true)
    }
    fn git_output_with_capture(&self, cwd: &Path, args: &[&str], journal_stdout: bool) -> Result<GitOutput, String> {
        let id = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        let executable = std::env::var_os("GIT_K_GIT").unwrap_or_else(|| "git".into());
        let mut effective_args = vec!["--no-pager"];
        effective_args.extend_from_slice(args);
        let args = effective_args.as_slice();
        let display = command_display(&executable.to_string_lossy(), args);
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
                    Ok(_) => { if journal_stdout { first.event(id, "stdout", String::from_utf8_lossy(&bytes).into_owned()); } collected.extend(bytes); },
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
        if !journal_stdout { self.event(id, "stdout", format!("{} bytes read for conflict preview (contents omitted)", out.len())); }
        self.event(id, if result.success() { "completed" } else { "failed" }, format!("exit {} · {} ms", result.code().map(|v| v.to_string()).unwrap_or_else(|| "interrupted".into()), now() - started));
        Ok(GitOutput { status: result, stdout: String::from_utf8_lossy(&out).into_owned(), stderr: String::from_utf8_lossy(&err).into_owned(), #[cfg(any(feature = "desktop", test))] stdout_bytes: out })
    }
}
fn now() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 }
fn command_display(executable: &str, args: &[&str]) -> String {
    #[cfg(windows)]
    {
        // Pass one CRT-escaped command line through the cmdlet, avoiding the
        // different native argument binders in Windows PowerShell 5.1 and PS 7.
        let arguments = args.iter().map(|value| windows_argument(value)).collect::<Vec<_>>().join(" ");
        format!("$gitKProcess = Start-Process -FilePath {} -ArgumentList {} -NoNewWindow -Wait -PassThru; $global:LASTEXITCODE = $gitKProcess.ExitCode", quote(executable), quote(&arguments))
    }
    #[cfg(not(windows))]
    { format!("{} {}", quote(executable), args.iter().map(|s| quote(s)).collect::<Vec<_>>().join(" ")) }
}
#[cfg(windows)]
fn windows_argument(value: &str) -> String {
    let mut result = String::from("\"");
    let mut backslashes = 0;
    for character in value.chars() {
        if character == '\\' { backslashes += 1; continue; }
        if character == '"' {
            result.push_str(&"\\".repeat(backslashes * 2 + 1));
        } else { result.push_str(&"\\".repeat(backslashes)); }
        result.push(character);
        backslashes = 0;
    }
    result.push_str(&"\\".repeat(backslashes * 2));
    result.push('"');
    result
}
fn quote(value: &str) -> String {
    if !value.is_empty() && value.chars().all(|c| c.is_ascii_alphanumeric() || "-_/.:=".contains(c)) { value.into() }
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
struct Snapshot { path: String, branch: String, head: String, merge_head: String, files: Vec<FileChange>, commits: Vec<Commit>, branches: Vec<LocalRef>, tags: Vec<LocalRef> }
#[derive(Serialize)]
struct CommitOutcome { output: String, snapshot: Option<Snapshot>, refresh_error: Option<String> }
#[derive(Clone, serde::Deserialize)]
struct MergeRequest {
    path: String, expected_head: String, expected_branch: String,
    target: String, source_branch: Option<String>, mode: String, message: String, abort: bool,
}
fn merge_head(runtime: &Runtime, root: &Path) -> Result<String, String> {
    let path = runtime.git(root, &["rev-parse", "--git-path", "MERGE_HEAD"])?;
    match fs::read_to_string(root.join(path.trim())) {
        Ok(value) => Ok(value.trim().into()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(format!("Could not read the current merge state: {error}")),
    }
}
fn protect_merge_paths(runtime: &Runtime, root: &Path, tree_oid: &str) -> Result<(), String> {
    // Git's ort merge path can overwrite ignored files even with
    // --no-overwrite-ignore. Inspect the computed result, including rename paths.
    let tree = runtime.git(root, &["ls-tree", "-r", "--name-only", "-z", tree_oid])?;
    let incoming: std::collections::BTreeSet<String> = tree.split('\0').filter(|path| !path.is_empty()).map(String::from).collect();
    let local = runtime.git(root, &["ls-files", "--others", "-z", "--"])?;
    for path in local.split('\0').filter(|path| !path.is_empty()) {
        let path = path.trim_end_matches('/');
        let prefix = format!("{path}/");
        let descendant = incoming.range(prefix.clone()..).next().is_some_and(|entry| entry.starts_with(&prefix));
        let mut ancestor = path;
        let mut collision = incoming.contains(path) || descendant;
        while let Some(index) = ancestor.rfind('/') {
            ancestor = &ancestor[..index]; collision |= incoming.contains(ancestor);
        }
        if collision { return Err(format!("A local untracked or ignored path collides with incoming history: {path}. Move or preserve it before merging.")); }
    }
    Ok(())
}
fn execute_merge(runtime: Runtime, request: MergeRequest) -> Result<CommitOutcome, String> {
    let result = (|| {
        let _guard = runtime.mutation.try_lock().map_err(|_| "Another repository change is still running.".to_string())?;
        runtime.event(0, "local", if request.abort { "Requested abort of the reviewed merge.".into() } else { format!("Requested {} merge of {} into {}.", request.mode, request.target, request.expected_branch) });
        let root = operation_target(&runtime, &request.path, &request.expected_head, &request.expected_branch, request.abort)?;
        let output = if request.abort {
            let actual = merge_head(&runtime, &root)?;
            if actual.is_empty() || actual != request.target { return Err("The pending merge changed. Refresh and review it before aborting.".into()); }
            runtime.git(&root, &["merge", "--abort"])?
        } else {
            if request.expected_branch == "Detached HEAD" || request.expected_head.is_empty() { return Err("Check out a branch with an existing commit before merging.".into()); }
            if !valid_oid(&request.target) { return Err("Select an existing commit to merge.".into()); }
            if !["normal", "merge-commit", "squash"].contains(&request.mode.as_str()) { return Err("Choose a supported merge mode.".into()); }
            if request.message.contains('\0') || (request.mode != "squash" && request.message.trim().is_empty()) { return Err("Enter a merge message without NUL characters.".into()); }
            if changes(&runtime, &root)?.iter().any(|file| file.state != "??") { return Err("Commit or clear staged and unstaged tracked changes before merging. This prototype never automatically stashes them.".into()); }
            if let Some(source) = &request.source_branch {
                validate_ref_name(&runtime, &root, source, true)?;
                let actual = runtime.git(&root, &["show-ref", "--hash", "--verify", &format!("refs/heads/{source}")])?;
                if actual.trim() != request.target { return Err("The incoming branch changed since review. Cancel and review its refreshed state.".into()); }
            }
            runtime.git(&root, &["cat-file", "-e", &format!("{}^{{commit}}", request.target)])?;
            // Preview also supplies paths produced by directory renames. A conflicted
            // preview has exit 1 and still supplies a tree; other failures stop here.
            let preview = runtime.git_output(&root, &["merge-tree", "--write-tree", &request.expected_head, &request.target])?;
            if !preview.status.success() && preview.status.code() != Some(1) { return Err(format!("Merge preview unavailable. Git 2.38+ is required; no merge was started. {}", preview.error())); }
            let tree = preview.stdout.lines().next().filter(|oid| valid_oid(oid)).ok_or("Merge preview did not return a valid result tree. No merge was started.")?;
            protect_merge_paths(&runtime, &root, tree)?;
            if request.mode == "squash" && !preview.status.success() {
                // Conflicting squashes have no MERGE_HEAD for merge --abort.
                return Err("Squash preview found conflicts. No merge was started; your index and working files stay in place. Use the later conflict workflow for this squash.".into());
            }
            // Clear branch merge flags so they cannot override the reviewed mode.
            // Configured identity, signing, hooks, attributes and merge drivers still apply.
            let branch_options = format!("branch.{}.mergeOptions=", request.expected_branch);
            let mut args = vec!["-c", &branch_options, "merge", "--no-edit", "--no-autostash", "--no-overwrite-ignore", "--no-rerere-autoupdate"];
            match request.mode.as_str() {
                "normal" => args.extend(["--no-squash", "--commit", "--ff"]),
                "merge-commit" => args.extend(["--no-squash", "--commit", "--no-ff"]),
                "squash" => args.extend(["--squash", "--no-commit", "--ff"]),
                _ => unreachable!(),
            }
            if request.mode != "squash" { args.extend(["--cleanup=verbatim", "--message", &request.message]); }
            args.extend(["--", &request.target]);
            runtime.git(&root, &args)?
        };
        runtime.event(0, "local", "Git completed the merge action. Refreshing repository state.");
        Ok(match snapshot(runtime.clone(), display_path(&root)) {
            Ok(snapshot) => CommitOutcome { output, snapshot: Some(snapshot), refresh_error: None },
            Err(error) => CommitOutcome { output, snapshot: None, refresh_error: Some(error) },
        })
    })();
    if let Err(error) = &result { runtime.event(0, "local", format!("Merge action reported an error; review actual state before retrying: {error}")); }
    result
}
#[cfg(feature = "desktop")]
#[tauri::command]
async fn merge_branch(runtime: tauri::State<'_, Runtime>, request: MergeRequest) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || execute_merge(runtime, request)).await.map_err(|e| e.to_string())?
}
#[cfg(feature = "desktop")]
#[tauri::command]
async fn inspect_conflict(
    runtime: tauri::State<'_, Runtime>,
    identity: MergeIdentity,
    file: String,
) -> Result<ConflictView, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || read_conflict(&runtime, &identity, &file))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg(feature = "desktop")]
#[tauri::command]
async fn apply_conflict(
    runtime: tauri::State<'_, Runtime>,
    identity: MergeIdentity,
    file: String,
    revision: String,
    action: String,
    value: Option<String>,
) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        resolve_conflict(runtime, identity, file, revision, action, value)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(feature = "desktop")]
#[tauri::command]
async fn review_pending_merge(
    runtime: tauri::State<'_, Runtime>,
    identity: MergeIdentity,
) -> Result<MergeReview, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || review_merge(&runtime, &identity))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg(feature = "desktop")]
#[tauri::command]
async fn complete_pending_merge(
    runtime: tauri::State<'_, Runtime>,
    identity: MergeIdentity,
    revision: String,
) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || finish_merge(runtime, identity, revision))
        .await
        .map_err(|e| e.to_string())?
}
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
#[cfg(feature = "desktop")]
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
        let history = runtime.git(&path, &["log", "--all", "--date-order", "-n", "60", "--format=%H%x1f%P%x1f%s%x1f%an%x1f%ar%x1f%D", "HEAD", "--"])?;
        history.lines().filter_map(|line| {
            let fields: Vec<_> = line.split('\x1f').collect();
            if fields.len() != 6 { return None; }
            Some(Commit { sha: fields[0].into(), parents: fields[1].split_whitespace().map(String::from).collect(), subject: fields[2].into(), author: fields[3].into(), age: fields[4].into(), refs: fields[5].into() })
        }).collect()
    } else { Vec::new() };
    let merge_head = merge_head(&runtime, &path)?;
    Ok(Snapshot { path: display_path(&path), branch: if branch.trim().is_empty() { "Detached HEAD".into() } else { branch.trim().into() }, head: head.trim().into(), merge_head, files, commits, branches, tags })
}
#[cfg(feature = "desktop")]
#[tauri::command]
fn initial_path() -> String {
    if let Some(path) = std::env::args_os().nth(1) { return display_path(Path::new(&path)); }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    cwd.ancestors().find(|path| path.join(".git").exists()).map(display_path).unwrap_or_default()
}
#[cfg(feature = "desktop")]
#[tauri::command]
fn record_interface(runtime: tauri::State<'_, Runtime>, action: String) -> Result<(), String> {
    match action.as_str() {
        "Activity panel collapsed" | "Activity panel opened" => { runtime.event(0, "local", action); Ok(()) },
        _ => Err("Unsupported interface activity.".into())
    }
}
#[cfg(feature = "desktop")]
#[tauri::command]
fn journal(runtime: tauri::State<'_, Runtime>, after: u64) -> Vec<Event> {
    runtime.events.lock().unwrap().iter().filter(|e| e.seq > after).take(1000).cloned().collect()
}
#[cfg(feature = "desktop")]
#[tauri::command]
async fn open_repository(runtime: tauri::State<'_, Runtime>, path: String) -> Result<Snapshot, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || snapshot(runtime, path)).await.map_err(|e| e.to_string())?
}
#[cfg(feature = "desktop")]
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
#[cfg(feature = "desktop")]
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
#[cfg(feature = "desktop")]
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
    operation_target(runtime, path, expected_head, expected_branch, false)
}
fn operation_target(runtime: &Runtime, path: &str, expected_head: &str, expected_branch: &str, allow_merge: bool) -> Result<PathBuf, String> {
    let requested = repository(path)?;
    let root = PathBuf::from(runtime.git(&requested, &["rev-parse", "--show-toplevel"])?.trim());
    let head = runtime.git(&root, &["rev-parse", "--revs-only", "HEAD"])?;
    let branch = runtime.git(&root, &["branch", "--show-current"])?;
    let branch = if branch.trim().is_empty() { "Detached HEAD" } else { branch.trim() };
    if head.trim() != expected_head || branch != expected_branch { return Err("The current branch or HEAD changed since this view was loaded. Review the refreshed repository and reopen any amend or branch dialog before trying again.".into()); }
    for marker in ["MERGE_HEAD", "CHERRY_PICK_HEAD", "REVERT_HEAD", "rebase-merge", "rebase-apply", "sequencer"] {
        if allow_merge && marker == "MERGE_HEAD" { continue; }
        let marker_path = runtime.git(&root, &["rev-parse", "--git-path", marker])?;
        if root.join(marker_path.trim()).exists() { return Err("An unfinished merge, rebase, cherry-pick or revert needs its own continuation workflow. Finish it outside this prototype before changing branches or committing.".into()); }
    }
    if !allow_merge && changes(runtime, &root)?.iter().any(conflicted) { return Err("Resolve repository conflicts before changing branches or committing.".into()); }
    Ok(root)
}
fn execute_commit(runtime: Runtime, path: String, expected_head: String, expected_branch: String, summary: String, description: String, amend: bool) -> Result<CommitOutcome, String> {
    let result = (|| {
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
    })();
    if let Err(error) = &result { runtime.event(0, "local", format!("Commit did not complete: {error}")); }
    result
}
#[cfg(feature = "desktop")]
#[tauri::command]
async fn create_commit(runtime: tauri::State<'_, Runtime>, path: String, expected_head: String, expected_branch: String, summary: String, description: String, amend: bool) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || execute_commit(runtime, path, expected_head, expected_branch, summary, description, amend)).await.map_err(|e| e.to_string())?
}

fn execute_branch(runtime: Runtime, path: String, expected_head: String, expected_branch: String, name: String, start_point: Option<String>, checkout: bool) -> Result<CommitOutcome, String> {
    let result = (|| {
        runtime.event(0, "local", format!("Requested {} branch: {name}", if start_point.is_some() { "creation of" } else { "checkout of" }));
        let _guard = runtime.mutation.try_lock().map_err(|_| "Another repository change is still running.".to_string())?;
        let root = commit_target(&runtime, &path, &expected_head, &expected_branch)?;
        validate_ref_name(&runtime, &root, &name, true)?;
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
    })();
    if let Err(error) = &result { runtime.event(0, "local", format!("Branch action reported an error; repository state may have changed: {error}")); }
    result
}
#[cfg(feature = "desktop")]
#[tauri::command]
async fn change_branch(runtime: tauri::State<'_, Runtime>, path: String, expected_head: String, expected_branch: String, name: String, start_point: Option<String>, checkout: bool) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || execute_branch(runtime, path, expected_head, expected_branch, name, start_point, checkout)).await.map_err(|e| e.to_string())?
}

#[cfg(feature = "desktop")]
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
        // --branch can expand @{-n}; accept only the exact literal spelling supplied.
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
            "checkout-commit" => {
                let target = request.start_point.as_deref().filter(|oid| valid_oid(oid)).ok_or("Select an existing commit to check out.")?;
                // Only immutable object IDs from the reviewed commit are accepted.
                runtime.git(&root, &["cat-file", "-e", &format!("{target}^{{commit}}")])?;
                runtime.git(&root, &["switch", "--detach", "--no-guess", "--no-overwrite-ignore", "--no-recurse-submodules", "--", target])?
            }
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
#[cfg(feature = "desktop")]
#[tauri::command]
async fn edit_reference(runtime: tauri::State<'_, Runtime>, request: RefRequest) -> Result<CommitOutcome, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || execute_ref_action(runtime, request)).await.map_err(|e| e.to_string())?
}
#[cfg(feature = "desktop")]
#[tauri::command]
async fn inspect_commit(runtime: tauri::State<'_, Runtime>, path: String, sha: String) -> Result<String, String> {
    if ![40, 64].contains(&sha.len()) || !sha.chars().all(|c| c.is_ascii_hexdigit()) { return Err("Invalid commit identifier.".into()); }
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || runtime.git(&repository(&path)?, &["show", "--no-ext-diff", "--no-textconv", "--no-color", "--format=fuller", &sha, "--"])).await.map_err(|e| e.to_string())?
}
#[cfg(feature = "desktop")]
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
#[cfg(feature = "desktop")]
#[tauri::command]
fn stop_watch(runtime: tauri::State<'_, Runtime>) { runtime.watch.store(false, Ordering::SeqCst); runtime.event(0, "local", "Requested stop: remaining refresh checks will not be scheduled."); }
#[cfg(feature = "desktop")]
#[tauri::command]
async fn demonstrate_failure(runtime: tauri::State<'_, Runtime>, path: String) -> Result<(), String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let root = repository(&path)?;
        let missing = format!("refs/git-k-prototype/missing-{}", now());
        runtime.git(&root, &["rev-parse", "--verify", &missing]).map(|_| ())
    }).await.map_err(|e| e.to_string())?
}
#[cfg(feature = "desktop")]
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
#[cfg(feature = "desktop")]
fn main() {
    tauri::Builder::default().plugin(tauri_plugin_dialog::init()).manage(Runtime::default())
        .invoke_handler(tauri::generate_handler![initial_path, record_interface, journal, open_repository, inspect_file, change_index, latest_message, create_commit, change_branch, merge_branch, inspect_conflict, apply_conflict, review_pending_merge, complete_pending_merge, edit_reference, inspect_tag, inspect_commit, watch_repository, stop_watch, demonstrate_failure, create_demo])
        .run(tauri::generate_context!()).expect("Could not launch Git-K prototype");
}

#[cfg(not(feature = "desktop"))]
fn main() { eprintln!("Enable the desktop feature to launch the Tauri application."); }

#[cfg(test)]
mod ref_tests;
