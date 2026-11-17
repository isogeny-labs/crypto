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
assert len(index)==1698 and len(plan)==849
counts=dict(cases=0,products=0,archived_products=0,carry_products=0,mutations_rejected=0,equal_wrap_rejections=0,radix_guard_rejections=0,aa_exact=True)
runs=[];aa={}
with (O/'cases.jsonl').open('rb') as f:
 for j,encoded in enumerate(f):
  z=json.loads(encoded);ix=index[j];iid=j%849;arm=['candidate','aa'][j//849];payload=z['payload'];tests=payload['tests'];num=len(tests)
  assert ix['offset']==f.tell()-len(encoded) and ix['bytes']==len(encoded) and ix['sha256']==hashlib.sha256(encoded).hexdigest()
  assert z['arm']==ix['arm']==arm and z['instance_id']==ix['instance_id']==str(iid) and z['replicate']==0 and z['seed']==plan[iid]['seed'] and payload['plan']==plan[iid]
  assert z['products']==num and z['mutations_rejected']==4*num and z['equal_wrap_rejections']==3*num and z['radix_guard_rejections']==(2*num if z['kind']=='carry' else 0) and z['status']=='verified'
  h=hashlib.sha256(json.dumps(payload,sort_keys=True).encode()).hexdigest();assert h==z['aa_sha256']
  if arm=='candidate':aa[iid]=h
  else:assert aa[iid]==h
  counts['cases']+=1;counts['products']+=num;counts[z['kind']+'_products']+=num
  for name in ['mutations_rejected','equal_wrap_rejections','radix_guard_rejections']:counts[name]+=z[name]
  for t in tests:
   c=t['counts'];assert c['coefficient_reductions']==t['n'] and c['integer_multiplications']==4*t['s']+8
   assert c['operand_bitlength_product_sum']==sum(b['a_bits']*b['b_bits'] for b in t['blocks'])
   assert c['product_result_bitlength_sum']==sum(b['product_bits'] for b in t['blocks'])
  runs.append(dict(run_id='fused-'+arm+'-'+str(iid),arm=arm,instance_id=str(iid),seed=z['seed'],replicate=0,warmup=False,status='succeeded',outcome_class='completed',attempts=1,resumed=0,checkpoint=None,measurements=dict(counted=dict(checked_cases=1,products=num,mutations_rejected=z['mutations_rejected']),wall_seconds=None,wall_valid=False,cpu_user_seconds=None,cpu_sys_seconds=None,max_rss_bytes=None,counters={}),isolation=dict(tier='none',contended=None,blockers=['Exact component checks; no clock claims']),verification=dict(status='verified',verifier='Archived dense-verified outputs; independent coefficient and native ledger references'),exclusion=dict(excluded=False,rule=None),artifacts=[art(O/'row-index.json')],started_at=z['started_at'],finished_at=z['finished_at'],worker='local-sequential'))
assert counts==summary==dict(cases=1698,products=34248,archived_products=32328,carry_products=1920,mutations_rejected=136992,equal_wrap_rejections=102744,radix_guard_rejections=3840,aa_exact=True)
for lock in ['source-lock.json','input-lock.json']:
 for path,h in load(D/lock).items():assert sha(D.parent/path)==h,path
assert load(O/'carry-certificate.json')==dict(radix=256,coefficients=[256,1],packed=512,decoded=[0,2],overflow_detected=True)
(O/'recomputed-summary.json').write_text(json.dumps(counts,indent=2)+'\n')
e=dict(schema='experiment.export/v1',export_id='companion-fused-validation',created=datetime.now(timezone.utc).isoformat(),protocol=dict(path='../protocol.json',sha256=p['freeze']['protocol_sha256'],protocol_id=p['protocol_id'],version=p['version']),producer=dict(runtime='codex',model=None,session='companion-fused-validation',role='executor'),source=dict(repo='primefield-ecdlp-research',commit=host['source_commit'],dirty=True),backend='local',host=dict(path='host.json',sha256=art(O/'host.json')['sha256'],class_id='shared-local',summary=dict(cpu_model='x86_64',logical_cpus=host['logical_cpus'],memory_bytes=None,kernel=host['platform'],hypervisor=None,isolation_tier_max='none')),arms=[dict(id=a['id'],role=a['role'],binary_sha256=a['instrument']['binary_sha256'],commit=a['instrument']['commit']) for a in p['arms']],runs=runs,flow=dict(planned=1698,attempted=1698,completed=1698,not_completed=0,interrupted=0,resumed=0,excluded=[]),files=[art(O/n) for n in ['host.json','cases.jsonl','summary.json','row-index.json','recomputed-summary.json','carry-certificate.json']]+[art(D/'protocol.json','protocol'),art(D/'source-lock.json','other'),art(D/'input-lock.json','other'),art(D/'plan.json','other')],analysis=dict(script=art(Path(__file__),'analysis'),blinded=False,blinding_key_sha256=None,outputs=[art(O/'recomputed-summary.json','summary')],unmasked_at=None),notes=[dict(kind='observation',text='All archived products retained; archival baseline explicitly declared. Exact coefficient, output and accounting checks; units separate and no timing inference.',by='executor')],reviews_dir='reviews')
(O/'export.json').write_text(json.dumps(e,indent=2)+'\n');print(counts)
