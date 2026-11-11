import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { withFrozenWorkflow } from './frozen-workflow-check.mjs';

const workflow = '.github/workflows/study.yml';
const study = 'research/study';
const manifest = study + '/FROZEN.json';
const historicalWorkflow = 'name: frozen historical workflow\n';
const liveWorkflow = 'name: current scoped workflow\n';
const liveSource = 'CURRENT_INTERFACE_V2\n';
const historicalLock = 'released nested Cargo.lock\n';
const currentLock = 'current root Cargo.lock\n';
const lockPath = 'research/archive/Cargo.lock';
const originalFreeze = 'research/rotation/FROZEN.json';
const inputViewManifest = '.github/scripts/n53-historical-cargo-lock-view-v1.json';
const sha = value => createHash('sha256').update(value).digest('hex');
const extraWorkflow = '.github/workflows/other-check.yml';
const extraManifest = 'research/other/FROZEN.json';
const historicalExtra = 'name: frozen other workflow\n';
const extraSpec = `${extraWorkflow}|${extraManifest}|pointer:/workflow_sha256/other-check.yml`;

async function fixture(run, { withPinnedInput = false, manifestFormat = 'files-relative', extra = false } = {}) {
  const cwd = mkdtempSync(path.join(os.tmpdir(), 'crypto-frozen-fixture-'));
  const write = (file, text) => {
    const target = path.join(cwd, file);
    mkdirSync(path.dirname(target), { recursive: true });
    writeFileSync(target, text);
  };
  const git = (...args) => execFileSync('git', args, { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
  try {
    git('init', '-q');
    write(workflow, historicalWorkflow);
    write('src/current.rs', 'HISTORICAL_INTERFACE_V1\n');
    write('Cargo.lock', historicalLock);
    write(lockPath, historicalLock);
    const frozenSourceBytes = JSON.stringify({ source_sha256: { cargo_lock: sha(historicalLock) } }) + '\n';
    write(originalFreeze, frozenSourceBytes);
    write(study + '/input.json', '{"evidence":"unchanged"}\n');
    if (extra) {
      write(extraWorkflow, historicalExtra);
      write(extraManifest, JSON.stringify({ workflow_sha256: { 'other-check.yml': sha(historicalExtra) } }) + '\n');
    }
    write(study + '/check.mjs', [
      "import assert from 'node:assert/strict';",
      "import { readFileSync, writeFileSync } from 'node:fs';",
      'assert.equal(readFileSync(' + JSON.stringify(workflow) + ", 'utf8'), " + JSON.stringify(historicalWorkflow) + ');',
      "assert.equal(readFileSync('src/current.rs', 'utf8'), " + JSON.stringify(liveSource) + ');',
      'assert.equal(readFileSync(' + JSON.stringify(lockPath) + ", 'utf8'), "
        + JSON.stringify(withPinnedInput ? historicalLock : currentLock) + ');',
      "assert.equal(readFileSync('Cargo.lock', 'utf8'), " + JSON.stringify(currentLock) + ');',
      ...(extra ? ['assert.equal(readFileSync(' + JSON.stringify(extraWorkflow) + ", 'utf8'), "
        + JSON.stringify(historicalExtra) + ');'] : []),
      'writeFileSync(process.argv[2], process.cwd());',
    ].join('\n') + '\n');
    const files = Object.fromEntries(['input.json', 'check.mjs'].map(file => [file, sha(readFileSync(path.join(cwd, study, file)))]));
    files[workflow] = sha(historicalWorkflow);
    const keyFreeze = keyPath => keyPath.split('.').reduceRight(
      (value, key) => ({ [key]: value }), sha(historicalWorkflow));
    const freeze = manifestFormat.startsWith('key:') ? { ...keyFreeze(manifestFormat.slice(4)), other: 'kept' }
      : manifestFormat === 'sha256-root'
        ? { sha256: Object.fromEntries(Object.entries(files).map(([file, digest]) =>
          [file.startsWith('.github/') ? file : study + '/' + file, digest])) }
        : { files };
    write(manifest, manifestFormat === 'sha256sum'
      ? Object.entries(files).map(([file, digest]) =>
        `${digest}  ${file.startsWith('.github/') ? file : study + '/' + file}\n`).join('')
      : JSON.stringify(freeze) + '\n');
    git('add', '.');
    git('-c', 'user.name=CI fixture', '-c', 'user.email=ci-fixture@example.invalid', 'commit', '-qm', 'original freeze');
    const revision = git('rev-parse', 'HEAD');
    write(workflow, liveWorkflow);
    if (extra) write(extraWorkflow, 'name: current other workflow\n');
    write('src/current.rs', liveSource);
    write('Cargo.lock', currentLock);
    write(lockPath, currentLock);
    write(inputViewManifest, JSON.stringify({
      schema: 'frozen-input-view/v1',
      inputs: [{
        target: lockPath,
        revision,
        source_path: lockPath,
        sha256: sha(historicalLock),
        original_manifest: originalFreeze,
        original_manifest_sha256: sha(frozenSourceBytes),
        original_source_key: 'cargo_lock',
        current_equal: [[lockPath, 'Cargo.lock']],
      }],
    }) + '\n');
    git('add', '.');
    git('-c', 'user.name=CI fixture', '-c', 'user.email=ci-fixture@example.invalid', 'commit', '-qm', 'current interface and CI metadata');
    const current = git('rev-parse', 'HEAD');
    const marker = path.join(cwd, 'view-location.txt');
    const options = { cwd, workflow, revision, manifest, manifestFormat, roots: [study, 'src',
      'research/archive', 'research/rotation', 'Cargo.lock'],
      command: [process.execPath, study + '/check.mjs', marker] };
    if (withPinnedInput) options.inputViewManifest = inputViewManifest;
    if (extra) options.extraWorkflows = [extraSpec];
    await run({ cwd, write, options, current, marker, withPinnedInput });
  } finally {
    rmSync(cwd, { recursive: true, force: true });
  }
}

test('historical metadata is isolated while current source and live checkout are retained', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    assert.equal(await withFrozenWorkflow(options), 0);
    const view = readFileSync(marker, 'utf8');
    assert.notEqual(view, cwd);
    assert.equal(existsSync(view), false, 'temporary source view must be cleaned up');
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
    assert.equal(readFileSync(path.join(cwd, 'src/current.rs'), 'utf8'), liveSource);
  });
});

