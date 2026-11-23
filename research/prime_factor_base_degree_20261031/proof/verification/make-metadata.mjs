// Proof/publication metadata only: hash existing bytes and inspect check receipts.
// All mathematical computation is in the Lean modules.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawnSync} from 'node:child_process';
const root=path.resolve(import.meta.dirname,'..');
const repo=path.resolve(root,'../../..');
const mathlib=path.join(root,'.lake/packages/mathlib');
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
function run(exe,args,cwd=root){const r=spawnSync(exe,args,{cwd,encoding:'utf8'});if(r.status!==0)throw new Error(`${exe} failed: ${r.stderr}`);return r.stdout.trim();}
const build=fs.readFileSync(path.join(root,'verification/build.log'),'utf8');
const audit=fs.readFileSync(path.join(root,'verification/axioms.log'),'utf8');
if(!build.includes('Build completed successfully.')||/\berror:|\bwarning:/.test(build))throw new Error('final build receipt is not clean');
const verdict=audit.match(/audited (\d+) theorems[\s\S]*?; (\d+) failing, (\d+) trusted \(native_decide\)/);
if(!verdict||verdict[2]!=='0'||verdict[3]!=='0'||/^FAIL|^TRUST/m.test(audit))throw new Error('transitive audit failed');
const sources=fs.readdirSync(root).filter(f=>f.endsWith('.lean')).sort();
for(const f of sources){if(/\b(sorry|admit|axiom|native_decide)\b/.test(fs.readFileSync(path.join(root,f),'utf8')))throw new Error(`unexpected trust token in ${f}`);}
const manifest=JSON.parse(fs.readFileSync(path.join(root,'lake-manifest.json'),'utf8'));
const dependencies=manifest.packages.map(p=>{if(p.type!=='git')throw new Error('non-Git dependency override');const dir=path.join(root,manifest.packagesDir,p.name);const head=run('git',['rev-parse','HEAD'],dir);if(head!==p.rev)throw new Error('dependency revision mismatch');run('git',['diff','--exit-code','--'],dir);return{name:p.name,url:p.url,revision:p.rev,checkout_matches:true};});
const pins=['lean-toolchain','lakefile.toml','lake-manifest.json'];
const receipts=['verification/build.log','verification/axioms.log','runs.tar.xz','runs.tar.xz.sha256','reports/coverage.svg','reports/formalization.typ','reports/formalization.pdf'];
const fileList=files=>files.map(p=>({path:p,sha256:hash(path.join(root,p)),bytes:fs.statSync(path.join(root,p)).size}));
const checker=spawnSync('which',['lean4checker'],{encoding:'utf8'});
const metadata={
  schema:'lean.formalization.receipt/v1',date:'2026-10-31',
  source_checkout_before_formalization:run('git',['rev-parse','HEAD'],repo),
  source_commit_scope:'New formalization bytes are bound individually below; the preceding local commit is provenance, not a claim that it contains these additions.',
  primary_theorems:['DegreeBridge.canonical_strict_reduction','CanonicalFixture.degree_five_to_three'],
  invariant:'First vanishing homogeneous image in the quotient by the canonical highest-total-degree ideal of the actual original generators.',
  lean_version:run('/workspace/prime-degree-lean/bin/lean',['--version']),
  lean_binary_sha256:hash('/workspace/prime-degree-lean/bin/lean'),
  mathlib_revision:run('git',['rev-parse','HEAD'],mathlib),dependencies,
  source_files:fileList([...sources,...pins]),
  receipts:fileList(receipts),
  build:{exit_code:0,warnings_are_failures:true,command:['lake','--no-cache','--wfail','build']},
  audit:{exit_code:0,declarations_including_generated_equations:Number(verdict[1]),failing:0,trusted_native:0,allowed_foundations:['propext','Classical.choice','Quot.sound'],program:'tools/lean/AuditAxioms.lean',program_sha256:hash(path.join(repo,'tools/lean/AuditAxioms.lean')),published_identical_program_commit:'ff7ad61cfed11908fb79de9dd485b696d0ada779',modules:sources.map(p=>p.slice(0,-5))},
  cache:{status:'unavailable',reason:'proxy CONNECT403 at the public Mathlib binary cache; pinned dependency built from source',network_policy_bypass:false},
  lean4checker:{available:checker.status===0,fresh_check_performed:false},
  experiment_or_cas_rerun:false,
  remaining_scope:['Full elliptic-curve group-law equivalence is written, not formalized.','No F4 solving-degree, runtime, Hilbert-series or independent-row theorem follows.','Literature-wide novelty remains open.'],
  artifact_history:'runs.tar.xz contains preserved failed development/build attempts; only the final clean receipts establish the proof.'
};
fs.writeFileSync(path.join(root,'verification/metadata.json'),JSON.stringify(metadata,null,2)+'\n');
process.stdout.write(JSON.stringify({modules:sources.length,build:'passed',axiom_audit:'passed',declarations:Number(verdict[1]),failing:0,trusted_native:0})+'\n');
