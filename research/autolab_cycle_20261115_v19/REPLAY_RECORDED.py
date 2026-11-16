"""Standalone exact replay; uses only the two published transparent raw JSONs."""
from pathlib import Path
import hashlib,json
R=Path(__file__).resolve().parents[2]
FILES={'research/k_linear_20261115_v19/raw.json':'d13e1eda936bcf483739a4c24fb1fbb9d2a54fccdcd48c7d2e7f9c39bdcbf3b6','research/k_singular_20261115_v19/raw.json':'638a997bb11cc31117ab5501fd81ef590fea0b7175eb3c49ee28db9e01a80dd7'}
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
for name,sha in FILES.items():
 raw=(R/name).read_bytes();assert hashlib.sha256(raw).hexdigest()==sha;data[name]=json.loads(raw)
first=data['research/k_linear_20261115_v19/raw.json']['out']['actual']['cases']
for c in first:verify(c['rows'],c['solution'],35)
second=data['research/k_singular_20261115_v19/raw.json']['out']
verify(second['intersection_rows'],second['intersection'],69)
assert not second['intersection']['consistent'] and second['cases']==[]
print(json.dumps(dict(conditional_k_fibers_verified=len(first),singular_intersection_inconsistent=True,no_positive_singular_witness_claim=True,independent_method='literal row-combination/kernel replay plus separate rank elimination',requires_original_fixture=False,global_roots_complete=False)))
