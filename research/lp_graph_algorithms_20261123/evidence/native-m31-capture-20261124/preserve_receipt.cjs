#!/usr/bin/env node
// File/JSON metadata only. Mathematical results are copied from native Rust.
// Usage: node preserve_receipt.cjs FROZEN_DOWNLOAD_DIRECTORY NEW_OUTPUT_DIRECTORY
const fs = require('fs');
const path = require('path');
const cp = require('child_process');
const [inputArg, outputArg] = process.argv.slice(2);
if (!inputArg || !outputArg || process.argv.length !== 4) throw Error('expected two directories');
const input = path.resolve(inputArg), output = path.resolve(outputArg);
const qRoot = 'extracted/home/runner/work/_temp/prime31-group-admission-qualification/';
const mRoot = qRoot + 'm31-retained-capture/';
const cap = 65536;
if (fs.existsSync(path.join(output, 'raw'))) throw Error('output already populated');
fs.mkdirSync(output, {recursive:true});
function write(name, bytes) {
  const dest = path.join(output, name);
  fs.mkdirSync(path.dirname(dest), {recursive:true});
  fs.writeFileSync(dest, bytes, {flag:'wx'});
}
function json(name, value) { write(name, JSON.stringify(value,null,2)+'\n'); }
function readJSON(name) { return JSON.parse(fs.readFileSync(path.join(input,name),'utf8')); }
function sha(file) { return cp.execFileSync('sha256sum',['--',file],{encoding:'utf8'}).slice(0,64); }
function copy(from,to) { write(to,fs.readFileSync(path.join(input,from))); }
const inventory = fs.readFileSync(path.join(input,'raw-files.sha256'),'utf8').trimEnd().split('\n').map(line=>{
  const match=line.match(/^([0-9a-f]{64})  (.+)$/);
  if(!match) throw Error('malformed native artifact manifest');
  const relative=match[2], absolute=path.resolve(input,relative);
  if(!absolute.startsWith(input+path.sep)) throw Error('artifact path escapes input');
  const stat=fs.statSync(absolute);
  if(!stat.isFile()) throw Error('artifact is not a regular file');
  let group='other', short=relative;
  if(relative.startsWith(mRoot)){group='m31';short=relative.slice(mRoot.length);}
  else if(relative.startsWith(qRoot)){group='qualification';short=relative.slice(qRoot.length);}
  return {group,path:short,bytes:stat.size,sha256:match[1],original:relative};
});
if(inventory.length!==470) throw Error('unexpected artifact file count');
const keepQualification=new Set(['admission-test-run.stdout','admission-test-run.stderr',
  'admission-test-run.exit','qualification.exit','script.exit','checkout.txt',
  'source-before.sha256','source-after.sha256','executables-before.sha256','executables-after.sha256',
  'rustc-version.stdout','cargo-version.stdout']);
const copied=[], omitted=[];
for(const item of inventory){
  const selected=item.group==='m31'||(item.group==='qualification'&&keepQualification.has(item.path));
  if(selected&&item.bytes<=cap){
    const destination='raw/'+item.group+'/'+item.path;
    copy(item.original,destination);
    if(sha(path.join(output,destination))!==item.sha256)throw Error('copied byte identity failed');
    copied.push({path:destination,bytes:item.bytes,sha256:item.sha256});
  }else if(selected) omitted.push({group:item.group,path:item.path,bytes:item.bytes,sha256:item.sha256});
}
// A compact complete manifest uses group roots rather than repeating them.
// Entries are [group,relative path,bytes,sha256]; no raw payload is split.
const manifest={schema:'graph-ic.artifact-files/v1',
  roots:{m31:mRoot,qualification:qRoot,other:''},
  columns:['group','path','bytes','sha256'],
  files:inventory.map(x=>[x.group,x.path,x.bytes,x.sha256])};
