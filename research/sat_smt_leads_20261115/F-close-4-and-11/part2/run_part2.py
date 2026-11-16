"""Part 2 runner.
  run_part2.py defect        -> defect_min.json  (part a: exhaustive min of dim V.V + samples)
  run_part2.py l21           -> raw_l21.jsonl    (part c)
  run_part2.py sat <l> <n>   -> raw_sat_l<l>.jsonl (part b, one cell)
Reuses yesterday's fb.py unchanged."""
import sys, os, json, random, time, itertools
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fb import *

HERE = os.path.dirname(os.path.abspath(__file__))
MASTER = 2026111505
NBASES = 16
NTARGETS = 8


# ---------- part (a): exhaustive minimum of dim(V.V), up to scaling (V contains 1) ----------
def subspaces_containing_one(F, l):
    """yield each l-dim F2-subspace containing 1, once (canonical reduced-echelon basis tuple)."""
    n = F.n
    seen = set()
    # choose l-1 extra generators; dedup by canonical span
    pool = [x for x in range(1 << n)]
    for extra in itertools.combinations(range(1, 1 << n), l - 1):
        gens = [1] + list(extra)
        if rank(gens) != l:
            continue
        key = tuple(span_basis(gens)[0])
        if key in seen:
            continue
        seen.add(key)
        yield list(key)


