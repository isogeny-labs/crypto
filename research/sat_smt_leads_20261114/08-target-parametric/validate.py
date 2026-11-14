"""Validation: l=3,n=11 verdict == real-point MITM truth; models lift. Also brute-force check of MITM truth."""
import random, sys, time, json
from pysat.solvers import Cadical153
from pdp import Setup, build2 as build, assumptions, lift_model

n, l = (int(sys.argv[1]), int(sys.argv[2])) if len(sys.argv) > 2 else (11, 3)
S = Setup(n, l, 20261114 + 1000 * l)
E = S.E
print('n', n, 'l', l, 'mod', bin(S.F.mod), 'signed base', len(S.base), 'sum2x', len(S.sum2x))
# brute force triple enumeration vs MITM (smallest cell only)
if l <= 3:
    bf = set()
    for P in S.base:
        for Q in S.base:
            PQ = E.add(P, Q)
            for T in S.base:
                X = E.add(PQ, T)
                if X is not None:
                    bf.add(X[0])
    # every rational x: compare
    mism = 0; tot = 0
    for x in range(1, 1 << n):
        R = E.lift(x)
        if R is None:
            continue
        tot += 1
        if S.decomposable(R) != (x in bf):
            mism += 1
    print('bruteforce-vs-MITM mismatches', mism, 'of', tot, 'decomposable x count', len(bf))
inst = build(S)
print('param CNF vars', inst['nv'], 'clauses', len(inst['clauses']), 'ANDs', inst['nand'], 'xors', inst['nxor'])
rng = random.Random(7)
targets = [S.random_point(rng) for _ in range(40)] + [S.planted(rng) for _ in range(10)]
targets = [R for R in targets if R is not None and R[0] not in S.Vset and R[0] != 0]
bad = 0; nsat = 0; liftfail = 0
t0 = time.time()
for R in targets:
    truth = S.decomposable(R)
    for mode in ('param', 'spec'):
        if mode == 'param':
            s = Cadical153(bootstrap_with=inst['clauses'])
            res = s.solve(assumptions=assumptions(inst, R[0]))
            I = inst
        else:
            I = build(S, R[0])
            s = Cadical153(bootstrap_with=I['clauses'])
            res = s.solve()
        if res != truth:
            bad += 1
            print('MISMATCH', mode, R, res, truth)
        if res:
            nsat += 1
            ok, xs = lift_model(S, I, s.get_model(), R)
            if not ok:
                liftfail += 1
                print('LIFT FAIL', mode, R, xs)
        s.delete()
print(json.dumps(dict(n=n, l=l, targets=len(targets), verdict_mismatch=bad, sat_calls=nsat, lift_fail=liftfail, secs=round(time.time() - t0, 2))))