test('root-relative sha256 manifest passes through the CLI without changing live inputs', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    const helper = fileURLToPath(new URL('./frozen-workflow-check.mjs', import.meta.url));
    execFileSync(process.execPath, [helper, '--workflow', workflow, '--revision', options.revision,
      '--manifest', manifest, '--manifest-format', 'sha256-root', '--root', study,
      '--root', 'src', '--root', 'research/archive', '--root', 'research/rotation',
      '--root', 'Cargo.lock', '--', ...options.command],
    { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
    assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
    assert.equal(readFileSync(path.join(cwd, 'src/current.rs'), 'utf8'), liveSource);
  }, { manifestFormat: 'sha256-root' });
});

test('root-relative frozen input drift is rejected before the validator executes', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    write(study + '/input.json', '{"evidence":"replacement"}\n');
    assert.throws(() => withFrozenWorkflow(options), /frozen input drift/);
    assert.equal(existsSync(marker), false);
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
  }, { manifestFormat: 'sha256-root' });
});

test('root-relative manifest cannot be repinned together with its input', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    const replacement = '{"evidence":"replacement"}\n';
    write(study + '/input.json', replacement);
    const freeze = JSON.parse(readFileSync(path.join(cwd, manifest), 'utf8'));
    freeze.sha256[study + '/input.json'] = sha(replacement);
    write(manifest, JSON.stringify(freeze) + '\n');
    assert.throws(() => withFrozenWorkflow(options), /original frozen manifest changed/);
    assert.equal(existsSync(marker), false);
  }, { manifestFormat: 'sha256-root' });
});

