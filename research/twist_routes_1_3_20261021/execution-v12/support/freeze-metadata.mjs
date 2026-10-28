// Operational hash/protocol metadata; no research computation.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v12';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const put=(p,x)=>fs.writeFileSync(p,JSON.stringify(x,null,2)+'\n',{flag:'wx'});
async function revision(){const c=spawn('git',['rev-parse','HEAD']);let out='';c.stdout.on('data',x=>out+=x);const n=await new Promise((r,j)=>{c.on('error',j);c.on('close',r);});if(n)throw Error('git revision failed');return out.trim();}
if(process.argv[2]==='pin'){
 const rev=await revision(),worker='/workspace/shared/twist-quotient-cycle-native-20261028/'+rev+'/worker';fs.mkdirSync(worker.slice(0,worker.lastIndexOf('/')),{recursive:true});fs.copyFileSync(s+'/runner/target/release/twist-quotient-cycle',worker,fs.constants.COPYFILE_EXCL);fs.chmodSync(worker,0o755);
 const p=JSON.parse(fs.readFileSync(v+'/protocol.json'));for(const a of p.arms)a.instrument.commit=rev;p.arms[0].instrument.binary_sha256=sha(worker);p.analysis_plan.script.sha256=sha(p.analysis_plan.script.path);fs.writeFileSync(v+'/protocol.json',JSON.stringify(p,null,2)+'\n');
 put(v+'/BINARY_PIN.json',{source_revision:rev,binary:{path:worker,sha256:sha(worker)}});
 const core=fs.readFileSync(s+'/runner/src/bin/twist-kernel-relations.rs','utf8').split('fn produce(')[0];if(!fs.readFileSync(s+'/runner/src/bin/twist-quotient-cycle.rs','utf8').startsWith(core))throw Error('prefix mismatch');
 put(v+'/CORE_SNAPSHOT.json',{schema:'twist.core-snapshot/v1',source:s+'/runner/src/bin/twist-kernel-relations.rs',target:s+'/runner/src/bin/twist-quotient-cycle.rs',prefix_bytes:Buffer.byteLength(core),prefix_sha256:crypto.createHash('sha256').update(core).digest('hex'),byte_equal:true});
}else if(process.argv[2]==='seal'){
 const pin=JSON.parse(fs.readFileSync(v+'/BINARY_PIN.json')),p=JSON.parse(fs.readFileSync(v+'/protocol.json'));if(p.freeze.status!=='frozen'||p.freeze.commit!==pin.source_revision)throw Error('protocol not frozen');
 const source_hashes={},input_hashes={};
 for(const x of ['runner/src/bin/twist-quotient-cycle.rs','runner/src/quotient_cycle_20261028.rs','runner/src/registry.rs','runner/Cargo.toml','runner/Cargo.lock','execution-v12/run-producer.mjs','execution-v12/PROTOCOL.md','lean-v8/TwistQuotientCycle.lean'])source_hashes[s+'/'+x]=sha(s+'/'+x);
 for(const x of ['src/cryptanalysis/twist_ic_field.rs','src/hash/sha256.rs'])source_hashes[x]=sha(x);
 for(const x of ['execution-v8/runs/kernel-v1/RESULTS.json','execution-v10/ADMISSION.json','execution-v11/ADMISSION.json'])input_hashes[s+'/'+x]=sha(s+'/'+x);
 put(v+'/FROZEN.json',{schema:'twist.quotient-cycle-freeze/v1',created:new Date().toISOString(),...pin,source_hashes,input_hashes,protocol_file_sha256:sha(v+'/protocol.json'),scientific_panel_started:false,wall_time_admitted:false});
}else throw Error('pin|seal');
