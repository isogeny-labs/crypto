#!/usr/bin/env python3
"""Independent source expansion and schoolbook bit-series check of R116."""
import hashlib
import importlib.util
import json
from pathlib import Path
root=Path(__file__).resolve().parent;rounds=root.parent
r=json.loads((root/'result.json').read_text())
paths={'donor51':rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'parser105':rounds/'105/jets105.py','result98':rounds/'98/result.json',
 'result111':rounds/'111/result.json','result112':rounds/'112/result.json',
 'result115':rounds/'115/result.json'}
for name,p in paths.items():assert hashlib.sha256(p.read_bytes()).hexdigest()==r['source_sha256'][name]
spec=importlib.util.spec_from_file_location('jets105',paths['parser105'])
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
f,g=m.parse_source(paths['donor51'].read_text())
P=640;MASK=(1<<P)-1

def mul(a,b):
 out=0
 for i in range(P):
  if b>>i&1:out^=a<<i
 return out&MASK

def power(a,n):
 out=1
 for _ in range(n):out=mul(out,a)
 return out

def residual(x,y):
 out=0
 for ye,exps in f:
  a=0
  for xe in exps:a^=power(x,xe)
  out^=mul(a,power(y,ye))
 return out

def eval_eq(terms,z):
 out=0
 for se,ze in terms:out^=power(z,ze)<<se
 return out&MASK

# Independent term generation: direct monomial weights for y=0, Lucas choices for y=1.
for key,b in r['branches'].items():
 terms=[tuple(p) for p in b['equation_support']]
 if key=='y0_e1':
  source_terms=set((xe+ye-2,ye) for ye,exps in f for xe in exps)
 elif key=='y0_e3':
  source_terms=set((3*xe+ye-4,ye) for ye,exps in f for xe in exps)
 elif key=='y1_e1':
  source_terms=set()
  for ye,exps in f:
   choices=[[(0,0),(1<<bit,0),(6<<bit,1<<bit)] for bit in range(3) if ye>>bit&1]
   combos=[(0,0)]
   for group in choices:combos=[(a+c,b+d) for a,b in combos for c,d in group]
   for xe in exps:
    for se,ze in combos:
     p=(xe+se-10,ze)
     if p in source_terms:source_terms.remove(p)
     else:source_terms.add(p)
 else:
  assert key=='y1_e2'
  source_terms=set(tuple(p) for p in json.loads(paths['result115'].read_text())['E_support'])
 assert sorted(source_terms)==sorted(terms)
 z=sum(1<<i for i in b['root_exponents'])
 x=sum(1<<i for i in b['x_exponents'])
 y=sum(1<<i for i in b['y_exponents'])
 assert eval_eq(terms,z)==0 and residual(x,y)==0
 assert (x&-x).bit_length()-1==b['x_order']
 assert b['denominator_pullback_order']==161*b['x_order']<640
 if key=='y0_e1':assert x==2 and y==mul(2,z)
 if key=='y0_e3':assert x==8 and y==mul(2,z)
 if key=='y1_e1':assert x==2 and y==1^2^mul(1<<6,z)
 if key=='y1_e2':
  old=json.loads(paths['result115'].read_text())
  assert [i for i in range(128) if x>>i&1]==old['x_exponents']
  assert [i for i in range(128) if y>>i&1]==old['y_exponents']
  assert y==1^x^power(x,2)^mul(power(x,3),2)
 print(key,'source residual zero mod s^640','x order',b['x_order'])
assert set(r['branches'])=={'y0_e1','y0_e3','y1_e1','y1_e2'}
print('SERIES116_INDEPENDENT_SOURCE_PASS')