test('manifest layouts require an explicit supported format', async () => {
  await fixture(async ({ options, marker }) => {
    assert.throws(() => withFrozenWorkflow({ ...options, manifestFormat: 'guessed' }), /unsupported frozen manifest format/);
    assert.throws(() => withFrozenWorkflow({ ...options, manifestFormat: 'sha256-root' }), /invalid frozen input digest map/);
    assert.equal(existsSync(marker), false);
  });
});

test('a validator failure remains a failed status and cleans its source view', async () => {
  await fixture(async ({ options, marker }) => {
    const code = "require('node:fs').writeFileSync(process.argv[1], process.cwd()); process.exit(17)";
    assert.equal(await withFrozenWorkflow({ ...options, command: [process.execPath, '-e', code, marker] }), 17);
    assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
  });
});

test('workflow CLI arguments reach the isolated validator', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    const helper = fileURLToPath(new URL('./frozen-workflow-check.mjs', import.meta.url));
    execFileSync(process.execPath, [helper, '--workflow', workflow, '--revision', options.revision,
      '--manifest', manifest, '--root', study, '--root', 'src', '--root', 'research/archive',
      '--root', 'research/rotation', '--root', 'Cargo.lock', '--', ...options.command],
    { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
    assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
  });
});

test('a shallow checkout fetches its exact missing pin and keeps current source', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    const parent = mkdtempSync(path.join(os.tmpdir(), 'crypto-frozen-shallow-'));
    const clone = path.join(parent, 'repo');
    try {
      execFileSync('git', ['clone', '-q', '--depth=1', '--no-tags', pathToFileURL(cwd).href, clone],
        { stdio: ['ignore', 'pipe', 'pipe'] });
      assert.throws(() => execFileSync('git', ['cat-file', '-e', options.revision + '^{commit}'],
        { cwd: clone, stdio: ['ignore', 'pipe', 'pipe'] }));
      assert.equal(await withFrozenWorkflow({ ...options, cwd: clone }), 0);
      assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
      assert.equal(readFileSync(path.join(clone, workflow), 'utf8'), liveWorkflow);
      assert.equal(readFileSync(path.join(clone, 'src/current.rs'), 'utf8'), liveSource);
    } finally {
      rmSync(parent, { recursive: true, force: true });
    }
  });
});

test('coordinated manifest and frozen-source replacement fails pinned manifest custody', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    const replacement = '{"evidence":"replacement"}\n';
    write(study + '/input.json', replacement);
    const freeze = JSON.parse(readFileSync(path.join(cwd, manifest), 'utf8'));
    freeze.files['input.json'] = sha(replacement);
    write(manifest, JSON.stringify(freeze) + '\n');
    await assert.rejects(async () => withFrozenWorkflow(options));
    assert.equal(existsSync(marker), false);
  });
});

test('changed frozen evidence is rejected without changing the live workflow', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    write(study + '/input.json', '{"evidence":"tampered"}\n');
    await assert.rejects(async () => withFrozenWorkflow(options));
    assert.equal(existsSync(marker), false);
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
  });
});

test('deleted frozen evidence fails closed', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    rmSync(path.join(cwd, study, 'input.json'));
    await assert.rejects(async () => withFrozenWorkflow(options));
    assert.equal(existsSync(marker), false);
  });
});

test('deleted manifest fails closed', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    rmSync(path.join(cwd, manifest));
    await assert.rejects(async () => withFrozenWorkflow(options));
    assert.equal(existsSync(marker), false);
  });
});

test('a revision with mismatched workflow bytes cannot satisfy the freeze', async () => {
  await fixture(async ({ options, current, marker }) => {
    await assert.rejects(async () => withFrozenWorkflow({ ...options, revision: current }));
    assert.equal(existsSync(marker), false);
  });
});

