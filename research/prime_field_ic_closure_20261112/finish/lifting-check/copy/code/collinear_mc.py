"""Realisation probability of a planted collinear relation under blind balanced lifts.  [pcc:unverified]

Planted: P3 = -(P1 + P2) in E(F_p) (so P1, P2, P3 are collinear mod p).  On any Weierstrass
cubic through the integer lifts, P1' + P2' + P3' = O holds in E'(Q) iff the three integer
lifts are collinear over Z (a line meets the cubic in exactly three points; the origin is
the flex at infinity).  So the realisation probability is Pr[det = 0 | det = 0 mod p].
"""
import sys, random, math, json, argparse
sys.path.insert(0, '.')
import xedni_toy as X


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--bits', type=int, nargs='+', default=[8, 10, 12, 14, 16, 18, 20])
    ap.add_argument('--samples', type=int, default=400000)
    ap.add_argument('--curves', type=int, default=4)
    ap.add_argument('--out', required=True)
    a = ap.parse_args()
    res = []
    for b in a.bits:
        rng = random.Random(777000 + b)
        tot = hits = 0
        ps = []
        for c in range(a.curves):
            # any curve works here; prime order not needed for the collinearity statistic,
            # but keep prime order for consistency with the pipeline
            tg = X.make_target(b, rng)
            p, A, B = tg['p'], tg['A'], tg['B']
            ps.append(p)
            n = a.samples // a.curves
            for _ in range(n):
                P1 = X.random_point(A, B, p, rng)
                P2 = X.random_point(A, B, p, rng)
                S12 = X.ec_add(P1, P2, A, p)
                if S12 is None:
                    continue
                P3 = X.ec_neg(S12, p)
                (x1, y1), (x2, y2), (x3, y3) = [(X.balanced(P[0], p), X.balanced(P[1], p)) for P in (P1, P2, P3)]
                det = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
                assert det % p == 0
                tot += 1
                hits += (det == 0)
        pbar = sum(ps) / len(ps)
        rec = dict(bits=b, p_mean=pbar, samples=tot, collinear=hits, rate=hits / tot,
                   rate_times_p=hits / tot * pbar)
        print(json.dumps(rec), flush=True)
        res.append(rec)
    json.dump(res, open(a.out, 'w'), indent=1)


if __name__ == '__main__':
    main()
