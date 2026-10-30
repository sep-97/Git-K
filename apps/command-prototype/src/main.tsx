import { useEffect, useMemo, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import './style.css';

type Commit = { sha: string; parents: string[]; subject: string; author: string; age: string; refs: string };
type FileChange = { path: string; state: string; previous: string | null };
type Side = 'staged' | 'unstaged';
type Snapshot = { path: string; branch: string; head: string; files: FileChange[]; commits: Commit[]; branches: string[] };
type Event = { seq: number; operation: number; timestamp: number; kind: string; text: string };
type Operation = { id: number; command: string; cwd: string; output: string; status: string; end: string; timestamp: number };

function Icon({ name, size = 18 }: { name: string; size?: number }) {
  const paths: Record<string, React.ReactNode> = {
    branch: <><circle cx="6" cy="5" r="2"/><circle cx="6" cy="19" r="2"/><circle cx="18" cy="7" r="2"/><path d="M6 7v10m12-8c0 5-12 3-12 8"/></>,
    folder: <path d="M3 7V5h6l2 2h10v12H3V7z"/>,
    refresh: <><path d="M20 7v5h-5M4 17v-5h5"/><path d="M6 7a7 7 0 0 1 12-1l2 3M4 15l2 3a7 7 0 0 0 12-1"/></>,
    terminal: <><path d="m5 6 5 5-5 5m8 1h6"/></>,
    file: <><path d="M6 3h8l4 4v14H6zM14 3v5h4M9 12h6m-6 4h6"/></>,
    history: <><circle cx="12" cy="12" r="8"/><path d="M12 7v5l3 2"/></>,
    arrow: <path d="m8 5 7 7-7 7"/>,
    check: <path d="m5 12 4 4 10-10"/>,
    close: <path d="m6 6 12 12M18 6 6 18"/>,
    play: <path d="m8 5 11 7-11 7z"/>,
    stop: <rect x="6" y="6" width="12" height="12" rx="2"/>,
  };
  return <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">{paths[name] || paths.file}</svg>;
}
function Graph({ commits, selected, onSelect }: { commits: Commit[]; selected: string; onSelect: (commit: Commit) => void }) {
  const positions = new Map(commits.map((commit, i) => [commit.sha, i]));
  const lanes: string[] = [];
  const layout = commits.map(commit => {
    let lane = lanes.indexOf(commit.sha);
    if (lane < 0) { lane = lanes.findIndex(x => !x); if (lane < 0) lane = lanes.length; lanes[lane] = commit.sha; }
    lanes[lane] = commit.parents[0] || '';
    if (commit.parents[1] && !lanes.includes(commit.parents[1])) { const free = lanes.findIndex((x, i) => !x && i !== lane); lanes[free < 0 ? lanes.length : free] = commit.parents[1]; }
    return lane;
  });
  const colors = ['#aa96ff', '#69cbb9', '#e7b46e', '#75aaf5'];
  return <div className="graph-list">
    <svg className="graph-lines" width="90" height={commits.length * 65}>
      {commits.flatMap((commit, i) => commit.parents.map(parent => {
        const j = positions.get(parent);
        if (j === undefined) return null;
        const x1 = 21 + layout[i] * 19, x2 = 21 + layout[j] * 19, y1 = i * 65 + 32, y2 = j * 65 + 32;
        return <path key={commit.sha + parent} d={`M${x1},${y1} C${x1},${y1 + 28} ${x2},${y2 - 28} ${x2},${y2}`} stroke={colors[layout[i] % 4]} fill="none" strokeWidth="2" opacity=".55"/>;
      }))}
      {commits.map((commit, i) => <circle key={commit.sha} cx={21 + layout[i] * 19} cy={i * 65 + 32} r={commit.parents.length > 1 ? 6 : 4.5} fill="#141923" stroke={colors[layout[i] % 4]} strokeWidth="2.5"/>)}
    </svg>
    {commits.map(commit => <button className={`commit-row ${selected === commit.sha ? 'selected' : ''}`} key={commit.sha} onClick={() => onSelect(commit)}>
      <div className="commit-content"><div className="commit-title">{commit.subject}</div><div className="commit-meta"><span className="avatar">{commit.author.slice(0, 1)}</span>{commit.author}<span>·</span>{commit.age}{commit.refs && <span className="ref-label">{commit.refs}</span>}</div></div>
      <code className="sha">{commit.sha.slice(0, 7)}</code>
    </button>)}
    {!commits.length && <div className="empty small">No commits yet. This repository has an unborn branch.</div>}
  </div>;
}
function Diff({ text }: { text: string }) {
  const lines = text.split('\n');
  return <div className="diff-code">{lines.slice(0, 600).map((line, index) => <div className={`code-line ${line.startsWith('+') && !line.startsWith('+++') ? 'addition' : line.startsWith('-') && !line.startsWith('---') ? 'deletion' : line.startsWith('@@') ? 'hunk' : ''}`} key={index}><span className="line-number">{index + 1}</span><span>{line || ' '}</span></div>)}{lines.length > 600 && <div className="preview-limit">Preview shows the first 600 lines. Captured command output remains in this session's action history.</div>}</div>;
}
function isConflict(file: FileChange) { return ['DD', 'AU', 'UD', 'UA', 'DU', 'AA', 'UU'].includes(file.state); }
function Changes({ files, selected, busy, onSelect, onChange }: { files: FileChange[]; selected: string; busy: boolean; onSelect: (file: FileChange, side: Side) => void; onChange: (file: FileChange | null, stage: boolean) => void }) {
  const conflicts = files.filter(isConflict);
  const staged = files.filter(file => !isConflict(file) && file.state !== '??' && file.state[0] !== ' ');
  const unstaged = files.filter(file => !isConflict(file) && (file.state === '??' || file.state[1] !== ' '));
  const group = (side: Side, entries: FileChange[]) => <section className="change-group" aria-label={`${side === 'staged' ? 'Staged' : 'Unstaged'} files`}>
    <div className="change-group-heading"><h2>{side === 'staged' ? 'Staged' : 'Unstaged'} <span>{entries.length}</span></h2><button disabled={busy || !entries.length || !!conflicts.length} onClick={() => onChange(null, side === 'unstaged')}>{side === 'staged' ? 'Unstage all' : 'Stage all'}</button></div>
    {!entries.length && <p className="group-empty">{side === 'staged' ? 'No staged changes.' : 'No unstaged changes.'}</p>}
    {entries.map(file => <div className={`file-row ${selected === `${side}:${file.path}` ? 'selected' : ''}`} key={file.path}>
      <button className="file-select" onClick={() => onSelect(file, side)} disabled={busy} aria-label={`Inspect ${side} ${file.path}`}><Icon name="file" size={17}/><span><strong>{file.path}</strong>{file.previous && <small>from {file.previous}</small>}</span><code className={`file-state ${file.state === '??' ? 'untracked' : ''}`}>{file.state === '??' ? 'U' : file.state[side === 'staged' ? 0 : 1]}</code></button>
      <button className="file-action" disabled={busy || !!conflicts.length} onClick={() => onChange(file, side === 'unstaged')} aria-label={`${side === 'staged' ? 'Unstage' : 'Stage'} ${file.path}`}>{side === 'staged' ? 'Unstage' : 'Stage'}</button>
    </div>)}
  </section>;
  return <div className="changes-list"><div className="changes-summary">Staging updates Git's index. Your working files stay in place.</div>{!!conflicts.length && <div className="conflict-notice">{conflicts.length} unresolved conflict(s). Staging controls are unavailable in this prototype until conflicts are resolved.{conflicts.map(file => <button key={file.path} disabled={busy} onClick={() => onSelect(file, 'unstaged')}>{file.path}</button>)}</div>}{group('unstaged', unstaged)}{group('staged', staged)}</div>;
}
function App() {
  const [repo, setRepo] = useState<Snapshot | null>(null);
  const [path, setPath] = useState('');
  const [events, setEvents] = useState<Event[]>([]);
  const [panel, setPanel] = useState(true);
  const [busy, setBusy] = useState('');
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const [selected, setSelected] = useState('');
  const [title, setTitle] = useState('Inspect a change');
  const [detail, setDetail] = useState('Select a commit or changed file to inspect real Git output.');
  const [scope, setScope] = useState('Real output from the selected repository');
  const [watching, setWatching] = useState(false);
  const [expanded, setExpanded] = useState<number | null>(null);
  const [tab, setTab] = useState<'history' | 'changes'>('history');
  const cursor = useRef(0);
  const selectionEpoch = useRef(0);
  useEffect(() => {
    let alive = true, polling = false;
    const poll = async () => {
      if (polling) return; polling = true;
      try { const next = await invoke<Event[]>('journal', { after: cursor.current }); if (alive && next.length) { cursor.current = next[next.length - 1].seq; setEvents(old => [...old, ...next]); } }
      catch (e) { if (alive) setError(String(e)); }
      finally { polling = false; }
    };
    const timer = setInterval(poll, 250);
    invoke<string>('initial_path').then(value => { if (alive) { setPath(value); void load(value); } }).catch(e => setError(String(e)));
    return () => { alive = false; clearInterval(timer); };
  }, []);
  async function load(value: string) {
    if (!value.trim()) return;
    setBusy('Reading repository…'); setError(''); setNotice(''); selectionEpoch.current++;
    try { const data = await invoke<Snapshot>('open_repository', { path: value }); setRepo(data); setPath(data.path); setSelected(''); setScope('Real output from the selected repository'); setTitle('Repository opened'); setDetail('Select a commit or a changed file. Every Git command is recorded in the activity panel below.'); }
    catch (e) { setError(String(e)); }
    finally { setBusy(''); }
  }
  async function browse() {
    try { const value = await open({ directory: true, multiple: false, title: 'Open a Git repository' }); if (typeof value === 'string') await load(value); }
    catch (e) { setError(String(e)); }
  }
  async function demo() {
    setBusy('Preparing disposable repository…'); setError('');
    try { const value = await invoke<string>('create_demo'); await load(value); setTab('changes'); }
    catch (e) { setError(String(e)); }
    finally { setBusy(''); }
  }
  async function inspectCommit(commit: Commit) {
    if (!repo || busy) return;
    const epoch = ++selectionEpoch.current;
    setSelected(commit.sha); setScope(commit.sha); setTitle(commit.subject); setDetail('Reading commit…');
    try { const value = await invoke<string>('inspect_commit', { path: repo.path, sha: commit.sha }); if (epoch === selectionEpoch.current) setDetail(value); }
    catch (e) { if (epoch === selectionEpoch.current) setDetail(String(e)); }
  }
  async function inspectFile(file: FileChange, side: Side) {
    if (!repo || busy) return;
    const epoch = ++selectionEpoch.current;
    setSelected(`${side}:${file.path}`); setScope(file.state === '??' ? 'Untracked file · local content read' : side === 'staged' ? 'Staged · HEAD → index' : 'Unstaged · index → working tree'); setTitle(file.path); setDetail('Reading file…');
    try { const value = await invoke<string>('inspect_file', { path: repo.path, file: file.path, side: file.state === '??' && side === 'unstaged' ? 'untracked' : side }); if (epoch === selectionEpoch.current) setDetail(value || 'No textual diff for this selection.'); }
    catch (e) { if (epoch === selectionEpoch.current) setDetail(String(e)); }
  }
  async function changeIndex(file: FileChange | null, stage: boolean) {
    if (!repo || busy) return;
    setBusy(stage ? 'Staging changes…' : 'Unstaging changes…'); setError(''); setNotice(''); selectionEpoch.current++;
    setSelected(''); setTitle('Index update'); setScope('Working tree / index'); setDetail('Waiting for Git…');
    try {
      const data = await invoke<Snapshot>('change_index', { path: repo.path, file: file?.path ?? null, stage });
      setRepo(data); setNotice(`${stage ? 'Staged' : 'Unstaged'} ${file?.path ?? 'all available files'}. Working files preserved.`); setDetail('Git finished. Select a staged or unstaged file to inspect its current diff.');
    } catch (e) {
      setError(String(e)); setDetail('The action did not finish successfully. Inspect its recorded command and error below.');
      try { setRepo(await invoke<Snapshot>('open_repository', { path: repo.path })); }
      catch (refreshError) { setRepo(null); setError(`${String(e)} · Could not refresh repository: ${String(refreshError)}`); }
    } finally { setBusy(''); }
  }
  async function watch() {
    if (!repo) return;
    if (watching) { await invoke('stop_watch'); return; }
    setWatching(true); setError('');
    try { await invoke('watch_repository', { path: repo.path }); }
    catch (e) { setError(String(e)); }
    finally { setWatching(false); }
  }
  async function fail() {
    if (!repo) return;
    setError('');
    try { await invoke('demonstrate_failure', { path: repo.path }); }
    catch (e) { setError(`Expected walkthrough failure: ${String(e).trim()}`); }
  }
  function togglePanel() {
    const next = !panel; setPanel(next);
    void invoke('record_interface', { action: next ? 'Activity panel opened' : 'Activity panel collapsed' });
  }
  const operations = useMemo(() => {
    const map = new Map<number, Operation>();
    for (const event of events) {
      if (event.kind === 'command') map.set(event.operation, { id: event.operation, command: event.text, cwd: '', output: '', status: 'running', end: '', timestamp: event.timestamp });
      const operation = map.get(event.operation); if (!operation) continue;
      if (event.kind === 'context') operation.cwd = event.text;
      if (['stdout', 'stderr'].includes(event.kind)) operation.output += event.text + (event.text.endsWith('\n') ? '' : '\n');
      if (['completed', 'failed'].includes(event.kind)) { operation.status = event.kind; operation.end = event.text; }
    }
    return [...map.values()].reverse();
  }, [events]);
  const local = events.filter(event => event.kind === 'local').slice(-4).reverse();
  const repoName = repo?.path.split(/[\\/]/).filter(Boolean).at(-1) || 'No repository';
  const failed = operations.filter(op => op.status === 'failed').length;
  const active = operations.filter(op => op.status === 'running').length;
  return <div className="app">
    <header className="titlebar"><div className="brand"><span className="brand-icon"><Icon name="branch" size={23}/></span>Git-K<span className="prototype-badge">EXECUTION PROTOTYPE</span></div><div className="window-note">Tauri <span>·</span> Rust <span>·</span> React</div></header>
    <div className="repo-tab"><Icon name="folder" size={16}/><span>{repoName}</span><span className="tab-dot"/><span className="tab-caption">Local repository</span></div>
    <div className="toolbar"><div className="path-control"><Icon name="folder"/><input aria-label="Repository folder" value={path} disabled={!!busy} onChange={e => setPath(e.target.value)} onKeyDown={e => { if (e.key === 'Enter' && !busy) void load(path); }}/><button onClick={() => void load(path)} disabled={!!busy} className="path-open">Open</button></div><button onClick={() => void browse()} disabled={!!busy}><Icon name="folder"/>Browse</button><div className="toolbar-divider"/><button onClick={() => repo && void load(repo.path)} disabled={!repo || !!busy}><Icon name="refresh"/>Refresh</button><button onClick={() => void demo()} disabled={!!busy} className="accent-button">Create demo workspace</button></div>
    <div className="question"><span className="question-dot"/><span><strong>Stage with a clear view.</strong> Move files between working changes and the index. Every Git command stays recorded.</span><span className="read-only-tag">Staging changes the index · working files stay in place</span></div>
    {error && <div className="error-banner" role="alert"><span>{error}</span><button onClick={() => setError('')} aria-label="Dismiss message"><Icon name="close" size={14}/></button></div>}
    {notice && <div className="notice-banner" role="status"><Icon name="check" size={15}/><span>{notice}</span></div>}
    <div className="workspace">
      <aside className="sidebar"><div className="sidebar-repo"><div className="repo-avatar">{repoName.slice(0, 1).toUpperCase()}</div><div><strong>{repoName}</strong><small>{repo ? 'Repository connected' : 'Open a folder to begin'}</small></div></div><div className="section-label">WORKSPACE</div><button className={`side-action ${tab === 'history' ? 'active' : ''}`} onClick={() => setTab('history')}><Icon name="history"/>Commit history<span>{repo?.commits.length || 0}</span></button><button className={`side-action ${tab === 'changes' ? 'active' : ''}`} onClick={() => setTab('changes')}><Icon name="file"/>Working changes<span>{repo?.files.length || 0}</span></button><div className="section-label">LOCAL BRANCHES <span>{repo?.branches.length || 0}</span></div>{repo?.branches.map(branch => <div className={`branch-item ${branch === repo.branch ? 'current' : ''}`} key={branch}><Icon name="branch" size={15}/><span>{branch}</span>{branch === repo.branch && <span className="current-dot"/>}</div>)}<div className="sidebar-bottom"><div className="section-label">THIS BUILD</div><p>Real repository data.<br/>Stage and unstage files.<br/>In-memory action history.</p><small>Prototype only. Commits, remote actions and interactive shells are not implemented.</small></div></aside>
      <main className="main-panel"><div className="panel-heading"><div><h1>{tab === 'history' ? 'Commit history' : 'Working changes'}</h1><span>{repo?.branch ? <><Icon name="branch" size={13}/>{repo.branch}</> : 'Choose a repository'}</span></div><div className="count-pill">{tab === 'history' ? `${repo?.commits.length || 0} loaded` : `${repo?.files.length || 0} changes`}</div></div>{busy && <div className="loading-line">{busy}</div>}{repo ? tab === 'history' ? <Graph commits={repo.commits} selected={selected} onSelect={commit => void inspectCommit(commit)}/> : <Changes files={repo.files} selected={selected} busy={!!busy} onSelect={(file, side) => void inspectFile(file, side)} onChange={(file, stage) => void changeIndex(file, stage)}/> : <div className="empty"><div className="empty-icon"><Icon name="branch" size={34}/></div><h2>A clear view of your Git work</h2><p>Open an existing repository, or create a disposable demo to explore branches, changes and recorded commands.</p><button className="accent-button" onClick={() => void demo()} disabled={!!busy}>Create demo workspace</button></div>}</main>
      <section className="detail-panel"><div className="detail-heading"><span className="eyebrow">INSPECTOR</span><h2 title={title}>{title}</h2><small>{scope}</small></div><Diff text={detail}/></section>
    </div>
    <section className={`activity ${panel ? 'expanded' : 'collapsed'}`}><div className="activity-heading"><button className="activity-toggle" onClick={togglePanel}><Icon name="terminal" size={17}/><strong>Action history</strong><span className="recording-dot"/><span>{operations.length} Git commands recorded</span><span className="collapse-glyph">{panel ? '⌄' : '⌃'}</span></button><div className="activity-tools"><button onClick={() => void watch()} disabled={!repo || !!busy} className={watching ? 'watching' : ''}><Icon name={watching ? 'stop' : 'play'} size={13}/>{watching ? 'Stop remaining checks' : 'Run 8 live checks'}</button><button onClick={() => void fail()} disabled={!repo || !!busy}>Show a real failure</button></div></div>{panel && <div className="activity-body"><div className="operation-list">{operations.slice(0, 150).map(operation => <div className={`operation ${operation.status}`} key={operation.id}><button className="operation-heading" onClick={() => setExpanded(expanded === operation.id ? null : operation.id)}><span className="operation-symbol">{operation.status === 'completed' ? '✓' : operation.status === 'failed' ? '×' : '◌'}</span><time>{new Date(operation.timestamp).toLocaleTimeString([], { hour12: false })}</time><code>{operation.command}</code><span className="operation-result">{operation.end || 'running'}</span><Icon name="arrow" size={12}/></button>{expanded === operation.id && <div className="operation-output"><div className="cwd">{operation.cwd}</div><pre>{operation.output || '(No output)'}</pre></div>}</div>)}{!operations.length && <div className="activity-empty">Commands will appear here when you open a repository.</div>}{operations.length > 150 && <div className="preview-limit">Showing the latest 150 commands in the panel. This session retains all recorded events in memory.</div>}</div><div className="activity-context"><span className="eyebrow">LOCAL ACTIVITY</span>{local.map(event => <p key={event.seq}>{event.text}</p>)}<div className="session-stats"><span>{active} running</span><span>{failed} failed</span><span>{events.length} events</span></div></div></div>}</section>
    <footer><div><span className="footer-dot"/>{busy || (watching ? 'Live repository checks are running' : 'Ready')}<span className="footer-separator">/</span><Icon name="branch" size={12}/>{repo?.branch || 'No repository'}</div><div>Local session only<span className="footer-separator">/</span>File staging prototype</div></footer>
  </div>;
}
createRoot(document.getElementById('root')!).render(<App/>);