test('paths outside the source view are rejected', async () => {
  await fixture(async ({ options, marker }) => {
    await assert.rejects(async () => withFrozenWorkflow({ ...options, roots: ['../'] }));
    assert.equal(existsSync(marker), false);
  });
});

test('released input overlay is isolated and records current lock provenance', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    assert.equal(await withFrozenWorkflow(options), 0);
    const view = readFileSync(marker, 'utf8');
    assert.notEqual(view, cwd);
    assert.equal(existsSync(view), false, 'temporary historical input view must be cleaned up');
    assert.equal(readFileSync(path.join(cwd, lockPath), 'utf8'), currentLock);
    assert.equal(readFileSync(path.join(cwd, 'Cargo.lock'), 'utf8'), currentLock);
  }, { withPinnedInput: true });
});

test('historical Git input must match the original freeze digest', async () => {
  await fixture(async ({ cwd, write, options, marker, current }) => {
    const config = JSON.parse(readFileSync(path.join(cwd, options.inputViewManifest), 'utf8'));
    config.inputs[0].revision = current;
    write(options.inputViewManifest, JSON.stringify(config) + '\n');
    await assert.rejects(async () => withFrozenWorkflow(options), /pinned Git source hash mismatch/);
    assert.equal(existsSync(marker), false);
    assert.equal(readFileSync(path.join(cwd, lockPath), 'utf8'), currentLock);
  }, { withPinnedInput: true });
});

test('current nested/root lock drift fails before the historical overlay', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    write('Cargo.lock', 'different current root lock\n');
    await assert.rejects(async () => withFrozenWorkflow(options), /current source inputs differ/);
    assert.equal(existsSync(marker), false);
    assert.equal(readFileSync(path.join(cwd, lockPath), 'utf8'), currentLock);
  }, { withPinnedInput: true });
});

test('current lock comparison cannot be reduced to a self-comparison', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    const config = JSON.parse(readFileSync(path.join(cwd, options.inputViewManifest), 'utf8'));
    config.inputs[0].current_equal = [[lockPath, lockPath]];
    write(options.inputViewManifest, JSON.stringify(config) + '\n');
    await assert.rejects(async () => withFrozenWorkflow(options), /invalid current-source equality pair/);
    assert.equal(existsSync(marker), false);
  }, { withPinnedInput: true });
});

test('pinned input digest cannot diverge from the original frozen manifest', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    const config = JSON.parse(readFileSync(path.join(cwd, options.inputViewManifest), 'utf8'));
    config.inputs[0].sha256 = '0'.repeat(64);
    write(options.inputViewManifest, JSON.stringify(config) + '\n');
    await assert.rejects(async () => withFrozenWorkflow(options), /does not match original frozen source/);
    assert.equal(existsSync(marker), false);
  }, { withPinnedInput: true });
});

test('a changed original source manifest rejects the pinned input view', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    write(originalFreeze, '{"source_sha256":{"cargo_lock":"tampered"}}\n');
    await assert.rejects(async () => withFrozenWorkflow(options), /original freeze manifest changed/);
    assert.equal(existsSync(marker), false);
  }, { withPinnedInput: true });
});

test('a key-pinned workflow is overlaid from history while the live checkout is retained', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    assert.equal(await withFrozenWorkflow(options), 0);
    assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
  }, { manifestFormat: 'key:workflow_sha256' });
});

test('a nested key path reaches the workflow digest through the CLI', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    const helper = fileURLToPath(new URL('./frozen-workflow-check.mjs', import.meta.url));
    execFileSync(process.execPath, [helper, '--workflow', workflow, '--revision', options.revision,
      '--manifest', manifest, '--manifest-format', 'key:first.workflow_sha256', '--root', study,
      '--root', 'src', '--root', 'research/archive', '--root', 'research/rotation',
      '--root', 'Cargo.lock', '--', ...options.command],
    { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
    assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
  }, { manifestFormat: 'key:first.workflow_sha256' });
});

