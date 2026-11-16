"""Independent standard-library replay of recorded finite-pencil certificates."""
from pathlib import Path
import hashlib,json
R=Path(__file__).resolve().parents[2]
p=R/'research/kernel_strata_20261115_v24/raw.json'
assert hashlib.sha256(p.read_bytes()).hexdigest()=='ec4a93f784e91160aa8e68438cc67e5b83e90b785d558258a54dc2574039b46d'
a=json.loads(p.read_text())['out']['actual']
p=R/'research/kernel_class_20261115_v23/raw.json'
assert hashlib.sha256(p.read_bytes()).hexdigest()=='8d52c93409bc65b5030aea2ad5985984723ffd75da207b5f639591ed2980e9d7'
families=json.loads(p.read_text())['out']['actual']['families']
def assemble(tables,u):
 rows=[(0,0)for _ in tables[0]]
 for i,table in enumerate(tables):
  if u>>i&1:rows=[(m^v,r^s)for(m,r),(v,s)in zip(rows,table)]
 return rows

def feasible(rows):
 piv={}
 for m,r in rows:
  while m:
   i=m.bit_length()-1
   if i not in piv:piv[i]=(m,r);break
   v,s=piv[i];m^=v;r^=s
  if not m and r:return False
 return True

def dual_check(rows,lam):
 m=r=0
 for j,(v,s)in enumerate(rows):
  if lam>>j&1:m^=v;r^=s
 assert m==0
 return r

for f,result in zip(families,a['families']):
 assert f['family']==result['family']
 tables=f['basis_tables']
 truth={u for u in range(512)if feasible(assemble(tables,u))}
 assert len(truth)==372
 for cell in result['strata']:
  d=cell['degree'];domain=range(1<<d,1<<(d+1))
  blocks=[assemble(tables,1<<i)for i in range(d+1)]
  for v in cell['duals']['vectors']:
   syndrome=sum(dual_check(rows,v['multiplier'])<<i for i,rows in enumerate(blocks))
   assert syndrome==v['syndrome']
  rows=[(v['syndrome']&((1<<d)-1),(v['syndrome']>>d)&1)for v in cell['duals']['vectors']]
  kept={u for u in domain if all(((m& (u^(1<<d))).bit_count()&1)==r for m,r in rows)}
  assert kept==set(cell['candidate_directions'])
  actual=truth&set(domain)
  assert actual==set(cell['actual_consistent']) and actual<=kept
  assert cell['exact_stratum']==(actual==kept)
 low=truth&set(range(64))
 assert low=={0,10,20,30,35,41,55,61}
 assert low=={(10 if i&1 else 0)^(20 if i&2 else 0)^(35 if i&4 else 0)for i in range(8)}
 assert all(all((u&m).bit_count()%2==0 for m in [20,33,42])for u in low)
 assert 20 in truth and 40 not in truth and 35 in truth
print('VERIFIED: both 512-direction systems; every stratum dual, candidate set and exactness flag; low6 subspace and divisibility counterexample.')
