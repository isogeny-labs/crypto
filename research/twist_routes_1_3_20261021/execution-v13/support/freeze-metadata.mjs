// Operational source/binary/input identity metadata only.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v13';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const put=(p,x)=>fs.writeFileSync(p,JSON.stringify(x,null,2)+'\n',{flag:'wx'});
async function revision(){const c=spawn('git',['rev-parse','HEAD']);let out='';c.stdout.on('data',x=>out+=x);const n=await new Promise((r,j)=>{c.on('error',j);c.on('close',r);});if(n)throw Error('git failed');return out.trim();}
if(process.argv[2]==='pin'){
 const rev=await revision(),worker='/workspace/shared/twist-quotient-reference-native-20261028/'+rev+'/worker';fs.mkdirSync(worker.slice(0,worker.lastIndexOf('/')),{recursive:true});fs.copyFileSync(s+'/runner/target/release/twist-kernel-publication',worker,fs.constants.COPYFILE_EXCL);fs.chmodSync(worker,0o755);
 const p=JSON.parse(fs.readFileSync(v+'/protocol.json'));p.arms[1].instrument.commit=rev;p.analysis_plan.script.sha256=sha(p.analysis_plan.script.path);fs.writeFileSync(v+'/protocol.json',JSON.stringify(p,null,2)+'\n');
 put(v+'/BINARY_PIN.json',{source_revision:rev,binary:{path:worker,sha256:sha(worker)}});
}else if(process.argv[2]==='seal'){
 const pin=JSON.parse(fs.readFileSync(v+'/BINARY_PIN.json')),p=JSON.parse(fs.readFileSync(v+'/protocol.json'));if(p.freeze.status!=='frozen'||p.freeze.commit!==pin.source_revision)throw Error('protocol not frozen');
 const source_hashes={},input_hashes={};
 for(const x of ['runner/src/bin/twist-kernel-publication.rs','execution-v13/run-reference.mjs','execution-v13/REFERENCE_V2.gp','execution-v13/REFERENCE_CORRECTION.json','execution-v13/PROTOCOL.md'])source_hashes[s+'/'+x]=sha(s+'/'+x);
 for(const x of ['execution-v12/runs/quotient-v1/RESULTS.json','execution-v12/runs/quotient-v1/point-checks.jsonl','execution-v12/runs/quotient-v1/verify.gp','execution-v12/FROZEN.json'])input_hashes[s+'/'+x]=sha(s+'/'+x);
 const transport=s+'/runner/target/release/twist-pari-transport';
 put(v+'/FROZEN.json',{schema:'twist.quotient-reference-freeze/v2',created:new Date().toISOString(),...pin,reused_native:JSON.parse(fs.readFileSync(s+'/execution-v12/BINARY_PIN.json')),transport:{path:transport,sha256:sha(transport)},source_hashes,input_hashes,protocol_file_sha256:sha(v+'/protocol.json'),scientific_panel_started:false,wall_time_admitted:false});
}else throw Error('pin|seal');
