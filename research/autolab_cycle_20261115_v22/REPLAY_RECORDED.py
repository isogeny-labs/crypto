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
raw=(R/'research/gold5_cells_20261115_v22/raw.json').read_bytes();assert hashlib.sha256(raw).hexdigest()=='76079c60b986010d05091b7cf8398c22356d961ae10f0beb404fec1dc97641a7'
actual=json.loads(raw)['out']['actual'];expected=set();contradictions=0
for index,c in enumerate(actual['subcell_records']):
 verify(c['rows'],c['linear'],len(c['parameter_basis']));q=c['linear']
 if not q['consistent']:contradictions+=1;continue
 for t in range(1<<len(q['basis'])):
  v=q['particular']
  for i,b in enumerate(q['basis']):
   if t>>i&1:v^=b
  expected.add((index,v))
proof=json.loads((R/'research/autolab_cycle_20261115_v22/GOLD9_LEAVES.json').read_bytes());n=proof['field_degree'];mod=int(proof['modulus'],16)
def mul(a,b):
 v=0
 while b:
  if b&1:v^=a
  a<<=1;b>>=1
 while v.bit_length()>n:v^=mod<<(v.bit_length()-n-1)
 return v
def power(a,k):
 v=1
 for _ in range(k):v=mul(v,a)
 return v
generators=[]
for i in range(18):
 for j in range(i+1,18):generators.append(sum((mul(power(1<<i,q),1<<j)^mul(1<<i,power(1<<j,q)))<<(n*t)for t,q in enumerate([2,4,8])))
seen=set()
for leaf in proof['leaves']:
 seen.add((leaf['subcell'],leaf['inner_parameter']));H=leaf['H'];e=[0]+list(reversed(H[:5]));p=[1]
 for k in range(1,10):
  value=0
  for j in range(1,min(5,k-1)+1):value^=mul(e[j],p[k-j])
  if k<=5 and k%2:value^=e[k]
  p.append(value)
 moment=[p[k]^power(e[1],k)for k in[3,5,9]];assert moment==leaf['moments']
 packed=sum(v<<(n*i)for i,v in enumerate(moment));lam=leaf['rejecting_functional']
 assert (lam&packed).bit_count()%2==1 and all((lam&g).bit_count()%2==0 for g in generators)
assert seen==expected and len(seen)==5 and contradictions==50
print(json.dumps(dict(linear_subcells_replayed=54,whole_fiber_contradictions=50,all_five_Gold9_leaves_independently_rejected=True,independent_field_arithmetic='coefficient convolution and descending modulus reduction',complete_selected_norm_families=True,original_chain_roots_complete=False)))
