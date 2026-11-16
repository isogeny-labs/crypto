"""Standalone exact replay; uses only the two published transparent raw JSONs."""
from pathlib import Path
import hashlib,json
R=Path(__file__).resolve().parents[2]
def combo(rows,mask):
 a=b=0
 for i,(m,v) in enumerate(rows):
  if mask>>i&1:a^=m;b^=v
 return a,b
def verify(rows,sol,n):
 for m,v,w in sol['pivots']:assert combo(rows,w)==(m,v)
 piv={}
 for row,rhs in rows:
  while row:
   p=row.bit_length()-1
   if p in piv:row^=piv[p]
   else:piv[p]=row;break
 assert len(piv)==sol['rank'] and len(sol['basis'])==n-len(piv)
 for k in sol['basis']:assert all((m&k).bit_count()%2==0 for m,v in rows)
 kp={}
 for row in sol['basis']:
  while row:
   p=row.bit_length()-1
   if p in kp:row^=kp[p]
   else:kp[p]=row;break
 assert len(kp)==len(sol['basis'])
 if sol['consistent']:assert all((m&sol['particular']).bit_count()%2==v for m,v in rows)
 else:assert combo(rows,sol['contradiction'])==(0,1)
data={}
for name,expected in [('research/kernel_incidence_20261115_v20/raw.json','9dbd415664625b90f7264a38f1a98af4681a8d8942d07973194a23ea4cecdaab'),('research/k_gold_compression_20261115_v20/raw.json','399ef79c2405610ab5935bdce3a8d00efc461642021e8e0b06dbee921d71ed6d')]:
 raw=(R/name).read_bytes();assert hashlib.sha256(raw).hexdigest()==expected;data[name]=json.loads(raw)
atlas=data['research/kernel_incidence_20261115_v20/raw.json']['out']['profile'];bad={}
for record in atlas['records']:
 n=73 if record['family']=='pre-Gold73'else 69;sy=record['intersection'];verify(sy['rows'],sy['solution'],n)
 if not sy['solution']['consistent']:bad.setdefault(record['family'],[]).append(record['u'])
 for point in record['verified_points']:
  if 'full_k_system'in point:verify(point['full_k_system']['rows'],point['full_k_system']['solution'],35)
assert bad['pre-Gold73']==bad['Gold69'] and len(bad['Gold69'])==9
gold=data['research/k_gold_compression_20261115_v20/raw.json']['out']['actual'];systems=0
for record in gold['records']:
 assert record['k0_retained']
 if 'full_systems'in record:
  sy=record['full_systems'];verify(sy['base']['rows'],sy['base']['solution'],35);verify(sy['full_rows'],sy['full_solution'],35);verify(sy['compact_rows'],sy['compact_solution'],35)
  assert sy['full_solution']['particular']==sy['compact_solution']['particular']and sy['full_solution']['basis']==sy['compact_solution']['basis'];systems+=1
assert sum(r['after_nullity']==0 for r in gold['records'])==202
assert sum(r['after_nullity']==1 for r in gold['records'])==18
print(json.dumps(dict(intersection_certificates=128,same_nine_obstructions_before_after_Gold=True,full_Gold_system_pairs_replayed=systems,known_k0_retained_points=220,global_roots_complete=False,method='literal row-combination/kernel replay and separate rank elimination; no original fixture needed')))
