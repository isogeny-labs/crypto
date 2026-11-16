// Generic provenance and protocol checks; no finite-field or EC arithmetic.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
const root = path.resolve(import.meta.dirname, '../..');
const study = path.join(root, 'research/s3_host_replay_20261113');
const p = JSON.parse(fs.readFileSync(path.join(study, 'protocol.json')));
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
function sorted(x) {
  if (Array.isArray(x)) return x.map(sorted);
  if (x !== null && typeof x === 'object') return Object.fromEntries(Object.keys(x).sort().map(k => [k, sorted(x[k])]));
  return x;
}
const body = {...p}; delete body.freeze;
const digest = sha(JSON.stringify(sorted(body)));
if (p.freeze.status !== 'frozen' || !p.freeze.filed_before_affected_data || digest !== p.freeze.protocol_sha256)
  throw Error('Continuation is not frozen with its exact canonical identity');
if (p.execution.backend !== 'ephemeral' || p.execution.retries.max_attempts !== 1 || p.extensions.paid_spend !== 0)
  throw Error('Unexpected execution or zero-spend contract');
for (const f of p.extensions.component_files) {
  const b = fs.readFileSync(path.join(root, f.path));
  if (b.length !== f.bytes || sha(b) !== f.sha256) throw Error('Component hash mismatch: ' + f.path);
}
if (process.env.GITHUB_ACTIONS === 'true') {
  for (const [key, expected] of Object.entries({GITHUB_EVENT_NAME: 'push', GITHUB_REF: 'refs/heads/main',
    GITHUB_REPOSITORY: 'isogeny-labs/crypto', GITHUB_RUN_ATTEMPT: '1'}))
    if (process.env[key] !== expected) throw Error('Unplanned hosted event: ' + key);
}
console.log(JSON.stringify({protocol_id: p.protocol_id, version: p.version, protocol_sha256: digest,
  workflow_sha256: p.extensions.workflow_sha256, event_sha: process.env.GITHUB_SHA ?? null,
  run_id: process.env.GITHUB_RUN_ID ?? null, run_attempt: process.env.GITHUB_RUN_ATTEMPT ?? null,
  source_commit: p.extensions.source_checkout, paid_spend: 0, clock_valid: false}));
