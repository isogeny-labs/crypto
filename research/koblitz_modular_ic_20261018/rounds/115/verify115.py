#!/usr/bin/env python3
"""Independent direct-source support and schoolbook series check for R115."""
import hashlib
import importlib.util
import json
from pathlib import Path
root=Path(__file__).resolve().parent;rounds=root.parent
r=json.loads((root/'result.json').read_text())
paths={'donor51':rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':rounds/'105/jets105.py','result113':rounds/'113/result.json'}
for name,p in paths.items():assert hashlib.sha256(p.read_bytes()).hexdigest()==r['source_sha256'][name]
spec=importlib.util.spec_from_file_location('jets105',paths['parser105'])
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
f,g=m.parse_source(paths['donor51'].read_text())

def pmul(a,b):
 out=set()
 for xa,ta in a:
  for xb,tb in b:
   p=(xa+xb,ta+tb)
   if p in out:out.remove(p)
   else:out.add(p)
 return out

def ppow(a,n):
 out={(0,0)}
 for _ in range(n):out=pmul(out,a)
 return out

y={(0,0),(1,0),(2,0),(3,1)}
terms=set()
for ye,exps in f:
 yp=ppow(y,ye)
 for xe in exps:
  for xp,tp in yp:
   p=(xe+xp,tp)
   if p in terms:terms.remove(p)
   else:terms.add(p)
assert all(xe>=8 for xe,te in terms)
E=sorted((te,xe-8) for xe,te in terms)
assert [list(p) for p in E]==r['E_support'] and len(E)==34
assert [te for te,xe in E if xe==0]==[2] and (0,1) in E
P=128;mask=(1<<P)-1

def bmul(a,b):
 out=0
 for i in range(P):
  if b>>i&1:out^=a<<i
 return out&mask

def bpow(a,n):
 out=1
 for _ in range(n):out=bmul(out,a)
 return out

x=sum(1<<i for i in r['x_exponents'])
out=0
for te,xe in E:out^=bpow(x,xe)<<te
assert out&mask==0
assert (x&3)==0 and x>>2&1
v=bmul(x,2);assert v>>3&1 and not(v&7)
y=1^x^bpow(x,2)^bmul(bpow(x,3),2)
assert [i for i in range(P) if y>>i&1]==r['y_exponents']
original=0
for ye,exps in f:
 c=0
 for xe in exps:c^=bpow(x,xe)
 original^=bmul(c,bpow(y,ye))
assert original==0
print('UNIFORMIZER115_INDEPENDENT_SOURCE_PASS')
