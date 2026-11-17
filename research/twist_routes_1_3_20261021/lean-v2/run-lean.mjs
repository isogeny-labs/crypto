// Process transport and resource receipts only; all proofs run in Lean.
import fs from 'node:fs';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
const [prefix,cwd,leanPath,...args]=process.argv.slice(2);
if(!prefix||!cwd||!leanPath||!args.length)throw Error('prefix cwd LEAN_PATH arguments required');
for(const s of ['json','stdout','stderr'])if(fs.existsSync(prefix+'.'+s))throw Error('fresh receipt required');
fs.mkdirSync(prefix.slice(0,prefix.lastIndexOf('/')),{recursive:true});
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const started=new Date().toISOString();
const native=args[0]==='--native';
const program=native?args[1]:'/tmp/twist-lean/bin/lean';
const argv=['-c','0',program,...(native?args.slice(2):['-j1','-M3072',...args])];
const child=spawn('/usr/bin/taskset',argv,{cwd,env:{...process.env,PATH:'/tmp/twist-lean/bin:'+process.env.PATH,LEAN_PATH:leanPath,MIMALLOC_ARENA_RESERVE:'65536'},stdio:['ignore',fs.openSync(prefix+'.stdout','wx'),fs.openSync(prefix+'.stderr','wx')]});
let peak=0,limit=null;
const poll=setInterval(()=>{
 try{const s=fs.readFileSync('/proc/'+child.pid+'/status','utf8');const m=s.match(/^VmRSS:\s+(\d+)/m);if(m){peak=Math.max(peak,Number(m[1]));if(peak>4194304){limit='sampled-resident-memory';child.kill('SIGKILL');}}}catch{}
},25);
const deadline=setTimeout(()=>{limit='600-second-deadline';child.kill('SIGKILL');},600000);
const [code,signal]=await new Promise((resolve,reject)=>{child.on('error',reject);child.on('exit',(c,s)=>resolve([c,s]));});
clearInterval(poll);clearTimeout(deadline);
const source=args.find(a=>a.endsWith('.lean'));
fs.writeFileSync(prefix+'.json',JSON.stringify({schema:'twist.lean-shard-transport/v2',cwd,argv,started_utc:started,finished_utc:new Date().toISOString(),exit_code:code,signal,resource_limit_fired:limit,peak_sampled_rss_kib:peak,rss_sampling_ms:25,resident_guard_kib:4194304,lean_managed_memory_mib:native?null:3072,timeout_s:600,cpu_affinity:[0],threads:1,virtual_address_limit:false,wrapper_sha256:hash(new URL(import.meta.url)),lean_sha256:hash('/tmp/twist-lean/bin/lean'),program_sha256:hash(program),native_compiled_auditor:native,source_path:source,source_sha256:source?hash(source.startsWith('/')?source:cwd+'/'+source):null,stdout_sha256:hash(prefix+'.stdout'),stderr_sha256:hash(prefix+'.stderr'),performance_claim:false},null,2)+'\n');
process.exitCode=code===0&&!limit?0:1;
