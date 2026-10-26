// Assemble comparisons of native and independently computed outputs; no estimator.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
const root='research/conductor_gap_replication_20261025';
const load=p=>JSON.parse(readFileSync(root+'/'+p,'utf8'));
const a=load('analysis/panel-A.json'), b=load('analysis/panel-B.json');
const independent=process.argv[3]?JSON.parse(readFileSync(process.argv[3],'utf8')):
  load('validation/independent.json');
const headline=[];
const add=(name,claimed,recomputed)=>{
  assert(Number.isFinite(claimed)&&Number.isFinite(recomputed),name);
  assert(Math.abs(claimed-recomputed)<=1e-12*Math.max(1,Math.abs(claimed)),name);
  headline.push({name,claimed,recomputed,rel_tol:1e-12});
};
for(const level of a.levels)add('A level '+level.level+' S2/S1',level.ratio,
  independent.levels.find(x=>x.level===level.level).geometric_mean);
add('A equal-level secondary S2/S1',a.equal_level_secondary.ratio,independent.equal_level_secondary.geometric_mean);
add('A A/A',a.controls.aa.ratio,independent.aa.geometric_mean);
add('A coordinate model 1 / model 0',a.controls.coordinate_model1_over_model0.ratio,independent.coordinate.geometric_mean);
add('A plain / negation positive control',a.controls.rho_plain_over_negation.ratio,independent.positive_control.geometric_mean);
assert.equal(b.cells.length,104);
for(const c of b.cells){
  const other=independent.b_cells.find(x=>['class','volcano','model','arm'].every(k=>x[k]===c[k]));
  assert(other);assert.equal(c.verified,other.verified);
  add('B '+[c.class,c.volcano,c.model,c.arm].join('/')+' mean counted S cost floor',c.mean_s,other.mean_s);
}
add('main verified rows',a.records+b.records,independent.verified);
assert.equal(headline.length,114);
const raw_data=['analysis/records-A.csv','analysis/records-B.csv'].map(p=>({
  path:p,sha256:createHash('sha256').update(readFileSync(root+'/'+p)).digest('hex')}));
const report={schema:'conductor-gap.validator-report/v1',status:'pass',independent:true,
  implementation:'standalone C++20, no shared curve arithmetic or analyzer',
  physical_runner:process.argv[3]?'caller provenance required; fresh C++ result supplied for this comparison':
    'same host; separate implementation, second-host replay pending CI',raw_data_base:root,raw_data,headline,
  limitations:['No complete-cost IC/rho gain is admitted.','Bootstrap intervals use independently seeded different RNGs; point estimates compared here.',
    'Missing setup operation counts cannot be repaired by conversion-price bands.']};
writeFileSync(process.argv[2]??root+'/validation/validator-report.json',JSON.stringify(report,null,2)+'\n',{flag:'wx'});
console.log('114 headline comparisons pass; 104 B cell verification counts also match.');
