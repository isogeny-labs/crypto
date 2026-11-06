#!/usr/bin/env node
// Runs every frozen replay listed in frozen-workflow-pins.json in an isolated
// view that restores its pinned workflow bytes (frozen-workflow-check.mjs), so
// research evidence keeps verifying while the live workflows follow current CI
// policy (runner, triggers, checkout). Before any replay, custody of every pin
// in the index is checked: the index revision still holds the pinned bytes and
// each listed evidence file still carries the pin.
//
//   node .github/scripts/run-frozen-workflow-replays.mjs [--custody-only] [--only <id>]...
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { withFrozenWorkflow } from './frozen-workflow-check.mjs';

const INDEX = '.github/scripts/frozen-workflow-pins.json';
const sha = bytes => createHash('sha256').update(bytes).digest('hex');

export function custodyProblems({ cwd, index }) {
  const problems = [];
  const show = file => execFileSync('git', ['show', `${index.revision}:${file}`],
    { cwd, maxBuffer: 16 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] });
  for (const [workflow, entry] of Object.entries(index.workflows)) {
    if (!/^[0-9a-f]{64}$/.test(entry.pinned_sha256 ?? '')) { problems.push(`${workflow}: invalid pinned_sha256`); continue; }
    if (!['replay', 'wrapper', 'record'].includes(entry.check)) problems.push(`${workflow}: unknown check ${entry.check}`);
    let historical;
    try { historical = show(workflow); } catch { problems.push(`${workflow}: absent at ${index.revision}`); continue; }
    if (sha(historical) !== entry.pinned_sha256) problems.push(`${workflow}: bytes at ${index.revision} do not hash to the pin`);
    if (!Array.isArray(entry.pinned_by) || !entry.pinned_by.length) problems.push(`${workflow}: no evidence file listed`);
    for (const file of entry.pinned_by ?? []) {
      let text;
      try { text = execFileSync('git', ['show', `HEAD:${file}`], { cwd, maxBuffer: 64 * 1024 * 1024,
        stdio: ['ignore', 'pipe', 'pipe'] }).toString('utf8'); } catch { problems.push(`${workflow}: pinning file gone: ${file}`); continue; }
      if (!text.includes(entry.pinned_sha256)) problems.push(`${workflow}: ${file} no longer carries its pin`);
    }
  }
  const covered = new Set();
  for (const replay of index.replays) {
    covered.add(replay.workflow);
    for (const extra of replay.extra_workflows) covered.add(extra.split('|')[0]);
  }
  for (const [workflow, entry] of Object.entries(index.workflows)) {
    if ((entry.check === 'replay') !== covered.has(workflow)) problems.push(`${workflow}: check=${entry.check} disagrees with the replay list`);
  }
  for (const workflow of covered) if (!index.workflows[workflow]) problems.push(`${workflow}: replayed but not indexed`);
  return problems;
}

function main(argv) {
  const cwd = process.cwd();
  const custodyOnly = argv.includes('--custody-only');
  const only = argv.flatMap((arg, i) => arg === '--only' ? [argv[i + 1]] : []);
  const index = JSON.parse(readFileSync(path.join(cwd, INDEX), 'utf8'));
  if (index.schema !== 'frozen-workflow-pins/v1' || !/^[0-9a-f]{40}$/.test(index.revision ?? '')) {
    throw new Error('invalid frozen-workflow-pins index');
  }
  const problems = custodyProblems({ cwd, index });
  for (const problem of problems) console.error(`custody: ${problem}`);
  console.log(`Custody of ${Object.keys(index.workflows).length} pinned workflows: ${problems.length} problems`);
  if (problems.length || custodyOnly) return problems.length ? 1 : 0;
  const unknown = only.filter(id => !index.replays.some(replay => replay.id === id));
  if (unknown.length) throw new Error(`unknown replay id: ${unknown.join(', ')}`);
  const results = [];
  for (const replay of index.replays.filter(replay => !only.length || only.includes(replay.id))) {
    console.log(`::group::${replay.id}`);
    let status;
    try {
      status = withFrozenWorkflow({ cwd, workflow: replay.workflow, revision: index.revision,
        manifest: replay.manifest, manifestFormat: replay.manifest_format, roots: replay.roots,
        extraWorkflows: replay.extra_workflows, command: replay.command });
    } catch (error) {
      console.error(`frozen workflow custody failed: ${error.message}`);
      status = 'custody';
    }
    console.log('::endgroup::');
    results.push({ id: replay.id, status });
  }
  for (const { id, status } of results) console.log(`${status === 0 ? 'PASS' : 'FAIL'} ${id}${status === 0 ? '' : ` (${status})`}`);
  const failed = results.filter(result => result.status !== 0).length;
  console.log(`Frozen replays in isolated views: ${results.length - failed} passed, ${failed} failed`);
  return failed ? 1 : 0;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    process.exitCode = main(process.argv.slice(2));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
