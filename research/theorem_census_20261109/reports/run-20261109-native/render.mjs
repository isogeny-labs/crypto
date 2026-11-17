// Reporting only: read recorded counters; never run the research instrument.
import fs from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import path from 'node:path';
const study=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const hash=b=>createHash('sha256').update(b).digest('hex');
const file=(p,kind)=>{const b=fs.readFileSync(path.join(study,p));return {path:p,sha256:hash(b),bytes:b.length,kind};};
const rows=fs.readFileSync(path.join(study,'evidence/semaev-native.jsonl'),'utf8').trim().split('\n').map(JSON.parse);
const plot=rows.filter(r=>r.m>3).sort((a,b)=>a.q-b.q||a.m-b.m);
let svg='<svg xmlns="http://www.w3.org/2000/svg" width="780" height="350" viewBox="0 0 780 350"><rect width="780" height="350" fill="white"/><g font-family="DejaVu Sans" font-size="13" fill="#19312b"><text x="18" y="24" font-size="18">Exact S4/S5 tuple populations</text><text x="18" y="45">20261109-native · every model and tuple within stated fields</text>';
for(let e=0;e<=6;e++){let x=140+e*80;svg+=`<path d="M${x},60V295" stroke="#ddd"/><text x="${x-7}" y="318">10<tspan dy="-5" font-size="9">${e}</tspan></text>`;}
for(let i=0;i<plot.length;i++){const r=plot[i],y=70+i*28,w=Math.log10(r.tuples)*80;svg+=`<text x="18" y="${y+13}">F${r.q}, m=${r.m}</text><rect x="140" y="${y}" width="${w}" height="19" fill="${r.m===4?'#386f87':'#b06a36'}"/><text x="${148+w}" y="${y+14}">${r.tuples.toLocaleString('en-US')}</text>`;}
svg+='<text x="180" y="345">Tested ordered coordinate tuples (count; logarithmic scale)</text></g></svg>\n';
fs.writeFileSync(path.join(study,'reports/run-20261109-native/tuple-counts.svg'),svg);
fs.writeFileSync(path.join(study,'reports/run-20261109-native/plot-data.json'),JSON.stringify(plot,null,2)+'\n');
// The export binds the real pre-run Markdown bounds, not a fabricated freeze.
const host=JSON.parse(fs.readFileSync(path.join(study,'evidence/host.json')));
const binary=fs.readFileSync(path.join(study,'evidence/semaev-binary.sha256'),'utf8').split(/\s+/)[0];
const common={seed:null,replicate:0,warmup:false,status:'succeeded',outcome_class:'completed',attempts:1,resumed:0,checkpoint:{policy:'none',generations_written:0,restored_from_generation:null},
 isolation:{tier:'shared-container',contended:null,blockers:['Single selected host; no isolated timing or independent-host claim']},
 verification:{status:'no_claim',verifier:'Producer assertions and controls only; no independent signed verdict'},
 exclusion:{excluded:false,rule:null},started_at:null,finished_at:null,worker:'selected-workspace'};
const measurements=counters=>({counted:{},wall_seconds:null,wall_valid:false,cpu_user_seconds:null,cpu_sys_seconds:null,max_rss_bytes:null,counters});
const firstFiles=['evidence/semaev-controls.stdout','evidence/semaev-controls.stderr'];
const finalFiles=['evidence/semaev-controls-final.stdout','evidence/semaev-controls-final.stderr'];
const broadFiles=['evidence/semaev-native.jsonl','evidence/semaev-native.stderr'];
const counters={};for(const r of rows)for(const [k,v]of Object.entries(r))if(!['characteristic','q','m'].includes(k))counters[`q${r.q}_m${r.m}_${k}`]=v;
const runs=[{...common,run_id:'controls-initial',arm:'initial-controls',instance_id:'fixed-controls',measurements:measurements({}),artifacts:firstFiles.map(p=>file(p,'log'))},
 {...common,run_id:'controls-strengthened',arm:'native-final',instance_id:'fixed-controls',measurements:measurements({}),artifacts:finalFiles.map(p=>file(p,'log'))},
 {...common,run_id:'census-full',arm:'native-final',instance_id:'bounded-population',measurements:measurements(counters),artifacts:broadFiles.map(p=>file(p,'raw'))}];
const files=[...firstFiles,...finalFiles,...broadFiles,'PROTOCOL.md','native/finite.hpp','native/semaev.cpp','evidence/semaev-source.sha256','evidence/semaev-binary.sha256','evidence/compiler.txt','evidence/host.json'];
const out={schema:'experiment.export/v1',export_id:'theorem-census-20261109-native',created:new Date().toISOString(),
 protocol:{path:'PROTOCOL.md',sha256:file('PROTOCOL.md','protocol').sha256,protocol_id:'native-semaev-markdown-scope',version:1},
 producer:{runtime:'codex',model:null,session:'theorem-census-20261109',role:'producer'},
 source:{repo:'isogeny-labs/crypto',commit:'9172a052abd0b6cf8fabbabbe0f21cf930111938',dirty:true},backend:'local',
 host:{path:'evidence/host.json',sha256:file('evidence/host.json','host').sha256,class_id:null,summary:{cpu_model:host.cpu_model,logical_cpus:host.logical_cpus,memory_bytes:host.memory_bytes,kernel:host.kernel,hypervisor:null,isolation_tier_max:'shared-container'}},
 arms:[{id:'initial-controls',role:'baseline',binary_sha256:null,commit:null},{id:'native-final',role:'candidate',binary_sha256:binary,commit:null}],runs,
 flow:{planned:3,attempted:3,completed:3,not_completed:0,interrupted:0,resumed:0,excluded:[]},
 files:files.map(p=>file(p,p.startsWith('native/')?'other':p==='PROTOCOL.md'?'protocol':'raw')),
 analysis:{script:file('reports/run-20261109-native/render.mjs','analysis'),blinded:false,blinding_key_sha256:null,outputs:[file('reports/run-20261109-native/plot-data.json','summary')],unmasked_at:null},
 notes:[{kind:'observation',by:'producer',text:'Bounds were recorded in pre-run PROTOCOL.md. No committed frozen JSON protocol exists. This is an observational export; structural/file validation is not frozen-protocol acceptance.'},
 {kind:'observation',by:'producer',text:'Sixteen aggregate rows came from one broad census execution, not sixteen independent executions. Initial controls predate strengthened S4/S5 controls; the old binary digest was not recorded. Clocks/start/finish timestamps were not captured.'}],reviews_dir:'reviews'};
fs.writeFileSync(path.join(study,'export.json'),JSON.stringify(out,null,2)+'\n');
