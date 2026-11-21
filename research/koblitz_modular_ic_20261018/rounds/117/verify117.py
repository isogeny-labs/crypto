#!/usr/bin/env python3
"""Independent schoolbook rational expansion for R117 coefficient rows."""
import hashlib
import importlib.util
import json
from pathlib import Path
root=Path(__file__).resolve().parent;rounds=root.parent
r=json.loads((root/'result.json').read_text())
paths={'donor51':rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':rounds/'105/jets105.py','result98':rounds/'98/result.json',
 'result116':rounds/'116/result.json'}
for name,p in paths.items():assert hashlib.sha256(p.read_bytes()).hexdigest()==r['source_sha256'][name]
spec=importlib.util.spec_from_file_location('jets105',paths['parser105'])
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
f,g=m.parse_source(paths['donor51'].read_text())
base=json.loads(paths['result116'].read_text())
P=640;mask=(1<<P)-1

def mul(a,b):
 out=0
 for i in range(P):
  if b>>i&1:out^=a<<i
 return out&mask

def power(a,n):
 out=1
 for _ in range(n):out=mul(out,a)
 return out

def val(a):return None if not a else (a&-a).bit_length()-1

def inv_unit(q,width):
 assert q&1
 z=1
 for k in range(1,width):
  bit=0
  for j in range(1,k+1):bit^=(q>>j&1)&(z>>(k-j)&1)
  if bit:z|=1<<k
 assert mul(q,z)&((1<<width)-1)==1
 return z

for key,b in base['branches'].items():
 x=sum(1<<i for i in b['x_exponents']);y=sum(1<<i for i in b['y_exponents'])
 acc={j:{} for j in (0,1,4,8,16)};bounds={j:P for j in acc}
 for le,ye,den,exps in g:
  a=0
  for xe in exps:a^=power(x,xe)
  a=mul(a,power(y,ye));q=power(x,den)
  n,d=val(a),val(q)
  assert d==den*b['x_order']<P
  if n is None:
   bounds[le]=min(bounds[le],P-d);continue
  width=P-max(n,d);offset=n-d;until=offset+width
  bounds[le]=min(bounds[le],until)
  ratio=mul(a>>n,inv_unit(q>>d,width))&((1<<width)-1)
  for k in range(width):
   if ratio>>k&1:
    ex=offset+k;acc[le][ex]=acc[le].get(ex,0)^1
 for row in r['branches'][key]['coefficient_rows']:
  j=row['lambda_exponent'];s=sorted(ex for ex,v in acc[j].items() if v and ex<bounds[j])
  assert row['known_until_exclusive']==bounds[j]
  assert row['nonzero_support_below_bound']==s
  assert row['s_order']==(s[0] if s else None)
  assert row['s_order_lower_bound']==(s[0] if s else bounds[j])
 print(key,'independent coefficient rows match')
assert [r['branches'][k]['newton_lower_hull_if_exact'] is not None for k in ('y0_e1','y0_e3','y1_e1','y1_e2')]==[True,False,True,False]
print('GCOEFF117_INDEPENDENT_SCHOOLBOOK_PASS')
