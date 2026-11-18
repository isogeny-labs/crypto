"""Hash/protocol/admission/export plumbing only. No research arithmetic.
All field, polynomial, GB, dimension and counting work is native C++/msolve.
"""
from pathlib import Path
import copy,hashlib,importlib.util,json,subprocess,sys

ROOT=Path('/workspace/crypto-audit')
STUDY=ROOT/'research/groebner_system_audit_20261113'
TOOL=Path('/workspace/crypto-autoresearcher-current/tools/experiment_protocol.py')
BIN=Path('/workspace/scratch/groebner-system-native')
FIXTURES=['control','edwards-affine','edwards-top','edwards-homogeneous',
          'jv-empty-affine','jv-empty-homogeneous','jv-split-affine','jv-split-homogeneous']
sp=importlib.util.spec_from_file_location('audit_experiment_protocol',TOOL)
ep=importlib.util.module_from_spec(sp);sys.modules[sp.name]=ep;sp.loader.exec_module(ep)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def head():return subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
def dump(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def prepare():
 assert not (STUDY/'protocol.json').exists()
 source=head();provenance=json.loads((STUDY/'evidence/build-provenance.json').read_text())
 paths=['native/analyze.cpp','run.sh','analyze.sh','metadata.py','evidence/build-provenance.json']
 paths += ['inputs/'+f+'.ms' for f in FIXTURES]
 paths += [str(BIN/'msolve'),str(BIN/'analyze'),str(TOOL)]
 paths += provenance['dynamic_library_paths']
 (STUDY/'inputs.sha256').write_text(''.join(sha(STUDY/x if not Path(x).is_absolute() else x)+'  '+x+'\n'for x in paths))
 # Reuse only stable schema scaffolding; replace all scientific scope below.
 p=copy.deepcopy(json.loads((ROOT/'research/bk_basefield_census_20261112/protocol-preflight.json').read_text()))
 p.update(protocol_id='groebner-source-systems-20261113',created='2026-11-13',
   study_dir=str(STUDY.relative_to(ROOT)),authors=['Codex; approved original fourteen-item audit'],
   links={'source_session':'Approved all14, item9; zero-paid local checkpoint',
          'spec':'Eight fixed source/control systems, each twice; two native calibration calls.',
          'note':'First native trials for these fixtures. Known Lean/source answers are unblinded; no heldout or performance inference.'},
   question='Compute exact GBs, finite staircases, rational zeros and initial-ideal Hilbert prefixes for the actual Edwards counterexample and the two original Joux-Vitse F101^3 toy descents, including literal homogenization.')
 p['claims']=[
   {'id':'G1','class':'exact_computation','text':'Bounded exact outputs for eight registered fixtures, native source reconstruction, independent native Buchberger/remainder/S-pair checks and deterministic repeats.','decided_by':['fixed-fixtures']},
   {'id':'G2','class':'theorem','text':'The Edwards actual input/descent/nonregularity and length-two algebra are decided by the separate frozen Lean55 project/PCC-bc105f.','decided_by':[]},
   {'id':'G3','class':'out_of_scope','text':'No generic/n>=3 FGHR refutation, complete JV internal-degree bound, solver/compiler refinement, timing or independent-host assertion.','decided_by':[]}]
 solver={'repo':'algebraic-solving/msolve','commit':'5e72b9d99eaea55fe87eb8ac945ec9e914a69327',
   'build':[['bash','research/solver_multiplicity_audit_20261109/build-msolve.sh']],
   'binary_sha256':sha(BIN/'msolve'),'threads':1,
   'flags':{'linear_algebra':'1','genericity':'0','verbosity':'2','print_gb':'2','source_provenance':'evidence/build-provenance.json'},
   'env':{'LD_LIBRARY_PATH':'/workspace/msolve-deps/root/usr/lib/x86_64-linux-gnu','OMP_NUM_THREADS':'1'}}
 analyzer={'repo':'isogeny-labs/crypto','commit':source,
   'build':[provenance['analyzer_compile_argv']], 'binary_sha256':sha(BIN/'analyze'),
   'threads':1,'flags':{'assertions':'enabled','mode':'bounded exact native checker'},'env':{}}
 arms=[{'id':'controls','role':'control_positive','description':'Native exact dual-number length2 versus simple length1 and correct GB/remainder control.','instrument':copy.deepcopy(analyzer),'parameters':{'component':'controls','argv':['--controls']}},
       {'id':'negative','role':'control_negative','description':'Same native path detects one wrong constant in an otherwise valid GB.','instrument':copy.deepcopy(analyzer),'parameters':{'component':'negative','argv':['--negative']}}]
 comparisons=[{'id':'fixed-fixtures','arms':['controls','negative'],'differs_in':['fixture','instrument'],'design':'absolute','threshold':'Every fixed raw output is retained; exact correctness and source discrepancies are reported separately. No cross-instrument causal ranking.'}]
 for f in FIXTURES:
  for aa in [False,True]:
   arm={'id':f+('-aa'if aa else''),'role':'control_aa'if aa else'reference',
    'description':('Identical repeat of 'if aa else'Exact source/control system ')+f,
    'instrument':copy.deepcopy(solver),'parameters':{'component':f,'input':'inputs/'+f+'.ms'}}
   if aa:arm['same_instrument_as']=f
   arms.append(arm);comparisons[0]['arms'].append(arm['id'])
  comparisons.append({'id':'aa-'+f,'arms':[f,f+'-aa'],'differs_in':['arm_label'],'design':'aa','threshold':'Exact GB output bytes match; independent native canonical GBs match for both.'})
 p['arms']=arms
 p['controls']={'aa':{'present':True,'comparison':'aa-control','not_applicable_reason':None},
  'positive':{'present':True,'arm':'controls','expected_detection':'Native length2/support1 versus simple length1; actual solver x²,y² must have finite length4/support1.','not_applicable_reason':None},
  'negative':{'present':True,'arm':'negative','null_population':{'description':'Same native reduction path with y-1 changed to y-2, keeping x².','generator':'Frozen native calibration constructs both exact lists and requires original input remainder nonzero against the wrong basis.','n':1,'why_n':'Exact deliberate detector control, no statistical inference.','matched_on':['field','variables','degree','reduction code','all other coefficients']},'not_applicable_reason':None}}
 p['variable_ledger']={'independent':[], 'factors':[
  {'name':'arm_label','treatment':'fixed','value':'Reference versus A/A output path only','how':'Identical binary/flags/input/stop rule within each pair.'},
  {'name':'fixture','treatment':'fixed','value':'Eight literal source/control systems; two native synthetic calibration components.','how':'One registered fixed batch, specified by each arm component before data.'},
  {'name':'instrument','treatment':'fixed','value':'Native checker for calibration; original msolve LA1 for solver arms.','how':'Exact per-arm identities. The absolute fixture record makes no cross-instrument causal comparison; A/A pairs keep the same instrument.'},
  {'name':'host','treatment':'measured','value':'Same shared local host','how':'Provenance retained; scientific clocks invalid.'},
  {'name':'analyst','treatment':'fixed','value':'Unblinded exact checker','how':'Known source/Lean answers stated in advance; no fitted test.'}],
  'comparisons':comparisons,'crosses_map':False}
 p['randomization']={'arm_order':'fixed','seed_derivation':{'method':'harness_drawn','note':'No randomness. Full fixed order and deterministic repeats.'},
  'instance_sampling':{'rule':'One registered batch; arm component selects the declared literal fixture/calibration, not a random method comparison.','independent_of_arm':True},
  'blinding':{'analysis_labels_masked':False,'how':'Known-answer exact mathematics; no subjective selection.'},
  'arm_order_reason':'No scientific clock, timing or statistical inference; every listed call executes once.'}
 p['sample_size']={'instances':1,'replicates':1,'warmups':0,
  'power':{'method':'exact','alpha':0.05,'tests_in_family':1,'correction':'none','passes':True,'note':'Exactly18calls: two native calibrations and16solver calls; no sampled-population inference.'},
  'preflight_runtime':{'fixed_seconds':0,'per_sample_seconds':10,'threads':1,'estimated_total_seconds':170,
  'measured_on':'Hard budget caps, not measured time estimates. Exact computation only; every scientific clock invalid.'}}
 p['metrics']={'primary':[{'id':'call_complete','name':'Fixed native call completed','unit':'Boolean','kind':'exact','definition':'Child exit0; independent frozen analysis is a separate validity gate.','source':'runs/main/*.exit','aggregation':'per_row','resolution':'integer','uncertainty':'none_exact'}],
  'secondary':[{'id':'basis_degree','name':'Returned GB maximal degree','unit':'degree','kind':'exact','definition':'Native parser computes the maximum degree of returned GB polynomials; separate from F4 selected-pair rounds and an unaudited internal maximum.','source':'analysis/analysis.jsonl','aggregation':'per_row','resolution':'integer','uncertainty':'none_exact'}]}
 p['measurement']={'isolation_required':'none','warmups':0,'order':'sequential','clock':'none','counters':[],
  'record':['all raw outputs/exits/argv','source/binary/library pins','native independent GB and algebra counts','UTC provenance times only'],'timed_primary_justification':None}
 p['execution']={'backend':'local','queue':None,'measurement_class':'search','timeout_seconds':600,
  'checkpoint':{'policy':'none','interval_seconds':None,'grace_seconds':None,'restore_validation':'none','after_restore':{'wall_time':'invalid','counted_units':'valid'}},
  'retries':{'infrastructure_only':True,'max_attempts':1}}
 p['stopping_rules']=[{'rule':'Execute both native calibration calls and every one of16solver calls once; preserve failures/timeouts and do not rerun.','kind':'fixed_n','preregistered':True},
  {'rule':'Native calibration5s each; solver10s each; frozen native analysis120s. Driver600s cap.','kind':'budget','preregistered':True}]
 p['invalidation_rules']=['Changed frozen input/binary/protocol/analysis pin, missing output, failed positive/negative/source/GB/remainder/S-pair/A/A gate invalidates an exact accepted result; all evidence is still retained.',
  'A source-prediction discrepancy is an outcome, not an excuse to delete or rerun it. Only finite checked scope may be reported.']
 p['decision_rule']={'text':'Accept bounded exact outputs only if all frozen integrity/control/source gates pass; report every source discrepancy separately.','comparison_ids':['fixed-fixtures'],
  'threshold':'Zero integrity discrepancies. Source predictions are separate observations.','statistic':'Exact canonical GB/initial monomial ideal/finite staircase/rational zero counts','interval':'none; fixed fixtures',
  'multiple_comparison_correction':'none','effect_size_measure':'Exact per-system integer quantities','verdict_vocabulary':['bounded_checks_pass','discrepancy','not_validated']}
 p['analysis_plan']={'script':{'path':'analyze.sh','sha256':sha(STUDY/'analyze.sh')},'inputs':['inputs/*.ms','runs/main/*'],
  'outputs':['analysis/analysis.jsonl','analysis/analysis.stderr','analysis/analysis.exit'],'blinded':False,'frozen_before_data':True}
 p['reporting']={'export_schema':'experiment.export/v1','review_schema':'experiment.review/v1','graphs':[],'report_dir':'reports/main'}
 p['inadmissible']=['No performance or security inference.','A finite special input does not refute generic hypotheses.','Returned basis degree or selected-pair degree does not certify the full internal maximum in JVAssumption2.','No independent-host/compiled-refinement assertion.']
 p['literature']=[{'ref':'FGHR2014 printed603–604/Definition5/§4.1.1/Hypothesis3; exact PDF/source review in prior Edwards study.','provenance':'retrieved','claim':'Use the literal a/d Edwards source formula and actual highest homogeneous components.'},
  {'ref':'Joux–Vitse author DLFq5.pdf SHA6f3268a0ef8c50b51a79f8b64deff0c60374228a827bea73d4e590284cb68377 pp7/9–10.','provenance':'retrieved','claim':'Use both exact original F101³ toy systems and literal quadratic homogenization; distinguish the stated computation-degree assumption.'}]
 p['amendments']=[];p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':False}
 p['extensions']={'original_item':9,'population_id':'groebner-source-fixed-batch','source_checkpoint':source,
  'input_manifest':{'path':'inputs.sha256','sha256':sha(STUDY/'inputs.sha256')},'native_binary_sha256':sha(BIN/'analyze'),
  'new_planned_calls':18,'solver_calls':16,'solver_fixture_count':8,'known_in_sample':True,
  'scientific_clock_valid':False,'checkpoint_exception':'Previously approved zero-paid local checkpoint; no agent paid compute/storage uploads.',
  'observed_internal_f4_degree_complete':False,'hilbert_scope':'Exact initial monomial ideal degrees0..12; no stabilization extrapolation.'}
 dump(STUDY/'protocol.json',p)
 print(json.dumps({'draft':'protocol.json','calls':18,'source_checkpoint':source}))
def admit():
 p=json.loads((STUDY/'protocol.json').read_text())
 errors=ep.schema_errors(p,'experiment.protocol/v1')+ep.lint_protocol(p,STUDY);assert not errors,errors
 assert p['freeze']['status']=='frozen' and p['freeze']['filed_before_affected_data']
 assert ep.protocol_hash(p)==p['freeze']['protocol_sha256']
 assert subprocess.check_output(['git','show','HEAD:'+str((STUDY/'protocol.json').relative_to(ROOT))],cwd=ROOT)==(STUDY/'protocol.json').read_bytes()
 assert sha(STUDY/'inputs.sha256')==p['extensions']['input_manifest']['sha256']
 for line in (STUDY/'inputs.sha256').read_text().splitlines():
  digest,path=line.split('  ',1);assert sha(Path(path)if Path(path).is_absolute()else STUDY/path)==digest,path
 assert not (STUDY/'runs/main').exists(),'Never rerun completed data.'
 print(json.dumps({'admitted':True,'protocol_sha256':p['freeze']['protocol_sha256']}))
def export():
 p=json.loads((STUDY/'protocol.json').read_text());out=STUDY/'export.json';assert not out.exists()
 e=ep.export_skeleton(p,'protocol.json','local',p['protocol_id']+'-export')
 e['producer'].update(runtime='codex',model='Codex',session='fourteen-target-20261113')
 e['source'].update(repo='isogeny-labs/crypto',commit=head(),dirty=True)
 e['host']={'path':'evidence/build-provenance.json','sha256':sha(STUDY/'evidence/build-provenance.json'),
   'class_id':'same-local-groebner-source-host','summary':{'cpu_model':None,'logical_cpus':None,'memory_bytes':None,'kernel':None,'hypervisor':None,'isolation_tier_max':'none'}}
 e['arms']=[{'id':a['id'],'role':a['role'],'binary_sha256':a['instrument']['binary_sha256'],'commit':a['instrument']['commit']}for a in p['arms']]
 ok=(STUDY/'analysis/analysis.exit').read_text().strip()=='0'
 records={}
 if (STUDY/'analysis/analysis.jsonl').exists():
  for line in (STUDY/'analysis/analysis.jsonl').read_text().splitlines():
   x=json.loads(line)
   if 'fixture'in x:records[x['fixture']+('-aa'if x['replicate']else'')]=x
 for a in p['arms']:
  arm=a['id'];prefix=STUDY/'runs/main'/arm;rc=int(prefix.with_suffix('.exit').read_text())
  status='succeeded'if rc==0 else('timeout'if rc in[124,137]else'failed')
  counted={'call_complete':int(rc==0)}
  counted.update({k:v for k,v in records.get(arm,{}).items()if isinstance(v,(int,float)) and not isinstance(v,bool)})
  e['runs'].append({'run_id':arm,'arm':arm,'instance_id':p['extensions']['population_id'],
   'seed':'No randomness; fixed source/control fixture','replicate':0,'warmup':False,'status':status,
   'outcome_class':'completed'if status in['succeeded','failed']else'not_completed','attempts':1,'resumed':0,'checkpoint':None,
   'measurements':{'counted':counted,'wall_seconds':None,'wall_valid':False,'cpu_user_seconds':None,'cpu_sys_seconds':None,'max_rss_bytes':None,'counters':{}},
   'isolation':{'tier':'none','contended':None,'blockers':['Shared local host; exact outputs only.']},
   'verification':{'status':'verified'if rc==0 and ok else'error','verifier':'Frozen native independent Buchberger/source checks; unsigned same-host evidence.'},
   'exclusion':{'excluded':False,'rule':None},'artifacts':[ep.file_entry(STUDY,str(f.relative_to(STUDY)))for f in sorted(prefix.parent.glob(arm+'.*'))if f.is_file()],
   'started_at':prefix.with_suffix('.start').read_text().strip(),'finished_at':prefix.with_suffix('.end').read_text().strip(),
   'worker':'same local shared host','backend_ref':'runs/main/'+arm})
 e['flow']=ep.compute_flow(e['runs'])
 e['files']=[ep.file_entry(STUDY,str(f.relative_to(STUDY)))for directory in[STUDY/'runs/main',STUDY/'analysis']for f in sorted(directory.iterdir())if f.is_file()]
 e['files']+=[ep.file_entry(STUDY,x)for x in['inputs.sha256','evidence/build-provenance.json','native/analyze.cpp','run.sh','analyze.sh','metadata.py']]
 e['analysis']={'script':ep.file_entry(STUDY,'analyze.sh','analysis'),'blinded':False,'blinding_key_sha256':None,
  'outputs':[ep.file_entry(STUDY,str(f.relative_to(STUDY)),'summary')for f in sorted((STUDY/'analysis').iterdir())if f.is_file()],'unmasked_at':None}
 e['notes']=[{'kind':'observation','by':'Codex','text':'Known in-sample fixtures; no random/general or performance inference. Source inputs and homogenization are checked. Returned basis degree/F4 selected pair degree are not a full internal-degree certificate.'}]
 errors=ep.schema_errors(e,'experiment.export/v1')+ep.lint_export(e,p,STUDY,True);assert not errors,errors
 ep.write_once(out,e);print(json.dumps({'export':'export.json','flow':e['flow']}))
if __name__=='__main__':
 assert len(sys.argv)==2 and sys.argv[1]in['prepare','admit','export']
 {'prepare':prepare,'admit':admit,'export':export}[sys.argv[1]]()
