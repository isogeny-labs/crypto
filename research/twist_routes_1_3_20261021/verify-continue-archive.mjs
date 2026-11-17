// Independent archive/member hashes and loose-record restoration only.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
const study='research/twist_routes_1_3_20261021',execution=study+'/execution-v3';
const [archive,output]=process.argv.slice(2);
if(!archive||!output||fs.existsSync(output))throw Error('archive and fresh output directory required');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const expected=JSON.parse(fs.readFileSync(execution+'/continue-20261026/RAW_ARTIFACTS.json','utf8'));
const inventory=JSON.parse(fs.readFileSync(execution+'/continue-20261026/RAW_MEMBERS.json','utf8')).members;
if(sha(archive)!==expected.archive.sha256||fs.statSync(archive).size!==expected.archive.bytes)throw Error('archive digest/size mismatch');
for(const r of inventory)if(!/^runs\/[A-Za-z0-9_.\/-]+$/.test(r.path)||r.path.split('/').includes('..'))throw Error('unsafe member path');
async function command(args){return await new Promise((resolve,reject)=>{const child=spawn('tar',args,{stdio:['ignore','pipe','pipe']});let out='',err='';child.stdout.on('data',s=>out+=s);child.stderr.on('data',s=>err+=s);child.on('error',reject);child.on('close',code=>code===0?resolve(out):reject(Error(err)));});}
const listed=(await command(['-tJf',archive])).trimEnd().split('\n').sort();
const names=inventory.filter(r=>r.storage==='archive').map(r=>r.path).sort();
if(JSON.stringify(names)!==JSON.stringify(listed))throw Error('archive member list mismatch');
fs.mkdirSync(output,{recursive:true});const extracted=path.resolve(output+'/extracted');fs.mkdirSync(extracted);
await command(['-xJf',archive,'--no-same-owner','-C',extracted]);
for(const r of inventory){
  const p=extracted+'/'+r.path;
  if(r.storage==='loose'){fs.mkdirSync(path.dirname(p),{recursive:true});fs.copyFileSync(execution+'/'+r.path,p);}
  const stat=fs.lstatSync(p);if(!stat.isFile()||stat.size!==r.bytes||sha(p)!==r.sha256)throw Error('member mismatch: '+r.path);
}
fs.writeFileSync(output+'/.gitignore','/extracted/\n');
fs.writeFileSync(output+'/ARCHIVE_CHECK.json',JSON.stringify({schema:'twist.block-archive-check/v1',status:'all-archived-and-loose-member-hashes-verified',checked_utc:new Date().toISOString(),archive_sha256:sha(archive),archive_bytes:fs.statSync(archive).size,members:inventory.length,archived_files:names.length,loose_files:inventory.filter(r=>r.storage==='loose').length,scientific_results_recomputed:false,remote_proven_by_this_check:false},null,2)+'\n');
console.log('ALL_CONTINUE_ARCHIVE_MEMBERS_VERIFIED');
