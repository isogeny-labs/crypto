#!/usr/bin/env python3
"""L14: digit-set frames, exact m=2 subgroup-coverage census (pure Python).

Factor bases on the x_W-line of a curve25519-family miniature:
  translation frame   x = L  + y,                y in [0, 2^t)      (engine form bits@L)
  digit-set frame     x = lo + sum_j c_j b_j,    b_j in {0,1}       (c_j in F_p, free)
The bit-encoded F4 system is identical in shape for both (only the linear
coefficients 2^j -> c_j change), so the digit set is a free selection space.

Instrument: research/curve25519_frame_search_20261103/scripts/frame_census.py
(build: exact full-group logs in Z/8r indexed by Montgomery u, x_W = u + A/3;
coverage: exact covered = |{R != O in E[r] : R = +-P_i +-P_j}|, cols, and the
log-free selector J = #unordered signed pairs whose cofactor cosets cancel).
Nothing here is timed; costs are counted (Legendre symbols, [r]P labels).
"""
import json, os, random, sys, time
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
FRAME_SEARCH = os.path.join(HERE, '..', '..', '..', 'curve25519_frame_search_20261103', 'scripts')
sys.path.insert(0, os.path.abspath(FRAME_SEARCH))
from frame_census import build, coverage  # frozen instrument (#3359)

LOG2R_FACTOR = 1.5  # group ops per [r]P label, frame_search convention


def digit_xs(p, lo, digits):
    """All 2^t subset sums lo + sum c_j b_j (mod p), as a list (with repeats)."""
    xs = [lo % p]
    for c in digits:
        xs = xs + [(x + c) % p for x in xs]
    return xs


def logs_of(xs, logu, A3, p):
    """Sign-folded full logs of the base points with abscissa in xs (distinct x only)."""
    out = set()
    for x in set(xs):
        k = int(logu[(x - A3) % p])
        if k:
            out.add(k)
    return sorted(out)


def evaluate(xs, logu, A3, p, n, r):
    logs = logs_of(xs, logu, A3, p)
    cov, cols, J = coverage(logs, n, r)
    return dict(cols=cols, covered=cov, J=J, distinct_x=len(set(xs)))


def part_a(t, p, A3, logu, n, r, K, rng):
    """K random translation frames, then K random digit sets, one rng stream."""
    trans, digit = [], []
    leg_t = rP_t = leg_d = rP_d = 0
    for i in range(K):
        L = rng.randrange(p)
        xs = [(L + y) % p for y in range(1 << t)]
        e = evaluate(xs, logu, A3, p, n, r)
        leg_t += e['distinct_x']; rP_t += e['cols']
        trans.append(dict(L=L, **e))
    for i in range(K):
        lo = rng.randrange(p)
        digits = [rng.randrange(1, p) for _ in range(t)]
        xs = digit_xs(p, lo, digits)
        e = evaluate(xs, logu, A3, p, n, r)
        leg_d += e['distinct_x']; rP_d += e['cols']
        digit.append(dict(lo=lo, digits=digits, **e))
    return dict(translation=trans, digit=digit,
                cost=dict(translation=dict(legendre=leg_t, rP=rP_t),
                          digit=dict(legendre=leg_d, rP=rP_d)))


def greedy(t, p, A3, logu, n, r, B, rng):
    """Choose lo at random, then c_1..c_t sequentially: at step j scan B random
    candidate digits and keep the one maximising J of the partial sumset
    (ties: fewer columns, then the earlier candidate). Counts Legendre symbols
    (new abscissae tested per candidate) and [r]P labels (new on-curve
    abscissae per candidate)."""
    lo = rng.randrange(p)
    S = [lo % p]
    digits, steps = [], []
    legendre = rP = 0
    base_eval = evaluate(S, logu, A3, p, n, r)
    steps.append(dict(j=0, digit=None, **base_eval))
    for j in range(1, t + 1):
        best = None
        for b in range(B):
            c = rng.randrange(1, p)
            new = [(x + c) % p for x in S]
            new_distinct = set(new) - set(S)
            legendre += len(new_distinct)
            rP += sum(1 for x in new_distinct if int(logu[(x - A3) % p]))
            e = evaluate(S + new, logu, A3, p, n, r)
            key = (e['J'], -e['cols'], -b)
            if best is None or key > best[0]:
                best = (key, c, e)
        _, c, e = best
        digits.append(c)
        S = S + [(x + c) % p for x in S]
        steps.append(dict(j=j, digit=c, **e))
    final = steps[-1]
    return dict(lo=lo, digits=digits, steps=steps,
                cols=final['cols'], covered=final['covered'], J=final['J'],
                distinct_x=final['distinct_x'],
                cost=dict(legendre=legendre, rP=rP,
                          group_ops=rP * LOG2R_FACTOR * np.log2(r)))


def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument('--k', type=int, required=True)
    ap.add_argument('--p', type=int, required=True)
    ap.add_argument('--A', type=int, required=True)
    ap.add_argument('--n', type=int, required=True)
    ap.add_argument('--t', type=int, nargs='+', default=[4, 5, 6])
    ap.add_argument('--K', type=int, default=2000)
    ap.add_argument('--B', type=int, default=2000)
    ap.add_argument('--R', type=int, default=5, help='greedy replicates per t')
    ap.add_argument('--seed-base', type=int, default=20261104)
    ap.add_argument('--out-dir', required=True)
    a = ap.parse_args()
    p, A, n = a.p, a.A, a.n
    t0 = time.time()
    logu, r = build(p, A, n)
    print(f'k={a.k}: log table built ({time.time()-t0:.1f}s), r={r}', file=sys.stderr, flush=True)
    A3 = A * pow(3, -1, p) % p
    os.makedirs(a.out_dir, exist_ok=True)
    for t in a.t:
        # fixed reference: bits lo=0 (instrument cross-check against frame_search summary)
        ref = evaluate([y % p for y in range(1 << t)], logu, A3, p, n, r)
        seed_a = a.seed_base + t
        pa = part_a(t, p, A3, logu, n, r, a.K, random.Random(seed_a))
        reps = []
        for rep in range(a.R):
            seed_b = a.seed_base + t + 1000 * (rep + 1)
            g = greedy(t, p, A3, logu, n, r, a.B, random.Random(seed_b))
            g['rep'] = rep; g['seed'] = seed_b
            reps.append(g)
        out = dict(k=a.k, p=p, A=A, n=n, r=r, t=t, K=a.K, B=a.B, R=a.R,
                   seed_part_a=seed_a, bits_lo0=ref, part_a=pa, greedy=reps,
                   instrument=dict(frame_census=os.path.abspath(os.path.join(FRAME_SEARCH, 'frame_census.py')),
                                   log2r_factor=LOG2R_FACTOR))
        path = os.path.join(a.out_dir, f'k{a.k}_t{t}.json')
        tmp = path + '.tmp'
        json.dump(out, open(tmp, 'w'))
        os.replace(tmp, path)
        print(f'k={a.k} t={t} done ({time.time()-t0:.0f}s): bits_lo0 {ref["covered"]}, '
              f'trans max {max(x["covered"] for x in pa["translation"])}, '
              f'digit max {max(x["covered"] for x in pa["digit"])}, '
              f'greedy rep0 {reps[0]["covered"]} (cols {reps[0]["cols"]})', file=sys.stderr, flush=True)


if __name__ == '__main__':
    main()
