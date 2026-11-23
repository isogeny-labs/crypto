// Operational process transport only. All research calculations run in Rust.
import fs from 'node:fs';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
const v='research/twist_routes_1_3_20261021/execution-v12';
const f=JSON.parse(fs.readFileSync(v+'/FROZEN.json'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
for(const [p,s]of Object.entries({...f.source_hashes,...f.input_hashes}))if(sha(p)!==s)throw Error('frozen file changed: '+p);
if(sha(f.binary.path)!==f.binary.sha256||sha(v+'/protocol.json')!==f.protocol_file_sha256)throw Error('frozen binary/protocol changed');
const prefix=v+'/runs/producer-transport';
if(fs.existsSync(prefix+'.json')||fs.existsSync(prefix+'.stdout'))throw Error('fresh transport required');
fs.mkdirSync(v+'/runs',{recursive:true});
const argv=['-c','ulimit -c 0; ulimit -v 4194304; ulimit -f 131072; exec "$@"','twist-kernels','/usr/bin/taskset','-c','0','/usr/bin/timeout','--signal=TERM','--kill-after=1s','300s',f.binary.path,v+'/runs/quotient-v1','research/twist_routes_1_3_20261021/execution-v8/runs/kernel-v1/RESULTS.json','research/twist_routes_1_3_20261021/execution-v10/ADMISSION.json','research/twist_routes_1_3_20261021/execution-v11/ADMISSION.json'];
const started=new Date().toISOString();
const child=spawn('/bin/sh',argv,{env:{...process.env,RAYON_NUM_THREADS:'1',OMP_NUM_THREADS:'1'},stdio:['ignore',fs.openSync(prefix+'.stdout','wx'),fs.openSync(prefix+'.stderr','wx')]});
const result=await new Promise((r,j)=>{child.on('error',j);child.on('close',(exit_code,signal)=>r({exit_code,signal}));});
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.kernel-transport/v1',argv:['/bin/sh',...argv],started,finished:new Date().toISOString(),...result,binary_sha256:sha(f.binary.path),stdout:prefix+'.stdout',stderr:prefix+'.stderr',stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),timeout_seconds:300,address_space_bytes:4294967296,cpu_affinity:[0],threads:1,wall_time_admitted:false},null,2)+'\n');
if(result.exit_code!==0)process.exitCode=1;
