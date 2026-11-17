// Operational execution/hash/process transport only; no mathematical work.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v15',mode=process.argv[2];
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const f=JSON.parse(fs.readFileSync(v+'/FROZEN.json'));
for(const [p,h]of Object.entries({...f.source_hashes,...f.input_hashes,[v+'/protocol.json']:f.protocol_file_sha256}))if(sha(p)!==h)throw Error('frozen byte mismatch: '+p);
for(const b of [f.binary,f.roi_library,f.gp])if(sha(b.path)!==b.sha256)throw Error('binary mismatch');
fs.mkdirSync(v+'/runs',{recursive:true});
let argv,prefix,script;
if(mode==='native'){
 if(fs.existsSync(v+'/runs/inputs-v1'))throw Error('fresh input producer output required');
 prefix=v+'/runs/producer-transport';
 argv=['--as=4294967296','--','/usr/bin/taskset','-c','0','/usr/bin/timeout','--signal=TERM','--kill-after=1s','600s',f.binary.path,'inputs',s+'/execution-v1/fixtures.json',v+'/runs/inputs-v1',f.roi_library.path];
}else if(mode==='pari'){
 if(fs.existsSync(v+'/runs/pari-v1'))throw Error('fresh GP output required');fs.mkdirSync(v+'/runs/pari-v1');prefix=v+'/runs/pari-v1/RECEIPT';script=v+'/runs/inputs-v1/verify.gp';
 argv=['--as=4294967296','--','/usr/bin/taskset','-c','0','/usr/bin/timeout','--signal=TERM','--kill-after=1s','600s',f.gp.path,'-f','-q','-s','64000000',script];
}else throw Error('native|pari');
for(const suffix of ['.json','.stdout','.stderr'])if(fs.existsSync(prefix+suffix))throw Error('fresh transport receipt required');
const started=new Date().toISOString();const c=spawn('/usr/bin/prlimit',argv,{env:{...process.env,LD_LIBRARY_PATH:'/workspace/shared/twist-tools-20261023/cas/root/usr/lib/x86_64-linux-gnu',RAYON_NUM_THREADS:'1'},stdio:['ignore',fs.openSync(prefix+'.stdout','wx'),fs.openSync(prefix+'.stderr','wx')]});
const [code,signal]=await new Promise((r,j)=>{c.on('error',j);c.on('close',(x,y)=>r([x,y]));});
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.cold-input-transport/v1',started,finished:new Date().toISOString(),argv:['/usr/bin/prlimit',...argv],exit_code:code,signal,native_binary_sha256:f.binary.sha256,gp_binary_sha256:f.gp.sha256,roi_library_sha256:f.roi_library.sha256,wrapper_sha256:sha(new URL(import.meta.url)),script_sha256:script?sha(script):null,stdout_sha256:sha(prefix+'.stdout'),stderr_sha256:sha(prefix+'.stderr'),cpu_affinity:[0],threads:1,address_space_cap_bytes:4294967296,timeout_s:600,wall_time_admitted:false},null,2)+'\n',{flag:'wx'});
process.exitCode=code===0?0:1;
