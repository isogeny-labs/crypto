"""Build the complete 42-case draft; freeze/commit separately before production."""
from pathlib import Path
import hashlib,json,subprocess
s=Path('research/n83_candidate_pdp_20261028');scratch=Path('/home/ec2-user/n83-candidate-recovery');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
p=json.loads((s/'protocol-prime-first-fall.json').read_text());p.pop('freeze');p['amendments']=[]
commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip();image=json.loads((scratch/'prime-terminal-support/image.json').read_text());assert image['source_revision']==commit
p.update(protocol_id='N83-PRIME-TERMINAL-20261030',created='2026-10-30',version=1,question='Do explicit terminal annihilators certify highest-form FFD upper bounds on all42 retained prime m3-9 systems, with exact affine consequences kept separate from strict LFD claims?')
p['links']={'design':'PRIME_TERMINAL_DESIGN.md'}
p['claims']=[{'id':'C1','class':'exact_computation','text':'Complete source checks and nontrivial terminal annihilator certify an upper bound on highest-form FFD under the written projection argument. No exact FFD claimed.','decided_by':[]},{'id':'C2','class':'exact_computation','text':'Store exact affine consequences with bounded derivation degree; strict lower-space nonmembership is not tested.','decided_by':[]},{'id':'C3','class':'out_of_scope','text':'No LFD lower/upper/exact certificate, solving-degree, elite-base advantage, timing or independent scientific review claim.','decided_by':[]}]
for arm in p['arms']:
 arm['description']=arm['id']+' retained prime m3-9 inputs';arm['parameters']['m']=list(range(3,10));arm['instrument'].update(commit=commit,binary_sha256=image['binary_sha256'],build=[['cargo','build','--locked','--release','--manifest-path',str(s/'Cargo.toml'),'--bin','prime_terminal']])
p['variable_ledger']['factors']=[{'name':n,'treatment':t,'how':h} for n,t,h in [('instrument','fixed','One Rust binary and digest-pinned per-commit image;single thread'),('input','fixed','All42 saved source,input and point-manifest hashes;no new targets'),('dataset','crossed','Original/baseline/spike40 retained identities;not independent statistical draws'),('host','measured','EKS Spot connected service;CLI AssumeRole denied;no performance claim'),('limits','fixed','60s controls,300s producer,300s replay,2GiB addressspace,1200s Job,no retries'),('analyst','fixed','Preserve all42 outcomes;FFD upper bound distinct from affine strictness')]]
p['variable_ledger']['comparisons'][0]['threshold']='All source,primality,annihilator,independent expansion and replay checks pass;affine strictness remains unknown'
p['controls']['positive']['not_applicable_reason']='No additional production arm:18 independent ordinary-exponent polynomial expansions and21 synthetic full-chain source combinations;inherited field controls pass.'
p['controls']['negative']['not_applicable_reason']='18 non-annihilator controls;wrong projection,mutated source,wrong base,invalid planted values,zero weights and truncated/corrupt factors rejected.'
p['sample_size']['instances']=14;p['sample_size']['power']['note']='All42 retained inputs,14perarm;deterministic certificate panel,no statistical inference.'
p['sample_size']['preflight_runtime'].update(estimated_total_seconds=1200,measured_on='Development controls only;fixed budget,not runtime prediction')
p['randomization']['instance_sampling']['rule']='Entire pre-existing42case panel;no sampling or target changes;original and paired planted targets differ.'
p['randomization']['seed_derivation']['note']='Historical input seed only;this certificate stage makes no new draws.'
def metric(name,text):return {'id':name,'name':name,'unit':'degree','kind':'counted','definition':text,'source':'results/summary.json '+name,'aggregation':'per_row','resolution':'1','uncertainty':'none_exact'}
p['metrics']={'primary':[metric('first_fall_upper_bound','Degree of a certified nontrivial highest-form syzygy;not equality')],'secondary':[metric('affine_derivation_degree_upper_bound','Degree bound on derivation of stored affine consequence'),metric('affine_consequence_degree','Exact degree after combining all compact circuit terms;omit if zero polynomial')]}
p['measurement']['counters']=[x['id'] for xs in p['metrics'].values() for x in xs]
p['execution'].update(backend='eks',queue='isolab/cpu-spot',timeout_seconds=1200);p['execution']['placement'].update(cpus=2,memory='4Gi')
p['stopping_rules']=[{'kind':'fixed_n','preregistered':True,'rule':'All42cases once,controls then producer then fresh replay;retain partials/errors;stop on any mismatch'},{'kind':'budget','preregistered':True,'rule':'60s controls,300s producer,300s replay;2GiB child addressspace;1200s Job,zero automatic retries'}]
p['invalidation_rules']=['Source,base,manifest,primality,annihilator,codec or replay mismatch invalidates','A degree drop is not strict filtered nonmembership;LFD fields remain null','Clock diagnostic only;no speed or quality ranking']
p['decision_rule'].update(text='Record FFD upper bound after full source and annihilator/projection checks and replay;preserve exact compact affine consequence without strictness claims.',verdict_vocabulary=['certified_upper','error','incomplete'])
p['analysis_plan'].update(script={'path':'src/bin/prime_terminal.rs','sha256':sha(s/'src/bin/prime_terminal.rs')},inputs=['42 unchanged input.json/system.sing pairs'],outputs=['results/summary.json','results/KEY/certificate.json','results/KEY/A0.poly','results/KEY/A1.poly','results/KEY/A2.poly','controls.json'])
p['reporting']['graphs']=['Per-m highest-form FFD upper certificates;null exact values explicit;no factor-base ranking']
p['literature']=[{'ref':'https://arxiv.org/html/2112.05579','provenance':'retrieved','claim':'Definition1.3 and section4 distinguish nontrivial highest-form syzygies from strict affine falls','verified_by':'Primary source read before design;PRIME_TERMINAL_DESIGN.md'}]
cases=[]
for arm in ['original','baseline','spike40']:
 for m in range(3,10):
  for target in ['planted','natural']:
   ident=f'prime-m{m}-{target}';directory=Path('/home/ec2-user/n83-fixed-degree-recovery/r1/results/inputs')/ident if arm=='original' else scratch/'pdp-r1'/arm/'inputs'/ident
   inp=json.loads((directory/'input.json').read_text());assert sha(directory/'system.sing')==inp['source_sha256']
   cases.append({'key':arm+'-'+ident,'arm':arm,'id':ident,'directory':str(directory),'input_sha256':sha(directory/'input.json'),'source_sha256':inp['source_sha256'],'factor_base_manifest_sha256':inp['factor_base_manifest_sha256']})
files=['src/bin/prime_terminal.rs','src/prime_terminal_math.rs','src/prime_first_fall_donor.rs','PRIME_TERMINAL_DESIGN.md','PRIME_TERMINAL_FORMAT.json','run-prime-terminal.py','freeze-prime-terminal.py']
p['extensions']={'cases':cases,'detailed_design':'PRIME_TERMINAL_DESIGN.md','file_layout':'PRIME_TERMINAL_FORMAT.json','image':image,'job_name':'n83-prime-terminal-20261030-r1','source_files':[{'path':n,'sha256':sha(s/n)} for n in files],'raw_upload_seconds':30,'retry_policy':'none','backend_reason':'Connected EKS service responded with no duplicate panel Job;CLI AssumeRole denied;use connected service with pinned per-commit image.','proof_boundary':'Native arithmetic and written projection argument;independent review and algebraic formalization pending'}
p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None}
out=s/'protocol-prime-terminal.json';assert not out.exists();out.write_text(json.dumps(p,indent=2)+'\n')
