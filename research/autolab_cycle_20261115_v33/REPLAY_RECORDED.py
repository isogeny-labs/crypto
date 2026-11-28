"""Independent centroid-domain/rank and compact aggregate certificate replay."""
from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parents[2]/'research/centroid_cover_20261115_v33/raw.json';data=p.read_bytes();assert hashlib.sha256(data).hexdigest()=='1cae9d46d578598c9ed9302d453e7eebc0e4976706cab3bee4bff7d5bc832723';raw=json.loads(data);assert raw['error']is None;out=raw['out']
def xor(vs):
 x=0
 for v in vs:x^=v
 return x
def mul(a,b,n,mod):
 v=xor(a<<i for i in range(n)if b>>i&1)
 while v.bit_length()>n:v^=mod<<(v.bit_length()-n-1)
 return v
def trace(x,n,mod):
 y=0
 for _ in range(n):y^=x;x=mul(x,x,n,mod)
 assert y in(0,1);return y
def rank(vs):
 piv={}
 for v in vs:
  while v:
   j=v.bit_length()-1
   if j not in piv:piv[j]=v;break
   v^=piv[j]
 return len(piv)
profiles=systems=states=empty=degenerate=0;dims=set()
for n,mod in [(2,7),(3,11),(4,19),(5,37)]:
 Vs=[tuple(1<<i for i in range(d))for d in range(min(n,3)+1)]
 for V in [(1,1<<(n-1)),(1<<(n-1),)]:
  if V not in Vs:Vs.append(V)
 for V in Vs:
  D={xor(v for i,v in enumerate(V)if t>>i&1)for t in range(1<<len(V))};profiles+=1;r=int(any(trace(v,n,mod)for v in V))
  for z in range(1<<n):
   truth=[h for h in range(1<<n)if mul(h,h,n,mod)^h^z in D];systems+=1;states+=1<<n
   count=0 if not r and trace(z,n,mod)else 1<<(len(V)+1-r);assert len(truth)==count
   if not truth:empty+=1
   else:dimension=count.bit_length()-1;degenerate+=dimension!=len(V);dims.add((n,len(V),r,dimension))
c=out['controls'];assert (profiles,systems,states,empty,degenerate)==(22,352,8128,76,76)and c['dimension_profiles']==[list(v)for v in sorted(dims)]
a=out['actual'];n=a['field_degree'];mod=a['modulus'];z=a['target_z'];d=a['domain'];B=d['basis'];part=d['particular'];cols=[mul(1<<i,1<<i,n,mod)^(1<<i)for i in range(n)];rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(cols)),(z>>j)&1)for j in range(18,83)]
assert rows==[tuple(v)for v in d['rows']] and  rank(m for m,r in rows)==65 and  rank(B)==18 and  len(B)==18 and  d['count']==262144
assert all((m&part).bit_count()%2==r for m,r in rows)and all(all((m&v).bit_count()%2==0 for m,r in rows)for v in B)
L=d['linearized_coefficients']
def evaluate(x):
 v=0
 for c in L:v^=mul(c,x,n,mod);x=mul(x,x,n,mod)
 return v
assert len(L)==19 and  L[-1]==1 and  L[0] and  all(evaluate(v)==0 for v in B)
constant=evaluate(part);assert constant==d['constant']==d['total_product'] and  constant and mul(d['total_inverse_sum'],constant,n,mod)==L[0]
assert all((m&a['known_fixed_h']).bit_count()%2==r for m,r in rows)
print('VERIFIED:352complete tiny domains; actualrank65/kernel18/full262144hcover; nonzero and compactproduct/inverse-sum certificates.')
