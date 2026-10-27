// Operational file packaging and host metadata; no research aggregation.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
const v='research/twist_routes_1_3_20261021/execution-v4';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write=(p,x)=>fs.writeFileSync(p,JSON.stringify(x,null,2)+'\n',{flag:'wx'});
async function run(cmd,args,options={}){
  const child=spawn(cmd,args,options);let out='',err='';
  child.stdout?.on('data',x=>out+=x);child.stderr?.on('data',x=>err+=x);
  const code=await new Promise((resolve,reject)=>{child.on('error',reject);child.on('close',resolve);});
  if(code!==0)throw Error(`${cmd} exit ${code}: ${err}`);return out.trim();
}
function members(root,rel='runs'){
  const rows=[];
  for(const ent of fs.readdirSync(path.join(root,rel),{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name))){
    const p=path.join(rel,ent.name),abs=path.join(root,p);
    if(ent.isSymbolicLink())throw Error('unexpected raw symlink');
    if(ent.isDirectory())rows.push(...members(root,p));
    else if(ent.isFile())rows.push({path:p,bytes:fs.statSync(abs).size,sha256:sha(abs)});
    else throw Error('unexpected raw member');
  }
  return rows;
}
const mode=process.argv[2];
if(mode==='host'){
  const lscpu=JSON.parse(await run('lscpu',['-J']));
  const get=k=>lscpu.lscpu.find(x=>x.field===k)?.data??null;
  const mem=Number(fs.readFileSync('/proc/meminfo','utf8').match(/^MemTotal:\s+(\d+) kB/m)[1])*1024;
  const rust=await run('/workspace/shared/twist-tools-20261023/cargo/bin/rustc',['+1.90.0','-Vv'],{env:{...process.env,RUSTUP_HOME:'/workspace/shared/twist-tools-20261023/rustup'}});
  write(v+'/HOST.json',{schema:'twist.host/v1',captured:new Date().toISOString(),capture_after_run:true,
    summary:{cpu_model:get('Model name:'),logical_cpus:Number(get('CPU(s):')),memory_bytes:mem,kernel:await run('uname',['-srvmo']),hypervisor:get('Hypervisor vendor:'),isolation_tier_max:'L0'},
    lscpu,rust,threads:1,cargo_profile:'release',build_jobs:1,scientific_wall_time_admitted:false,
    limits:{address_space_bytes:4294967296,worker_soft_seconds:2,worker_hard_seconds:4,kill_grace_seconds:1,per_file_bytes:33554432},
    note:'Same managed environment as execution. Host metadata captured after the run; no claim of native-time isolation.'});
}else if(mode==='pack'){
  const archive=v+'/runs.tar.xz';if(fs.existsSync(archive))throw Error('fresh archive required');
  const rows=members(v);write(v+'/RAW_MEMBERS.json',{schema:'twist.raw-members/v1',root:'runs',members:rows});
  await run('/bin/bash',['-o','pipefail','-c',"tar --sort=name --mtime='2026-10-27 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > runs.tar.xz"],{cwd:v});
  fs.writeFileSync(archive+'.sha256',sha(archive)+'  runs.tar.xz\n',{flag:'wx'});
  write(v+'/PACKING.json',{schema:'twist.raw-packing/v1',created:new Date().toISOString(),archive:{path:archive,sha256:sha(archive),bytes:fs.statSync(archive).size},manifest_sha256:sha(v+'/RAW_MEMBERS.json'),members:rows.length,deterministic_tar_mtime:'2026-10-27 00:00:00Z',compression:'xz -6 -T1',scientific_calculations:false});
}else if(mode==='verify'){
  const archive=process.argv[3],target=process.argv[4];if(!archive||!target||fs.existsSync(target))throw Error('fresh extraction directory required');
  fs.mkdirSync(target,{recursive:true});await run('tar',['-xJf',path.resolve(archive),'-C',target]);
  const expected=JSON.parse(fs.readFileSync(v+'/RAW_MEMBERS.json')).members,actual=members(target);
  if(JSON.stringify(actual)!==JSON.stringify(expected))throw Error('member inventory/hash mismatch');
  await run('/bin/bash',['-o','pipefail','-c',"tar --sort=name --mtime='2026-10-27 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > repacked.tar.xz"],{cwd:target});
  if(sha(archive)!==sha(target+'/repacked.tar.xz'))throw Error('repacked bytes mismatch');
  write(v+'/RAW_VERIFICATION.json',{schema:'twist.raw-archive-verification/v1',created:new Date().toISOString(),archive:archive,sha256:sha(archive),bytes:fs.statSync(archive).size,members_verified:actual.length,all_member_hashes_equal:true,deterministic_repack_sha256:sha(target+'/repacked.tar.xz'),fresh_remote_download:archive.includes('fresh-download'),status:'archive-and-all-members-and-repack-verified'});
}else throw Error('usage: package-evidence.mjs host|pack|verify [archive fresh-extraction]');
