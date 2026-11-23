from itertools import combinations_with_replacement
from pathlib import Path
import json,hashlib
from .kernel import pmul,rem,poly,code,apply

def build(P):
 d=json.loads((P/'DATA.json').read_text());raw=(P/'snapshot/MATRIX.txt').read_bytes();assert hashlib.sha256(raw).hexdigest()==d['matrix_sha256'];rows=[[int(x,16)for x in s.split()]for s in raw.decode().splitlines()[1:]];base=rows[0];hc=rows[1:];assert len(hc)==73 and all(v[4]==0 for v in hc);E=[0]*76;eb=0;offset=0;tables=[];widths=[];details=[];M=d['modulus']
 for index,(s,m)in enumerate(d['residue_fields']):
  mask=(1<<(4*s))-1;cols=[code(v,s,m)for v in hc]+[rem(pmul(1<<j,M),m)for j in range(3)];bb=code(base,s,m);trace=rem(base[4],m);allowed=set()
  for xs in combinations_with_replacement(range(1<<s),5):
   value=0
   for x in xs:value^=x
   if value!=trace:continue
   coeff=poly(xs,m);assert coeff[4]==trace;allowed.add(sum(v<<(s*j)for j,v in enumerate(coeff[:4])))
  expected=__import__('math').comb((1<<s)+4,5)//(1<<s);assert len(allowed)==expected;details.append({'s':s,'m':m,'trace':trace,'count':len(allowed),'sha256':hashlib.sha256(json.dumps(sorted(allowed)).encode()).hexdigest()})
  if index<7:
   for j,v in enumerate(cols):E[j]^=v<<offset
   eb^=bb<<offset;offset+=4*s;widths.append(4*s);tables.append(allowed)
  else:F=cols;fb=bb;last=allowed
 assert offset==72
 old=json.loads((P/'snapshot/research__residue_cover_certificate_20261116_v84b__raw.json').read_text())['out'];assert all(apply(E,v)==1<<i for i,v in enumerate(old['right_inverse']));assert all(apply(E,v)==0 for v in old['kernel_basis'])
 return E,eb,widths,tables,F,fb,last,details