test('a key that does not hold the workflow digest fails before the validator runs', async () => {
  await fixture(async ({ options, marker }) => {
    for (const manifestFormat of ['key:missing_sha256', 'key:other', 'key:workflow_sha256.deeper']) {
      assert.throws(() => withFrozenWorkflow({ ...options, manifestFormat }),
        /historical workflow does not match its frozen digest/, manifestFormat);
    }
    for (const manifestFormat of ['key:', 'key:a..b', 'key:../x', 'key:a b']) {
      assert.throws(() => withFrozenWorkflow({ ...options, manifestFormat }),
        /invalid frozen manifest key path/, manifestFormat);
    }
    assert.equal(existsSync(marker), false);
  }, { manifestFormat: 'key:workflow_sha256' });
});

test('a key pin cannot be moved to the live workflow by rewriting the manifest', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    write(manifest, JSON.stringify({ workflow_sha256: sha(liveWorkflow), other: 'kept' }) + '\n');
    assert.throws(() => withFrozenWorkflow(options), /original frozen manifest changed/);
    assert.equal(existsSync(marker), false);
    assert.equal(readFileSync(path.join(cwd, workflow), 'utf8'), liveWorkflow);
  }, { manifestFormat: 'key:workflow_sha256' });
});

test('an extra workflow is overlaid from history through a JSON Pointer pin', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    assert.equal(await withFrozenWorkflow(options), 0);
    assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
    assert.equal(readFileSync(path.join(cwd, extraWorkflow), 'utf8'), 'name: current other workflow\n');
  }, { extra: true });
});

test('extra workflows pass through the CLI and are required by the validator', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    const helper = fileURLToPath(new URL('./frozen-workflow-check.mjs', import.meta.url));
    const base = [helper, '--workflow', workflow, '--revision', options.revision, '--manifest', manifest,
      '--root', study, '--root', 'src', '--root', 'research/archive', '--root', 'research/rotation',
      '--root', 'Cargo.lock'];
    execFileSync(process.execPath, [...base, '--extra-workflow', extraSpec, '--', ...options.command],
      { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
    assert.equal(existsSync(readFileSync(marker, 'utf8')), false);
    rmSync(marker);
    // Without the overlay the validator sees the live extra workflow and fails.
    assert.throws(() => execFileSync(process.execPath, [...base, '--', ...options.command],
      { cwd, stdio: ['ignore', 'pipe', 'pipe'] }));
    assert.equal(existsSync(marker), false);
  }, { extra: true });
});

test('an extra workflow pin is held to the same custody as the main one', async () => {
  await fixture(async ({ cwd, write, options, marker }) => {
    const pointer = `${extraWorkflow}|${extraManifest}|pointer:/workflow_sha256/missing.yml`;
    assert.throws(() => withFrozenWorkflow({ ...options, extraWorkflows: [pointer] }),
      /historical workflow does not match its frozen digest: .github\/workflows\/other-check.yml/);
    for (const spec of ['no-separators', `${extraWorkflow}||key:a`, `${extraWorkflow}|${extraManifest}`]) {
      assert.throws(() => withFrozenWorkflow({ ...options, extraWorkflows: [spec] }), /expected --extra-workflow/, spec);
    }
    assert.throws(() => withFrozenWorkflow({ ...options,
      extraWorkflows: [`${extraWorkflow}|${extraManifest}|pointer:workflow_sha256`] }), /invalid frozen manifest key path/);
    assert.throws(() => withFrozenWorkflow({ ...options, extraWorkflows: [`${workflow}|${manifest}|files-relative`] }),
      /each workflow may be overlaid once/);
    write(extraManifest, JSON.stringify({ workflow_sha256: { 'other-check.yml': sha('name: current other workflow\n') } }) + '\n');
    assert.throws(() => withFrozenWorkflow(options), /original frozen manifest changed/);
    assert.equal(existsSync(marker), false);
    assert.equal(readFileSync(path.join(cwd, extraWorkflow), 'utf8'), 'name: current other workflow\n');
  }, { extra: true });
});

