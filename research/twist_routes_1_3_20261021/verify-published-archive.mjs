// Archive transport integrity only; no scientific calculations.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
const study='research/twist_routes_1_3_20261021',execution=study+'/execution-v2';
const [root,output]=process.argv.slice(2);
if(!root||!output||fs.existsSync(output))throw Error('extracted root and fresh receipt path required');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const manifest=execution+'/RAW_MEMBERS_FINAL.json',looseManifest=execution+'/LOOSE_METADATA.json';
const rows=read(manifest),loose=new Set(read(looseManifest).map(p=>path.relative(execution,p)));
let archived=0,restored=0,totalBytes=0;
const names=new Set(rows.map(r=>r.archive_member));
if(names.size!==rows.length)throw Error('duplicate full-manifest member');
function files(dir){return fs.readdirSync(dir,{withFileTypes:true}).flatMap(entry=>{
  const p=path.join(dir,entry.name);
  if(entry.isDirectory())return files(p);
  if(!entry.isFile())throw Error('nonregular extracted archive entry');
  return [path.relative(root,p)];
});}
const archiveNames=files(root);
for(const member of archiveNames)if(!names.has(member)||loose.has(member))throw Error('unexpected archive member: '+member);
for(const row of rows){
  const member=row.archive_member;
  if(member.startsWith('/')||member.split('/').some(p=>p==='..'||p===''))throw Error('unsafe member');
  const destination=path.join(root,member);
  if(loose.has(member)){
    if(fs.existsSync(destination))throw Error('loose metadata unexpectedly in archive');
    const source=path.join(execution,member);
    if(sha(source)!==row.sha256||fs.statSync(source).size!==row.bytes)throw Error('loose metadata mismatch');
    fs.mkdirSync(path.dirname(destination),{recursive:true});fs.copyFileSync(source,destination);restored++;
  }else archived++;
  if(!fs.lstatSync(destination).isFile()||sha(destination)!==row.sha256||fs.statSync(destination).size!==row.bytes)throw Error('restored member mismatch: '+member);
  totalBytes+=row.bytes;
}
if(restored!==loose.size||archiveNames.length!==archived||files(root).length!==rows.length)throw Error('member inventory mismatch');
const receipt={schema:'twist.downloaded-archive-check/v1',verified_utc:new Date().toISOString(),
  extracted_root:root,manifest_sha256:sha(manifest),loose_manifest_sha256:sha(looseManifest),
  archived_members_verified:archived,loose_metadata_restored_and_verified:restored,
  complete_tree_members_verified:rows.length,complete_tree_bytes:totalBytes,unexpected_members:0,
  status:'all-original-member-bytes-verified',scientific_replay_performed:false,
  verifier_sha256:sha(new URL(import.meta.url))};
fs.writeFileSync(output,JSON.stringify(receipt,null,2)+'\n');console.log(JSON.stringify(receipt));
