#!/usr/bin/env node
// Static proposal/document validation; no field/group/solver computation.
'use strict';
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const dir = __dirname;
const root = path.resolve(dir, '../../..');
const read = p => fs.readFileSync(path.join(root, p));
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
const design = JSON.parse(fs.readFileSync(path.join(dir, 'experiments.json')));
const report = fs.readFileSync(path.join(dir, 'README.md'), 'utf8');
const fail = m => { throw new Error(m); };
const check = (v, m) => {if (!v) fail(m);};
check(design.status === 'proposed_not_started', 'wrong design status');
for (const key of ['execution_started', 'scheduled', 'novelty_established', 'speedup_established']) check(design[key] === false, `${key} must be false`);
check(design.funding_units === 0, 'funding changed');
check(design.experiments.length === 12, 'expected 12 proposals');
check(hash(read('research/twist_routes_1_3_20261021/execution-v1/fixtures.json')) === design.fixtures_sha256, 'fixture hash mismatch');
const byId = new Map(design.experiments.map(e => [e.id, e]));
check(byId.size === 12, 'duplicate ID');
for (let i=0; i<12; i++) check(byId.has(`E${18+i}`), 'noncontiguous IDs');
const visited = new Set();
function visit(id, stack = new Set()) {
  check(byId.has(id), `missing dependency ${id}`);
  check(!stack.has(id), `dependency cycle at ${id}`);
  if (visited.has(id)) return;
  const next = new Set(stack); next.add(id);
  for (const dep of byId.get(id).dependencies) visit(dep, next);
  visited.add(id);
}
for (const e of design.experiments) {
  visit(e.id);
  check(report.includes(`## ${e.id}\n`), `missing heading ${e.id}`);
  check(report.includes(e.question), `missing question ${e.id}`);
  check(e.status === 'proposed_not_started', `wrong experiment status ${e.id}`);
  for (const p of e.references) check(fs.existsSync(path.join(root,p)), `missing source ${p}`);
  const c = JSON.parse(read(`backlog/${e.backlog_id}/card.json`));
  check(c.status === 'blocked', `wrong card status ${e.id}`);
  check(c.cairn.funding.units === 0 && c.run.argv.join() === 'false', `runnable/funded proposal ${e.id}`);
  check(c.protocol.path === null && c.code.commit === null, `proposal has a misleading run pin ${e.id}`);
  check(c.question === e.question && c.decision_value === e.decision, `card/design mismatch ${e.id}`);
}
let links = 0;
const markdownPaths = [path.join(dir,'README.md'), ...design.experiments.map(e => path.join(root,'backlog',e.backlog_id,'README.md'))];
for (const p of markdownPaths) {
  const s = fs.readFileSync(p,'utf8');
  for (const m of s.matchAll(/\]\(([^)]+)\)/g)) {
    const target = m[1];
    if (/^https?:/.test(target) || target.startsWith('#')) continue;
    check(fs.existsSync(path.resolve(path.dirname(p),target.split('#')[0])), `broken local link ${p}: ${target}`);
    links++;
  }
}
const files = ['write-proposals.cjs','experiments.json','README.md','PROGRAM.dot','PROGRAM.svg','PROGRAM.pdf','REPORT.tex','REPORT.pdf','render.sh','layout.lua','validate.cjs',
  'CHECKS.md','checks/VALIDATION.json','checks/root-lib.stdout','checks/root-lib.stderr',
  'checks/socket-module.stdout','checks/socket-module.stderr','checks/backlog.stdout','checks/backlog.stderr']
  .map(p => path.relative(root,path.join(dir,p)))
  .concat(design.experiments.flatMap(e => [`backlog/${e.backlog_id}/card.json`,`backlog/${e.backlog_id}/README.md`]));
const artifacts = files.map(p => {const b = read(p); return {path:p,bytes:b.length,sha256:hash(b)};});
const receipt = {
  schema: 'twist.proposal-artifacts/v1', created: '2026-10-22',
  evidence_base_commit: design.evidence_base_commit,
  status: 'proposal_documents_validated', scientific_experiments_run: 0,
  proposals: 12, blocked_unfunded_cards: 12, local_links_checked: links,
  dependency_graph_acyclic: true,
  validation_scope: 'Static proposal/card cross-checks, file/link existence and artifact hashes only. Native backlog validation, PDF visual inspection and repository tests are separate checks.',
  artifacts
};
const receiptPath = path.join(dir,'ARTIFACTS.json');
if (process.argv.includes('--write-manifest')) fs.writeFileSync(receiptPath,JSON.stringify(receipt,null,2)+'\n');
else {
  const previous = JSON.parse(fs.readFileSync(receiptPath));
  check(JSON.stringify(previous) === JSON.stringify(receipt), 'artifact manifest differs; rebuild/inspect before refreshing');
}
console.log(JSON.stringify({status:'pass',proposals:12,blocked_unfunded_cards:12,local_links_checked:links,artifacts_hashed:artifacts.length,scientific_experiments_run:0}));