test('a JSON Pointer reads names with dots and escapes, and only own fields', async () => {
  await fixture(async ({ options }) => {
    for (const pointer of ['pointer:/workflow_sha256/other-check.yml', 'pointer:/workflow_sha256/other-check~2yml']) {
      const spec = `${extraWorkflow}|${extraManifest}|${pointer}`;
      if (pointer.includes('~2')) {
        assert.throws(() => withFrozenWorkflow({ ...options, extraWorkflows: [spec] }), /invalid frozen manifest key path/);
      } else {
        assert.equal(await withFrozenWorkflow({ ...options, extraWorkflows: [spec] }), 0);
      }
    }
    assert.throws(() => withFrozenWorkflow({ ...options,
      extraWorkflows: [`${extraWorkflow}|${extraManifest}|pointer:/constructor`] }),
    /historical workflow does not match its frozen digest/);
  }, { extra: true });
});

test('replay runner: a command after -- needs exactly one --only', async () => {
  const { parseArgs } = await import('./run-frozen-workflow-replays.mjs');
  assert.deepEqual(parseArgs([]), { custodyOnly: false, only: [], command: null });
  assert.deepEqual(parseArgs(['--only', 'a', '--only', 'b']), { custodyOnly: false, only: ['a', 'b'], command: null });
  assert.deepEqual(parseArgs(['--only', 'a', '--', 'bash', '-c', 'x --only y']),
    { custodyOnly: false, only: ['a'], command: ['bash', '-c', 'x --only y'] });
  for (const argv of [['--', 'true'], ['--only', 'a', '--only', 'b', '--', 'true'], ['--only', 'a', '--'],
    ['--only', 'a', '--custody-only', '--', 'true'], ['--only'], ['--only', '--custody-only'], ['--bogus']]) {
    assert.throws(() => parseArgs(argv), undefined, argv.join(' '));
  }
});

test('replay runner: --only scopes custody to the pins of the selected replays', async () => {
  const { custodyProblems } = await import('./run-frozen-workflow-replays.mjs');
  const repo = mkdtempSync(path.join(os.tmpdir(), 'frozen-replay-custody-'));
  try {
    const git = (...args) => execFileSync('git', args, { cwd: repo, stdio: ['ignore', 'pipe', 'pipe'] }).toString().trim();
    git('init', '-q');
    git('config', 'user.email', 'test@example.invalid');
    git('config', 'user.name', 'test');
    const hash = text => createHash('sha256').update(text).digest('hex');
    mkdirSync(path.join(repo, '.github/workflows'), { recursive: true });
    for (const name of ['a', 'b', 'c']) writeFileSync(path.join(repo, `.github/workflows/${name}.yml`), `name: ${name}\n`);
    const pins = Object.fromEntries(['a', 'b', 'c'].map(name => [name, hash(`name: ${name}\n`)]));
    // b's evidence file drops its pin, so only replays that cover b see a problem.
    writeFileSync(path.join(repo, 'a.json'), pins.a);
    writeFileSync(path.join(repo, 'b.json'), 'unpinned');
    writeFileSync(path.join(repo, 'c.json'), pins.c);
    git('add', '.');
    git('commit', '-qm', 'pins');
    const index = { revision: git('rev-parse', 'HEAD'),
      workflows: Object.fromEntries(['a', 'b', 'c'].map(name => [`.github/workflows/${name}.yml`,
        { pinned_sha256: pins[name], check: 'replay', pinned_by: [`${name}.json`] }])),
      replays: [
        { id: 'ra', workflow: '.github/workflows/a.yml', extra_workflows: [] },
        { id: 'rb', workflow: '.github/workflows/c.yml', extra_workflows: ['.github/workflows/b.yml|b.json|key:x'] },
      ] };
    assert.deepEqual(custodyProblems({ cwd: repo, index, only: ['ra'] }), []);
    assert.deepEqual(custodyProblems({ cwd: repo, index, only: ['rb'] }),
      ['.github/workflows/b.yml: b.json no longer carries its pin']);
    assert.deepEqual(custodyProblems({ cwd: repo, index }), ['.github/workflows/b.yml: b.json no longer carries its pin']);
  } finally {
    rmSync(repo, { recursive: true, force: true });
  }
});

