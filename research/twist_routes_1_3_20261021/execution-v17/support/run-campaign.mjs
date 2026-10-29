// Operational fresh-only execution. All scientific work/analysis is native.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const d=process.argv[2];if(!d)throw Error('wave required');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const f=JSON.parse(fs.readFileSync(d+'/FROZEN.json'));
for(const[p,h]of Object.entries(f.files))if(sha(p)!==h)throw Error('frozen campaign pin mismatch '+p);
for(const x of[f.harness,f.summary])if(sha(x.path)!==x.sha256)throw Error('frozen executable changed');
if(fs.existsSync(d+'/runs'))throw Error('fresh campaign required');fs.mkdirSync(d+'/runs');
const prefix=d+'/runs/TRANSPORT',argv=[f.harness.path,'run',d+'/PLAN.json',d+'/runs/campaign-v1'];
const started=new Date().toISOString();const c=spawn(argv[0],argv.slice(1),{env:{...process.env,RAYON_NUM_THREADS:'1'},stdio:['ignore',fs.openSync(prefix+'.stdout','wx'),fs.openSync(prefix+'.stderr','wx')]});
const[exit_code,signal]=await new Promise((r,j)=>{c.on('error',j);c.on('close',(x,y)=>r([x,y]));});
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.cold-main-transport/v1',started,finished:new Date().toISOString(),argv,exit_code,signal,harness_sha256:f.harness.sha256,plan_sha256:sha(d+'/PLAN.json'),stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),wall_time_admitted:false},null,2)+'\n',{flag:'wx'});process.exitCode=exit_code===0?0:1;
