// Provenance/record processing only. Never executes a scientific population.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {gzipSync,gunzipSync} from 'node:zlib';
const study=path.dirname(import.meta.filename), root=path.resolve(study,'../..');
const evidence=path.join(study,'evidence/hosted');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const write=(p,x)=>fs.writeFileSync(p,JSON.stringify(x,null,2)+'\n');
const protocol=read(path.join(study,'protocol.json'));
const run=read(path.join(evidence,'run.json'));
const jobs=read(path.join(evidence,'jobs.json'));
assert.equal(run.id,43501438026);assert.equal(run.run_attempt,1);
assert.equal(run.event,'push');assert.equal(run.head_branch,'main');
assert.equal(run.head_sha,'43875b707caedfb7b8175de0961e219a1152bbb3');
assert.equal(run.conclusion,'success');
assert.equal(jobs.jobs.length,1);assert.equal(jobs.jobs[0].id,132147937956);
assert.equal(jobs.jobs[0].conclusion,'success');
const raw=gunzipSync(fs.readFileSync(path.join(evidence,'job.log.gz')));
const unprefixed=[];
const lines=raw.toString('utf8').replace(/^\uFEFF/,'').split('\n').map((l,i)=>{
  if(l==='')return '';
  if(!/^\d{4}-\d\d-\d\dT[^ ]+Z /.test(l)){unprefixed.push({line:i+1,text:l});return l;}
  return l.replace(/^\d{4}-\d\d-\d\dT[^ ]+Z /,'');
});
const identity=lines.filter(l=>l.startsWith('{"protocol_id":"s3-hosted-continuation-'));
assert.equal(identity.length,1);const event=JSON.parse(identity[0]);
assert.equal(event.protocol_sha256,protocol.freeze.protocol_sha256);
assert.equal(event.event_sha,run.head_sha);assert.equal(event.run_id,String(run.id));
assert.ok(protocol.freeze.filed_before_affected_data);
assert.ok(protocol.freeze.frozen_at<run.created_at);
write(path.join(evidence,'event.json'),event);
function extract(prefix,dir,names){
  fs.mkdirSync(path.join(evidence,dir),{recursive:true});
  for(const name of names){
    const begin=prefix+'_BEGIN '+name,end=prefix+'_END '+name;
    const starts=lines.flatMap((v,i)=>v===begin?[i]:[]),ends=lines.flatMap((v,i)=>v===end?[i]:[]);
    assert.equal(starts.length,1,begin);assert.equal(ends.length,1,end);
    assert.ok(ends[0]>starts[0]);
    assert.ok(!unprefixed.some(x=>x.line>starts[0]+1 && x.line<ends[0]+1),'unprefixed line inside data block');
    const body=lines.slice(starts[0]+1,ends[0]);
    // The driver's printf adds exactly one separator newline before END.
    assert.equal(body.pop(),'');
    const bytes=Buffer.from(body.length?body.join('\n')+'\n':'');
    if(name==='compiler.txt'){
      const packed=gzipSync(bytes);assert.deepEqual(gunzipSync(packed),bytes);
      fs.writeFileSync(path.join(evidence,dir,name+'.gz'),packed);
    }else fs.writeFileSync(path.join(evidence,dir,name),bytes);
  }
}
const common=['build.exit','build.stdout','build.stderr','actual.jsonl','actual.stderr','comparison.json','comparison.stderr','run.exit','comparison.exit'];
extract('S3_HOST_CPP','cpp',['source.json','compiler.txt','binary.sha256',...common]);
extract('S3_VALIDATOR','rust',['host.json',...common,'start-epoch','end-epoch']);
assert.equal(lines.filter(l=>l.startsWith('S3_HOST_CONTINUATION_STATUS')).length,1);
assert.ok(lines.includes('S3_HOST_CONTINUATION_STATUS cpp_build=0 cpp_run=0 cpp_compare=0 rust=0'));
const cpp=read(path.join(evidence,'cpp/source.json')),host=read(path.join(evidence,'rust/host.json'));
assert.equal(cpp.source_commit,protocol.extensions.source_checkout);
assert.equal(host.git_sha,protocol.extensions.source_checkout);
assert.equal(cpp.clock_valid,false);assert.equal(host.clock_valid,false);
assert.equal(cpp.cpu,host.cpu);assert.equal(cpp.release,host.release);
const values={},binaries={};
for(const arm of ['cpp','rust']){
  for(const n of ['build.exit','run.exit','comparison.exit'])
    assert.equal(fs.readFileSync(path.join(evidence,arm,n),'utf8').trim(),'0');
  assert.equal(read(path.join(evidence,arm,'comparison.json')).matched,true);
  values[arm]=fs.readFileSync(path.join(evidence,arm,'actual.jsonl'),'utf8').trim().split('\n').map(JSON.parse);
  assert.deepEqual(values[arm].map(x=>x.q),[5,7,11,13,17,19]);
  for(const row of values[arm]){assert.equal(row.false_zeros,0);assert.equal(row.missing_zeros,0);}
}
for(let i=0;i<6;i++)for(const k of ['q','m','curves','tuples','zeros','false_zeros','missing_zeros'])
  assert.equal(values.cpp[i][k],values.rust[i][k],k);
