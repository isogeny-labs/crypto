#!/usr/bin/env sage
"""Trace +22 degree-seven screen for four sparse branch-pole shapes."""
import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

F3 = GF(3)
wall0=time.perf_counter();cpu0=time.process_time()
Z = PolynomialRing(F3, 'z'); z = Z.gen()
K = GF(3^5, name='a', modulus=z^5+2*z+1); a = K.gen()
assert len(sys.argv) == 2
bmask = int(sys.argv[1])
assert bmask in (68,84)
b = sum(K((bmask // (3^i)) % 3)*a^i for i in range(5))
R = PolynomialRing(K,'x'); x = R.gen()
L = x
f = x^11+x^9+2*x^8+x^7+x^5+x^4+x^3+2*x
e = x^3+x^2+b
assert f.is_squarefree() and e.is_squarefree()
assert EllipticCurve(K,[0,1,0,0,b]).cardinality() == 222
Q = [p for p,m in f.factor() if p.degree() == 2 for _ in range(m)]
assert len(Q) == 5 and L*prod(Q) == f
rational_roots = e.roots(multiplicities=False)
assert len(rational_roots) == 1
root0 = R(rational_roots[0])
root_reps = []
for i,q in enumerate(Q):
    ext = K.extension(q,names=('w%d'%i,))
    vals = sorted([v for v,m in e.change_ring(ext).roots() for _ in range(m)],key=str)
    assert len(vals) == 3 and len(set(vals)) == 3
    root_reps.append([R(v.lift().list()) for v in vals])

shapes = ('one','Qi','Qi3','QiQj')
counts = {s:{'branch_assignments':0,'numerator_degree_seven':0,'coprime':0,'f_divides':0,'full_equation_hits':0} for s in shapes}
hits = []

def candidates(indices):
    moduli = [L] + [Q[j] for j in indices]
    M = prod(moduli)
    for choices in itertools.product(range(3),repeat=len(indices)):
        residues = [root0] + [root_reps[j][k] for j,k in zip(indices,choices)]
        yield choices,CRT_list(residues,moduli),M

def check(kind,A,B,choices,poles,lam=None):
    if A.degree() != 7:return
    counts[kind]['numerator_degree_seven'] += 1
    if gcd(A,B) != 1:return
    counts[kind]['coprime'] += 1
    N = B*(A^3+A^2*B+b*B^3)
    C2,rem = N.quo_rem(f)
    if rem != 0:return
    counts[kind]['f_divides'] += 1
    if not C2.is_square():return
    C=C2.sqrt()
    assert f*C^2 == N and gcd(A,B)==1
    counts[kind]['full_equation_hits'] += 1
    hits.append({'shape':kind,'A':str(A),'B':str(B),'C':str(C),'choices':list(choices),'poles':list(poles),'lambda':None if lam is None else str(lam)})

for choices,h,M in candidates(tuple(range(5))):
    counts['one']['branch_assignments'] += 1
    check('one',h,R(1),choices,[])
for i in range(5):
    indices=tuple(j for j in range(5) if j != i)
    for choices,h,M in candidates(indices):
        for kind,B in (('Qi',Q[i]),('Qi3',Q[i]^3)):
            counts[kind]['branch_assignments'] += 1
            check(kind,(B*h)%M,B,choices,[i])
for i,j in itertools.combinations(range(5),2):
    indices=tuple(k for k in range(5) if k not in (i,j))
    for choices,h,M in candidates(indices):
        counts['QiQj']['branch_assignments'] += 1
        B=Q[i]*Q[j]
        residue=(B*h)%M
        assert M.degree()==7
        for lam in K:
            if lam != 0:check('QiQj',residue+lam*M,B,choices,[i,j],lam)

assert [counts[s]['branch_assignments'] for s in shapes] == [243,405,405,270]
print(json.dumps({'schema':'ghs.degree7.trace22-sparse/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),'field':'F3[a]/(a^5+2a+1)','bmask':bmask,'target_order':222,'counts':counts,'hits':hits,'wall_seconds':time.perf_counter()-wall0,'cpu_seconds':time.process_time()-cpu0},sort_keys=True,default=int))
