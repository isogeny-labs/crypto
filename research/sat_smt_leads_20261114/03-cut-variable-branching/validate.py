import random, json, sys
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from pdp import *; from arms import run_arm
# 1. n=7, l=2 exhaustive, independent of quad solver
F = GF(7); E = Curve(F); l = 2
U = {u for x1 in range(4) for x2 in range(4) for u in range(128) if E.S3(x1, x2, u) == 0}
agree = 0; nsat = 0
for xR in range(128):
    brute = any(E.S3(u, v, xR) == 0 for u in U for v in U)
    orc = oracle(E, l, xR)[0]
    en, xb, ub, vb = encode(E, l, xR)
    res = []
    for arm in ('D', 'C'):
        r, model = run_arm(arm, en.cl, ub, 100000)
        res.append(r['result'] == 'SAT')
        if model: assert check_model(E, l, xR, model, xb, ub, vb)[0]
    assert brute == orc == res[0] == res[1], (xR, brute, orc, res)
    agree += 1; nsat += brute
print('n=7 l=2: all 128 targets agree (brute/oracle/D/C); SAT count', nsat)
# curve sanity: random point adds stay on curve, S3 vanishes on P,Q,P+Q
F = GF(11); E = Curve(F); rng = random.Random(1); k = 0
while k < 200:
    P = E.lift(rng.randrange(1, 1 << 11)); Q = E.lift(rng.randrange(1, 1 << 11))
    if P is None or Q is None: continue
    R = E.add(P, Q)
    if R is None: continue
    assert E.on(R) and E.S3(P[0], Q[0], R[0]) == 0; k += 1
    for u in F.quad(F.sq(P[0] ^ Q[0]), F.mul(P[0], Q[0]), F.sq(F.mul(P[0], Q[0])) ^ 1): assert E.S3(P[0], Q[0], u) == 0
print('n=11 curve/S3/quad sanity ok')
# 2. planted n=19 l=3
for i in range(3):
    xR, xs = gen_planted(E, 3, random.Random(100 + i))
    assert oracle(E, 3, xR)[0]
    en, xb, ub, vb = encode(E, 3, xR)
    for arm in ('D', 'Dp', 'C', 'C-lean', 'C-keep'):
        r, model = run_arm(arm, en.cl, ub, 400000)
        print(i, r, check_model(E, 3, xR, model, xb, ub, vb)[:2] if model else None, 'vars', en.nv, 'clauses', len(en.cl))
