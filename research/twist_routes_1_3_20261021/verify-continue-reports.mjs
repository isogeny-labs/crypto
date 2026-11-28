// Transport integrity only; the report values were independently audited in Rust.
import fs from 'node:fs';import path from 'node:path';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v3',base=v+'/continue-20261026';
const [archive,out]=process.argv.slice(2);if(!archive||!out||fs.existsSync(out))throw Error('archive and fresh output required');
const read=p=>JSON.parse(fs.readFileSync(p)),sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const meta=read(base+'/REPORT_ARTIFACTS.json'),members=read(base+'/REPORT_MEMBERS.json').members;
if(sha(archive)!==meta.archive.sha256||fs.statSync(archive).size!==meta.archive.bytes)throw Error('report archive mismatch');
for(const r of members)if(!r.path.startsWith(s+'/')||r.path.split('/').includes('..')||r.path.includes('\n'))throw Error('unsafe report path');
const command=async args=>await new Promise((resolve,reject)=>{const p=spawn('tar',args,{stdio:['ignore','pipe','pipe']});let a='',b='';p.stdout.on('data',s=>a+=s);p.stderr.on('data',s=>b+=s);p.on('error',reject);p.on('close',c=>c===0?resolve(a):reject(Error(b)));});
const names=(await command(['-tJf',archive])).trimEnd().split('\n').sort();if(JSON.stringify(names)!==JSON.stringify(members.map(r=>r.path).sort()))throw Error('report archive list differs');
fs.mkdirSync(out,{recursive:true});const extracted=path.resolve(out+'/extracted');fs.mkdirSync(extracted);await command(['-xJf',archive,'--no-same-owner','-C',extracted]);
for(const r of members){const p=extracted+'/'+r.path,stat=fs.lstatSync(p);if(!stat.isFile()||stat.size!==r.bytes||sha(p)!==r.sha256)throw Error('report member differs: '+r.path);}
fs.writeFileSync(out+'/.gitignore','/extracted/\n');fs.writeFileSync(out+'/ARCHIVE_CHECK.json',JSON.stringify({schema:'twist.report-archive-check/v1',status:'all-report-member-hashes-verified',checked_utc:new Date().toISOString(),archive_sha256:sha(archive),archive_bytes:fs.statSync(archive).size,members:members.length,scientific_results_recomputed:false,source_tree_remains_committed_loose:true},null,2)+'\n');console.log('ALL_REPORT_MEMBERS_VERIFIED');
