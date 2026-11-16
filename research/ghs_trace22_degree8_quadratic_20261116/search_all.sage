#!/usr/bin/env sage
"""Exact degree-eight B=L*H^2 screen on all trace-22 carriers."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv)==3
fmask=int(sys.argv[1]);bmask=int(sys.argv[2])
assert fmask in (32280,35349,36380,51009,67186,106121,
                 112490,113379,127268,168265,175537,175771)
assert bmask in (68,84)
HERE=Path(__file__).resolve().parent
outpath=HERE/('allcell_%d_%d.json'%(fmask,bmask))
assert not outpath.exists(), 'refusing to overwrite immutable receipt'
wall0=time.perf_counter();cpu0=time.process_time()
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
b=sum(K((bmask//(3^i))%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen()
f=x^11+sum(K((fmask//(3^i))%3)*x^i for i in range(11))
e=x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality()==222
Q=[p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
L=next((p for p,n in f.factor() if p.degree()==1 and n==1),None)
assert L is not None and len(Q)==5 and L*prod(Q)==f
M=prod(Q);assert M.degree()==10
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])

U=PolynomialRing(K,'u');u=U.gen()
V=PolynomialRing(U,'v');v=V.gen()
S=PolynomialRing(K,'v')
points=[z for z in K if f(z)!=0][:10]
assert len(set(points))==10 and all(f(z)!=0 for z in points)
squares={z^2 for z in K}
counts={'systems':0,'u_specializations':0,'zero_polynomial_fallbacks':0,
        'parameter_roots':0,'allowed_H':0,'degree_eight':0,'coprime':0,
        'point_survivors':0,'f_divides':0,'exact_hits':0}
hits=[]

for choices in itertools.product(range(3),repeat=5):
 h=CRT_list([roots[i][j] for i,j in enumerate(choices)],Q)
 assert h.degree()<10
 counts['systems']+=1
 rems=[(L*x^i*h)%M for i in range(5)]
 F=V(rems[4][9])+V(2*rems[3][9])*u+V(rems[2][9])*(u^2+2*v)+V(2*rems[1][9])*u*v+V(rems[0][9])*v^2
 for u0 in K:
  counts['u_specializations']+=1
  spec=S([K(t(u0)) for t in F.list()])
  if spec==0:
   counts['zero_polynomial_fallbacks']+=1;vs=list(K)
  else:vs=spec.roots(multiplicities=False)
  for v0 in vs:
   if spec(v0)!=0:continue
   counts['parameter_roots']+=1
   H=x^2+u0*x+v0
   if gcd(H,f)!=1 or H.discriminant()==0:continue
   counts['allowed_H']+=1
   B=L*H^2;A=(B*h)%M
   if A.degree()!=8:continue
   counts['degree_eight']+=1
   if gcd(A,B)!=1:continue
   counts['coprime']+=1
   N=B*(A^3+A^2*B+b*B^3)
   if any(f(z)==0 or N(z)/f(z) not in squares for z in points):continue
   counts['point_survivors']+=1
   C2,rem=N.quo_rem(f)
   if rem!=0:continue
   counts['f_divides']+=1
   if not C2.is_square():continue
   C=C2.sqrt();assert f*C^2==N
   counts['exact_hits']+=1
   hits.append({'choices':list(choices),'H':str(H),'A':str(A),'B':str(B),'C':str(C)})
 if counts['systems']%27==0:
  print(json.dumps({'phase':'progress','systems':counts['systems'],
                    'parameter_roots':counts['parameter_roots'],
                    'hits':counts['exact_hits'],'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int),flush=True)

assert counts['systems']==243 and counts['u_specializations']==243^2
out={'schema':'ghs.trace22.degree8-quadratic-square-all/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'fmask':fmask,'bmask':bmask,'target_order':222,
     'field':'F3[a]/(a^5+2a+1)','carrier':str(f),'target':str(e),
     'shape':'B=L*H^2; L unique rational branch factor, H monic quadratic, squarefree, coprime to f',
     'rational_branch_factor':str(L),
     'counts':counts,'hits':hits,
     'wall_seconds':time.perf_counter()-wall0,'cpu_seconds':time.process_time()-cpu0}
outpath.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'phase':'complete','fmask':fmask,'bmask':bmask,
                  'counts':counts,'cpu_seconds':out['cpu_seconds']},sort_keys=True,default=int),flush=True)
