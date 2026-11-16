#!/usr/bin/env python3
"""Validation: field/curve self-tests, planted targets, brute force on smallest cell."""
import random, json, sys
from pysat.solvers import Solver
from pdp import Field, Curve, Truth, Enc, lift_ok, S3

out = {}
rng = random.Random(20261114)
for n in (7, 11, 13, 17, 19):
    F = Field(n)
    E = Curve(F)
    for _ in range(200):
        a = rng.randrange(1, F.q)
        b = rng.randrange(1, F.q)
        assert F.mul(a, F.inv(a)) == 1 and F.mul(a, b) == F._slowmul(a, b)
        assert F.sq(F.sqrt(a)) == a
    for _ in range(100):
        P, Q, R = (E.random_point(rng) for _ in range(3))
        assert E.on(E.add(P, Q))
        assert E.add(E.add(P, Q), R) == E.add(P, E.add(Q, R))
        assert E.add(P, E.neg(P)) is None
        T = E.add(P, Q)
        if T is not None:
            assert S3(F, P[0], Q[0], T[0]) == 0
    out[f"field_curve_n{n}"] = "ok"

# planted: must be SAT, lift, and truth must agree
planted = []
for n, l in ((7, 2), (7, 3), (11, 3), (11, 5), (13, 3), (13, 5), (13, 6)):
    F = Field(n)
    E = Curve(F)
    tr = Truth(E, l)
    xs = sorted(tr.pts)
    ok = 0
    tot = 0
    spurious = 0
    for i in range(10):
        Ps = [tr.pts[rng.choice(xs)] for _ in range(3)]
        Ps = [P if rng.random() < .5 else E.neg(P) for P in Ps]
        R = E.add(E.add(Ps[0], Ps[1]), Ps[2])
        if R is None or E.add(Ps[0], Ps[1]) is None or R[0] == 0:
            continue
        tot += 1
        enc = Enc(F, l, R[0])
        with Solver(name="glucose4", bootstrap_with=enc.clauses) as s:
            sat = s.solve()
            good = sat and lift_ok(E, enc.xs_from_model(s.get_model()), R)
            if sat and not good:
                # known spurious class: x1 = x2 = 1 (twist point), t = 0 (2-torsion), R = P3 + (0,1)
                xs_ = enc.xs_from_model(s.get_model())
                assert xs_[0] == xs_[1] and E.lift(xs_[0]) is None, xs_
                spurious += 1
                s.add_clause(list(enc.t))  # forbid t = 0 and re-solve
                good = s.solve() and lift_ok(E, enc.xs_from_model(s.get_model()), R)
        assert tr.query(R)["nsol"] > 0
        ok += bool(good)
    planted.append(dict(n=n, l=l, planted=tot, sat_and_lifted=ok, first_model_spurious_twist=spurious))
    assert ok == tot, (n, l, ok, tot)
out["planted"] = planted

# brute force smallest cell: all x-triples, all t
n, l = 11, 3
F = Field(n)
E = Curve(F)
tr = Truth(E, l)
agree = 0
tot = 0
nsat = 0
for i in range(12):
    R = E.random_point(rng)
    xR = R[0]
    # brute: t candidates from eq2 for each x3, then check eq1 over x1,x2
    bf = 0
    for x3 in range(1 << l):
        ts = [t for t in range(F.q) if S3(F, x3, xR, t) == 0]
        for t in ts:
            for x1 in range(1 << l):
                for x2 in range(1 << l):
                    if S3(F, x1, x2, t) == 0:
                        bf += 1
    enc = Enc(F, l, xR)
    with Solver(name="glucose4", bootstrap_with=enc.clauses) as s:
        sat = s.solve()
    q = tr.query(R)
    tot += 1
    nsat += sat
    agree += (sat == (bf > 0)) and ((q["nsol"] > 0) == (bf > 0))
    if not ((sat == (bf > 0)) and ((q["nsol"] > 0) == (bf > 0))):
        print("MISMATCH", i, sat, bf, q)
out["bruteforce_n11_l3"] = dict(targets=tot, agree=agree, sat=nsat)
# planted-like dense check: l=5 n=13 brute force is 2^15 * roots; do field-level algebraic brute on a few
print(json.dumps(out, indent=1))
json.dump(out, open(sys.argv[1], "w"), indent=1) if len(sys.argv) > 1 else None
