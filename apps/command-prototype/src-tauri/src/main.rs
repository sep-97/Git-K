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
        let display = format!("git {}", args.iter().map(|s| quote(s)).collect::<Vec<_>>().join(" "));
        self.event(id, "command", display);
        self.event(id, "context", cwd.display().to_string());
        let mut command = Command::new(executable);
        command.args(args).current_dir(cwd).env("GIT_TERMINAL_PROMPT", "0").env("GIT_OPTIONAL_LOCKS", "0")
            .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
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
        if !result.success() { return Err(String::from_utf8_lossy(&err).trim().to_string()); }
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
struct FileChange { path: String, state: String }
#[derive(Serialize)]
struct Commit { sha: String, parents: Vec<String>, subject: String, author: String, age: String, refs: String }
#[derive(Serialize)]
struct Snapshot { path: String, branch: String, head: String, files: Vec<FileChange>, commits: Vec<Commit>, branches: Vec<String> }
fn snapshot(runtime: Runtime, input: String) -> Result<Snapshot, String> {
    runtime.event(0, "local", format!("Open / refresh repository: {input}"));
    let requested = repository(&input)?;
    let root = runtime.git(&requested, &["rev-parse", "--show-toplevel"])?;
    let path = PathBuf::from(root.trim());
    let branch = runtime.git(&path, &["branch", "--show-current"])?;
    let status = runtime.git(&path, &["status", "--porcelain=v1", "--untracked-files=all", "-z"])?;
    let mut entries = status.split('\0').filter(|s| !s.is_empty());
    let mut files = Vec::new();
    while let Some(entry) = entries.next() {
        if entry.len() < 3 { continue; }
        let state = entry[..2].to_string();
        files.push(FileChange { path: entry[3..].into(), state: state.clone() });
        if state.contains('R') || state.contains('C') { entries.next(); }
    }
    let branches = runtime.git(&path, &["for-each-ref", "--format=%(refname:short)", "refs/heads"])?;
    let head = runtime.git(&path, &["rev-parse", "--verify", "HEAD"]);
    let commits = if head.is_ok() {
        let history = runtime.git(&path, &["log", "--all", "--date-order", "-n", "60", "--format=%H%x1f%P%x1f%s%x1f%an%x1f%ar%x1f%D"])?;
        history.lines().filter_map(|line| {
            let fields: Vec<_> = line.split('\x1f').collect();
            if fields.len() != 6 { return None; }
            Some(Commit { sha: fields[0].into(), parents: fields[1].split_whitespace().map(String::from).collect(), subject: fields[2].into(), author: fields[3].into(), age: fields[4].into(), refs: fields[5].into() })
        }).collect()
    } else { Vec::new() };
    Ok(Snapshot { path: display_path(&path), branch: if branch.trim().is_empty() { "Detached HEAD".into() } else { branch.trim().into() }, head: head.unwrap_or_default().trim().into(), files, commits, branches: branches.lines().map(String::from).collect() })
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
async fn inspect_file(runtime: tauri::State<'_, Runtime>, path: String, file: String, untracked: bool) -> Result<String, String> {
    let runtime = runtime.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let root = repository(&path)?;
        let relative = Path::new(&file);
        if relative.is_absolute() || relative.components().any(|c| matches!(c, std::path::Component::ParentDir | std::path::Component::Prefix(_))) { return Err("File must belong to the selected repository.".into()); }
        if untracked {
            let selected = root.join(relative).canonicalize().map_err(|e| e.to_string())?;
            if !selected.starts_with(&root) { return Err("Selected file resolves outside this repository.".into()); }
            runtime.event(0, "local", format!("Read untracked file: {file}"));
            let bytes = fs::read(selected).map_err(|e| e.to_string())?;
            return Ok(String::from_utf8_lossy(&bytes).into_owned());
        }
        let unstaged = runtime.git(&root, &["diff", "--no-ext-diff", "--no-textconv", "--no-color", "--", &file])?;
        let staged = runtime.git(&root, &["diff", "--cached", "--no-ext-diff", "--no-textconv", "--no-color", "--", &file])?;
        Ok(format!("{unstaged}{staged}"))
    }).await.map_err(|e| e.to_string())?
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
        .invoke_handler(tauri::generate_handler![initial_path, record_interface, journal, open_repository, inspect_file, inspect_commit, watch_repository, stop_watch, demonstrate_failure, create_demo])
        .run(tauri::generate_context!()).expect("Could not launch Git-K prototype");
}
