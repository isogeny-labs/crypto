// Metadata-only extraction of the predeclared, disjoint calibration specs.
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
const root='research/conductor_gap_replication_20261025';
const dir=root+'/calibration';
mkdirSync(dir,{recursive:true});
const read=p=>JSON.parse(readFileSync(root+'/'+p,'utf8'));
const arms=(name,id,role,params={})=>({name,role,method:{id,params}});
const specs=[];
for(const ci of [2,5]){
  const s=read(`prepared-a/specs/a-c${ci}-controls.json`);
  s.workloads.curves=s.workloads.curves.slice(0,4);
  s.arms.push(arms('bsgs-neg','bsgs.negation','control'));
  specs.push([`pilot-a-c${ci}`,s]);
}
for(const ci of [0,2]){
  const s=read(`prepared-b-v2/specs/b-c${ci}.json`);
  s.workloads.curves=s.workloads.curves.slice(0,1);
  const v=read(`prepared-b-v2/specs/b-c${ci}-velu.json`);
  s.arms.push(v.arms[0]);
  specs.push([`pilot-b-c${ci}`,s]);
}
for(const [name,s]of specs){
  s.label='Excluded runtime calibration: '+name;
  s.workloads.targets_per_curve=4;
  s.workloads.target_seed=2026102520;
  s.measurement.seed=2026102521;
  writeFileSync(`${dir}/${name}.json`,JSON.stringify(s,null,2)+'\n',{flag:'wx'});
}
writeFileSync(dir+'/manifest.json',JSON.stringify({expected_records:128,specs:specs.map(([name])=>({name,path:`${dir}/${name}.json`}))},null,2)+'\n',{flag:'wx'});
