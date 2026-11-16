#!/usr/bin/env sage
"""Exact PGL2(F3) transformations among the twelve trace +22 carriers."""
import hashlib
import itertools
import json
from pathlib import Path

F=GF(3);R=PolynomialRing(F,'x');x=R.gen()
masks=(32280,35349,36380,51009,67186,106121,
       112490,113379,127268,168265,175537,175771)
def carrier(mask):
 return x^11+sum(F((mask//(3^i))%3)*x^i for i in range(11))
def mask_of(g):
 return sum(int(g[i])*3^i for i in range(11))

matrices={}
for a,b,c,d in itertools.product(F,repeat=4):
 if a*d-b*c==0:continue
 v=(a,b,c,d);unit=next(z for z in v if z!=0)
 t=tuple(int(z/unit) for z in v)
 matrices[t]=t
assert len(matrices)==24

edges={m:{} for m in masks}
for m in masks:
 f=carrier(m)
 for a,b,c,d in matrices.values():
  A=F(a)*x+F(b);B=F(c)*x+F(d)
  # Homogeneous branch form has degree 12 and equals Z*f(X/Z).
  transformed=sum(f[i]*A^i*B^(12-i) for i in range(12))
  if transformed.degree()==11 and transformed[11]==1:
   n=mask_of(transformed)
   if n in edges:
    assert transformed==carrier(n)
    edges[m][n]=[a,b,c,d]

components=[];unseen=set(masks)
while unseen:
 start=min(unseen);todo={start};component=set()
 while todo:
  m=todo.pop()
  if m in component:continue
  component.add(m);todo.update(set(edges[m])-component)
 components.append(sorted(component));unseen-=component
assert components==[
 [32280,36380,106121,113379,168265,175771],
 [35349,51009,67186,112490,127268,175537]]
assert all(len(edges[m])==6 for m in masks)
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.trace22.pgl2-carrier-orbits/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field':'F3','pgl2_matrices':24,'carrier_masks':list(masks),
                  'components':components,
                  'representative_maps':{
                   str(m):{str(n):edges[m][n] for n in components[i]}
                   for i,m in enumerate((32280,35349))}},sort_keys=True))
