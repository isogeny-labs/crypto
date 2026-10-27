// Metadata and reproducibility pins only; no mathematical calculations.
import fs from 'node:fs';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v8';
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write=(p,x)=>fs.writeFileSync(p,JSON.stringify(x,null,2)+'\n');
async function run(c,a){const child=spawn(c,a);let o='',e='';child.stdout.on('data',x=>o+=x);child.stderr.on('data',x=>e+=x);const code=await new Promise((r,j)=>{child.on('error',j);child.on('close',r);});if(code)throw Error(e);return o.trim();}
const mode=process.argv[2];
if(mode==='draft'){
  if(fs.existsSync(v+'/protocol.json'))throw Error('fresh draft required');
  const p=JSON.parse(fs.readFileSync(s+'/execution-v7/protocol.json'));
  p.protocol_id='twist-kernel-relations-v1';p.study_dir=v;
  p.question='Do twist-rational7-torsion kernels define certified source isogenies that preserve a public DLP and complete relation rows?';
  p.claims=[{id:'maps',text:'All eight twist-derived degree7 maps and duals are certified with generic polynomial identities and exhaustive rational checks.',class:'exact_computation',decided_by:['exact-check']},{id:'relations',text:'All576 signed decomposition query sets and18 linear systems agree under same-base transport; recovered scalars are checked against public points.',class:'exact_computation',decided_by:['exact-check']},{id:'scope',text:'Known isogeny theory establishes a connection, not novelty, more transported relations, a solver speedup, original campaign completion or a NIST result.',class:'out_of_scope',decided_by:[]}];
  p.arms[0].description='Actual native exhaustive twist torsion, eight maps/duals, signed relation collection and non-oracle modular linear algebra.';
  p.arms[0].instrument.build[0][p.arms[0].instrument.build[0].length-1]='twist-kernel-relations';
  p.arms[1].description='Independent PARI engine builds all eight maps and reverse maps, verifies generic compositions, rational images/recovery and all576 signed witness sets and18 ranks.';
  for(const a of p.arms){a.instrument.commit=null;a.instrument.binary_sha256=null;}
  p.controls.positive.not_applicable_reason='Embedded public generator/dual/scalar checks and separate F11/F101 development controls; no separate measured control arm.';
  p.controls.negative.not_applicable_reason='Eight corrupt-map coefficients must fail; every witness with one flipped sign must fail. No separate measured arm.';
  p.variable_ledger.independent=[{name:'kernel line',levels:[0,1,2,3,4,5,6,7]},{name:'base construction',levels:['ordinary ascending x plus anchor G','maximal twist progression shifted by1 plus anchor G']},{name:'arity',levels:[2,3]},{name:'offset',levels:[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]}];
  p.variable_ledger.factors[0].value='Pinned native worker and independent PARI engine; GP export source and native generic group law frozen.';
  p.variable_ledger.factors[1].value='One public p827 source and target; all8 kernels, both bases, both arities and all16 offsets. Target never selects a kernel or base.';
  p.variable_ledger.factors[1].how='Exhaustive same-base signed witness equality and exact rank/recovery; all exceptional kernel/infinity cases checked.';
  p.variable_ledger.factors[3].how='Shared author/host; independent CAS map/group/linear algebra, exact native replay and conditional Lean group implication.';
  p.variable_ledger.comparisons[0].threshold='All8 maps and duals,16 generic dual compositions,all576 signed query sets and18 relation-system ranks/recoveries pass; every corrupt/sign control is rejected.';
  p.randomization.arm_order_reason='Native producer then independent CAS exact verification; no performance comparison.';
  p.randomization.instance_sampling.rule='One fixed public source/DLP, complete eight-kernel/both-base/both-arity/16-offset panel. Offsets are correlated relation queries for one Q.';
  p.sample_size.preflight_runtime.estimated_total_seconds=1200;
  p.sample_size.preflight_runtime.measured_on='Unmeasured prospective caps:300s producer,300s CAS,300s native audit plus metadata allowance. Not an observed performance result.';
  p.metrics.primary=[{id:'query-cases',name:'Complete signed decomposition query cases',unit:'query cases in one fixed finite panel',kind:'exact',definition:'Nine models,two bases,two arities,sixteen offsets;64source and512destination. Equality includes every empty case and repeated-index witness.',source:'ADMISSION.json summary.query_cases',aggregation:'per_row',uncertainty:'none_exact'}];
  p.metrics.secondary=[{id:'kernel-lines',name:'Certified twist-derived maps',unit:'degree7 maps',kind:'exact',definition:'All8 lines in full7-torsion, with generic map and dual identities.',source:'ADMISSION.json summary.kernel_lines',aggregation:'per_row',uncertainty:'none_exact'}];
  p.measurement.counters=['kernel lines','complete signed query sets','generic dual checks','rational image/recovery checks','relation-system ranks'];
  p.stopping_rules[0].rule='Exactly eight kernels and576 query cases; retain any failure/cap. No outcome-dependent enlargement, target/base replacement or retries.';
  p.decision_rule.text='Admit the executable twist-derived scalar/relation-preserving route only after native and independent CAS exact checks. Costs and any destination-solving benefit remain unknown.';
  p.decision_rule.statistic='Exact generic identities, finite point/witness equalities and matrix ranks.';
  p.decision_rule.effect_size_measure='Exact preservation of witnesses and ranks, not a performance effect.';
  p.analysis_plan.script={path:s+'/runner/src/bin/twist-kernel-relations.rs',sha256:sha(s+'/runner/src/bin/twist-kernel-relations.rs')};
  p.analysis_plan.inputs=['fixed public p827 parameters in frozen source','native output','actual PARI receipt and output hashes'];
  p.analysis_plan.outputs=['RESULTS.json','verify.gp','ADMISSION.json','experiment.export.json'];
  p.reporting.graphs=['Recorded relation rows and ranks by base/model, with identical transported systems retained.'];
  p.reporting.report_dir=s+'/reports/kernel-relations-20261027';
  p.inadmissible=['Calling standard isogeny theory novel','Calling same-base transported rows new independent information','Claiming a solver speedup without charged costs and strongest matched controls','Inferring a NIST route from this public p827 example','Closing original full-campaign criteria'];
  p.literature=[{ref:v+'/LITERATURE.md',provenance:'internal',claim:'Primary MIT kernel descent and Velu formulas, primary PARI help and atlas reviewed before implementation.',verified_by:'Source-linked review and separate control curves before main data.'}];
  p.amendments=[];
  p.freeze={status:'draft',protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:false};
  p.extensions={scientific_panel_started:false,original_full_campaign_discharged:false,embedded_controls_are_not_separate_arms:true,backend_authority:'Standing authorization and documented EKS access/image fallback; bounded local L0 only, no paid resource.'};
  write(v+'/protocol.json',p);
  const cpu=await run('lscpu',[]),kernel=await run('uname',['-a']);
  write(v+'/HOST.json',{schema:'twist.host/v1',created:new Date().toISOString(),summary:'Managed x86_64 Intel Xeon Platinum8573C KVM host;CPU0,one thread,L0 exact correctness only',lscpu:cpu,kernel,isolation:'L0',wall_time_admitted:false});
}else if(mode==='pin'){
  if(fs.existsSync(v+'/FROZEN.json'))throw Error('fresh freeze required');
  const rev=await run('git',['rev-parse','HEAD']);
  const binDir='/workspace/shared/twist-kernel-native-20261027/'+rev;fs.mkdirSync(binDir,{recursive:true});
  fs.copyFileSync(s+'/runner/target/release/twist-kernel-relations',binDir+'/worker');
  const gp='/workspace/shared/twist-tools-20261023/cas/root/usr/bin/gp';
  const p=JSON.parse(fs.readFileSync(v+'/protocol.json'));
  for(const a of p.arms){a.instrument.commit=rev;a.instrument.binary_sha256=sha(a.id==='native'?binDir+'/worker':gp);}
  write(v+'/protocol.json',p);
  await run('python3',['/workspace/crypto-autoresearcher/tools/experiment_protocol.py','freeze',v+'/protocol.json','--commit',rev,'--filed-before-affected-data']);
  const sources=[s+'/runner/src/bin/twist-kernel-relations.rs',s+'/runner/src/registry.rs',s+'/runner/Cargo.toml',s+'/runner/Cargo.lock','src/cryptanalysis/twist_ic_field.rs','src/hash/sha256.rs',v+'/run-producer.mjs',v+'/PROTOCOL.md',s+'/lean-v5/TwistKernelRelations.lean'];
  write(v+'/FROZEN.json',{schema:'twist.kernel-freeze/v1',created:new Date().toISOString(),source_revision:rev,binary:{path:binDir+'/worker',sha256:sha(binDir+'/worker')},CAS:{path:gp,sha256:sha(gp)},source_hashes:Object.fromEntries(sources.map(p=>[p,sha(p)])),protocol_file_sha256:sha(v+'/protocol.json'),scientific_panel_started:false,wall_time_admitted:false});
  console.log(rev);
}else throw Error('draft|pin');
