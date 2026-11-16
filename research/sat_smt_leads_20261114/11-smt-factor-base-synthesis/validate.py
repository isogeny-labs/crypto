import random, json, sys, time
from fb import *
rng = random.Random(11)
out = {}
for n in (7, 11, 13, 15, 17, 19, 23):
    F = Field(n)
    for _ in range(20):
        a, b, c = (rng.randrange(1, 1 << n) for _ in range(3))
        assert F.mul(a, F.inv(a)) == 1 and F.sq(a) == F.mul(a, a) and F.sq(F.sqrt(a)) == a
        assert F.tr(a) == F._tr(a)
        if n % 2: assert F.ht(a) == F._ht(a) and F.sq(F.ht(a)) ^ F.ht(a) == a ^ F.tr(a)
        P, Q, S = F.rand_point(rng), F.rand_point(rng), F.rand_point(rng)
        assert F.on_curve(F.add(P, Q)) and F.add(F.add(P, Q), S) == F.add(P, F.add(Q, S))
        assert F.add(P, F.neg(P)) is None and F.on_curve(F.add(P, P))
        T = (0, F.sqrt_b); assert F.add(P, T)[0] == F.mul(F.sqrt_b, F.inv(P[0]))
print("field/curve ok")
# smallest cell: brute force vs L21 vs SAT3 (both solvers), random and planted, all 3 base kinds
F = Field(11); l = 3; agree = 0; ndec = 0
names = [a for a in sys.argv[1:]] or ['cadical153', 'glucose4']
for trial in range(24):
    vs = poly_span(F, l, rng) if trial % 2 else rand_subspace(11, l, rng)
    B = Base(F, vs)
    if trial % 3 == 0:
        R = None
        while R is None:
            R = None
            for _ in range(3): R = F.add(R, rng.choice(B.points))
    else: R = F.rand_point(rng)
    if R[0] == 0: continue
    bf = brute3(F, B, R); r = l21(F, B, R); assert r['dec'] == bf, (trial, bf, r)
    if trial % 3 == 0: assert bf
    cnf, c = cnf_m3(F, B, R)
    for nm in names:
        res, st, model = sat_solve(cnf, nm)
        assert res == bf, (trial, nm, res, bf)
        if res:
            xs = model_xs(B, c, model)
            assert lifts_to(F, xs, R) or lifts_to(F, xs, F.neg(R))
    # m=2
    R2 = F.add(rng.choice(B.points), rng.choice(B.points)) if trial % 2 else F.rand_point(rng)
    if R2 is not None and R2[0] != 0:
        tr2, _ = m2_truth(F, B, R2)
        bf2 = any(F.add(P, Q) == R2 for P in B.points for Q in B.points)
        assert tr2 == bf2
        cnf2, c2 = cnf_m2(F, B, R2); res, st2, model = sat_solve(cnf2, names[0]); assert res == bf2
        if res:
            xs = model_xs(B, c2, model); assert lifts_to(F, xs, R2) or lifts_to(F, xs, F.neg(R2))
    agree += 1; ndec += bf
print("brute/L21/SAT3/SAT2 agree on", agree, "instances;", ndec, "decomposable; last stats", st)
# positive control: subfield
F = Field(15)
for k in (3, 5):
    g = [x for x in range(1, 1 << 15) if all(False for _ in ()) ]
    sub = [x for x in range(1 << 15) if (lambda y: y == x)(__import__('functools').reduce(lambda a, _: F.mul(a, a), range(k), x))]
    assert len(sub) == 1 << k
    vs, _ = span_basis(sub); print("subfield k", k, "dim", len(vs), "prod_dim", prod_dim(F, vs))
# probe SAT3 sizes
for (l, n) in ((4, 13), (5, 17)):
    F = Field(n); B = Base(F, rand_subspace(n, l, rng)); R = F.rand_point(rng)
    cnf, c = cnf_m3(F, B, R)
    for nm in names[:1]:
        t0 = time.time(); res, st, _ = sat_solve(cnf, nm); print(l, n, nm, res, st, round(time.time() - t0, 1), flush=True)