binaries.cpp=fs.readFileSync(path.join(evidence,'cpp/binary.sha256'),'utf8').split(/\s/)[0];
binaries.rust=host.binary_sha256;
for(const x of Object.values(binaries))assert.match(x,/^[0-9a-f]{64}$/);
function file(rel,kind='raw'){
  const b=fs.readFileSync(path.join(study,rel));return {path:rel,sha256:sha(b),bytes:b.length,kind};
}
const counts=arm=>({fields:values[arm].length,models:values[arm].reduce((s,x)=>s+x.curves,0),tuples:values[arm].reduce((s,x)=>s+x.tuples,0),zeros:values[arm].reduce((s,x)=>s+x.zeros,0),comparison:1});
for(const arm of ['cpp','rust'])assert.deepEqual(counts(arm),{fields:6,models:942,tuples:4188162,zeros:244236,comparison:1});
const paths=dir=>fs.readdirSync(path.join(study,dir),{withFileTypes:true}).flatMap(x=>x.isDirectory()?paths(dir+'/'+x.name):[dir+'/'+x.name]);
const files=paths('evidence/hosted').filter(p=>!p.endsWith('/extraction.json')).sort().map(p=>file(p,p.endsWith('.log')?'log':'raw'));
const exportRows=['cpp','rust'].map((arm,i)=>({run_id:`43501438026/132147937956/${arm}`,arm:arm+'-hosted',instance_id:'0',seed:0,replicate:0,warmup:false,status:'succeeded',outcome_class:'completed',attempts:1,resumed:0,checkpoint:null,measurements:{counted:counts(arm),wall_seconds:null,wall_valid:false,cpu_user_seconds:null,cpu_sys_seconds:null,max_rss_bytes:null,counters:{false_zeros:0,missing_zeros:0}},isolation:{tier:'none',contended:null,blockers:['Correctness-only public hosted runner; no isolation or valid clock claim.']},verification:{status:'no_claim',verifier:'Exact known-answer comparators pass; no signed execution verdict.'},exclusion:{excluded:false,rule:null},artifacts:files.filter(x=>x.path.includes('/'+arm+'/')),started_at:null,finished_at:null,worker:'GitHub-hosted ubuntu-24.04',backend_ref:'https://github.com/isogeny-labs/crypto/actions/runs/43501438026/job/132147937956'}));
const exp={schema:'experiment.export/v1',export_id:'s3-hosted-continuation-43501438026',created:new Date().toISOString(),protocol:{path:'protocol.json',sha256:protocol.freeze.protocol_sha256,protocol_id:protocol.protocol_id,version:protocol.version},producer:{runtime:'ci',model:null,session:'43501438026/1',role:'executor; export assembled from complete public logs'},source:{repo:'crypto',commit:run.head_sha,dirty:false},backend:'ephemeral',host:{path:'evidence/hosted/rust/host.json',sha256:file('evidence/hosted/rust/host.json').sha256,class_id:'github-hosted-ubuntu-24.04-20261111.370.1',summary:{cpu_model:host.cpu,logical_cpus:null,memory_bytes:null,kernel:host.release,hypervisor:null,isolation_tier_max:'none'}},arms:protocol.arms.map((a,i)=>({id:a.id,role:a.role,binary_sha256:binaries[i===0?'cpp':'rust'],commit:a.instrument.commit})),runs:exportRows,flow:{planned:2,attempted:2,completed:2,not_completed:0,interrupted:0,resumed:0,excluded:[]},files:[file('protocol.json','protocol'),...files],analysis:{script:file('compare-cpp.mjs','analysis'),blinded:false,blinding_key_sha256:null,outputs:[file('evidence/hosted/cpp/comparison.json','analysis'),file('evidence/hosted/rust/comparison.json','analysis')],unmasked_at:null},notes:[{kind:'observation',text:'Two fresh native calls on first main-push run attempt. Original failed manual route remains not completed; no historical executions imported. Per-field records agree in both directions. No clocks used.',by:'public run43501438026 logs'},{kind:'observation',text:'Source.commit is workflow event commit43875b7; CPP instrument commit6925923 is the frozen continuation producer and both arithmetic source checkouts are pinnede3658b4. Exact original source/component pins match. Compiler/runtime/physical host and author independence are not proved; signed execution standing not established.',by:'public log event/source/host records'},{kind:'observation',text:'Timestamped complete public log is retained unchanged. Extracted files remove only timestamp prefixes and one explicit marker separator; extracted byte hashes identify reconstructed files, not remote-file attestation. Per-arm native-only start/end times were not logged; row times null. Rust start/end epochs cover compile plus native and are not used as duration.',by:'extract-hosted.mjs'}],reviews_dir:'reviews',backend_record:{run_id:run.id,job_id:jobs.jobs[0].id,run_attempt:1,run_url:run.html_url,event_sha:run.head_sha,source_checkout:host.git_sha,log_sha256:sha(raw),paid_spend:0,agent_uploads:0}};
write(path.join(study,'export.json'),exp);
write(path.join(evidence,'extraction.json'),{schema:'s3-host-log-extraction/v1',log_sha256:sha(raw),log_bytes:raw.length,unprefixed_lines_outside_data_blocks:unprefixed,cpp_parts:12,rust_parts:12,recorded_field_rows_per_arm:6,counts:counts('cpp'),cross_arm_fields_equal:true,binary_sha256:binaries,complete_public_log_retained:true,extracted_remote_bytes_attested:false,canonical_export:'../../export.json',signed_execution_standing:'not established'});
console.log(JSON.stringify({flow:exp.flow,counts:counts('cpp'),binaries,log_bytes:raw.length}));
