// Operational deterministic packaging and fresh-download/member-byte verification.
import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const[mode,dir,arg,target]=process.argv.slice(2),sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const put=(p,x)=>fs.writeFileSync(p,JSON.stringify(x,null,2)+'\n',{flag:'wx'});
async function run(cmd,args,opts={}){const c=spawn(cmd,args,opts);let e='';c.stderr?.on('data',x=>e+=x);const n=await new Promise((r,j)=>{c.on('error',j);c.on('close',r);});if(n)throw Error(cmd+' '+n+': '+e);}
function members(base,rel){const out=[];for(const e of fs.readdirSync(path.join(base,rel),{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name))){const p=path.join(rel,e.name),q=path.join(base,p);if(e.isDirectory())out.push(...members(base,p));else if(e.isFile())out.push({path:p,bytes:fs.statSync(q).size,sha256:sha(q)});else throw Error('unexpected raw symlink/special member');}return out;}
const packcmd="tar --sort=name --mtime='2026-10-30 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf -";
if(mode==='pack'){
 const roots=arg?arg.split(','):['runs'];if(roots.some(x=>!['runs','reference-v2/runs'].includes(x)))throw Error('undeclared root');
 const archive=dir+'/runs.tar.xz';if(fs.existsSync(archive))throw Error('fresh archive required');const rows=roots.flatMap(x=>members(dir,x));
 put(dir+'/RAW_MEMBERS.json',{schema:'twist.raw-members/v1',roots,members:rows});
 await run('/bin/bash',['-o','pipefail','-c',packcmd+' '+roots.join(' ')+' | xz -6 -T1 > runs.tar.xz'],{cwd:dir,stdio:['ignore','ignore','pipe']});
 fs.writeFileSync(archive+'.sha256',sha(archive)+'  runs.tar.xz\n',{flag:'wx'});
 put(dir+'/PACKING.json',{schema:'twist.raw-packing/v1',created:new Date().toISOString(),archive:{path:archive,sha256:sha(archive),bytes:fs.statSync(archive).size},manifest_sha256:sha(dir+'/RAW_MEMBERS.json'),members:rows.length,roots,deterministic_tar_mtime:'2026-10-30 00:00:00Z',compression:'xz -6 -T1',scientific_calculations:false});
}else if(mode==='verify'){
 if(!arg||!target||fs.existsSync(target))throw Error('fresh archive/extraction required');fs.mkdirSync(target,{recursive:true});await run('tar',['-xJf',path.resolve(arg),'-C',target]);
 const inventory=JSON.parse(fs.readFileSync(dir+'/RAW_MEMBERS.json')),rows=inventory.roots.flatMap(x=>members(target,x));
 if(JSON.stringify(rows)!==JSON.stringify(inventory.members))throw Error('raw member mismatch');
 await run('/bin/bash',['-o','pipefail','-c',packcmd+' '+inventory.roots.join(' ')+' | xz -6 -T1 > repacked.tar.xz'],{cwd:target,stdio:['ignore','ignore','pipe']});
 if(sha(arg)!==sha(target+'/repacked.tar.xz'))throw Error('repack bytes changed');
 put(dir+'/RAW_VERIFICATION.json',{schema:'twist.raw-archive-verification/v1',created:new Date().toISOString(),archive:arg,sha256:sha(arg),members_verified:rows.length,all_member_hashes_equal:true,deterministic_repack_sha256:sha(target+'/repacked.tar.xz'),fresh_remote_download:arg.includes('fresh-download'),status:'archive-and-all-members-and-repack-verified'});
}else throw Error('pack WAVE [roots] | verify WAVE FRESH_DOWNLOADED_ARCHIVE FRESH_EXTRACTION');
