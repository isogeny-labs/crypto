#!/usr/bin/env python3
"""LEAD-ICP-gfp5-symmetrized-gaudry toy: decomposition-yield conservation, n=3, E/F_{q^3}, FB = {x in F_q}.
Stdlib only. --smoke for a trivial size."""
import sys, json, random, math, time

def run(qs, ncurves, seed, budget):
    t0 = time.time()
    curves = []
    for q in qs:
        rng = random.Random(seed * 1000003 + q)
        # irreducible cubic t^3 + u t + v over F_q (no root)
        while True:
            u, v = rng.randrange(q), rng.randrange(q)
            if all((x**3 + u*x + v) % q for x in range(q)):
                break
        def mul(a, b):
            c0 = a[0]*b[0]
            c1 = a[0]*b[1] + a[1]*b[0]
            c2 = a[0]*b[2] + a[1]*b[1] + a[2]*b[0]
            c3 = a[1]*b[2] + a[2]*b[1]
            c4 = a[2]*b[2]
            # t^3=-u t - v ; t^4=-u t^2 - v t
            return ((c0 - v*c3) % q, (c1 - u*c3 - v*c4) % q, (c2 - u*c4) % q)
        def add_(a, b): return ((a[0]+b[0]) % q, (a[1]+b[1]) % q, (a[2]+b[2]) % q)
        def sub_(a, b): return ((a[0]-b[0]) % q, (a[1]-b[1]) % q, (a[2]-b[2]) % q)
        ONE = (1, 0, 0); ZERO = (0, 0, 0)
        Q3 = q**3
        def inv(a):
            r, e, b = ONE, Q3 - 2, a
            while e:
                if e & 1: r = mul(r, b)
                b = mul(b, b); e >>= 1
            return r
        els = [(i, j, k) for i in range(q) for j in range(q) for k in range(q)]
        sqcnt = {}
        for y in els:
            s = mul(y, y); sqcnt[s] = sqcnt.get(s, 0) + 1
        for ci in range(ncurves):
            if time.time() - t0 > budget: break
            a = tuple(rng.randrange(q) for _ in range(3)); b = tuple(rng.randrange(q) for _ in range(3))
            def f(x): return add_(add_(mul(mul(x, x), x), mul(a, x)), b)
            disc_ok = True
            N = 1; fb = []
            ysq = {}
            for y in els: pass
            # point count
            for x in els:
                N += sqcnt.get(f(x), 0)
            # FB: x in F_q, need y: build sqrt for needed values only
            need = {}
            for xv in range(q):
                x = (xv, 0, 0); fx = f(x)
                if sqcnt.get(fx, 0): need[fx] = None
            if need:
                for y in els:
                    s = mul(y, y)
                    if s in need and need[s] is None: need[s] = y
            for xv in range(q):
                x = (xv, 0, 0); fx = f(x)
                y = need.get(fx)
                if y is None and fx != ZERO and fx not in need: continue
                if y is None: continue
                fb.append((x, y))
                if y != ZERO: fb.append((x, sub_(ZERO, y)))
            def padd(P, Q_):
                if P is None: return Q_
                if Q_ is None: return P
                (x1, y1), (x2, y2) = P, Q_
                if x1 == x2:
                    if add_(y1, y2) == ZERO: return None
                    lam = mul(add_(mul(mul(x1, x1), (3, 0, 0)), a), inv(add_(y1, y1)))
                else:
                    lam = mul(sub_(y2, y1), inv(sub_(x2, x1)))
                x3 = sub_(sub_(mul(lam, lam), x1), x2)
                return (x3, sub_(mul(lam, sub_(x1, x3)), y1))
            F = len(fb)
            neg = lambda P: (P[0], sub_(ZERO, P[1]))
            sums_main = {}; T = 0
            sums_deg = {}; Td = 0
            for i in range(F):
                for j in range(i+1, F):
                    pij = padd(fb[i], fb[j])
                    anti_ij = (fb[i][0] == fb[j][0] and fb[i][1] != fb[j][1])
                    for k in range(j+1, F):
                        s = padd(pij, fb[k])
                        key = s
                        sums_deg[key] = sums_deg.get(key, 0) + 1; Td += 1
                        xi, xj, xk = fb[i][0], fb[j][0], fb[k][0]
                        if anti_ij or xi == xk or xj == xk: continue
                        sums_main[key] = sums_main.get(key, 0) + 1; T += 1
            curves.append(dict(q=q, N=N, F=F, T=T, D=len(sums_main), Td=Td, Dd=len(sums_deg)))
    # statistic: collision-excess ratio r=(T-D)/(T-E[D_null]); E[D_null]=N(1-(1-1/N)^T)
    def r_of(c, deg=False):
        T = c['Td'] if deg else c['T']; D = c['Dd'] if deg else c['D']
        ed = c['N'] * (1 - math.exp(T * math.log1p(-1.0 / c['N'])))
        return (T - D) / max(T - ed, 1e-9)
    def pooled(cs, deg=False):
        num = sum((c['Td'] - c['Dd']) if deg else (c['T'] - c['D']) for c in cs)
        den = sum((c['Td'] if deg else c['T']) - c['N'] * (1 - math.exp((c['Td'] if deg else c['T']) * math.log1p(-1.0 / c['N']))) for c in cs)
        return num / max(den, 1e-9)
    def boot(cs, deg=False, B=400):
        rg = random.Random(seed + 7); out = []
        for _ in range(B):
            out.append(pooled([cs[rg.randrange(len(cs))] for _ in cs], deg))
        out.sort(); return out[int(.025*B)], out[int(.975*B)]
    # null: random balls
    rn = random.Random(seed + 11); nullr = []
    for c in curves:
        bins = set(rn.randrange(c['N']) for _ in range(c['T']))
        nc = dict(c); nc['D'] = len(bins); nullr.append(nc)
    r_main = pooled(curves); ci_main = boot(curves)
    r_deg = pooled(curves, True); ci_deg = boot(curves, True)
    r_null = pooled(nullr); ci_null = boot(nullr)
    A = curves[0::2]; Bc = curves[1::2]
    if len(A) and len(Bc):
        ra, rb = pooled(A), pooled(Bc)
        aa_diff = abs(ra - rb); aa_tol = 0.5 * ((boot(A)[1]-boot(A)[0]) + (boot(Bc)[1]-boot(Bc)[0])) / 2 * 2
    else:
        ra = rb = aa_diff = aa_tol = None
    pos_ok = ci_deg[0] > 1.5
    null_ok = ci_null[0] <= 1.0 <= ci_null[1] or (0.9 <= r_null <= 1.1)
    aa_ok = (aa_diff is None) or aa_diff <= aa_tol + 0.05
    if not (pos_ok and null_ok and aa_ok):
        verdict = "inconclusive"
    elif 0.9 <= ci_main[0] and ci_main[1] <= 1.1:
        verdict = "consistent_with_prediction"
    elif ci_main[1] < 0.9 or ci_main[0] > 1.1:
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    return dict(lead="LEAD-ICP-gfp5-symmetrized-gaudry", n=3, seed=seed, qs=qs, curves_per_q=ncurves,
                n_curves=len(curves), curves=curves,
                statistic="pooled collision-excess ratio r (observed/expected extra collisions, antipodal pairs excluded)",
                r_main=r_main, ci_main=ci_main, r_positive_control_degenerate_triples=r_deg, ci_pos=ci_deg,
                r_null_random_balls=r_null, ci_null=ci_null, aa_r=[ra, rb], aa_diff=aa_diff,
                controls=dict(pos_ok=pos_ok, null_ok=null_ok, aa_ok=aa_ok),
                threshold="consistent iff 95% bootstrap CI of r_main within [0.9,1.1] and all controls pass",
                elapsed_s=round(time.time()-t0, 1), verdict=verdict)

if __name__ == "__main__":
    if "--smoke" in sys.argv:
        res = run([7, 11], 2, 20261110, 8)
    else:
        res = run([29, 31, 37, 41, 43], 8, 20261110, 480)
    print(json.dumps(res))
    sys.exit(0)
