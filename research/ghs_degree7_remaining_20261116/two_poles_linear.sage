#!/usr/bin/env sage
"""Exact degree-seven screen B=Qi*Qj*(x-c)^2 for trace +22."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

assert len(sys.argv) == 2
bmask = int(sys.argv[1]); assert bmask in (68, 84)
wall0 = time.perf_counter(); cpu0 = time.process_time()
F3 = GF(3)
Z = PolynomialRing(F3, 'z'); z = Z.gen()
K = GF(3^5, name='a', modulus=z^5+2*z+1); a = K.gen()
b = sum(K((bmask//(3^i)) % 3)*a^i for i in range(5))
R = PolynomialRing(K, 'x'); x = R.gen()
f = x^11+x^9+x^8+2*x^7+2*x^6+2*x^4+x^3+x^2+2*x
e = x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K, [0,1,0,0,b]).cardinality() == 222
Q = [p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
assert len(Q)==5 and x*prod(Q)==f
r0 = e.roots(multiplicities=False)
assert len(r0)==1
r0=R(r0[0])
roots=[]
for i,q in enumerate(Q):
    ext=K.extension(q,names=('w%d'%i,))
    vals=sorted([v for v,n in e.change_ring(ext).roots() for _ in range(n)],key=str)
    assert len(vals)==3 and len(set(vals))==3
    roots.append([R(v.lift().list()) for v in vals])

points=[K(v) for v in (1,2,a,a+1,a+2,a^2,a^2+1,a^2+2,a^3,a^4)]
assert len(set(points))==10 and all(v!=0 for v in points)
squares={v^2 for v in K}
lambdas=[v for v in K if v!=0 and v in squares]
assert len(lambdas)==121
cs=[v for v in K if v!=0]
assert len(cs)==242
counts={'pole_pairs':0,'branch_assignments':0,'denominator_choices':0,
        'logical_candidates':0,'evaluation_survivors':0,'coprime':0,
        'divisible':0,'polynomial_squares':0,'full_equation_hits':0}
hits=[]

for i,j in itertools.combinations(range(5),2):
    counts['pole_pairs']+=1
    S=Q[i]*Q[j]
    M,rem=f.quo_rem(S)
    assert rem==0
    assert M.degree()==7 and M[7]==1
    indices=[k for k in range(5) if k not in (i,j)]
    Mvals=[M(z) for z in points]
    allowed=[{mv*q for q in squares} for mv in Mvals]
    assert all(mv!=0 for mv in Mvals)
    Svals=[S(z) for z in points]
    for choice in itertools.product(range(3),repeat=3):
        counts['branch_assignments']+=1
        h=CRT_list([r0]+[roots[k][v] for k,v in zip(indices,choice)],
                   [x]+[Q[k] for k in indices])
        for c in cs:
            counts['denominator_choices']+=1
            H=x-c; B=S*H^2
            residue=(B*h)%M
            rvals=[residue(z) for z in points]
            Bvals=[sv*(z-c)^2 for sv,z in zip(Svals,points)]
            Bcubes=[b*v^3 for v in Bvals]
            for lam in lambdas:
                counts['logical_candidates']+=1
                for rv,mv,bv,bc,allowed_z in zip(rvals,Mvals,Bvals,Bcubes,allowed):
                    av=rv+lam*mv
                    if av^3+av^2*bv+bc not in allowed_z:
                        break
                else:
                    counts['evaluation_survivors']+=1
                    A=residue+lam*M
                    assert A.degree()==7
                    if gcd(A,B)!=1:
                        continue
                    counts['coprime']+=1
                    T=A^3+A^2*B+b*B^3
                    D2,rem=T.quo_rem(M)
                    if rem!=0:
                        continue
                    counts['divisible']+=1
                    if not D2.is_square():
                        continue
                    counts['polynomial_squares']+=1
                    D=D2.sqrt();C=H*D
                    assert f*C^2==B*T and gcd(A,B)==1
                    counts['full_equation_hits']+=1
                    hits.append({'poles':[i,j],'choice':list(choice),
                                 'c':str(c),'lambda':str(lam),'A':str(A),
                                 'B':str(B),'C':str(C)})
    sys.stderr.write('pole_pair=%d,%d candidates=%d cpu=%.3f\n' %
                     (i,j,counts['logical_candidates'],time.process_time()-cpu0))
    sys.stderr.flush()

assert counts['pole_pairs']==10 and counts['branch_assignments']==270
assert counts['denominator_choices']==10*27*242
assert counts['logical_candidates']==7906140
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.degree7.two-poles-linear/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field':'F3[a]/(a^5+2a+1)','carrier':str(f),
                  'bmask':bmask,'target_order':222,'counts':counts,'hits':hits,
                  'wall_seconds':time.perf_counter()-wall0,
                  'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int))
