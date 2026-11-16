// Operational process transport, file hashes and actual exit receipt only.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const v='research/twist_routes_1_3_20261021/execution-v9',f=JSON.parse(fs.readFileSync(v+'/FROZEN.json'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
if(sha(f.binary.path)!==f.binary.sha256||sha(f.input.path)!==f.input.sha256)throw Error('frozen bytes changed');
const prefix=v+'/runs/producer-transport';if(fs.existsSync(prefix+'.json'))throw Error('fresh receipt required');fs.mkdirSync(v+'/runs',{recursive:true});
const argv=['-c','ulimit -c 0; ulimit -v 4194304; exec "$@"','twist-rank','/usr/bin/taskset','-c','0','/usr/bin/timeout','--signal=TERM','--kill-after=1s','300s',f.binary.path,f.input.path,v+'/runs/rank-v1'];
const started=new Date().toISOString(),child=spawn('/bin/sh',argv,{stdio:['ignore',fs.openSync(prefix+'.stdout','wx'),fs.openSync(prefix+'.stderr','wx')]});
const result=await new Promise((r,j)=>{child.on('error',j);child.on('close',(exit_code,signal)=>r({exit_code,signal}));});
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.rank-transport/v1',argv:['/bin/sh',...argv],started,finished:new Date().toISOString(),...result,binary_sha256:sha(f.binary.path),input_sha256:sha(f.input.path),stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),timeout_seconds:300,address_space_bytes:4294967296,cpu_affinity:[0],threads:1,wall_time_admitted:false},null,2)+'\n');if(result.exit_code!==0)process.exitCode=1;
