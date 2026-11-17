// Metadata-only plan validation and provenance. No research algorithms.
import {readFileSync,writeFileSync,readdirSync,mkdirSync,statSync,existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import assert from 'node:assert/strict';
const root='research/conductor_gap_replication_20261025';
const sha=p=>createHash('sha256').update(readFileSync(p)).digest('hex');
mkdirSync(root+'/plans',{recursive:true});
const panels=[['prepared-a',41856],['prepared-b-v2',1664],['calibration',128]];
const results=[];
for(const [panel,expected]of panels){
  const manifest=JSON.parse(readFileSync(root+'/'+panel+'/manifest.json','utf8'));
  let count=0;
  for(const s of manifest.specs){
    const raw=execFileSync('./target/release/ecbench',['plan','--spec',s.path,'--json'],{maxBuffer:32*1024*1024});
    const plan=JSON.parse(raw);
    assert(plan.workloads.every(w=>w.curve.registered),'all measured models must be registered');
    count+=plan.executions;
    const dest=root+'/plans/'+s.name+'.json';
    if(existsSync(dest))assert.equal(readFileSync(dest).compare(raw),0,'frozen plan unchanged');
    else writeFileSync(dest,raw,{flag:'wx'});
  }
  assert.equal(count,expected);
  results.push({panel,specs:manifest.specs.length,executions:count,all_registered:true});
}
const walk=p=>readdirSync(p).flatMap(n=>{const q=p+'/'+n;return statSync(q).isDirectory()?walk(q):[q]});
const files=[...walk(root).filter(p=>!p.includes('/cards/')&&!p.includes('/sessions/')),
  'examples/conductor_gap_replication.rs','src/bin/volcano_gap.rs',
  'src/cryptanalysis/ic_framework/cm_orbit.rs','docs/curves/registry.json',
  'src/cryptanalysis/curve_aliases.json','target/release/ecbench',
  'target/release/examples/conductor_gap_replication'].sort().map(path=>({path,sha256:sha(path)}));
const receipt={schema:'conductor-gap.preflight/v1',source_commit:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),
  compiler:execFileSync('/workspace/crypto-toolchain/cargo/bin/rustc',['--version'],{encoding:'utf8'}).trim(),
  panels:results,files};
writeFileSync(root+'/'+(process.argv[2]??'PREFLIGHT.json'),JSON.stringify(receipt,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({source_commit:receipt.source_commit,panels:results,bound_files:files.length}));
