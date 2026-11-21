#!/usr/bin/env node
// Runs every frozen replay listed in frozen-workflow-pins.json in an isolated
// view that restores its pinned workflow bytes (frozen-workflow-check.mjs), so
// research evidence keeps verifying while the live workflows follow current CI
// policy (runner, triggers, checkout). Before any replay, custody of every pin
// in the index is checked: the index revision still holds the pinned bytes and
// each listed evidence file still carries the pin.
//
//   node .github/scripts/run-frozen-workflow-replays.mjs [--custody-only] [--only <id>]...
//   node .github/scripts/run-frozen-workflow-replays.mjs --only <id> -- <command>...
//
// The second form is how a per-study workflow runs its own steps (preflight,
// evidence replay) in its replay's isolated view: one --only, and the command
// after -- replaces the index's default command. With --only, custody covers
// only the pins of the selected replays. A replay entry may name an
// `input_view_manifest` (frozen-input-view/v1), whose historical inputs are
// overlaid in the same view, as frozen-workflow-check.mjs --input-view-manifest does.
//
// A replay may carry its own `revision`, overriding the index's top-level one,
// when its manifest pins a different version of its workflow (or of an extra
// workflow) than the one the index holds. Custody then also checks that the
// replay's manifest carries the digest of each of its workflows at that revision.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { ensureCommit, withFrozenWorkflow } from './frozen-workflow-check.mjs';

const INDEX = '.github/scripts/frozen-workflow-pins.json';
const sha = bytes => createHash('sha256').update(bytes).digest('hex');

export function custodyProblems({ cwd, index, only = null }) {
  const problems = [];
  const show = file => execFileSync('git', ['show', `${index.revision}:${file}`],
    { cwd, maxBuffer: 16 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] });
  const selected = only && new Set(index.replays.filter(replay => only.includes(replay.id))
    .flatMap(replay => [replay.workflow, ...replay.extra_workflows.map(extra => extra.split('|')[0])]));
  for (const [workflow, entry] of Object.entries(index.workflows)) {
    if (selected && !selected.has(workflow)) continue;
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
  for (const replay of index.replays) {
    if (only && !only.includes(replay.id)) continue;
    if (replay.revision === undefined) continue;
    if (!/^[0-9a-f]{40}$/.test(replay.revision ?? '')) { problems.push(`replay ${replay.id}: invalid revision`); continue; }
    const pins = [[replay.workflow, replay.manifest],
      ...replay.extra_workflows.map(extra => extra.split('|').slice(0, 2))];
    for (const [workflow, manifest] of pins) {
      let historical;
      try {
        historical = execFileSync('git', ['show', `${replay.revision}:${workflow}`],
          { cwd, maxBuffer: 16 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] });
      } catch { problems.push(`replay ${replay.id}: ${workflow} absent at ${replay.revision}`); continue; }
      let text;
      try { text = execFileSync('git', ['show', `HEAD:${manifest}`], { cwd, maxBuffer: 64 * 1024 * 1024,
        stdio: ['ignore', 'pipe', 'pipe'] }).toString('utf8'); } catch { problems.push(`replay ${replay.id}: manifest gone: ${manifest}`); continue; }
      if (!text.includes(sha(historical))) problems.push(`replay ${replay.id}: ${manifest} does not pin ${workflow} at ${replay.revision}`);
    }
  }
  const covered = new Set();
  for (const replay of index.replays) {
    covered.add(replay.workflow);
    for (const extra of replay.extra_workflows) covered.add(extra.split('|')[0]);
  }
  for (const [workflow, entry] of Object.entries(index.workflows)) {
    if (selected && !selected.has(workflow)) continue;
    if ((entry.check === 'replay') !== covered.has(workflow)) problems.push(`${workflow}: check=${entry.check} disagrees with the replay list`);
  }
  for (const replay of index.replays) {
    if (only && !only.includes(replay.id)) continue;
    if (replay.input_view_manifest !== undefined && (typeof replay.input_view_manifest !== 'string'
      || !/^\.github\/scripts\/[A-Za-z0-9._-]+\.json$/.test(replay.input_view_manifest))) {
      problems.push(`${replay.id}: input_view_manifest must be a JSON file under .github/scripts/`);
    }
  }
  for (const workflow of covered) if ((!selected || selected.has(workflow)) && !index.workflows[workflow]) problems.push(`${workflow}: replayed but not indexed`);
  return problems;
}

export function parseArgs(argv) {
  const split = argv.indexOf('--');
  const flags = split < 0 ? argv : argv.slice(0, split);
  const command = split < 0 ? null : argv.slice(split + 1);
  const only = [];
  let custodyOnly = false;
  for (let i = 0; i < flags.length; i++) {
    if (flags[i] === '--custody-only') custodyOnly = true;
    else if (flags[i] === '--only' && flags[i + 1] && !flags[i + 1].startsWith('--')) only.push(flags[++i]);
    else throw new Error(`unexpected argument: ${flags[i]}`);
  }
  if (command && (!command.length || only.length !== 1 || custodyOnly)) {
    throw new Error('a command after -- needs exactly one --only <id> and no --custody-only');
  }
  return { custodyOnly, only, command };
}

function main(argv) {
  const cwd = process.cwd();
  const { custodyOnly, only, command } = parseArgs(argv);
  const index = JSON.parse(readFileSync(path.join(cwd, INDEX), 'utf8'));
  if (index.schema !== 'frozen-workflow-pins/v1' || !/^[0-9a-f]{40}$/.test(index.revision ?? '')) {
    throw new Error('invalid frozen-workflow-pins index');
  }
  const unknown = only.filter(id => !index.replays.some(replay => replay.id === id));
  if (unknown.length) throw new Error(`unknown replay id: ${unknown.join(', ')}`);
  // A per-study workflow's default checkout is shallow; fetch the pinned commit.
  ensureCommit(cwd, index.revision);
  const selectedReplays = index.replays.filter(replay => !only.length || only.includes(replay.id));
  for (const replay of selectedReplays) {
    if (/^[0-9a-f]{40}$/.test(replay.revision ?? '')) ensureCommit(cwd, replay.revision);
  }
  const problems = custodyProblems({ cwd, index, only: only.length ? only : null });
  for (const problem of problems) console.error(`custody: ${problem}`);
  console.log(`Custody of ${only.length ? `the pins of ${only.join(', ')}` : `${Object.keys(index.workflows).length} pinned workflows`}: ${problems.length} problems`);
  if (problems.length || custodyOnly) return problems.length ? 1 : 0;
  const results = [];
  for (const replay of selectedReplays) {
    console.log(`::group::${replay.id}`);
    let status;
    try {
      status = withFrozenWorkflow({ cwd, workflow: replay.workflow, revision: replay.revision ?? index.revision,
        manifest: replay.manifest, manifestFormat: replay.manifest_format, roots: replay.roots,
        inputViewManifest: replay.input_view_manifest,
        extraWorkflows: replay.extra_workflows, command: command ?? replay.command });
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
