"""Post-run record/seed/count reconciliation; does not execute any operator."""
from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,sys
D=Path(__file__).resolve().parent;R=D.parent
sys.path.insert(0,str(R/'tools'))
from experiment_protocol import seeds_table
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for n in ['source-lock.json','input-lock.json']:
 for f,h in json.loads((D/n).read_text()).items():assert sha(R/f)==h,f
plan=json.loads((D/'plan.json').read_text());p=json.loads((D/'protocol.json').read_text())
table=seeds_table(p);total=dict(verified=True,cases=0,source_cases=0,synthetic_cases=0,matvecs=0,principal_identities=0,inverse_identities=0,singular_rejections=0,equal_wrap_rejections=0,mutations_rejected=0,boundary_queries=0,aa_exact=True);aa={}
for j,line in enumerate((D/'runs/cases.jsonl').open()):
 z=json.loads(line);payload=z['payload'];cell=table[j]
 assert z['arm']==cell['arm'] and z['instance_id']==cell['instance'] and z['seed']==cell['seed'] and z['replicate']==cell['replicate']
 assert payload['plan']==plan[int(z['instance_id'])] and z['status']=='verified'
 inv=int(payload['inverse'] is not None and payload['inverse']['status']=='reference_inverse_transport_verified')
 singular=int(payload['inverse'] is not None and payload['inverse']['status']=='singular_oracle_rejected')
 products=len(payload['tests']);principal=len(payload['restrictions']);guards=3+int(z['kind']=='source');mut=2*products+principal+inv
 assert z['matvecs']==products and z['mutations_rejected']==mut and z['equal_wrap_rejections']==guards
 fingerprint=hashlib.sha256(json.dumps(payload,sort_keys=True).encode()).hexdigest();assert fingerprint==z['aa_sha256']
 if z['arm']=='candidate':aa[z['instance_id']]=fingerprint
 else:assert aa[z['instance_id']]==fingerprint
 total['cases']+=1;total[z['kind']+'_cases']+=1;total['matvecs']+=products;total['principal_identities']+=principal;total['inverse_identities']+=inv;total['singular_rejections']+=singular;total['equal_wrap_rejections']+=guards;total['mutations_rejected']+=mut
 if payload['acquisition'] is not None:total['boundary_queries']+=payload['acquisition']['counts']['boundary_queries']
assert total==json.loads((D/'expected-summary.json').read_text())==json.loads((D/'runs/summary.json').read_text())
assert len(table)==total['cases'] and (D/'exit-code.txt').read_text().strip()=='0'
out=dict(status='verified',observed_at=datetime.now(timezone.utc).isoformat(),scope='Post-run metadata, input bindings, raw nested check counts and AA reconciliation; no operator rerun',source_locks_verified=9,input_locks_verified=1,seed_cells_verified=len(table),summary=total,raw_sha256=sha(D/'runs/cases.jsonl'),protocol_sha256=p['freeze']['protocol_sha256'])
(D/'reconciliation.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
