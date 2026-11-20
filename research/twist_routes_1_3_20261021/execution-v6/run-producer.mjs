// Operational process/receipt transport only; all research arithmetic is native.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const v='research/twist_routes_1_3_20261021/execution-v6';
const frozen=JSON.parse(fs.readFileSync(v+'/FROZEN.json'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const binary=frozen.binary.path,input=frozen.input.path;
if(sha(binary)!==frozen.binary.sha256||sha(input)!==frozen.input.sha256)throw Error('frozen binary/input changed');
const prefix=v+'/runs/producer-transport';if(fs.existsSync(prefix+'.json')||fs.existsSync(prefix+'.stdout'))throw Error('fresh transport required');fs.mkdirSync(v+'/runs',{recursive:true});
const argv=['-c','ulimit -c 0; ulimit -v 4194304; ulimit -f 131072; exec "$@"','generator-census','/usr/bin/timeout','--signal=TERM','--kill-after=1s','600s',binary,'produce',input,v+'/runs/generator-v1'];
const started=new Date().toISOString();const child=spawn('/bin/sh',argv,{env:{...process.env,RAYON_NUM_THREADS:'1',OMP_NUM_THREADS:'1'},stdio:['ignore',fs.openSync(prefix+'.stdout','wx'),fs.openSync(prefix+'.stderr','wx')]});const result=await new Promise((r,j)=>{child.on('error',j);child.on('close',(exit_code,signal)=>r({exit_code,signal}));});
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.exact-generator-transport/v1',argv:['/bin/sh',...argv],started,finished:new Date().toISOString(),...result,binary_sha256:sha(binary),input_sha256:sha(input),stdout:prefix+'.stdout',stderr:prefix+'.stderr',stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),environment_overrides:{RAYON_NUM_THREADS:'1',OMP_NUM_THREADS:'1'},timeout_seconds:600,address_space_bytes:4294967296,file_limit_bytes:67108864,wall_time_admitted:false},null,2)+'\n');if(result.exit_code!==0)process.exitCode=1;
