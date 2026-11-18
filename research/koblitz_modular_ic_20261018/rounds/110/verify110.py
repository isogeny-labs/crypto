#!/usr/bin/env python3
"""Independent Hasse/schoolbook check of the R110 degree-7 lift."""
import hashlib
import importlib.util
import json
from pathlib import Path

root=Path(__file__).resolve().parent
rounds=root.parent
r=json.loads((root/'result.json').read_text())
protocol=json.loads((root/'protocol.json').read_text())
paths={
 'donor51': rounds/'51/runs/base23-struct51-03/marked23-triangular51.sing',
 'audit102': rounds/'102/audit102.py', 'result107': rounds/'107/result.json',
 'result108': rounds/'108/result.json', 'result109': rounds/'109/result.json',
 'norm109': rounds/'109/norm109.py',
 'jets105': rounds/'105/jets105.py',
}
for name in protocol['input_sha256']:
 assert hashlib.sha256(paths[name].read_bytes()).hexdigest()==protocol['input_sha256'][name]
assert hashlib.sha256(paths['jets105'].read_bytes()).hexdigest()=='4306bbc6a7c5de1b21716a5fc5a1fd1a8030ff1340409c814c39d6217292a3e4'

def load(name,path):
 spec=importlib.util.spec_from_file_location(name,path)
 m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

v109=load('verify109',rounds/'109/verify109.py')
jets=load('jets105',paths['jets105'])
_,gterms=jets.parse_source(paths['donor51'].read_text())
assert len(gterms)==29
P=40

def fmul(a,b):
 x=0
 for i in range(2):
  if b>>i&1:x^=a<<i
 if x&4:x^=7
 return x

def pmul(a,b):
 out=[0]*P
 for i,x in enumerate(a):
  for j,y in enumerate(b[:P-i]):
   if x and y:out[i+j]^=fmul(x,y)
 return out

def ppow(a,n):
 out=[1]+[0]*(P-1)
 while n:
  if n&1:out=pmul(out,a)
  a=pmul(a,a);n//=2
 return out

def compose(terms,v):
 out=[0]*P
 for j,c in terms.items():
  a=pmul(c,ppow(v,j))
  out=[x^y for x,y in zip(out,a)]
 return out

base=json.loads(paths['result108'].read_text())
edge=json.loads(paths['result107'].read_text())
for key,shift,expect in (('y0',0,35),('y1',1,28)):
 z=sum(1<<i for i in base[key]['z_exponents'])
 t=v109.mul(2,z);y=z^shift;x=1^t
 xinv=v109.inverse(x,128)
 g={j:0 for j in (0,1,4,8,16)}
 for le,ye,den,exps in gterms:
  num=0
  for e in exps:num^=v109.pow2(x,e)
  g[le]^=v109.mul(v109.mul(num,v109.pow2(xinv,den)),v109.pow2(y,ye))
 tr={0:(g[0]^g[8],g[1]^g[4]^g[8]^g[16])}
 for j in (1,4,8,16):tr[j]=(g[j],0)
 gt={}
 for row in edge[key]['translated_rows']:
  j=row['u_exponent'];assert tr[j][0]&((1<<32)-1)==sum(1<<e for e in row['one_support'])
  assert tr[j][1]&((1<<32)-1)==sum(1<<e for e in row['alpha_support'])
  a=[0]*P
  for part,scalar in ((tr[j][0],1),(tr[j][1],2)):
   for e in range(128):
    k=7*e+23*j-184
    if part>>e&1 and 0<=k<P:a[k]^=scalar
  gt[j]=a
 v=[0]*P
 for i,a in r[key]['v_nonzero']:
  assert i<P;v[i]=a
 assert v[0]==1 and compose(gt,v)==[0]*P

 nc={j:{} for j in range(15)};nb=[]
 for le,ye,pbits,qbits in v109.terms:
  a=v109.mul(v109.eval_xpoly(pbits,t),v109.pow2(y,ye))
  b=v109.eval_xpoly(qbits,t)
  n,d=v109.val(a),v109.val(b)
  assert n is not None and d is not None
  width=128-max(n,d)
  ratio=v109.mul(a>>n,v109.inverse(b>>d,width))&((1<<width)-1)
  for j in range(le+1):
   if j&~le:continue
   nb.append(7*(n-d+width)+23*j)
   scalar=v109.alpha_power(le-j)
   for k in range(width):
    if ratio>>k&1:
     e=n-d+k;nc[j][e]=nc[j].get(e,0)^scalar
 assert min(nb)==r[key]['n_precision_bound_r'] and min(nb)>P
 nt={}
 for j,c in nc.items():
  a=[0]*P
  for e,scalar in c.items():
   k=7*e+23*j
   if scalar and k<0:raise ValueError('negative weighted order')
   if scalar and k<P:a[k]^=scalar
  nt[j]=a
 n=compose(nt,v)
 assert all(a==0 for a in n[:expect]) and n[expect]==2
 assert r[key]['n_valuation']==expect
 assert r[key]['n_nonzero']==[[i,a] for i,a in enumerate(n) if a]
 print(key,'valuation',expect,'G residual zero mod r^40','N leading scalar',n[expect])
print('LIFT110_INDEPENDENT_SOURCE_PASS')
