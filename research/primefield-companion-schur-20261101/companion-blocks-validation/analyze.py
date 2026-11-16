from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,os
from functools import lru_cache
D=Path(__file__).resolve().parent;O=D/'runs';load=lambda p:json.loads(p.read_text())
@lru_cache(maxsize=None)
def art(p,kind='raw'):return dict(path=os.path.relpath(p,O),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),bytes=p.stat().st_size,kind=kind)
p=load(D/'protocol.json');s=load(O/'summary.json');host=load(O/'host.json');rows=[json.loads(x) for x in (O/'cases.jsonl').read_text().splitlines()]
assert s==load(D/'expected-summary.json')
assert len(rows)==738 and {(z['arm'],z['instance_id'],z['replicate']) for z in rows}=={(a,str(i),0) for a in ['candidate','aa'] for i in range(369)}
assert all(z['status']=='verified' for z in rows)
index=load(O/'row-index.json');raw=(O/'cases.jsonl').read_bytes();assert len(index)==len(rows)
for z,i in zip(rows,index):
 part=raw[i['offset']:i['offset']+i['bytes']]
 assert hashlib.sha256(part).hexdigest()==i['sha256'] and json.loads(part)==z
 assert z['arm']==i['arm'] and z['instance_id']==i['instance_id']
del raw
runs=[]
for z in rows:
 runs.append(dict(run_id='dual-'+z['arm']+'-'+z['instance_id']+'-'+str(z['replicate']),arm=z['arm'],instance_id=z['instance_id'],seed=z['seed'],replicate=z['replicate'],warmup=False,status='succeeded',outcome_class='completed',attempts=1,resumed=0,checkpoint=None,measurements=dict(counted=dict(checked_cases=1,matvecs=z['matvecs'],mutations_rejected=z['mutations_rejected']),wall_seconds=None,wall_valid=False,cpu_user_seconds=None,cpu_sys_seconds=None,max_rss_bytes=None,counters={}),isolation=dict(tier='none',contended=None,blockers=['Exact operator panel; no timing claims']),verification=dict(status='verified',verifier='Independent dense products and displacement identities; inverse identities, boundary queries and exact ledgers'),exclusion=dict(excluded=False,rule=None),artifacts=[art(O/'row-index.json')],started_at=z['started_at'],finished_at=z['finished_at'],worker='local-sequential'))
e=dict(schema='experiment.export/v1',export_id='companion-blocks-validation',created=datetime.now(timezone.utc).isoformat(),protocol=dict(path='../protocol.json',sha256=p['freeze']['protocol_sha256'],protocol_id=p['protocol_id'],version=p['version']),producer=dict(runtime='codex',model=None,session='companion-blocks-validation',role='executor'),source=dict(repo='primefield-ecdlp-research',commit=host['source_commit'],dirty=True),backend='local',host=dict(path='host.json',sha256=art(O/'host.json')['sha256'],class_id='shared-local',summary=dict(cpu_model='x86_64',logical_cpus=host['logical_cpus'],memory_bytes=None,kernel=host['platform'],hypervisor=None,isolation_tier_max='none')),arms=[dict(id=a['id'],role=a['role'],binary_sha256=a['instrument']['binary_sha256'],commit=a['instrument']['commit']) for a in p['arms']],runs=runs,flow=dict(planned=738,attempted=738,completed=738,not_completed=0,interrupted=0,resumed=0,excluded=[]),files=[art(O/n) for n in ['host.json','cases.jsonl','summary.json','row-index.json']]+[art(D/'protocol.json','protocol'),art(D/'source-lock.json','other'),art(D/'input-lock.json','other'),art(D/'plan.json','other'),art(D/'expected-summary.json','other')],analysis=dict(script=art(Path(__file__),'analysis'),blinded=False,blinding_key_sha256=None,outputs=[art(O/'summary.json','summary')],unmasked_at=None),notes=[dict(kind='observation',text='All81source and288synthetic contexts perarm; general dimension, two wraps, principal correction and oracle-based inverse transport. No fast solver or new elliptic recovery executed.',by='executor')],reviews_dir='reviews')
(O/'export.json').write_text(json.dumps(e,indent=2)+'\n');print('DUAL_EXPORTED',s)
