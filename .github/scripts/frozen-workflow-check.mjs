#!/usr/bin/env node
// Historical replay manifests bind workflow metadata as well as study inputs.
// Scheduling may change without rewriting that record: execute the unchanged
// validator in a copied current source view with only its pinned workflow bytes.
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const sha = bytes => createHash('sha256').update(bytes).digest('hex');

function relative(file) {
  if (typeof file !== 'string' || !file || path.posix.isAbsolute(file)
      || file.includes('\\') || file.split('/').includes('..') || file.split('/').includes('.git')) {
    throw new Error(`unsafe source-view path: ${file}`);
  }
  return path.posix.normalize(file);
}

function copyCurrent(cwd, view, file) {
  const source = path.join(cwd, file);
  const destination = path.join(view, file);
  const info = lstatSync(source); // Missing current inputs must fail, never roll back.
  if (info.isSymbolicLink()) throw new Error(`symbolic source-view input: ${file}`);
  if (info.isDirectory()) {
    mkdirSync(destination, { recursive: true });
    for (const child of readdirSync(source)) copyCurrent(cwd, view, path.posix.join(file, child));
  } else if (info.isFile()) {
    mkdirSync(path.dirname(destination), { recursive: true });
    copyFileSync(source, destination);
  } else {
    throw new Error(`unsupported source-view input: ${file}`);
  }
}

function regularBytes(root, file, label) {
  const target = path.join(root, file);
  const info = lstatSync(target);
  if (!info.isFile() || info.isSymbolicLink()) throw new Error(`${label} is not a regular file: ${file}`);
  return readFileSync(target);
}

