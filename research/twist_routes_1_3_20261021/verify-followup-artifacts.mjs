// Static metadata, file hashes and local links. No mathematical/statistical work.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
const study='research/twist_routes_1_3_20261021',ex=study+'/execution-v2';
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const output=process.argv[2];if(!output||fs.existsSync(output))throw Error('fresh check output required');
const tracked=await new Promise((resolve,reject)=>{let data='';const p=spawn('git',['ls-files','-z'],{stdio:['ignore','pipe','inherit']});p.stdout.on('data',c=>data+=c);p.on('error',reject);p.on('close',code=>code===0?resolve(new Set(data.split('\0'))):reject(Error('git list failed')));});
const cards=fs.readdirSync('backlog').filter(s=>/^XP-2026102[123]-twist-/.test(s));
const mds=[study+'/README.md',ex+'/README.md','problems/twist-followup-evidence-publication.md','problems/twist-trace-constraint-and-solver-gap.md','research/notes/index-calculus/RESEARCH_TWIST_IC_ROUTES_1_3.md',...cards.map(s=>'backlog/'+s+'/README.md'),...fs.readdirSync(study+'/reports/followup-20261023').filter(s=>s.endsWith('.md')).map(s=>study+'/reports/followup-20261023/'+s)];
let linkCount=0;
for(const p of mds){const text=fs.readFileSync(p,'utf8');for(const match of text.matchAll(/\]\(([^)]+)\)/g)){const href=match[1].split('#')[0];if(!href||/^[a-z]+:/i.test(href))continue;const full=path.normalize(path.join(path.dirname(p),decodeURIComponent(href)));if(!fs.existsSync(full)&&!tracked.has(full))throw Error(`missing local link ${p}: ${href}`);linkCount++;}}
for(const id of cards){const c=read('backlog/'+id+'/card.json');if(c.status!=='blocked'||c.cairn.funding.units!==0)throw Error('card state/funding changed');}
const meta=read(ex+'/RAW_ARTIFACTS.json');
if(meta.remote_uploaded||meta.store_name_resolvable||meta.store_index_entry_committed)throw Error('publication gap must stay explicit until a real upload');
if(sha(meta.archive.local_path)!==meta.archive.sha256||fs.statSync(meta.archive.local_path).size!==meta.archive.bytes)throw Error('prepared bundle mismatch');
const raw=read(ex+'/RAW_MEMBERS_FINAL.json');
for(const r of raw){const p=ex+'/'+r.archive_member;if(fs.statSync(p).size!==r.bytes||sha(p)!==r.sha256)throw Error('raw member mismatch '+p);}
const loose=read(ex+'/LOOSE_METADATA.json');
for(const p of loose)if(!fs.existsSync(p))throw Error('missing live metadata '+p);
const a=read(ex+'/assessment-20261023-v3/ASSESSMENT.json');
if(a.full_campaign_complete||a.performance_claim||a.novelty_established||a.NIST_DLP_attempted)throw Error('unsupported assessment promotion');
for(const r of a.results)if(!['partial','not-attempted'].includes(r.status))throw Error('unexpected experiment completion');
const e30=read(ex+'/runs/incidence-20261023-v1-adjudication/ADJUDICATION.json');if(!e30.accepted)throw Error('E30 replay not admitted');
const report=study+'/reports/followup-20261023';
const render=read(report+'/RENDER_INPUTS.json');for(const r of render.reports)for(const f of [r.html,r.pdf,r.graph])if(!fs.existsSync(report+'/'+f))throw Error('missing report artifact');
const files=[];function walk(dir){for(const n of fs.readdirSync(dir).sort()){const p=dir+'/'+n;if(fs.statSync(p).isDirectory())walk(p);else if(!p.endsWith('ARTIFACTS.json'))files.push({path:p,bytes:fs.statSync(p).size,sha256:sha(p)});}}
walk(report);walk(study+'/lean-v2/receipts');walk(ex+'/identities-20261023-v1');
for(const p of [ex+'/ASSESSMENT_PROTOCOL_V1.md',ex+'/ASSESSMENT_AMENDMENT_V3.md',ex+'/INCIDENCE_PROTOCOL_V1.md',ex+'/IDENTITIES_PROTOCOL_V1.md',ex+'/RAW_ARTIFACTS.json',ex+'/RAW_MEMBERS_FINAL.json'])files.push({path:p,bytes:fs.statSync(p).size,sha256:sha(p)});
fs.writeFileSync(output,JSON.stringify({schema:'twist.static-artifact-check/v1',local_links_checked:linkCount,cards_checked:cards,all_cards_blocked_unfunded:true,raw_members_hashed:raw.length,live_metadata_files:loose.length,prepared_archive_sha256:meta.archive.sha256,remote_raw_publication:meta.status,publication_reason:meta.reason,mathematical_statistics_recomputed_here:false,files},null,2)+'\n');
