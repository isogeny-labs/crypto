"""Q2: height ratios rho_G on Mestre-type curves with rank-7..9 subgroups.

prod_{i=1..8}(x-a_i) = q(x)^2 - r(x), deg r <= 3; points (a_i, q(a_i)) on y^2 = r(x), sum = O.
Extra points: small-height search on y^2 = r(x) (x = m/n), kept if they raise the Gram rank.
usage: mestre.py SEED NCURVES ABITS NDOUBLE SEARCHBOX OUTJSONL
"""
import sys, json, random, time
from fractions import Fraction as Fr
from math import isqrt, gcd
import numpy as np
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from heights import Curve, gram, svp_gram


def polymul(a, b):
    r = [Fr(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] += x * y
    return r


def mestre8(avals):
    p = [Fr(1)]
    for a in avals:
        p = polymul(p, [Fr(-a), Fr(1)])  # coefficients low->high
    # q monic deg 4: q^2 matches p in degrees 8..4
    P = p[::-1]  # high->low, P[0]=1
    q = [Fr(1)]  # high->low
    for k in range(1, 5):
        # coefficient of x^{8-k} in q^2 = sum_{i+j=k} q_i q_j
        s = sum(q[i] * q[k - i] for i in range(1, k)) if k > 1 else Fr(0)
        q.append((P[k] - s) / 2)
    qlow = q[::-1]
    q2 = polymul(qlow, qlow)
    r = [q2[i] - p[i] for i in range(9)]
    assert all(r[i] == 0 for i in range(4, 9)), r
    return qlow, r[:4]


def evalp(c, x):
    s = Fr(0)
    for a in reversed(c):
        s = s * x + a
    return s


def integral_model(r):
    c0, c1, c2, c3 = r
    if c3 == 0:
        return None
    A2, A4, A6 = c2, c1 * c3, c0 * c3 * c3
    u = 1
    while not ((u ** 2 * A2).denominator == 1 and (u ** 4 * A4).denominator == 1 and (u ** 6 * A6).denominator == 1):
        u += 1
        if u > 10 ** 6:
            raise RuntimeError('u')
    E = Curve(int(u ** 2 * A2), int(u ** 4 * A4), int(u ** 6 * A6))
    mp = lambda x, y: (Fr(u ** 2) * c3 * x, Fr(u ** 3) * c3 * y)
    return E, mp


def search_points(r, box, xr=40, limit=None):
    """rational x=m/n, |m/n|<=xr, 1<=n<=box with y^2 = r(x)."""
    c0, c1, c2, c3 = r
    D = 1
    for c in r:
        D = D * c.denominator // gcd(D, c.denominator)
    C = [int(c * D) for c in r]
    out = []
    for n in range(1, box + 1):
        for m in range(-xr * n, xr * n + 1):
            if gcd(m, n) != 1:
                continue
            F = C[3] * m ** 3 + C[2] * m * m * n + C[1] * m * n * n + C[0] * n ** 3  # = D n^3 r(m/n)
            v = F * n * D  # (n^2 y)^2 * D^2 = D n^4 r * D
            if v < 0:
                continue
            s = isqrt(v)
            if s * s == v:
                out.append((Fr(m, n), Fr(s, n * n * D)))
                if limit and len(out) >= limit:
                    return out
    return out


def rank_tol(M, tol=1e-4):
    w = np.linalg.eigvalsh(M)
    return int(np.sum(w > tol * max(w.max(), 1e-12)))


def main():
    seed, ncurves, abits, nd, box, outp = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5]), sys.argv[6]
    rng = random.Random(seed)
    fo = open(outp, 'w')
    done = 0
    tries = 0
    while done < ncurves and tries < 20 * ncurves:
        tries += 1
        lim = 30 if abits == 0 else 2 ** abits
        avals = []
        while len(avals) < 8:
            a = rng.randrange(-lim, lim + 1)
            if a not in avals:
                avals.append(a)
        q, r = mestre8(avals)
        im = integral_model(r)
        if im is None:
            continue
        E, mp = im
        pts = [mp(Fr(a), evalp(q, Fr(a))) for a in avals]
        assert all(E.on(P) for P in pts)
        S = None
        for P in pts:
            S = E.add(S, P)
        sum_is_O = S is None
        t0 = time.time()
        # controls on height code
        h1, e1, _ = E.hhat(pts[0], nd)
        h2, _, _ = E.hhat(E.add(pts[0], pts[0]), nd)
        Ppq = E.add(pts[0], pts[1]); Pmq = E.add(pts[0], E.neg(pts[1]))
        hb, _, _ = E.hhat(pts[1], nd)
        hpq, _, _ = E.hhat(Ppq, nd); hmq, _, _ = E.hhat(Pmq, nd)
        ctrl = {'dbl_rel': abs(h2 - 4 * h1) / (4 * h1), 'par_rel': abs(hpq + hmq - 2 * h1 - 2 * hb) / (2 * h1 + 2 * hb)}
        base = pts[:7]  # P8 = -(P1+..+P7)
        extras = []
        if box > 0:
            cand = search_points(r, box, xr=max(40, 2 * max(abs(a) for a in avals)), limit=60)
            cur = list(base)
            Mc, _ = gram(E, cur, nd)
            rk = rank_tol(Mc)
            for (x, y) in cand:
                Pn = mp(x, y)
                if not E.on(Pn) or any(Pn[0] == Q[0] for Q in cur) or any(Pn[0] == Q[0] for Q in pts):
                    continue
                M2, _ = gram(E, cur + [Pn], nd)
                if rank_tol(M2, 1e-3) > rk:
                    cur.append(Pn); extras.append(Pn); rk += 1
                    if len(extras) >= 2:
                        break
        rec = {'seed': seed, 'a': avals, 'abits': abits, 'sum_is_O': sum_is_O, 'ctrl': ctrl,
               'a2a4a6_bits': [abs(E.a2).bit_length(), abs(E.a4).bit_length(), abs(E.a6).bit_length()],
               'cases': []}
        for s, gens, allpts in [(7, base, pts), (8, base + extras[:1], pts + extras[:1]), (9, base + extras[:2], pts + extras[:2])]:
            if len(gens) < s:
                continue
            M, err = gram(E, gens, nd)
            w = np.linalg.eigvalsh(M)
            if w.min() <= 1e-6 * w.max():
                rec['cases'].append({'s': s, 'degenerate': True}); continue
            lam, v = svp_gram(M)
            # exact check: is the SVP combination torsion (=> rank deficiency masked by height error)?
            Sx = None
            for ci, Pg in zip(v.tolist(), gens):
                if ci:
                    Sx = E.add(Sx, E.mul(int(ci), Pg))
            tors = any(E.mul(m, Sx) is None for m in range(1, 13)) if Sx is not None else True
            if tors:
                rec['cases'].append({'s': s, 'degenerate': True, 'reason': 'svp combination is torsion', 'svp': v.tolist()}); continue
            hs = [E.hhat(P, nd)[0] for P in allpts]
            H = max(hs)
            reg = float(np.linalg.det(M))
            rec['cases'].append({'s': s, 'k': len(allpts), 'H': H, 'hmin_pts': min(hs), 'lambda_G': lam, 'svp': v.tolist(),
                                 'rho_G': H / lam, 'reg': reg, 'maxerr': float(err.max()),
                                 'min_eig': float(w.min())})
        rec['sec'] = time.time() - t0
        fo.write(json.dumps(rec) + '\n'); fo.flush()
        print(json.dumps({k: rec[k] for k in ('a', 'ctrl', 'sec')}), [(c['s'], round(c.get('rho_G', -1), 3)) for c in rec['cases']], flush=True)
        done += 1


if __name__ == '__main__':
    main()
