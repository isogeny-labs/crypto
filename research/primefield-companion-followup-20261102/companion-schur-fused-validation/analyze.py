from pathlib import Path
from datetime import datetime,timezone
from functools import lru_cache
import json,hashlib,os
D=Path(__file__).resolve().parent;O=D/'runs';load=lambda p:json.loads(p.read_text())
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
@lru_cache(maxsize=None)
def art(p,kind='raw'):return dict(path=os.path.relpath(p,O),sha256=sha(p),bytes=p.stat().st_size,kind=kind)
p=load(D/'protocol.json');summary=load(O/'summary.json');host=load(O/'host.json');plan=load(D/'plan.json');index=load(O/'row-index.json')
assert len(index)==642 and len(plan)==321
counts=dict(verified=True,cases=0,source_cases=0,synthetic_cases=0,valid_schur_cases=0,obstruction_cases=0,synthetic_valid=0,synthetic_obstructions=0,full_invertible_obstructions=0,directional_checks=0,mutations_rejected=0,equal_wrap_rejections=0,lift_fixtures=0,normalized_witnesses=0,lift_obstructions=0,lift_mutations_rejected=0,aa_exact=True)
runs=[];aa={}
with (O/'cases.jsonl').open('rb') as f:
 for j,encoded in enumerate(f):
  z=json.loads(encoded);ix=index[j];iid=j%321;arm=['candidate','aa'][j//321];payload=z['payload'];valid=z['outcome']=='schur_verified';tests=len(payload['tests']);mut=2+4*tests if valid else 1
  assert ix['offset']==f.tell()-len(encoded) and ix['bytes']==len(encoded) and ix['sha256']==hashlib.sha256(encoded).hexdigest()
  assert z['arm']==ix['arm']==arm and z['instance_id']==ix['instance_id']==str(iid) and z['replicate']==0 and z['seed']==plan[iid]['seed'] and payload['plan']==plan[iid]
  assert z['directional_checks']==tests and z['mutations_rejected']==mut and z['status']=='verified'
  h=hashlib.sha256(json.dumps(payload,sort_keys=True).encode()).hexdigest();assert h==z['aa_sha256']
  if arm=='candidate':aa[iid]=h
  else:assert aa[iid]==h
  counts['cases']+=1;counts[z['kind']+'_cases']+=1;counts['valid_schur_cases' if valid else 'obstruction_cases']+=1
  if z['kind']=='synthetic':counts['synthetic_valid' if valid else 'synthetic_obstructions']+=1
  counts['full_invertible_obstructions']+=int(payload['plan'].get('matrix')=='swap_obstruction')
  counts['directional_checks']+=tests;counts['mutations_rejected']+=mut;counts['equal_wrap_rejections']+=1
  lifts=payload['lifts'];cut=payload['plan']['cut'];N=payload['plan'].get('N',payload['plan']['r']);m=N-cut
  if valid:
   assert [t['label'] for t in lifts[:3]]==['zero','ones','seeded'] and len(lifts) in [3,4]
   if len(lifts)==4:assert lifts[3]['label']=='normalized' and lifts[3]['certificate']['normalized_exclusion']
   counts['lift_fixtures']+=len(lifts)
   counts['normalized_witnesses']+=len(lifts)-3
   for t in lifts:
    normalized=t['label']=='normalized';assert t['mutations_rejected']==4+2*normalized
    counts['lift_mutations_rejected']+=t['mutations_rejected']
    assert t['oracle_work']==dict(forward_calls=2,transpose_calls=1,dense_terms=3*cut*cut,coefficient_reductions=3*cut)
    assert len(t['source_receipts'])==3 and [v['transpose'] for v in t['source_receipts']]==[False,False,True]
    assert t['certificate']['primal_residual']==[0]*cut+t['certificate']['reduced_residual']
    assert t['certificate']['dual_residual']==[0]*cut+t['certificate']['reduced_dual_residual']
    assert t['certificate']['full_pairing']==t['certificate']['reduced_pairing']
    if normalized:assert t['certificate']['full_pairing']==1 and not any(t['certificate']['dual_residual'])
  else:
   assert [t['adapter'] for t in lifts]==['rhs','primal','dual']
   assert [len(t['source_receipts']) for t in lifts]==[0,1,1]
   assert all(t['full_matrix_rank']=='undetermined' and t['oracle_work']['dense_terms']==0 for t in lifts)
   counts['lift_obstructions']+=3

  runs.append(dict(run_id='schur-'+arm+'-'+str(iid),arm=arm,instance_id=str(iid),seed=z['seed'],replicate=0,warmup=False,status='succeeded',outcome_class='completed',attempts=1,resumed=0,checkpoint=None,measurements=dict(counted=dict(checked_cases=1,directional_checks=tests,leading_obstruction=int(not valid),mutations_rejected=mut),wall_seconds=None,wall_valid=False,cpu_user_seconds=None,cpu_sys_seconds=None,max_rss_bytes=None,counters={}),isolation=dict(tier='none',contended=None,blockers=['Exact component checks; no clock claims']),verification=dict(status='verified',verifier='Independent dense Schur or certified leading nullspace,exact source/oracle ledgers and raw row hashes'),exclusion=dict(excluded=False,rule=None),artifacts=[art(O/'row-index.json')],started_at=z['started_at'],finished_at=z['finished_at'],worker='local-sequential'))
assert counts==summary and counts['cases']==642 and counts['source_cases']==162 and counts['synthetic_cases']==480 and counts['synthetic_valid']==288 and counts['synthetic_obstructions']==192 and counts['full_invertible_obstructions']==96
for lock in ['source-lock.json','input-lock.json']:
 for path,h in load(D/lock).items():assert sha(D.parent/path)==h,path
(O/'recomputed-summary.json').write_text(json.dumps(counts,indent=2)+'\n')
e=dict(schema='experiment.export/v1',export_id='companion-schur-fused-validation',created=datetime.now(timezone.utc).isoformat(),protocol=dict(path='../protocol.json',sha256=p['freeze']['protocol_sha256'],protocol_id=p['protocol_id'],version=p['version']),producer=dict(runtime='codex',model=None,session='companion-schur-fused-validation',role='executor'),source=dict(repo='primefield-ecdlp-research',commit=host['source_commit'],dirty=True),backend='local',host=dict(path='host.json',sha256=art(O/'host.json')['sha256'],class_id='shared-local',summary=dict(cpu_model='x86_64',logical_cpus=host['logical_cpus'],memory_bytes=None,kernel=host['platform'],hypervisor=None,isolation_tier_max='none')),arms=[dict(id=a['id'],role=a['role'],binary_sha256=a['instrument']['binary_sha256'],commit=a['instrument']['commit']) for a in p['arms']],runs=runs,flow=dict(planned=642,attempted=642,completed=642,not_completed=0,interrupted=0,resumed=0,excluded=[]),files=[art(O/n) for n in ['host.json','cases.jsonl','summary.json','row-index.json','recomputed-summary.json']]+[art(D/'protocol.json','protocol'),art(D/'source-lock.json','other'),art(D/'input-lock.json','other'),art(D/'plan.json','other')],analysis=dict(script=art(Path(__file__),'analysis'),blinded=False,blinding_key_sha256=None,outputs=[art(O/'recomputed-summary.json','summary')],unmasked_at=None),notes=[dict(kind='observation',text='Full original Schur cohort and vector schedules retained. Added primal/dual residual and normalized witness checks; kernel descriptor correctness is supplied by the separately completed locked kernel panel. Leading oracle costs remain explicit.',by='executor')],reviews_dir='reviews')
(O/'export.json').write_text(json.dumps(e,indent=2)+'\n');print(counts)
