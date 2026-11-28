// Source-only no-op rebase audit; no cryptographic code is run.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawnSync} from 'node:child_process';
const dir = path.dirname(new URL(import.meta.url).pathname);
const root = path.dirname(dir);
const read = name => fs.readFileSync(path.join(dir, name), 'utf8');
const write = (name, data) => fs.writeFileSync(path.join(dir, name), data);
const assert = (yes, message) => { if (!yes) throw new Error(message); };
const run = (cmd, args, options={}) => spawnSync(cmd, args, {encoding:'utf8',maxBuffer:8*1024*1024,...options});
const sha256 = s => crypto.createHash('sha256').update(s).digest('hex');
const blob = s => crypto.createHash('sha1').update(`blob ${Buffer.byteLength(s)}\0`).update(s).digest('hex');
const tests = s => Array.from(s.matchAll(/#\[test\](?:\s*#\[[^\]]*\])*\s*(?:async\s+)?fn\s+(\w+)/g),m=>m[1]);

const current = read('ic_boundary.current.rs');
const candidate = read('ic_boundary.restored.rs');
const spec = read('spec.current.rs');
const oldBoundary = fs.readFileSync(path.join(root,'ci_ic_boundary_audit/ic_boundary.restored.rs'),'utf8');
const oldSpec = fs.readFileSync(path.join(root,'ci_syntax_audit/spec.restored.rs'),'utf8');
assert(blob(current)==='0b0d7fb149307d484075529bfb9852621aaa4ff0','Wrong current boundary blob');
assert(blob(spec)==='6878f0b872ef6349324e7ba43c8a6d61d7ffcedf','Wrong current spec blob');
assert(blob(oldBoundary)==='d57e6de59cc01de91e860d3ee32dcd849bbdbd67','Wrong historical boundary candidate');
assert(blob(oldSpec)==='ed85acbb242a11a1412da7c842cef70872235dcc','Wrong historical spec candidate');
assert(current===candidate,'No source change is required or permitted');
const marker='/// One row of a [`TargetWalk`]';
const prefixEnd=oldBoundary.indexOf(marker);
assert(prefixEnd>0 && current.startsWith(oldBoundary.slice(0,prefixEnd)),'Restored method prefix differs');

const oldTestNames=tests(oldBoundary), newTestNames=tests(current), specNames=tests(spec);
assert(newTestNames.length===(current.match(/#\[test\]/g)||[]).length,'Unparsed boundary test attribute');
assert(specNames.length===(spec.match(/#\[test\]/g)||[]).length,'Unparsed spec test attribute');
assert(oldTestNames.every(n=>newTestNames.includes(n)),'Historical boundary test removed');
const added=newTestNames.filter(n=>!oldTestNames.includes(n));
assert(added.length===4 && newTestNames.length===104,'Unexpected upstream test inventory');
assert(JSON.stringify(tests(candidate))===JSON.stringify(newTestNames),'Candidate tests differ');
const testMarker='#[cfg(test)]\nmod tests {';
const oldTestStart=oldSpec.indexOf(testMarker), newTestStart=spec.indexOf(testMarker);
assert(oldTestStart>=0 && newTestStart>=0,'Spec test module absent');
assert(oldSpec.slice(oldTestStart)===spec.slice(newTestStart),'Historical spec test module changed');
assert(specNames.length===15,'Unexpected spec test count');

// Empty patch is intentional: the old eleven-line restoration is already
// upstream. Demonstrate this using a separate reverse/apply round trip.
write('restore_ic_boundary.patch','');
const isolated=path.join(dir,'isolated-roundtrip');
fs.mkdirSync(path.join(isolated,'src/cryptanalysis'),{recursive:true});
const isolatedFile=path.join(isolated,'src/cryptanalysis/ic_boundary.rs');
fs.writeFileSync(isolatedFile,current);
const oldPatch=path.join(root,'ci_ic_boundary_audit/restore_ic_boundary.patch');
const steps=[];
for(const args of [['apply','--reverse','--check',oldPatch],['apply','--reverse',oldPatch],['apply','--check',oldPatch],['apply',oldPatch]]){
  const r=run('git',args,{cwd:isolated});
  steps.push({args:args.slice(0,-1).concat('<historical_patch>'),exit:r.status,stderr:r.stderr});
  assert(r.status===0,'Historical restoration roundtrip failed');
}
assert(fs.readFileSync(isolatedFile,'utf8')===current,'Roundtrip changed upstream bytes');

const parser={};
for(const name of ['ic_boundary.current.rs','spec.current.rs']){
  const r=run('rustfmt',['--edition','2021','--emit','stdout','--config','skip_children=true',path.join(dir,name)]);
  write(name+'.parser.log',r.stderr);
  parser[name]={exit:r.status,stderr_bytes:Buffer.byteLength(r.stderr)};
  assert(r.status===0,'Current module parser failed');
}
const diffStats=(a,b,name)=>{
  const r=run('diff',['-u','--label','historical/'+path.basename(a),'--label','main-f0/'+path.basename(b),a,b]);
  assert(r.status===1,'Expected upstream source changes');
  write(name,r.stdout);
  const lines=r.stdout.split('\n');
  return {insertions:lines.filter(x=>x.startsWith('+')&&!x.startsWith('+++')).length,deletions:lines.filter(x=>x.startsWith('-')&&!x.startsWith('---')).length,sha256:sha256(r.stdout)};
};
const boundaryDelta=diffStats(path.join(root,'ci_ic_boundary_audit/ic_boundary.restored.rs'),path.join(dir,'ic_boundary.current.rs'),'boundary-upstream.diff');
const specDelta=diffStats(path.join(root,'ci_syntax_audit/spec.restored.rs'),path.join(dir,'spec.current.rs'),'spec-upstream.diff');
const receipt={
  schema:'prime31.current-main-source-rebase-audit/v1',
  repository:'isogeny-labs/crypto',main_commit:'f0ca664eca0081a30adfcc79dcf62a32de40d2af',
  disposition:'no_op_historical_boundary_restoration_already_upstream',
  historical_boundary_audit:{source_commit:'fc184b03d47e13ec922c9b489f3b3f8a976a8bd9',source_blob:'9cf97a9f56846ff39b574967a49a0703733ae028',candidate_blob:blob(oldBoundary),candidate_sha256:sha256(oldBoundary),receipt_sha256:sha256(fs.readFileSync(path.join(root,'ci_ic_boundary_audit/receipt.json'))),patch_sha256:sha256(fs.readFileSync(oldPatch))},
  boundary:{current_blob:blob(current),candidate_blob:blob(candidate),sha256:sha256(current),bytes:Buffer.byteLength(current),lines:current.split('\n').length-1,exact_source_delta:{insertions:0,deletions:0},all_eleven_historical_lines_already_present:true,restored_prefix_bytes:Buffer.byteLength(oldBoundary.slice(0,prefixEnd)),historical_test_count:oldTestNames.length,current_test_count:newTestNames.length,added_upstream_tests:added,current_candidate_test_inventory_identical:true,historical_test_names_preserved:true,upstream_delta_from_historical_candidate:boundaryDelta},
  spec:{blob:blob(spec),sha256:sha256(spec),bytes:Buffer.byteLength(spec),lines:spec.split('\n').length-1,baseline_blob:blob(oldSpec),modified_by_this_audit:false,upstream_delta:specDelta,test_count:specNames.length,test_module_byte_identical_to_historical_restored_source:true,test_names:specNames,reviewed_change:'PairedCurvesV1 execution-order/seed policy above unchanged historical tests; no inherited syntax omission observed'},
  parser:{version:run('rustfmt',['--version']).stdout.trim(),command:'rustfmt --edition 2021 --emit stdout --config skip_children=true <complete-direct-module>',outcomes:parser},
  historical_patch_roundtrip:{steps,exact_upstream_bytes_preserved:true},
  empty_patch:{bytes:0,sha256:sha256(''),reason:'All eleven restoration lines already exist on current main; do not apply the historical patch twice'},
  limits:{tests_executed:false,cryptographic_code_executed:false,typecheck_performed:false,workspace_compile_performed:false,remote_mutations:false,repo_audit_mutations:false,pcc_mutations:false,main_input_mutations:false,signed_verification:false}
};
write('receipt.json',JSON.stringify(receipt,null,2)+'\n');
console.log(JSON.stringify({disposition:receipt.disposition,boundary:receipt.boundary,spec:receipt.spec,parser:receipt.parser,receipt_sha256:sha256(read('receipt.json'))},null,2));
