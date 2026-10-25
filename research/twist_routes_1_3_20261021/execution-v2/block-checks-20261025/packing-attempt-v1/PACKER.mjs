// Filesystem inventory, hashes and deterministic archive transport only.
// No mathematical or performance results are computed here.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {execFileSync} from 'node:child_process';

const study='research/twist_routes_1_3_20261021';
const execution=study+'/execution-v2';
const output=execution+'/block-20261025';
const archiveRoot=process.argv[2];
if(!archiveRoot||fs.existsSync(output)||fs.existsSync(archiveRoot))throw Error('fresh manifest and archive directories required');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const walk=p=>fs.readdirSync(p,{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name,'en')).flatMap(e=>{
  const f=p+'/'+e.name;
  if(e.isSymbolicLink())throw Error('archive symlink forbidden: '+f);
  return e.isDirectory()?walk(f):e.isFile()?[f]:(()=>{throw Error('non-file: '+f)})();
});
const roots=fs.readdirSync(execution+'/runs').filter(n=>n.startsWith('block-20261025-NONE')||/^block-(inputs|matched|panel|worker-cost|batch-cost|cas-adjudication|rank-adjudication|gp|rank-gp)-20261025-/.test(n)).sort();
if(!roots.includes('block-inputs-20261025-v1')||!roots.includes('block-panel-20261025-v2')||!roots.includes('block-batch-cost-20261025-v1'))throw Error('incomplete E26 roots');
const files=roots.flatMap(n=>{const p=execution+'/runs/'+n;return fs.statSync(p).isDirectory()?walk(p):[p];}).sort();
const named=new Set(['SUMMARY.json','WORKLOADS.json','MATCHED_CONTROL_PROOFS.json','PANEL.json','ASSESSMENT.json','ADJUDICATION.json','COST.json','RECORDS.json']);
const members=files.map(p=>({path:p.slice(execution.length+1),bytes:fs.statSync(p).size,sha256:sha(p),storage:named.has(path.basename(p))||!p.slice(execution.length+6).includes('/')?'loose':'archive'}));
const packed=members.filter(r=>r.storage==='archive');
const loose=members.filter(r=>r.storage==='loose');
for(const r of loose)if(r.bytes>10*1024*1024)throw Error('loose metadata exceeds Git limit: '+r.path);
fs.mkdirSync(output,{recursive:true});fs.mkdirSync(archiveRoot,{recursive:true});
const list=path.resolve(output+'/ARCHIVE_FILES.nul');
fs.writeFileSync(list,Buffer.from(packed.map(r=>r.path).join('\0')+'\0'));
const archive=path.resolve(archiveRoot+'/runs.tar.xz');
const packScript="set -euo pipefail\ntar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu --no-recursion --null -T \"$TASK_E26_LIST\" -cf - | xz -6 -T1 > \"$TASK_E26_ARCHIVE\"";
execFileSync('bash',['-c',packScript],{cwd:execution,env:{...process.env,TASK_E26_LIST:list,TASK_E26_ARCHIVE:archive},stdio:'inherit'});
const extracted=path.resolve(archiveRoot+'/extracted');fs.mkdirSync(extracted);
execFileSync('tar',['-xJf',archive,'-C',extracted]);
for(const r of packed){const p=extracted+'/'+r.path;if(fs.statSync(p).size!==r.bytes||sha(p)!==r.sha256)throw Error('extracted member mismatch: '+r.path);}
const repack=path.resolve(archiveRoot+'/repacked.tar.xz');
execFileSync('bash',['-c',packScript],{cwd:extracted,env:{...process.env,TASK_E26_LIST:list,TASK_E26_ARCHIVE:repack},stdio:'inherit'});
if(sha(archive)!==sha(repack))throw Error('non-deterministic repack');
fs.writeFileSync(output+'/RAW_MEMBERS.json',JSON.stringify({schema:'twist.raw-members/v1',members},null,2)+'\n');
fs.writeFileSync(output+'/LOOSE_METADATA.json',JSON.stringify({schema:'twist.loose-metadata/v1',members:loose},null,2)+'\n');
fs.writeFileSync(output+'/runs.tar.xz.sha256',sha(archive)+'  runs.tar.xz\n');
const revisions=execFileSync('git',['log','--format=%H','88a27612152e9fdd74c1ed0a17f964442c4c3159..HEAD'],{encoding:'utf8'}).trim().split('\n').reverse();
const manifest={schema:'twist.block-raw-publication/v1',created_utc:new Date().toISOString(),status:'deterministic-archive-prepared-locally',archive:{local_path:archive,sha256:sha(archive),bytes:fs.statSync(archive).size,format:'GNU tar and single-threaded xz -6',mtime:'2026-10-25 00:00:00Z',deterministic_repack_verified:true,extracted_tree_repack_sha256:sha(repack)},expected_store_name:'evidence/twist_routes_1_3_20261021/block-20261025/runs.tar.xz',remote:'s3://setec-astronomy-artifacts/store',source_commits:revisions,roots,full_member_manifest:'RAW_MEMBERS.json',loose_metadata_manifest:'LOOSE_METADATA.json',archive_file_list:'ARCHIVE_FILES.nul',repack_recipe_source:study+'/pack-block-evidence.mjs',archived_files:packed.length,loose_files:loose.length,remote_uploaded:false,store_index_entry_committed_locally:false,store_index_entry_pushed:false,original_committed_trees_repacked:false,scientific_results_computed:false,complete_cost_or_speedup_established:false,github_delivery:'blocked: HTTP 403 Git / HTTP 404 connector'};
fs.writeFileSync(output+'/RAW_ARTIFACTS.json',JSON.stringify(manifest,null,2)+'\n');
fs.writeFileSync(output+'/.gitignore','/runs.tar.xz\n/extracted/\n');
console.log(JSON.stringify({archive:manifest.archive,archived_files:packed.length,loose_files:loose.length}));
