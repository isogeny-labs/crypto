import random, json, sys, time
from fb import *
MASTER = 2026111411
def make_bases(F, l, rng):
    out = []
    def ok(gen):  # resample bases with fewer than 3 rational points (planted targets impossible)
        while True:
            vs = gen()
            if len(Base(F, vs).points) >= 3: return vs
    for i in range(4):
        out.append(('S', i, ok(lambda: poly_span(F, l, rng))))
        out.append(('A', i, ok(lambda: rand_subspace(F.n, l, rng))))
        out.append(('B', i, ok(lambda: rand_subspace(F.n, l, rng))))
    return out
def planted(F, B, m, rng):
    while True:
        R = None
        for _ in range(m): R = F.add(R, rng.choice(B.points))
        if R is not None and R[0] != 0: return R
def run_cell(m, l, n, fh, sat):
    seed = MASTER * 1000 + 37 * l + n + (500 if m == 2 else 0)
    rng = random.Random(seed); F = Field(n)
    shared = [F.rand_point(rng) for _ in range(4)]
    for arm, bi, vs in make_bases(F, l, rng):
        B = Base(F, vs)
        targets = [('random', R) for R in shared] + [('planted', planted(F, B, m, rng)) for _ in range(4)]
        for ti, (kind, R) in enumerate(targets):
            row = dict(m=m, l=l, n=n, seed=seed, arm=arm, base=bi, vs=vs, w=B.w, npoints=len(B.points),
                       tkind=kind, tidx=ti, R=list(R))
            if m == 3:
                t0 = time.process_time(); r = l21(F, B, R); r['cpu'] = time.process_time() - t0
                row['l21'] = r; row['dec'] = r['dec']
                if kind == 'planted': assert r['dec']
                if sat:
                    cnf, c = cnf_m3(F, B, R)
            else:
                dec, nc = m2_truth(F, B, R); row['dec'] = dec; row['leaf_cands'] = nc
                if kind == 'planted': assert dec
                cnf, c = cnf_m2(F, B, R)
            if sat:
                res, st, model = sat_solve(cnf, 'cadical153')
                assert res == row['dec'], ('SAT/truth mismatch', row)
                if res:
                    xs = model_xs(B, c, model); assert lifts_to(F, xs, R) or lifts_to(F, xs, F.neg(R))
                row['sat'] = st
            fh.write(json.dumps(row) + '\n'); fh.flush()
    print('cell done', m, l, n, time.strftime('%H:%M:%S'), flush=True)
if __name__ == '__main__':
    which = sys.argv[1]
    with open(f'raw_{which}.jsonl', 'w') as fh:
        if which == 'l21':
            for (l, n) in ((5, 17), (6, 19), (7, 23)): run_cell(3, l, n, fh, False)
        elif which == 'sat3':
            for (l, n) in ((2, 7), (3, 11), (4, 13)): run_cell(3, l, n, fh, True)
        elif which == 'sat2':
            for (l, n) in ((4, 11), (5, 11), (6, 13), (7, 17), (8, 17)): run_cell(2, l, n, fh, True)