test('a coreutils sha256sum manifest pins the workflow and its root-relative inputs', async () => {
  await fixture(async ({ cwd, options, marker }) => {
    assert.equal(withFrozenWorkflow(options), 0);
    assert.notEqual(readFileSync(marker, 'utf8'), cwd);
    // sha256sum --check itself passes in the view and fails on the live checkout.
    const check = { ...options, command: ['sha256sum', '--check', '--quiet', manifest] };
    assert.equal(withFrozenWorkflow(check), 0);
    assert.throws(() => execFileSync('sha256sum', ['--check', '--quiet', manifest], { cwd, stdio: 'pipe' }));
  }, { manifestFormat: 'sha256sum' });
});

test('a sha256sum manifest rejects drifted inputs and malformed lines', async () => {
  await fixture(async ({ write, options }) => {
    write(study + '/input.json', '{"evidence":"changed"}\n');
    assert.throws(() => withFrozenWorkflow(options), /frozen input drift: research\/study\/input.json/);
  }, { manifestFormat: 'sha256sum' });
  for (const bad of ['not a digest line\n', `${'a'.repeat(64)} research/x\n`, `${'a'.repeat(64)}  x\n${'b'.repeat(64)}  x\n`]) {
    await fixture(async ({ cwd, write, options }) => {
      write(manifest, bad);
      const git = (...args) => execFileSync('git', args, { cwd, stdio: ['ignore', 'pipe', 'pipe'] });
      git('add', '.');
      git('-c', 'user.name=CI fixture', '-c', 'user.email=ci-fixture@example.invalid', 'commit', '-qm', 'bad manifest');
      const revision = git('rev-parse', 'HEAD').toString().trim();
      assert.throws(() => withFrozenWorkflow({ ...options, revision }), /invalid sha256sum manifest line/);
    }, { manifestFormat: 'sha256sum' });
  }
});

// A repository whose workflow has two pinned versions: the index revision holds
// v2 (pinned by v2.json), and a sha256sum manifest pins v1 at an earlier commit.
function twoVersionRepo() {
  const repo = mkdtempSync(path.join(os.tmpdir(), 'frozen-replay-revision-'));
  const git = (...args) => execFileSync('git', args, { cwd: repo, stdio: ['ignore', 'pipe', 'pipe'] }).toString().trim();
  const write = (file, text) => {
    mkdirSync(path.dirname(path.join(repo, file)), { recursive: true });
    writeFileSync(path.join(repo, file), text);
  };
  git('init', '-q');
  git('config', 'user.email', 'test@example.invalid');
  git('config', 'user.name', 'test');
  const w = '.github/workflows/w.yml';
  const m = 'research/run/ARTIFACTS.sha256';
  write(w, 'name: v1\n');
  write('research/run/out.json', '{"row":1}\n');
  write(m, `${sha('{"row":1}\n')}  research/run/out.json\n${sha('name: v1\n')}  ${w}\n`);
  git('add', '.');
  git('commit', '-qm', 'v1 run');
  const first = git('rev-parse', 'HEAD');
  write(w, 'name: v2\n');
  write('v2.json', sha('name: v2\n'));
  git('add', '.');
  git('commit', '-qm', 'v2 evidence');
  const second = git('rev-parse', 'HEAD');
  write(w, 'name: live\n');
  git('add', '.');
  git('commit', '-qm', 'live CI policy');
  const replay = { id: 'run', workflow: w, manifest: m, manifest_format: 'sha256sum', revision: first,
    extra_workflows: [], roots: [], command: ['sha256sum', '--check', m] };
  const index = { schema: 'frozen-workflow-pins/v1', revision: second,
    workflows: { [w]: { pinned_sha256: sha('name: v2\n'), check: 'replay', pinned_by: ['v2.json'] } },
    replays: [replay] };
  return { repo, git, write, index, replay, first, second, w, m };
}

