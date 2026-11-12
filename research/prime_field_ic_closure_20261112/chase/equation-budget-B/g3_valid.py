#!/usr/bin/env python3
"""G3 on valid targets: k=3, base x in GF(p). Count GF(p) solutions of the 3 descended S4 equations
for targets R' in E(GF(p^3)) with x(R') not in GF(p); cross-check by group-theoretic brute force
(all ordered triples of base points summing to R').  [pcc:unverified]"""
import sys, random, itertools, json
sys.path.insert(0, '.')
from descent import Ext, Curve, O, points_x_in_base, count_base_points
from groebner_descent import S3, split, x1, x2, x3, y, t
from sympy import resultant, Poly, symbols
rng = random.Random(11)
p = 31
c = next(z for z in range(2, p) if pow(z, (p - 1) // 3, p) != 1)
while True:
    a, b = rng.randrange(p), rng.randrange(p)
    if (4 * a ** 3 + 27 * b ** 2) % p: break
Xs = symbols('X')
S4p = Poly(resultant(S3(x1, x2, y, a, b), S3(x3, y, Xs, a, b), y), x1, x2, x3, Xs, modulus=p)
F = Ext(p, [(-c) % p, 0, 0]); E = Curve(F, a, b)
base = points_x_in_base(E)
nE = count_base_points(p, a, b)
pair = {}
for P in base + [O]:
    for Q in base + [O]:
        S = E.add(P, Q); pair[S] = pair.get(S, 0) + 1
rows = []
cnt_t = 0
while len(rows) < 6:
    X = (rng.randrange(p), rng.randrange(1, p), rng.randrange(p))
    yy = F.sqrt(E.rhs(X))
    if yy is None: continue
    R = (X, yy)
    eqs = [e for e in split(S4p.as_expr().subs(Xs, X[0] + X[1] * t + X[2] * t ** 2), t ** 3 - c, 3, (x1, x2, x3), p) if e != 0]
    Ps = [Poly(e, x1, x2, x3, modulus=p) for e in eqs]
    sol = sum(1 for s in itertools.product(range(p), repeat=3) if all(P.eval(s) % p == 0 for P in Ps))
    # group check: ordered triples from base u {O} summing to R
    trip = sum(pair.get(E.add(R, E.neg(P3)), 0) for P3 in base + [O])
    rows.append({"X": X, "descended_eqs": len(eqs), "Fp_solutions_of_descended_system": sol,
                 "ordered_triples_of_x_in_Fp_points_summing_to_R": trip})
    print(json.dumps(rows[-1]), flush=True)
print(json.dumps({"p": p, "a": a, "b": b, "c": c, "#E(Fp)": nE, "|base x in Fp| (all in E(Fp), odd k)": len(base),
                  "all_base_y_in_Fp": all(F.isbase(P[1]) for P in base)}))
