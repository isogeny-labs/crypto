// File-integrity mutations of an explicitly marked synthetic fixture only.
import fs from 'node:fs';
import {spawnSync} from 'node:child_process';
const {GB_VERIFY,GB_SYNTH_INPUT,GB_SYNTH_OUTPUT,GB_NEGATIVE_OUT}=process.env;
if(!GB_VERIFY||!GB_SYNTH_INPUT||!GB_SYNTH_OUTPUT||!GB_NEGATIVE_OUT)throw Error('Set GB_VERIFY,GB_SYNTH_INPUT,GB_SYNTH_OUTPUT,GB_NEGATIVE_OUT');
if(!fs.existsSync(GB_SYNTH_INPUT+'/SYNTHETIC_FIXTURE.txt'))throw Error('Synthetic fixture marker required');
fs.mkdirSync(GB_NEGATIVE_OUT,{recursive:false});
const cases=[
 ['wrong-point','points.tsv',d=>{const r=d[1].split('\t');r[9]='999999';d[1]=r.join('\t');},'point/group check failed'],
 ['wrong-trait','points.tsv',d=>{const r=d[1].split('\t');r[11]='999999';d[1]=r.join('\t');},'trait value mismatch'],
 ['unknown-as-zero','historical-points.tsv',d=>{const i=d.findIndex(x=>x.includes('\tunknown\t'));const r=d[i].split('\t');r[3]='0';d[i]=r.join('\t');},'unknown historical point changed'],
 ['false-recipe','recipes.tsv',d=>{const r=d[1].split('\t');r[5]=r[5]==='1'?'0':'1';d[1]=r.join('\t');},'recipe equality mismatch'],
 ['duplicate-cutoff','cutoffs.tsv',d=>{d[2]=d[1];},'duplicate cutoff row'],
 ['wrong-difference','point-pairwise.tsv',d=>{const r=d[1].split('\t');r[8]='999999';d[1]=r.join('\t');},'difference mismatch'],
 ['missing-point','points.tsv',d=>{d.pop();},'missing point record/catalog'],
 ['changed-inherited','existing-feature-matrix.tsv',d=>{const r=d[1].split('\t');r[4]='999999';d[1]=r.join('\t');},'inherited value changed'],
 ['false-summary','summary.json',d=>{const r=JSON.parse(d[0]);r.exact_single_separators=999999;d[0]=JSON.stringify(r);},'headline summary mismatch']
];
const records=[];
for(const[id,file,edit,reason]of cases){
 const dir=GB_NEGATIVE_OUT+'/'+id;fs.mkdirSync(dir);
 for(const f of fs.readdirSync(GB_SYNTH_OUTPUT)){
  if(f===file){const lines=fs.readFileSync(GB_SYNTH_OUTPUT+'/'+f,'utf8').trimEnd().split('\n');edit(lines);fs.writeFileSync(dir+'/'+f,lines.join('\n')+'\n');}
  else fs.symlinkSync(GB_SYNTH_OUTPUT+'/'+f,dir+'/'+f);
 }
 const r=spawnSync(GB_VERIFY,[GB_SYNTH_INPUT,dir,dir+'/rejected-receipt.json'],{encoding:'utf8',timeout:60000});
 fs.writeFileSync(dir+'/stdout',r.stdout||'');fs.writeFileSync(dir+'/stderr',r.stderr||'');
 if(r.status!==1||!(r.stderr||'').includes(reason)||fs.existsSync(dir+'/rejected-receipt.json'))throw Error('Control did not refuse as intended: '+id);
 records.push({id,status:r.status,expected_refusal:reason,passed:true});
}
fs.writeFileSync(GB_NEGATIVE_OUT+'/summary.json',JSON.stringify({kind:'synthetic-integrity-controls/v1',production_census_executed:false,cases:records},null,2)+'\n');
process.stdout.write('All nine synthetic integrity controls refused as intended\n');
