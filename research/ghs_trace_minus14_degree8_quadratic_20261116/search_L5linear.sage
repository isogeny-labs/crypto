#!/usr/bin/env sage
"""Exact degree-eight B=L^5*(x-c)^2 screen for trace-minus14 orbit reps."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv)==3
fmask=int(sys.argv[1]);bmask=int(sys.argv[2])
assert fmask in (9735,17371) and bmask in (11,202,38,80)
HERE=Path(__file__).resolve().parent
outpath=HERE/('l5cell_%d_%d.json'%(fmask,bmask))
assert not outpath.exists(), 'refusing to overwrite immutable receipt'
wall0=time.perf_counter();cpu0=time.process_time()
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
b=sum(K((bmask//3^i)%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen()
f=x^11+sum(K((fmask//3^i)%3)*x^i for i in range(11))
e=x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality()==258
fac=[p for p,n in f.factor() for _ in range(n)]
L=next((p for p in fac if p.degree()==1),None)
Q=[p for p in fac if p.degree()==2]
assert L is not None and len(Q)==5 and L*prod(Q)==f
M=prod(Q);assert M.degree()==10
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])
points=[z for z in K if f(z)!=0][:10]
assert len(points)==10 and len(set(points))==10
squares={z^2 for z in K}
cs=[c for c in K if L(c)!=0]
assert len(cs)==242
counts={'branch_assignments':0,'parameter_pairs':0,'degree_eight':0,
        'coprime':0,'point_survivors':0,'f_divides':0,'exact_hits':0}
hits=[]
for choices in itertools.product(range(3),repeat=5):
 h=CRT_list([roots[i][j] for i,j in enumerate(choices)],Q)
 counts['branch_assignments']+=1
 for c in cs:
  counts['parameter_pairs']+=1
  H=x-c;B=L^5*H^2;A=(B*h)%M
  if A.degree()!=8:continue
  counts['degree_eight']+=1
  if gcd(A,B)!=1:continue
  counts['coprime']+=1
  N=B*(A^3+A^2*B+b*B^3)
  if any(N(z)/f(z) not in squares for z in points):continue
  counts['point_survivors']+=1
  C2,rem=N.quo_rem(f)
  if rem!=0:continue
  counts['f_divides']+=1
  if not C2.is_square():continue
  C=C2.sqrt();assert f*C^2==N and gcd(A,B)==1
  counts['exact_hits']+=1
  hits.append({'choices':list(choices),'c':str(c),'A':str(A),'B':str(B),'C':str(C)})
 if counts['branch_assignments']%27==0:
  print(json.dumps({'phase':'progress','branch_assignments':counts['branch_assignments'],
                    'hits':counts['exact_hits'],'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int),flush=True)
assert counts['branch_assignments']==243 and counts['parameter_pairs']==243*242
out={'schema':'ghs.trace-minus14.degree8-L5-linear-square-orbits/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'fmask':fmask,'bmask':bmask,'target_order':258,'trace':-14,
     'field':'F3[a]/(a^5+2a+1)','carrier':str(f),'target':str(e),
     'shape':'B=L^5*(x-c)^2; L unique rational branch factor; L(c)!=0',
     'counts':counts,'hits':hits,
     'wall_seconds':time.perf_counter()-wall0,'cpu_seconds':time.process_time()-cpu0}
outpath.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'phase':'complete','fmask':fmask,'bmask':bmask,
                  'counts':counts,'cpu_seconds':out['cpu_seconds']},sort_keys=True,default=int),flush=True)