const manifestBytes=JSON.stringify(manifest)+'\n';
if(Buffer.byteLength(manifestBytes)>cap)throw Error('compact manifest exceeds bound');
write('ARTIFACT_FILES.json',manifestBytes);
copy('receipt-summary.json','receipt.json');
copy('graph-prefix-summary.json','graph-prefix-summary.json');
const audit=readJSON(mRoot+'derived/conversion-audit.json');
const {point_tables,...auditWithoutTables}=audit;
json('conversion-audit-without-point-tables.json',{
  schema:'graph-ic.native-audit-excerpt/v1',
  original_artifact_path:mRoot+'derived/conversion-audit.json',
  original_sha256:sha(path.join(input,mRoot+'derived/conversion-audit.json')),
  transformation:'Copy all native audit fields except point_tables; no mathematical recomputation.',
  native_audit:auditWithoutTables,
  point_table_source_identity:point_tables.source_identity
});
const api=readJSON('api-metadata.json');
json('api-metadata.json',{repository:api.repository,
  run:Object.fromEntries(['id','head_sha','status','conclusion','created_at','updated_at','run_started_at','html_url','run_attempt','event','path'].map(k=>[k,api.run[k]])),
  jobs:api.jobs.jobs,artifacts:api.artifacts.artifacts});
const job=fs.readFileSync(path.join(input,'job.log'),'utf8');
json('job-test-result-excerpts.json',{
  schema:'graph-ic.raw-log-excerpts/v1',
  original_bytes:fs.statSync(path.join(input,'job.log')).size,
  original_sha256:sha(path.join(input,'job.log')),
  selection:'Exact lines containing test result:; line numbers are one-based metadata.',
  lines:job.split('\n').flatMap((text,i)=>text.includes('test result:')?[{line:i+1,text}]:[])
});
const archive=path.join(input,'graph-ic-prime31-45309117216.zip');
if(sha(archive)!=='fcc0a3f2946215a7bb247fc0747732352730fc03384419219f402d1b27428b6a')throw Error('archive mismatch');
json('PUBLICATION.json',{
  schema:'graph-ic.compact-publication/v1',
  tested_commit:'3ba271b1f7100bdfd06eafbac8af283ec9d55094',run_id:45309117216,job_id:138160138786,
  artifact_id:14882048762,archive_bytes:fs.statSync(archive).size,archive_sha256:sha(archive),
  artifact_url:'https://github.com/isogeny-labs/crypto/actions/runs/45309117216/artifacts/14882048762',
  expires_at:'2026-12-08T02:03:47Z',
  artifact_file_count:inventory.length,raw_file_copy_count:copied.length,
  raw_file_max_bytes:cap,copied,selected_large_files_omitted:omitted,
  full_log:{bytes:fs.statSync(path.join(input,'job.log')).size,sha256:sha(path.join(input,'job.log')),committed:false},
  original_raw_manifest:{bytes:fs.statSync(path.join(input,'raw-files.sha256')).size,sha256:sha(path.join(input,'raw-files.sha256')),committed:false},
  limits:['Full ZIP, executable binaries, full job log and oversized derived records remain outside Git; no durable S3 custody is claimed.',
    'All 470 extracted files are described in ARTIFACT_FILES.json. Existing upstream raw input remains bound to its original Git commit.',
    'Counts and ranks are native producer records with unsigned unverified standing. Presentation and metadata inspection do not run scientific verification.',
    'No timings are interpreted, no target uniqueness/recovery is performed, and no complete cold-cost comparison is claimed.'],
  standing:'unverified',scientific_verifier_run:false,signature_created:false
});
function walk(dir,p=''){return fs.readdirSync(dir,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(path.join(dir,e.name),p+e.name+'/'):[p+e.name]);}
const files=walk(output).filter(p=>p!=='SHA256SUMS').sort();
write('SHA256SUMS',files.map(p=>sha(path.join(output,p))+'  '+p+'\n').join(''));
process.stdout.write(JSON.stringify({artifactFiles:inventory.length,copiedRawFiles:copied.length,
  omittedSelectedLargeFiles:omitted.length,manifestBytes:Buffer.byteLength(manifestBytes),output})+'\n');