export function ensureCommit(cwd, revision) {
  if (!/^[0-9a-f]{40,64}$/.test(revision ?? '')) throw new Error('expected an immutable Git revision');
  try {
    execFileSync('git', ['cat-file', '-e', `${revision}^{commit}`], { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
  } catch {
    execFileSync('git', ['fetch', '--filter=blob:none', '--no-tags', '--depth=1', 'origin', revision],
      { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
  }
}

function applyPinnedInputView({ cwd, view, manifest }) {
  manifest = relative(manifest);
  const configBytes = regularBytes(cwd, manifest, 'pinned-input manifest');
  const config = JSON.parse(configBytes.toString('utf8'));
  if (config.schema !== 'frozen-input-view/v1' || !Array.isArray(config.inputs) || !config.inputs.length) {
    throw new Error('invalid pinned-input manifest schema');
  }
  const overlays = [];
  for (const item of config.inputs) {
    const target = relative(item.target);
    const revision = item.revision;
    const sourcePath = relative(item.source_path);
    const originalManifest = relative(item.original_manifest);
    // The original freeze names the input either by a `source_sha256` key or,
    // for a `files`-style map whose names hold slashes, by an RFC 6901 pointer.
    const sourceKey = item.original_source_key;
    const sourcePointer = item.original_source_pointer;
    const expected = item.sha256;
    if ((sourceKey === undefined) === (sourcePointer === undefined)) {
      throw new Error(`pinned input needs exactly one of original_source_key or original_source_pointer: ${target}`);
    }
    if ((sourceKey !== undefined && typeof sourceKey !== 'string') || !/^[0-9a-f]{64}$/.test(expected ?? '')
        || !/^[0-9a-f]{64}$/.test(item.original_manifest_sha256 ?? '')) {
      throw new Error(`invalid pinned-input digests: ${target}`);
    }
    const originalBytes = regularBytes(cwd, originalManifest, 'original freeze manifest');
    if (sha(originalBytes) !== item.original_manifest_sha256) {
      throw new Error(`original freeze manifest changed: ${originalManifest}`);
    }
    const viewOriginalBytes = regularBytes(view, originalManifest, 'view original freeze manifest');
    if (!viewOriginalBytes.equals(originalBytes)) throw new Error(`view changed original freeze manifest: ${originalManifest}`);
    const frozen = JSON.parse(originalBytes.toString('utf8'));
    const frozenDigest = sourceKey !== undefined ? frozen.source_sha256?.[sourceKey]
      : selectField(frozen, `pointer:${sourcePointer}`);
    if (frozenDigest !== expected) {
      throw new Error(`pinned input does not match original frozen source: ${sourceKey ?? sourcePointer}`);
    }
    const live = regularBytes(cwd, target, 'live pinned-input target');
    const currentSources = {};
    // Either the live target is checked against a current twin (lockstep), or,
    // when it has none because current code moved on, a committed drift note
    // must name the target: the overlay never hides drift.
    let driftNote = null;
    if (item.current_equal === undefined && item.current_drift_note !== undefined) {
      driftNote = relative(item.current_drift_note);
      if (!regularBytes(cwd, driftNote, 'current drift note').toString('utf8').includes(target)) {
        throw new Error(`current drift note does not name the pinned input: ${target}`);
      }
    } else if (item.current_drift_note !== undefined || !Array.isArray(item.current_equal)
      || !item.current_equal.some(pair => Array.isArray(pair) && pair.length === 2 && pair.includes(target))) {
      throw new Error(`pinned input lacks a current-source equality check: ${target}`);
    }
    for (const pair of item.current_equal ?? []) {
      if (!Array.isArray(pair) || pair.length !== 2) throw new Error(`invalid current-source pair: ${target}`);
      const [left, right] = pair.map(relative);
      if (left === right || pair.filter(name => relative(name) === target).length !== 1) {
        throw new Error(`invalid current-source equality pair: ${target}`);
      }
      const leftBytes = regularBytes(cwd, left, 'current source');
      const rightBytes = regularBytes(cwd, right, 'current source');
      currentSources[left] = sha(leftBytes);
      currentSources[right] = sha(rightBytes);
      if (!leftBytes.equals(rightBytes)) throw new Error(`current source inputs differ: ${left} != ${right}`);
    }
    ensureCommit(cwd, revision);
    const historical = execFileSync('git', ['show', `${revision}:${sourcePath}`],
      { cwd, maxBuffer: 16 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] });
    if (sha(historical) !== expected) throw new Error(`pinned Git source hash mismatch: ${sourcePath}`);
    const viewTarget = path.join(view, target);
    if (!existsSync(viewTarget) || !regularBytes(view, target, 'view pinned-input target').equals(live)) {
      throw new Error(`current source view missing or changed before overlay: ${target}`);
    }
    writeFileSync(viewTarget, historical);
    if (!regularBytes(view, target, 'overlaid pinned-input target').equals(historical)) {
      throw new Error(`historical input overlay failed: ${target}`);
    }
    overlays.push({ target, revision, source_path: sourcePath, sha256: expected,
      original_manifest: originalManifest, original_manifest_sha256: item.original_manifest_sha256,
      ...(sourceKey !== undefined ? { original_source_key: sourceKey } : { original_source_pointer: sourcePointer }),
      live_target_sha256: sha(live), current_sources: currentSources,
      ...(driftNote ? { current_drift_note: driftNote } : {}),
      view_sha256: sha(historical) });
  }
  return overlays;
}

// Field selectors for a study's own FROZEN.json: `key:a.b` walks dotted
// names; `pointer:/a/b.yml` is an RFC 6901 JSON Pointer, for names that hold
// dots or hyphens. Either yields one workflow digest, and those validators hash
// their other inputs themselves, so the digest map holds the workflow alone.
function selectField(frozen, manifestFormat) {
  let keys;
  if (manifestFormat.startsWith('key:')) {
    const keyPath = manifestFormat.slice(4);
    if (!/^[A-Za-z0-9_]+(\.[A-Za-z0-9_]+)*$/.test(keyPath)) throw new Error('invalid frozen manifest key path');
    keys = keyPath.split('.');
  } else {
    const pointer = manifestFormat.slice('pointer:'.length);
    if (!/^(\/([^~/]|~[01])+)+$/.test(pointer)) throw new Error('invalid frozen manifest key path');
    keys = pointer.slice(1).split('/').map(key => key.replaceAll('~1', '/').replaceAll('~0', '~'));
  }
  return keys.reduce((node, key) => (node && typeof node === 'object' && !Array.isArray(node)
    && Object.prototype.hasOwnProperty.call(node, key) ? node[key] : undefined), frozen);
}

function sha256sumDigests(bytes) {
  const digests = {};
  for (const line of bytes.toString('utf8').split('\n')) {
    if (!line.trim()) continue;
    const match = /^([0-9a-f]{64}) [ *](.+)$/.exec(line);
    if (!match || Object.prototype.hasOwnProperty.call(digests, match[2])) throw new Error('invalid sha256sum manifest line');
    digests[match[2]] = match[1];
  }
  return digests;
}

// The pin custody shared by the main workflow and every --extra-workflow: the
// manifest is byte-identical to its copy at the revision, and that revision's
// workflow bytes hash to the pinned digest.
function pinnedWorkflow({ cwd, revision, workflow, manifest, manifestFormat }) {
  const fromGit = file => execFileSync('git', ['show', `${revision}:${file}`],
    { cwd, maxBuffer: 16 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'] });
  const manifestBytes = readFileSync(path.join(cwd, manifest));
  if (!manifestBytes.equals(fromGit(manifest))) throw new Error('original frozen manifest changed');
  // `sha256sum` is a coreutils manifest (`<hex>  <root-relative path>` per
  // line, as `sha256sum --check` reads it); every other format is JSON.
  const frozen = manifestFormat === 'sha256sum' ? sha256sumDigests(manifestBytes)
    : JSON.parse(manifestBytes.toString('utf8'));
  const field = manifestFormat.startsWith('key:') || manifestFormat.startsWith('pointer:');
  if (!field && !['files-relative', 'sha256-root', 'sha256sum'].includes(manifestFormat)) {
    throw new Error('unsupported frozen manifest format');
  }
  // Both map layouts retain the original manifest and every original input digest.
  // Root-relative sha256 maps occur in the native-m3 archive; opting in changes
  // path interpretation only, never the bytes admitted by the historical gate.
  const digests = field ? { [workflow]: selectField(frozen, manifestFormat) }
    : manifestFormat === 'sha256-root' ? frozen.sha256 : manifestFormat === 'sha256sum' ? frozen : frozen.files;
  if (!digests || typeof digests !== 'object' || Array.isArray(digests)) {
    throw new Error('invalid frozen input digest map');
  }
  const workflowBytes = fromGit(workflow);
  if (!/^[0-9a-f]{64}$/.test(digests[workflow] ?? '') || sha(workflowBytes) !== digests[workflow]) {
    throw new Error(`historical workflow does not match its frozen digest: ${workflow}`);
  }
  return { digests, workflowBytes };
}

function parseExtraWorkflow(spec) {
  const parts = typeof spec === 'string' ? spec.split('|') : [];
  if (parts.length !== 3 || parts.some(part => !part)) {
    throw new Error(`expected --extra-workflow 'workflow|manifest|format': ${spec}`);
  }
  return { workflow: relative(parts[0]), manifest: relative(parts[1]), manifestFormat: parts[2] };
}

export function withFrozenWorkflow({ cwd = process.cwd(), workflow, revision, manifest, inputViewManifest,
  manifestFormat = 'files-relative', roots = [], extraWorkflows = [], command }) {
  workflow = relative(workflow);
  manifest = relative(manifest);
  roots = roots.map(relative);
  const extras = extraWorkflows.map(parseExtraWorkflow);
  if (new Set([workflow, ...extras.map(extra => extra.workflow)]).size !== extras.length + 1) {
    throw new Error('each workflow may be overlaid once');
  }
  ensureCommit(cwd, revision);
  if (!Array.isArray(command) || !command.length || command.some(arg => typeof arg !== 'string')) {
    throw new Error('expected a command and argument array');
  }
  const { digests, workflowBytes } = pinnedWorkflow({ cwd, revision, workflow, manifest, manifestFormat });
  const extraPins = extras.map(extra => ({ ...extra, ...pinnedWorkflow({ cwd, revision, ...extra }) }));
  const liveWorkflowSha = sha(readFileSync(path.join(cwd, workflow)));
  const ownInputs = Object.entries(digests).map(([name, digest]) => {
    const file = relative(['sha256-root', 'sha256sum'].includes(manifestFormat) || name.startsWith('.github/')
      ? name : path.posix.join(path.posix.dirname(manifest), name));
    if (!/^[0-9a-f]{64}$/.test(digest)) throw new Error(`invalid frozen digest: ${name}`);
    return { file, digest };
  });
  const view = mkdtempSync(path.join(os.tmpdir(), 'crypto-frozen-workflow-'));
  try {
    // Real copies prevent validators/tests from altering the live checkout.
    for (const root of roots) copyCurrent(cwd, view, root);
    copyCurrent(cwd, view, manifest);
    for (const extra of extraPins) copyCurrent(cwd, view, extra.manifest);
    for (const { file } of ownInputs) if (file !== workflow) copyCurrent(cwd, view, file);
    for (const [file, bytes] of [[workflow, workflowBytes], ...extraPins.map(extra => [extra.workflow, extra.workflowBytes])]) {
      mkdirSync(path.dirname(path.join(view, file)), { recursive: true });
      writeFileSync(path.join(view, file), bytes);
    }
    for (const { file, digest } of ownInputs) {
      if (sha(readFileSync(path.join(view, file))) !== digest) throw new Error(`frozen input drift: ${file}`);
    }
    const pinnedInputViews = inputViewManifest
      ? applyPinnedInputView({ cwd, view, manifest: inputViewManifest }) : [];
    // Original preflights retain their Git ancestry assertions, against the
    // current checkout's unchanged object database; no historical code checkout.
    const gitPath = path.join(cwd, '.git');
    if (!existsSync(gitPath)) throw new Error('current Git metadata is missing');
    symlinkSync(gitPath, path.join(view, '.git'));
    console.log('FROZEN_WORKFLOW_METADATA_VIEW ' + JSON.stringify({ workflow, revision,
      live_workflow_sha256: liveWorkflowSha, view_workflow_sha256: sha(workflowBytes),
      extra_workflows: extraPins.map(extra => ({ workflow: extra.workflow, manifest: extra.manifest,
        view_workflow_sha256: sha(extra.workflowBytes) })),
      pinned_input_views: pinnedInputViews,
      scope: 'workflow metadata and explicitly pinned historical inputs are substituted in an isolated view; current checkout and evidence remain unchanged' }));
    // The enclosing workflow's existing job budget governs the validator.
    const result = spawnSync(command[0], command.slice(1), { cwd: view, stdio: 'inherit',
      env: { ...process.env, PYTHONDONTWRITEBYTECODE: '1' } });
    if (result.error) throw result.error;
    if (result.status === null) throw new Error(`validator terminated: ${result.signal}`);
    return result.status;
  } finally {
    rmSync(view, { recursive: true, force: true });
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  const options = { roots: [], extraWorkflows: [] };
  try {
    while (args.length && args[0] !== '--') {
      const flag = args.shift();
      if (!['--workflow', '--revision', '--manifest', '--manifest-format', '--input-view-manifest', '--root',
        '--extra-workflow'].includes(flag) || !args.length) {
        throw new Error(`unknown or incomplete argument: ${flag}`);
      }
      const value = args.shift();
      if (flag === '--root') options.roots.push(value);
      else if (flag === '--extra-workflow') options.extraWorkflows.push(value);
      else if (flag === '--input-view-manifest') options.inputViewManifest = value;
      else if (flag === '--manifest-format') options.manifestFormat = value;
      else options[flag.slice(2)] = value;
    }
    if (args.shift() !== '--') throw new Error('expected -- before the validator command');
    options.command = args;
    process.exitCode = withFrozenWorkflow(options);
  } catch (error) {
    console.error(`frozen workflow custody failed: ${error.message}`);
    process.exitCode = 1;
  }
}