def defect_min():
    out = {'tag': 'pcc:unverified pilot', 'exhaustive': [], 'sampled': [], 'control': []}
    # exhaustive cells (small): min of w over all V, via V containing 1
    # exhaustive up to scaling (V contains 1). (3,13)+ omitted: ~10^7+ Python iters, infeasible in
    # budget; covered by the sampled set and the HLX/BSZ argument instead.
    exhaustive_cells = [(2, 3), (2, 5), (2, 7), (2, 11), (2, 13), (3, 7), (3, 11), (4, 7)]
    for (l, n) in exhaustive_cells:
        if n not in MOD:
            continue
        F = Field(n)
        hist = {}
        count = 0
        t0 = time.time()
        for vs in subspaces_containing_one(F, l):
            w = prod_dim(F, vs)
            hist[w] = hist.get(w, 0) + 1
            count += 1
        minw = min(hist) if hist else None
        out['exhaustive'].append(dict(l=l, n=n, lower_bound_2lm1=2 * l - 1, count=count,
                                      w_hist={str(k): v for k, v in sorted(hist.items())},
                                      min_w=minw, min_equals_2lm1=(minw == 2 * l - 1),
                                      any_below=(minw is not None and minw < 2 * l - 1),
                                      secs=round(time.time() - t0, 1)))
        print('exhaustive', l, n, 'min_w', minw, '2l-1', 2 * l - 1, hist, round(time.time() - t0, 1), flush=True)
    # sampled cells (too large to enumerate): random + structured, min w only
    sampled_cells = [(3, 13), (4, 11), (4, 13), (5, 17), (6, 19)]
    for (l, n) in sampled_cells:
        if n not in MOD:
            continue
        F = Field(n)
        rng = random.Random(MASTER * 1000 + 37 * l + n)
        rws = [prod_dim(F, rand_subspace(n, l, rng)) for _ in range(5000)]
        poly = [prod_dim(F, poly_span(F, l, rng)) for _ in range(50)]
        frob = [prod_dim(F, frob_span(F, l, rng)) for _ in range(50)]
        tr0 = [prod_dim(F, trace0_span(F, l, rng)) for _ in range(50)]
        out['sampled'].append(dict(l=l, n=n, lower_bound_2lm1=2 * l - 1,
                                   random_min=min(rws), random_median=sorted(rws)[len(rws) // 2],
                                   n_random=len(rws), poly_w=sorted(set(poly)),
                                   frob_min=min(frob), trace0_min=min(tr0),
                                   any_below_2lm1=min(rws + poly + frob + tr0) < 2 * l - 1))
        print('sampled', l, n, 'random_min', min(rws), 'poly', sorted(set(poly)), '2l-1', 2 * l - 1, flush=True)
    # positive control: subfield of F_2^15 gives w = l < random
    F = Field(15)
    rng = random.Random(MASTER)
    for k in (3, 5):
        sub = [x for x in range(1 << 15)
               if (lambda y: y == x)(__import__('functools').reduce(lambda a, _: F.mul(a, a), range(k), x))]
        vs, _ = span_basis(sub)
        rnd = sorted(prod_dim(F, rand_subspace(15, k, rng)) for _ in range(200))
        out['control'].append(dict(n=15, l=k, subfield_w=prod_dim(F, vs),
                                   random_w_min=rnd[0], random_w_median=rnd[100]))
    json.dump(out, open(os.path.join(HERE, 'defect_min.json'), 'w'), indent=1)
    allhold = all(c['min_equals_2lm1'] for c in out['exhaustive']) and not any(c['any_below_2lm1'] for c in out['sampled'])
    print('ALL covered cells: min w == 2l-1 and no V below:', allhold)


# ---------- bases / targets ----------
def make_bases(F, l, rng):
    def ok(gen):
        while True:
            vs = gen()
            if len(Base(F, vs).points) >= 3:
                return vs
    S = [('S', i, ok(lambda: poly_span(F, l, rng))) for i in range(NBASES)]
    A = [('A', i, ok(lambda: rand_subspace(F.n, l, rng))) for i in range(NBASES)]
    B = [('B', i, ok(lambda: rand_subspace(F.n, l, rng))) for i in range(NBASES)]
    # interleave S,A,B,... so a truncated run keeps the three arms balanced (A/A floor stays valid)
    bases = []
    for i in range(NBASES):
        bases += [S[i], A[i], B[i]]
    return bases


def shared_targets(F, rng, m=NTARGETS):
    out = []
    while len(out) < m:
        R = F.rand_point(rng)
        if R[0] != 0:
            out.append(R)
    return out


# ---------- part (c): L21 ----------
def run_l21():
    out = open(os.path.join(HERE, 'raw_l21.jsonl'), 'w')
    for (l, n) in ((3, 11), (4, 13), (5, 17)):
        seed = MASTER * 1000 + 37 * l + n
        rng = random.Random(seed)
        F = Field(n)
        bases = make_bases(F, l, rng)
        tgts = shared_targets(F, rng)
        t0 = time.time()
        for arm, bi, vs in bases:
            B = Base(F, vs)
            for ti, R in enumerate(tgts):
                r = l21(F, B, R)
                out.write(json.dumps(dict(unit='l21', l=l, n=n, arm=arm, base=bi, w=B.w,
                                          npoints=len(B.points), tidx=ti, dec=r['dec'],
                                          leaves=r['leaves'], cands=r['cands'], le1=r['le1'],
                                          trunc=r['trunc'])) + '\n')
                out.flush()
        print('l21 cell', l, n, round(time.time() - t0, 1), flush=True)
    out.close()


# ---------- part (b): SAT ----------
def run_sat(l, n, budget=200000):
    from pysat.solvers import Solver
    out = open(os.path.join(HERE, f'raw_sat_l{l}.jsonl'), 'w')
    seed = MASTER * 1000 + 37 * l + n
    rng = random.Random(seed)
    F = Field(n)
    bases = make_bases(F, l, rng)
    tgts = shared_targets(F, rng)
    t0 = time.time()
    for arm, bi, vs in bases:
        B = Base(F, vs)
        for ti, R in enumerate(tgts):
            dec = l21(F, B, R)['dec']
            cnf, c = cnf_m3(F, B, R)
            with Solver(name='cadical153', bootstrap_with=cnf.cl) as s:
                s.conf_budget(budget)
                res = s.solve_limited(expect_interrupt=False)
                stt = s.accum_stats()
                model = s.get_model() if res else None
            censored = res is None
            if res:
                xs = model_xs(B, c, model)
                assert lifts_to(F, xs, R) or lifts_to(F, xs, F.neg(R))
            if res is not None:
                assert res == dec, ('SAT/truth mismatch', arm, bi, ti)
            out.write(json.dumps(dict(unit='sat3', l=l, n=n, arm=arm, base=bi, w=B.w,
                                      npoints=len(B.points), tidx=ti, dec=dec, censored=censored,
                                      conflicts=int(stt['conflicts']), decisions=int(stt['decisions']),
                                      propagations=int(stt['propagations']),
                                      nvars=cnf.nv, nclauses=len(cnf.cl))) + '\n')
            out.flush()
        print('sat cell', l, n, 'base', arm, bi, round(time.time() - t0, 1), flush=True)
    out.close()


if __name__ == '__main__':
    if sys.argv[1] == 'defect':
        defect_min()
    elif sys.argv[1] == 'l21':
        run_l21()
    elif sys.argv[1] == 'sat':
        run_sat(int(sys.argv[2]), int(sys.argv[3]))
