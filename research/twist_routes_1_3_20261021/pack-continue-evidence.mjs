// Filesystem inventory, hashes and deterministic archive transport only.
// No mathematical or performance results are computed here.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';

async function command(program,args,options={}) {
  return await new Promise((resolve,reject)=>{
    const child=spawn(program,args,{...options,stdio:['ignore','pipe','pipe']});
    let stdout='',stderr='';
    child.stdout.on('data',s=>stdout+=s);child.stderr.on('data',s=>stderr+=s);
    child.on('error',reject);
    child.on('close',(code,signal)=>code===0?resolve(stdout):reject(Error(JSON.stringify({program,args,code,signal,stderr}))));
  });
}

const study='research/twist_routes_1_3_20261021';
const execution=study+'/execution-v3';
const output=execution+'/continue-20261026';
const archiveRoot=process.argv[2];
if(!archiveRoot||fs.existsSync(output)||fs.existsSync(archiveRoot))throw Error('fresh manifest and archive directories required');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const walk=p=>fs.readdirSync(p,{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name,'en')).flatMap(e=>{
  const f=p+'/'+e.name;
  if(e.isSymbolicLink())throw Error('archive symlink forbidden: '+f);
  return e.isDirectory()?walk(f):e.isFile()?[f]:(()=>{throw Error('non-file: '+f)})();
});
const roots=fs.readdirSync(execution+'/runs').sort();
for(const name of ['sweep-20261026-v1','decomposition-20261026-v1','transfer-cost-20261026-v1','frontier-20261026-v1','control-20261026-v1','incidence-cost-20261026-v1'])if(!roots.includes(name))throw Error('missing new-wave run: '+name);
const files=roots.flatMap(n=>{const p=execution+'/runs/'+n;return fs.statSync(p).isDirectory()?walk(p):[p];}).sort();
const named=new Set(['SUMMARY.json','WORKLOADS.json','MATCHED_CONTROL_PROOFS.json','PANEL.json','ASSESSMENT.json','ADJUDICATION.json','COST.json','RECORDS.json','ROWS.json']);
const members=files.map(p=>({path:p.slice(execution.length+1),bytes:fs.statSync(p).size,sha256:sha(p),storage:named.has(path.basename(p))||!p.slice(execution.length+6).includes('/')?'loose':'archive'}));
const packed=members.filter(r=>r.storage==='archive');
const loose=members.filter(r=>r.storage==='loose');
for(const r of loose)if(r.bytes>10*1024*1024)throw Error('loose metadata exceeds Git limit: '+r.path);
fs.mkdirSync(output,{recursive:true});fs.mkdirSync(archiveRoot,{recursive:true});
const list=path.resolve(output+'/ARCHIVE_FILES.nul');
fs.writeFileSync(list,Buffer.from(packed.map(r=>r.path).join('\0')+'\0'));
const archive=path.resolve(archiveRoot+'/runs.tar.xz');
const packScript="set -euo pipefail\ntar --sort=name --mtime='2026-10-26 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu --no-recursion --null -T \"$TASK_E26_LIST\" -cf - | xz -6 -T1 > \"$TASK_E26_ARCHIVE\"";
await command('bash',['-c',packScript],{cwd:execution,env:{...process.env,TASK_E26_LIST:list,TASK_E26_ARCHIVE:archive}});
const extracted=path.resolve(archiveRoot+'/extracted');fs.mkdirSync(extracted);
await command('tar',['-xJf',archive,'-C',extracted]);
for(const r of packed){const p=extracted+'/'+r.path;if(fs.statSync(p).size!==r.bytes||sha(p)!==r.sha256)throw Error('extracted member mismatch: '+r.path);}
const repack=path.resolve(archiveRoot+'/repacked.tar.xz');
await command('bash',['-c',packScript],{cwd:extracted,env:{...process.env,TASK_E26_LIST:list,TASK_E26_ARCHIVE:repack}});
if(sha(archive)!==sha(repack))throw Error('non-deterministic repack');
fs.writeFileSync(output+'/RAW_MEMBERS.json',JSON.stringify({schema:'twist.raw-members/v1',members},null,2)+'\n');
fs.writeFileSync(output+'/LOOSE_METADATA.json',JSON.stringify({schema:'twist.loose-metadata/v1',members:loose},null,2)+'\n');
fs.writeFileSync(output+'/runs.tar.xz.sha256',sha(archive)+'  runs.tar.xz\n');
const revisions=(await command('git',['log','--format=%H','faf2e0dc72f530d6cad8f865bba5ac5ca163dceb..HEAD'])).trim().split('\n').reverse();
const manifest={schema:'twist.block-raw-publication/v1',created_utc:new Date().toISOString(),status:'deterministic-archive-prepared-locally',archive:{local_path:archive,sha256:sha(archive),bytes:fs.statSync(archive).size,format:'GNU tar and single-threaded xz -6',mtime:'2026-10-26 00:00:00Z',deterministic_repack_verified:true,extracted_tree_repack_sha256:sha(repack)},expected_store_name:'evidence/twist_routes_1_3_20261021/continue-20261026/runs.tar.xz',remote:'s3://setec-astronomy-artifacts/store',source_commits:revisions,roots,full_member_manifest:'RAW_MEMBERS.json',loose_metadata_manifest:'LOOSE_METADATA.json',archive_file_list:'ARCHIVE_FILES.nul',repack_recipe_source:study+'/pack-continue-evidence.mjs',archived_files:packed.length,loose_files:loose.length,remote_uploaded:false,store_index_entry_committed_locally:false,store_index_entry_pushed:false,original_committed_trees_repacked:false,scientific_results_computed:false,complete_cost_or_speedup_established:false,github_delivery:'existing canonical PR isogeny-labs/crypto#2380; delivery verified separately'};
fs.writeFileSync(output+'/RAW_ARTIFACTS.json',JSON.stringify(manifest,null,2)+'\n');
fs.writeFileSync(output+'/.gitignore','/runs.tar.xz\n/extracted/\n');
console.log(JSON.stringify({archive:manifest.archive,archived_files:packed.length,loose_files:loose.length}));
