import sys, json, random
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from pdp import *; from arms import run_arm
n, l, n_unsat, n_plant, cap, arms, out = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5]), sys.argv[6].split(','), sys.argv[7]
F = GF(n); E = Curve(F); rows = []; targets = []; seen = set()
base = 20261114000 + 10000 * n + 100 * l
for i in range(n_unsat):
    xR, tries = gen_random_unsat(E, l, random.Random(base + i)); targets.append((base + i, 'nondecomposable', xR))
mode = sys.argv[8] if len(sys.argv) > 8 else 'planted'
if mode == 'relaxed':
    for i in range(n_plant):
        rng = random.Random(base + 50 + i)
        while True:
            x = rng.randrange(1, 1 << n)
            if E.lift(x) and oracle(E, l, x)[0]: break
        targets.append((base + 50 + i, 'decomposable', x))
elif n_plant and any(E.lift(x) for x in range(1, 1 << l)):
    i = 0
    while sum(t[1] == 'decomposable' for t in targets) < n_plant and i < 200:
        seed = base + 50 + i; i += 1
        xR, xs = gen_planted(E, l, random.Random(seed))
        if xR in seen: continue
        seen.add(xR); targets.append((seed, 'decomposable', xR))
for ti, (seed, kind, xR) in enumerate(targets):
    dec, wit, nq, nU = oracle(E, l, xR)
    assert dec == (kind == 'decomposable')
    en, xb, ub, vb = encode(E, l, xR)
    order = arms[ti % len(arms):] + arms[:ti % len(arms)]
    for pos, arm in enumerate(order):
        r, model = run_arm(arm, en.cl, ub, cap)
        r.update(n=n, l=l, seed=seed, kind=kind, xR=xR, order_pos=pos, cap_conflicts=cap, vars=en.nv, clauses=len(en.cl),
                 oracle_quad_solves=nq, U_size=nU)
        if r['result'] != 'CAP': assert (r['result'] == 'SAT') == dec, r
        if model:
            ok, lift, *_ = check_model(E, l, xR, model, xb, ub, vb); assert ok; r['model_check'] = ok; r['lift'] = lift
        rows.append(r); print(json.dumps(r), flush=True)
        json.dump(rows, open(out, 'w'))
