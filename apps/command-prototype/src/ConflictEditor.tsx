import { useRef, useState } from 'react';

export type MergeIdentity = { path: string; head: string; branch: string; merge: string };
export type MergeReview = { revision: string; summary: string; message: string };
export type ConflictView = {
  file: string;
  revision: string;
  current_exists: boolean;
  incoming_exists: boolean;
  current: string | null;
  incoming: string | null;
  output: string | null;
  marker_size: number;
  external_tool: string | null;
  can_take_side: boolean;
};
export type Resolution = 'edit' | 'current' | 'incoming' | 'external';

function conflictBlocks(output: string, size: number) {
  const marker = (line: string, char: string) => {
    if (size > line.length) return false;
    const rest = line.slice(size);
    return line.startsWith(char.repeat(size)) && (!rest || /^[ \t\n]/.test(rest));
  };
  const lines = output.match(/[^\n]*\n|[^\n]+$/g) ?? [];
  const blocks: { start: number; end: number; current: string; incoming: string }[] = [];
  let offset = 0;
  let block: { start: number; side: 'current' | 'base' | 'incoming'; current: string; incoming: string } | undefined;
  for (const line of lines) {
    if (marker(line, '<')) block = { start: offset, side: 'current', current: '', incoming: '' };
    else if (block) {
      if (block.side === 'current' && marker(line, '|')) block.side = 'base';
      else if (block.side !== 'incoming' && marker(line, '=')) block.side = 'incoming';
      else if (block.side === 'incoming' && marker(line, '>')) {
        blocks.push({ start: block.start, end: offset + line.length, current: block.current, incoming: block.incoming });
        block = undefined;
      } else if (block.side !== 'base') block[block.side] += line;
    }
    offset += line.length;
  }
  return blocks;
}

export function ConflictEditor({ view, paths, busy, onOpen, onResolve }: {
  view: ConflictView;
  paths: string[];
  busy: boolean;
  onOpen: (path: string) => void;
  onResolve: (action: Resolution, value?: string) => void;
}) {
  const original = view.output?.replace(/\r\n?/g, '\n') ?? '';
  const [output, setOutput] = useState(original);
  const [search, setSearch] = useState('');
  const [blockIndex, setBlockIndex] = useState(0);
  const [searchStatus, setSearchStatus] = useState('');
  const [review, setReview] = useState<Exclude<Resolution, 'edit'> | null>(null);
  const textarea = useRef<HTMLTextAreaElement>(null);
  const dirty = original !== output;
  const fileIndex = paths.indexOf(view.file);
  const blocks = conflictBlocks(output, view.marker_size);
  const index = Math.min(blockIndex, blocks.length - 1);
  const block = blocks[index];
  function select(start: number, end: number) {
    textarea.current?.focus();
    textarea.current?.setSelectionRange(start, end);
  }
  function searchNext() {
    if (!search) return;
    const start = textarea.current?.selectionEnd ?? 0;
    const next = output.indexOf(search, start);
    const position = next < 0 ? output.indexOf(search) : next;
    if (position < 0) setSearchStatus('No matches in output.');
    else {
      select(position, position + search.length);
      setSearchStatus(`Match on line ${output.slice(0, position).split('\n').length}.`);
    }
  }
  function moveBlock(step: number) {
    const target = (index + step + blocks.length) % blocks.length;
    setBlockIndex(target);
    select(blocks[target].start, blocks[target].end);
  }
  function takeBlock(side: 'current' | 'incoming') {
    if (!block) return;
    setOutput(output.slice(0, block.start) + block[side] + output.slice(block.end));
  }
  const wholeFile = (side: 'current' | 'incoming', exists: boolean) => exists ? `Take ${side} file` : `Take ${side} deletion`;
  return <div className="conflict-editor">
    <div className="conflict-tools">
      <label>Unresolved file <select value={view.file} disabled={busy || dirty} onChange={e => onOpen(e.target.value)}>
        {paths.map(path => <option key={path}>{path}</option>)}
      </select></label>
      <button disabled={busy || dirty || fileIndex < 1} onClick={() => onOpen(paths[fileIndex - 1])}>Previous file</button>
      <button disabled={busy || dirty || fileIndex >= paths.length - 1} onClick={() => onOpen(paths[fileIndex + 1])}>Next file</button>
    </div>
    <div className="conflict-sides">
      {([['Current', view.current, view.current_exists], ['Incoming', view.incoming, view.incoming_exists]] as const).map(([label, content, exists]) =>
        <section aria-label={`${label} version`} key={label}><h3>{label}</h3><pre>{content ?? (exists ? 'Text preview unavailable.' : 'Deleted on this side.')}</pre></section>)}
    </div>
    {view.output !== null ? <>
      <div className="conflict-tools">
        <button disabled={busy || !block} onClick={() => moveBlock(-1)}>Previous block</button>
        <button disabled={busy || !block} onClick={() => moveBlock(1)}>Next block</button>
        <span aria-live="polite">{block ? `Block ${index + 1} of ${blocks.length}` : 'No conflict blocks'}</span>
        <button disabled={busy || !block} onClick={() => takeBlock('current')}>Use current block</button>
        <button disabled={busy || !block} onClick={() => takeBlock('incoming')}>Use incoming block</button>
      </div>
      <form className="conflict-tools" onSubmit={e => { e.preventDefault(); searchNext(); }}>
        <label>Search output <input value={search} onChange={e => { setSearch(e.target.value); setSearchStatus(''); }} /></label>
        <button type="submit" disabled={busy || !search}>Find next</button><span role="status">{searchStatus}</span>
      </form>
      <label htmlFor="conflict-output">Output</label>
      <textarea id="conflict-output" ref={textarea} disabled={busy} value={output} onChange={e => setOutput(e.target.value)} spellCheck={false} wrap="off" rows={8} />
      {dirty && <small>Save or reset this draft before opening a different conflict.</small>}
    </> : <p>Text editing is unavailable for this file. Choose a whole version or use an external merge tool.</p>}
    <div className="conflict-tools">
      <button disabled={busy || !view.can_take_side} onClick={() => setReview('current')}>{wholeFile('current', view.current_exists)}</button>
      <button disabled={busy || !view.can_take_side} onClick={() => setReview('incoming')}>{wholeFile('incoming', view.incoming_exists)}</button>
      <button disabled={busy || !view.external_tool} onClick={() => setReview('external')}>Open external tool…</button>
      {view.output !== null && <><button disabled={busy || !dirty} onClick={() => { setOutput(original); setBlockIndex(0); }}>Reset draft</button>
        <button className="accent-button" disabled={busy || !!block || !!review} onClick={() => onResolve('edit', output)}>Save and stage output</button></>}
    </div>
    {!view.external_tool && <small>Set Git’s <code>merge.tool</code> to enable the external tool.</small>}
    {review && <div className="conflict-tools" role="group" aria-label="Review whole-file resolution">
      <span>{review === 'external' ? `Run ${view.external_tool} through Git? It can change and stage this file.` : `${wholeFile(review, review === 'current' ? view.current_exists : view.incoming_exists)} and stage the result?`} {dirty && 'This replaces your editor draft if it succeeds.'}</span>
      <button disabled={busy} onClick={() => setReview(null)}>Cancel action</button>
      <button className="accent-button" disabled={busy} onClick={() => { setReview(null); onResolve(review, review === 'external' ? view.external_tool ?? undefined : undefined); }}>Apply resolution</button>
    </div>}
  </div>;
}
