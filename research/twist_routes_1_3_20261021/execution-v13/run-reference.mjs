// Operational process transport only. GP is the independent exact verifier.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const v='research/twist_routes_1_3_20261021/execution-v13',f=JSON.parse(fs.readFileSync(v+'/FROZEN.json'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
for(const [p,h]of Object.entries({...f.source_hashes,...f.input_hashes}))if(sha(p)!==h)throw Error('frozen file changed: '+p);
for(const b of [f.binary,f.transport,f.reused_native.binary])if(sha(b.path)!==b.sha256)throw Error('frozen binary changed');
if(sha(v+'/protocol.json')!==f.protocol_file_sha256)throw Error('protocol changed');
const prefix=v+'/runs/reference-transport';for(const x of ['json','stdout','stderr'])if(fs.existsSync(prefix+'.'+x))throw Error('fresh receipt required');fs.mkdirSync(v+'/runs',{recursive:true});
const argv=['-c','ulimit -c 0; ulimit -v 4194304; exec "$@"','quotient-reference','/usr/bin/taskset','-c','0',f.transport.path,v+'/REFERENCE_V2.gp',v+'/runs/pari-v2'];
const started=new Date().toISOString(),child=spawn('/bin/sh',argv,{env:{...process.env,RAYON_NUM_THREADS:'1',OMP_NUM_THREADS:'1'},stdio:['ignore',fs.openSync(prefix+'.stdout','wx'),fs.openSync(prefix+'.stderr','wx')]});
const result=await new Promise((r,j)=>{child.on('error',j);child.on('close',(exit_code,signal)=>r({exit_code,signal}));});
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.reference-transport/v1',argv:['/bin/sh',...argv],started,finished:new Date().toISOString(),...result,script_sha256:sha(v+'/REFERENCE_V2.gp'),stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),timeout_seconds:300,address_space_bytes:4294967296,cpu_affinity:[0],threads:1,wall_time_admitted:false},null,2)+'\n');if(result.exit_code!==0)process.exitCode=1;