test('replay runner: a per-replay revision overrides the index revision in custody', async () => {
  const { custodyProblems } = await import('./run-frozen-workflow-replays.mjs');
  const { repo, git, write, index, replay, first, second, w, m } = twoVersionRepo();
  try {
    assert.deepEqual(custodyProblems({ cwd: repo, index }), []);
    assert.deepEqual(custodyProblems({ cwd: repo, index, only: ['run'] }), []);
    // At the index revision the manifest's pin (v1) is not the workflow's bytes.
    const atIndex = { ...index, replays: [{ ...replay, revision: second }] };
    assert.deepEqual(custodyProblems({ cwd: repo, index: atIndex }), [`replay run: ${m} does not pin ${w} at ${second}`]);
    for (const revision of ['HEAD', first.slice(0, 12), first.toUpperCase(), '', null]) {
      assert.deepEqual(custodyProblems({ cwd: repo, index: { ...index, replays: [{ ...replay, revision }] } }),
        ['replay run: invalid revision'], String(revision));
    }
    // Scoped custody ignores a broken override on an unselected replay.
    const other = { ...index, replays: [replay, { ...replay, id: 'other', revision: 'bad' }] };
    assert.deepEqual(custodyProblems({ cwd: repo, index: other, only: ['run'] }), []);
    write(m, `${sha('{"row":1}\n')}  research/run/out.json\n`);
    git('add', '.');
    git('commit', '-qm', 'drop the workflow pin');
    assert.deepEqual(custodyProblems({ cwd: repo, index }), [`replay run: ${m} does not pin ${w} at ${first}`]);
  } finally {
    rmSync(repo, { recursive: true, force: true });
  }
});

test('replay runner: a per-replay revision restores its own workflow version in the view', () => {
  const runner = fileURLToPath(new URL('./run-frozen-workflow-replays.mjs', import.meta.url));
  const { repo, write, index, replay, second, m } = twoVersionRepo();
  const run = (idx, ...argv) => {
    write('.github/scripts/frozen-workflow-pins.json', JSON.stringify(idx));
    return spawnSync(process.execPath, [runner, ...argv], { cwd: repo, encoding: 'utf8' });
  };
  try {
    // The live checkout itself fails the manifest: its workflow bytes are not v1.
    assert.throws(() => execFileSync('sha256sum', ['--check', '--quiet', m], { cwd: repo, stdio: 'pipe' }));
    let result = run(index);
    assert.equal(result.status, 0, result.stdout + result.stderr);
    assert.match(result.stdout, /PASS run/);
    result = run(index, '--only', 'run', '--', 'bash', '-eo', 'pipefail', '-c', `sha256sum -c ${m}`);
    assert.equal(result.status, 0, result.stdout + result.stderr);
    assert.match(result.stdout, /\.github\/workflows\/w\.yml: OK/);
    // Without the override the replay uses the index revision (v2) and fails custody of its manifest.
    const { revision: _, ...plain } = replay;
    result = run({ ...index, replays: [plain] });
    assert.equal(result.status, 1);
    assert.match(result.stderr, /historical workflow does not match its frozen digest/);
    // A tampered pin: the override names a commit whose bytes the manifest does not pin.
    result = run({ ...index, replays: [{ ...replay, revision: second }] });
    assert.equal(result.status, 1);
    assert.match(result.stderr, /custody: replay run: .* does not pin/);
  } finally {
    rmSync(repo, { recursive: true, force: true });
  }
});
