#!/usr/bin/env python3
"""Added check (deviation D2): codomain type for ALL 135 Lagrangians (8 partial
inversions x 64 graphs, with repetition). argv[1] = control | random (seed 1).
Same instrument as theta_arm.py, imported unchanged."""
import itertools, sys
sys.path.insert(0, '/home/user/crypto/research/g3_transfer_gates_20261120')
import theta_arm as t, mpmath
from mpmath import mpf
def curve(kind):
    if kind=='control':
        s, a, b = mpf(4), mpf("2.6"), mpf("1.5")
        pos = sorted([s*a, s/a, s*b, s/b]); return sorted([-x for x in pos] + pos)
    import random
    rng = random.Random(1); return [mpf(3*i)+mpf(rng.uniform(0.2,1.8)) for i in range(8)]
es = curve(sys.argv[1])
asym, pd, signs, tau = t.period_matrix(es, 4)
T = mpmath.matrix([[(tau[i,j]+tau[j,i])/2 for j in range(3)] for i in range(3)])
summ = {}
for I in itertools.product((0,1), repeat=3):
    A = mpmath.diag([1-x for x in I]); C = mpmath.diag(list(I)); B = -C; D = A
    TI = (A*T+B)*(C*T+D)**-1
    for U in itertools.product((0,1), repeat=6):
        S = mpmath.matrix(3,3); idx=0
        for i in range(3):
            for j in range(i,3):
                S[i,j]=S[j,i]=U[idx]; idx+=1
        Tp = [[(TI[i,j]+TI[j,i]+2*S[i,j])/4 for j in range(3)] for i in range(3)]
        th,R,lam = t.thetanulls(Tp)
        typ, small, r0, r1 = t.classify(th)
        k = "%s(%d)" % (typ, len(small)); summ[k] = summ.get(k,0)+1
print(sys.argv[1], summ)
