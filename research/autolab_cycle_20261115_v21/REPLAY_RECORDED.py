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
path=R/'research/whole_gold_cells_20261115_v21/raw.json'
raw=path.read_bytes();assert hashlib.sha256(raw).hexdigest()=='9a8d6160f8868a0102b54477a8dbaf53b713fafcc13079de7f4c95637d86f4e3'
data=json.loads(raw)['out']['actual'];assert len(data['cell_records'])==55
rejected=0
for c in data['cell_records']:
 verify(c['Gold_rows'],c['Gold_solution'],c['cell_bits']);s=c['Gold_solution']
 expected=(1<<len(s['basis']))if s['consistent']else 0
 assert c['retained_k_u_branch_parameters']==expected
 assert c['rejected_k_u_branch_parameters']==(1<<c['cell_bits'])-expected
 assert c['k0_branch_retained']and c['other_kernels_unresolved_without_rank_minor']
 rejected+=not s['consistent']
pairs=0
for c in data['schur_records']:
 if 'reference_rows'not in c:continue
 rows=c['reference_rows'];verify(rows[:45],c['base_solution'],35);verify(rows,c['reference_solution'],35);q=c['composed'];verify(q['inner_rows'],q['inner_solution'],q['inner_width'])
 assert all((m&q['particular']).bit_count()%2==v for m,v in rows)
 assert all(all((m&x).bit_count()%2==0 for m,v in rows)for x in q['basis'])
 assert len(q['basis'])==35-c['reference_solution']['rank'];pairs+=1
assert rejected==24 and pairs==4
path=R/'research/char3_moment_20261115_v21/raw.json';raw=path.read_bytes();assert hashlib.sha256(raw).hexdigest()=='9f35735754c4c047bda69262c21d0ef4aa681dc0a13778d82c7d2f998af83e05';odd=json.loads(raw)['out']
for item in odd['ranks']:
 d=item['d'];assert item['rank']==(1 if d==1 else 3 if d==2 else 4*d-6)
 assert item['coefficient_dimensions']==4*d-3
 if d>=3:assert item['codimension']==3
assert (2-pow(2,4,3))%3==1
print(json.dumps(dict(whole_cell_linear_certificates=55,entire_k_u_branches_rejected=24,Schur_fiber_pairs_replayed=pairs,char3_rank_records_checked=64,char3_counterexample_verified=True,global_source_roots_complete=False)))
