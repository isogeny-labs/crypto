#!/usr/bin/env sage
"""Degree-eight L*Qi*Qj and L^3*Qi*Qj exact bitset cover screen."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv)==3
fmask=int(sys.argv[1]);bmask=int(sys.argv[2])
assert fmask in (32280,35349) and bmask in (68,84)
wall0=time.perf_counter();cpu0=time.process_time()
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
b=sum(K((bmask//(3^i))%3)*a^i for i in range(5))
R=PolynomialRing(K,'x');x=R.gen()
f=x^11+sum(K((fmask//(3^i))%3)*x^i for i in range(11))
e=x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality()==222
L=x
Q=[p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
assert len(Q)==5 and L*prod(Q)==f
roots=[]
for i,q in enumerate(Q):
 ext=K.extension(q,names=('w%d'%i,))
 vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
 assert len(vals)==3 and len(set(vals))==3
 roots.append([R(v.lift().list()) for v in vals])

els=list(K);q=K.cardinality();assert q==243
idx={v:i for i,v in enumerate(els)}
add=[[idx[u+v] for v in els] for u in els]
mul=[[idx[u*v] for v in els] for u in els]
zero=idx[K(0)];neg=[idx[-v] for v in els]
assert all(add[i][neg[i]]==zero for i in range(q))
squares={v^2 for v in els}
lambdas=[i for i,v in enumerate(els) if v!=0 and v in squares]
assert len(lambdas)==121
points=[K(v) for v in (1,2,a,a+1,a+2,2*a,2*a+1,2*a+2,
 a^2,a^2+1,a^2+2,2*a^2,2*a^2+1,2*a^2+2,
 a^3,a^3+1,a^3+2,2*a^3,a^4,a^4+1)]
assert len(points)==20 and len(set(points))==20 and all(v!=0 for v in points)
point_idx=[idx[z] for z in points]
point_sq_idx=[idx[z^2] for z in points]
FULL=(1<<q)-1
counts={'pole_pairs':0,'shapes':0,'branch_assignments':0,
        'logical_candidates':0,'point_filter_survivors':0,
        'coprime':0,'divisible':0,'polynomial_squares':0,
        'full_equation_hits':0,'bitset_controls':0}
hits=[]

for poles in itertools.combinations(range(5),2):
 counts['pole_pairs']+=1
 base=L*Q[poles[0]]*Q[poles[1]]
 M,rem=f.quo_rem(base);assert rem==0 and M.degree()==6 and M[6]==1
 indices=[i for i in range(5) if i not in poles]
 Mvals=[M(z) for z in points]
 assert all(v!=0 for v in Mvals)
 allowed_eval=[{mv*s for s in squares} for mv in Mvals]
 for m in (1,3):
  counts['shapes']+=1
  B=L^(m-1)*base
  for choice in itertools.product(range(3),repeat=3):
   counts['branch_assignments']+=1
   h=CRT_list([roots[k][v] for k,v in zip(indices,choice)],
              [Q[k] for k in indices])
   residue=(B*h)%M
   rvals=[residue(z) for z in points]
   Bvals=[B(z) for z in points]
   translates=[]
   for z,mv,bv,rv,allowed in zip(points,Mvals,Bvals,rvals,allowed_eval):
    bterm=b*bv^3
    vset={idx[(av-rv)/mv] for av in els
          if av^3+av^2*bv+bterm in allowed}
    shifts=[]
    for delta in range(q):
     bits=0;minus_delta=neg[delta]
     for v in vset:bits|=1<<add[v][minus_delta]
     shifts.append(bits)
    translates.append(shifts)
   li=lambdas[0];mi=zero;ni=zero
   for k,(z,mv,bv,rv,allowed) in enumerate(zip(points,Mvals,Bvals,rvals,allowed_eval)):
    delta=add[mul[li][point_sq_idx[k]]][mul[mi][point_idx[k]]]
    bit=bool(translates[k][delta] & (1<<ni))
    av=rv+mv*(els[li]*z^2+els[mi]*z+els[ni])
    direct=av^3+av^2*bv+b*bv^3 in allowed
    assert bit==direct
    counts['bitset_controls']+=1
   for li in lambdas:
    lam=els[li]
    for mi,mu in enumerate(els):
     counts['logical_candidates']+=q
     mask=FULL
     for k in range(20):
      delta=add[mul[li][point_sq_idx[k]]][mul[mi][point_idx[k]]]
      mask &= translates[k][delta]
      if mask==0:break
     while mask:
      one=mask & -mask;ni=one.bit_length()-1;mask-=one
      counts['point_filter_survivors']+=1
      nu=els[ni]
      A=residue+M*(lam*x^2+mu*x+nu)
      assert A.degree()==8
      if gcd(A,B)!=1:continue
      counts['coprime']+=1
      T=A^3+A^2*B+b*B^3
      D2,rem=T.quo_rem(M)
      if rem!=0:continue
      counts['divisible']+=1
      if not D2.is_square():continue
      counts['polynomial_squares']+=1
      D=D2.sqrt();C=L^((m-1)//2)*D
      assert f*C^2==B*T and gcd(A,B)==1
      counts['full_equation_hits']+=1
      hits.append({'poles':list(poles),'rational_pole_exponent':m,
                   'choice':list(choice),'lambda':str(lam),
                   'mu':str(mu),'nu':str(nu),'A':str(A),
                   'B':str(B),'C':str(C)})
 sys.stderr.write('poles=%s candidates=%d survivors=%d cpu=%.3f\n'%
                  (str(poles),counts['logical_candidates'],
                   counts['point_filter_survivors'],time.process_time()-cpu0))
 sys.stderr.flush()

assert counts['pole_pairs']==10 and counts['shapes']==20
assert counts['branch_assignments']==540
assert counts['logical_candidates']==3858261660
assert counts['bitset_controls']==540*20
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.trace22.degree8-two-poles/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field':'F3[a]/(a^5+2a+1)','fmask':fmask,'carrier':str(f),
                  'bmask':bmask,'target_order':222,'counts':counts,'hits':hits,
                  'wall_seconds':time.perf_counter()-wall0,
                  'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int))
