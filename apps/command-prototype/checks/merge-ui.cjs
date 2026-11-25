// Browser contract checks use an explicit IPC fixture. Real Git outcomes are
// checked separately by src-tauri/src/merge_tests.rs; this is not native evidence.
const { chromium } = require('playwright');
const assert = require('node:assert/strict');
const { spawn } = require('node:child_process');
const path = require('node:path');

async function run() {
  const root = path.resolve(__dirname, '..');
  const server = spawn(process.execPath, [path.join(root, 'node_modules/vite/bin/vite.js'), '--host', '127.0.0.1', '--port', '1421', '--strictPort'], { cwd: root, stdio: ['ignore', 'pipe', 'pipe'] });
  let serverOutput = ''; let exited = false;
  server.stdout.on('data', value => { serverOutput += value; });
  server.stderr.on('data', value => { serverOutput += value; });
  server.once('exit', () => { exited = true; });
  let browser;
  try {
    let ready = false;
    for (let attempt = 0; attempt < 100; attempt++) {
      if (exited) throw new Error(`Vite stopped: ${serverOutput}`);
      if (serverOutput.includes('http://127.0.0.1:1421')) { ready = true; break; }
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    assert(ready, `Vite did not start: ${serverOutput}`);
    browser = await chromium.launch({ headless: true, ...(process.env.GIT_K_BROWSER ? { executablePath: process.env.GIT_K_BROWSER } : {}) });
    const errors = [];
    async function pageFor(kind = 'clean') {
      const page = await browser.newPage({ viewport: { width: 1400, height: 850 } });
      page.setDefaultTimeout(10000);
      page.on('pageerror', error => errors.push(String(error)));
      await page.addInitScript(({ kind }) => {
        const head = 'a'.repeat(40), incoming = 'b'.repeat(40);
        window.mergeFixture = {
          state: { path: '/merge-ui-fixture', branch: kind === 'detached' ? 'Detached HEAD' : 'main', head, merge_head: '', files: kind === 'dirty' ? [{ path: 'draft.txt', state: ' M', previous: null }] : [],
            commits: [{ sha: incoming, parents: [head], subject: 'Incoming topic', author: 'Fixture', age: 'now', refs: 'topic&literal' }, { sha: head, parents: [], subject: 'Current history', author: 'Fixture', age: 'now', refs: 'main' }],
            branches: [{ name: 'main', oid: head, kind: 'commit' }, { name: 'topic&literal', oid: incoming, kind: 'commit' }], tags: [] },
          requests: [], steps: [], behavior: kind, panel: [],
        };
        window.__TAURI_INTERNALS__ = { invoke: async (command, args) => {
          const fixture = window.mergeFixture;
          if (command === 'initial_path') return fixture.state.path;
          if (command === 'journal') return [];
          if (command === 'record_interface') { fixture.panel.push(args.action); return; }
          if (command === 'inspect_commit') return 'Explicit UI fixture commit details';
          if (command === 'open_repository') return structuredClone(fixture.state);
          if (command === 'inspect_conflict') {
            const file = args.file;
            if (file === 'binary.dat') return { file, revision: 'binary-1', current_exists: true, incoming_exists: true, current: null, incoming: null, output: null, marker_size: 7, external_tool: 'fixture', can_take_side: true };
            return { file, revision: 'text-1', current_exists: true, incoming_exists: true, current: 'current', incoming: 'incoming', output: 'header\n<<<<<<< HEAD\ncurrent\n||||||| base\noriginal\n=======\nincoming\n>>>>>>> topic\nfooter\n', marker_size: 7, external_tool: null, can_take_side: true };
          }
          if (command === 'review_pending_merge') return { revision: 'index-1', summary: 'file.txt | 1 +', message: 'Merge fixture' };
          if (command === 'apply_conflict' || command === 'complete_pending_merge') {
            fixture.steps.push(structuredClone(args));
            if (command === 'apply_conflict') {
              if (args.value === 'fail') throw 'Fixture index lock';
              fixture.state.files = fixture.state.files.filter(file => file.path !== args.file);
            } else {
              fixture.state.merge_head = ''; fixture.state.head = 'd'.repeat(40);
            }
            return { output: 'Fixture Git result', snapshot: structuredClone(fixture.state), refresh_error: null };
          }
          if (command !== 'merge_branch') throw `Unexpected fixture command: ${command}`;
          const request = args.request; fixture.requests.push(structuredClone(request));
          if (request.abort) { fixture.state.merge_head = ''; fixture.state.files = []; }
          else if (['conflict', 'resolve'].includes(fixture.behavior)) {
            fixture.state.merge_head = request.target; fixture.state.files = [{ path: 'file.txt', state: 'UU', previous: null }];
            if (fixture.behavior === 'resolve') fixture.state.files.push({ path: 'binary.dat', state: 'UU', previous: null });
            throw 'Fixture merge conflict';
          } else if (fixture.behavior === 'stale') { fixture.state.head = 'c'.repeat(40); throw 'Fixture source changed'; }
          else if (request.mode === 'squash') { fixture.state.files = [{ path: 'topic.txt', state: 'A ', previous: null }]; }
          else { fixture.state.head = request.target; }
          return { output: 'Explicit UI fixture result', snapshot: fixture.behavior === 'refresh-failure' ? null : structuredClone(fixture.state), refresh_error: fixture.behavior === 'refresh-failure' ? 'Fixture refresh failed' : null };
        }};
      }, { kind });
      await page.goto('http://127.0.0.1:1421');
      await page.getByRole('button', { name: /Incoming topic/ }).click();
      return page;
    }
    const openGraphMerge = page => page.getByRole('button', { name: 'Merge into main…', exact: true }).click();
    const mergeDialog = page => page.getByRole('dialog', { name: 'Merge into main', exact: true });

    let page = await pageFor();
    await openGraphMerge(page);
    await mergeDialog(page).getByRole('button', { name: 'Cancel', exact: true }).click();
    await openGraphMerge(page); await page.keyboard.press('Escape');
    assert.equal(await page.evaluate(() => window.mergeFixture.requests.length), 0);
    await page.getByRole('button', { name: /Action history/ }).click();
    await openGraphMerge(page);
    await mergeDialog(page).getByLabel('Merge mode').selectOption('merge-commit');
    await mergeDialog(page).getByLabel('Merge commit message').fill('Merge "literal" & $(text) — فارسی');
    await mergeDialog(page).getByRole('button', { name: 'Merge', exact: true }).click();
    await page.getByRole('status').filter({ hasText: 'Git completed the merge into main' }).waitFor();
    const submitted = await page.evaluate(() => window.mergeFixture.requests[0]);
    assert.equal(submitted.target, 'b'.repeat(40)); assert.equal(submitted.expected_head, 'a'.repeat(40)); assert.equal(submitted.expected_branch, 'main');
    assert.equal(submitted.mode, 'merge-commit'); assert.equal(submitted.message, 'Merge "literal" & $(text) — فارسی'); assert.equal(submitted.source_branch, null);
    assert.deepEqual(await page.evaluate(() => window.mergeFixture.panel), ['Activity panel collapsed']);
    await page.close();

    page = await pageFor();
    await page.getByRole('button', { name: /Working changes/ }).click();
    await page.getByRole('textbox', { name: 'Commit summary' }).fill('Keep my commit draft');
    await page.getByRole('button', { name: /Commit history/ }).click();
    await page.getByRole('button', { name: 'Manage branch topic&literal', exact: true }).click();
    await page.getByRole('dialog', { name: 'Manage local branch' }).getByRole('button', { name: 'Merge into main…', exact: true }).click();
    assert.equal(await page.locator('dialog[open]').count(), 1);
    await mergeDialog(page).getByLabel('Merge mode').selectOption('squash');
    assert.equal(await mergeDialog(page).getByLabel('Merge commit message').count(), 0);
    await mergeDialog(page).getByRole('button', { name: 'Squash into staging', exact: true }).click();
    await page.getByRole('region', { name: 'Staged files', exact: true }).getByRole('button', { name: 'Inspect staged topic.txt' }).waitFor();
    assert.equal(await page.getByRole('textbox', { name: 'Commit summary' }).inputValue(), 'Keep my commit draft');
    assert.equal(await page.evaluate(() => window.mergeFixture.requests[0].source_branch), 'topic&literal');
    await page.close();

    page = await pageFor('conflict'); await openGraphMerge(page);
    await mergeDialog(page).getByRole('button', { name: 'Merge', exact: true }).click();
    await page.getByRole('button', { name: 'Abort merge…', exact: true }).waitFor();
    assert.equal(await page.locator('dialog[open]').count(), 0);
    assert.equal(await page.getByRole('button', { name: 'Commit staged', exact: true }).isDisabled(), true);
    await page.getByRole('button', { name: 'Abort merge…', exact: true }).click();
    await page.getByRole('dialog', { name: 'Abort pending merge' }).getByRole('button', { name: 'Cancel', exact: true }).click();
    assert.equal(await page.evaluate(() => window.mergeFixture.requests.length), 1);
    await page.getByRole('button', { name: 'Abort merge…', exact: true }).click();
    await page.getByRole('dialog', { name: 'Abort pending merge' }).getByRole('button', { name: 'Abort merge', exact: true }).click();
    await page.getByRole('button', { name: 'Abort merge…', exact: true }).waitFor({ state: 'hidden' });
    const abort = await page.evaluate(() => window.mergeFixture.requests[1]); assert.equal(abort.abort, true); assert.equal(abort.target, 'b'.repeat(40));
    await page.close();

    page = await pageFor('resolve');
    await page.getByRole('button', { name: /Action history/ }).click();
    await openGraphMerge(page);
    await mergeDialog(page).getByRole('button', { name: 'Merge', exact: true }).click();
    assert.equal(await page.getByRole('button', { name: 'Finish merge…' }).isDisabled(), true);
    await page.getByRole('button', { name: 'file.txt', exact: true }).click();
    let editor = page.getByRole('dialog', { name: 'Resolve file.txt' });
    await editor.getByRole('region', { name: 'Current version' }).getByText('current', { exact: true }).waitFor();
    await editor.getByRole('region', { name: 'Incoming version' }).getByText('incoming', { exact: true }).waitFor();
    await editor.getByRole('button', { name: 'Next file' }).click();
    editor = page.getByRole('dialog', { name: 'Resolve binary.dat' });
    await editor.getByText('Text editing is unavailable', { exact: false }).waitFor();
    await editor.getByRole('button', { name: 'Open external tool…' }).click();
    await editor.getByRole('button', { name: 'Cancel action' }).click();
    await editor.getByRole('button', { name: 'Take incoming file' }).click();
    await editor.getByRole('button', { name: 'Apply resolution' }).click();
    await page.getByRole('button', { name: 'file.txt', exact: true }).click();
    editor = page.getByRole('dialog', { name: 'Resolve file.txt' });
    await editor.getByRole('textbox', { name: 'Search output' }).fill('footer');
    await editor.getByRole('button', { name: 'Find next' }).click();
    await editor.getByText(/Match on line/).waitFor();
    const editorOutput = editor.getByRole('textbox', { name: 'Output', exact: true });
    assert.equal(await editorOutput.evaluate(node => node.value.slice(node.selectionStart, node.selectionEnd)), 'footer');
    await editor.getByRole('button', { name: 'Next block' }).click();
    assert.match(await editorOutput.evaluate(node => node.value.slice(node.selectionStart, node.selectionEnd)), /<<<<<<< HEAD/);
    await editor.getByRole('button', { name: 'Use current block' }).click();
    assert.equal(await editorOutput.inputValue(), 'header\ncurrent\nfooter\n');
    await editorOutput.fill('fail');
    await editor.getByRole('button', { name: 'Save and stage output' }).click();
    await editor.getByRole('alert').filter({ hasText: 'Fixture index lock' }).waitFor();
    assert.equal(await editorOutput.inputValue(), 'fail');
    await editorOutput.fill('header\ncombined\nfooter\n');
    await editor.getByRole('button', { name: 'Save and stage output' }).click();
    await page.getByRole('button', { name: 'Finish merge…' }).click();
    const finish = page.getByRole('dialog', { name: 'Finish merge into main' });
    await finish.getByText('Merge fixture', { exact: true }).waitFor();
    await finish.getByRole('button', { name: 'Cancel' }).click();
    await page.getByRole('button', { name: 'Finish merge…' }).click();
    await finish.getByRole('button', { name: 'Create merge commit' }).click();
    await page.getByRole('status').filter({ hasText: 'Git created the merge commit.' }).waitFor();
    const steps = await page.evaluate(() => window.mergeFixture.steps);
    assert.deepEqual(steps.map(step => [step.file, step.action, step.revision, step.value]), [
      ['binary.dat', 'incoming', 'binary-1', null], ['file.txt', 'edit', 'text-1', 'fail'],
      ['file.txt', 'edit', 'text-1', 'header\ncombined\nfooter\n'], [undefined, undefined, 'index-1', undefined],
    ]);
    assert.deepEqual(await page.evaluate(() => window.mergeFixture.panel), ['Activity panel collapsed']);
    await page.close();

    page = await pageFor('stale'); await openGraphMerge(page);
    await mergeDialog(page).getByRole('button', { name: 'Merge', exact: true }).click();
    await mergeDialog(page).getByRole('alert').filter({ hasText: 'Cancel and review refreshed state' }).waitFor();
    assert.equal(await mergeDialog(page).getByRole('button', { name: 'Merge', exact: true }).isDisabled(), true);
    await page.keyboard.press('Escape'); await page.close();

    page = await pageFor('refresh-failure'); await openGraphMerge(page);
    await mergeDialog(page).getByRole('button', { name: 'Merge', exact: true }).click();
    await page.getByRole('alert').filter({ hasText: 'succeeded, but refreshing failed' }).waitFor();
    assert.equal(await page.locator('dialog[open]').count(), 0);
    assert.equal(await page.getByRole('button', { name: 'New branch', exact: true }).isDisabled(), true);
    await page.close();

    for (const kind of ['dirty', 'detached']) {
      page = await pageFor(kind);
      assert.equal(await page.getByRole('button', { name: /Merge into .*…/ }).isDisabled(), true);
      await page.close();
    }
    assert.deepEqual(errors, []);
    console.log('Merge UI checks passed: graph and branch entry points, Cancel/Escape, reviewed arguments, collapsed panel, squash/draft preservation, conflict editing/continuation/abort, stale reviews, failed refresh and disabled states. IPC was stubbed; this is not native workflow evidence.');
  } finally {
    if (browser) await browser.close();
    server.kill();
  }
}
run().catch(error => { console.error(error); process.exitCode = 1; });
