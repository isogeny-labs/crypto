#!/usr/bin/env python3
"""LEAD-ICP-gfp5-odd-char-cover toy: does a subfield (x in F_p) Gaudry factor base
cover E(F_{p^n}) better than a random factor base of equal size? stdlib only."""
import sys, json, random, time, math
SMOKE = '--smoke' in sys.argv
T0 = time.time(); BUDGET = 450

class Fld:
    def __init__(s, p, n, rng):
        s.p, s.n, s.q = p, n, p**n
        while True:  # monic x^n + a x + b irreducible (n prime): x^(p^n)=x, x^p!=x
            a, b = rng.randrange(1, p), rng.randrange(1, p)
            s.red = [b, a] + [0]*(n-2)  # x^n = -(a x + b)
            s.red = [(-c) % p for c in s.red]
            if s._irred(): break
        s._tables(rng)
    def pmul(s, u, v):
        p, n = s.p, s.n
        r = [0]*(2*n-1)
        for i, x in enumerate(u):
            if x:
                for j, y in enumerate(v): r[i+j] += x*y
        for k in range(2*n-2, n-1, -1):
            c = r[k] % p; r[k] = 0
            if c:
                for i, rc in enumerate(s.red): r[k-n+i] += c*rc
        return [c % p for c in r[:n]]
    def _irred(s):
        n = s.n; X = [0, 1] + [0]*(n-2)
        def pw(e):
            r = [1]+[0]*(n-1); b = X
            while e:
                if e & 1: r = s.pmul(r, b)
                b = s.pmul(b, b); e >>= 1
            return r
        return pw(s.p) != X and pw(s.q) == X
    def enc(s, v):
        r = 0
        for c in reversed(v): r = r*s.p + c
        return r
    def dec(s, e):
        v = []
        for _ in range(s.n): e, c = divmod(e, s.p); v.append(c)
        return v
    def _tables(s, rng):
        q, n = s.q, s.n
        while True:
            g = [rng.randrange(s.p) for _ in range(n)]
            if not any(g): continue
            ex = [0]*(q-1); cur = [1]+[0]*(n-1); ok = True
            for i in range(q-1):
                e = s.enc(cur)
                if i and e == 1: ok = False; break
                ex[i] = e; cur = s.pmul(cur, g)
            if ok and s.enc(cur) == 1: break
        s.ex = ex; s.lg = [None]*q
        for i, e in enumerate(ex): s.lg[e] = i
    def add(s, a, b):
        p = s.p; r = 0; m = 1
        for _ in range(s.n):
            a, x = divmod(a, p); b, y = divmod(b, p)
            r += ((x+y) % p)*m; m *= p
        return r
    def neg(s, a):
        p = s.p; r = 0; m = 1
        for _ in range(s.n):
            a, x = divmod(a, p); r += ((-x) % p)*m; m *= p
        return r
    def mul(s, a, b):
        if a == 0 or b == 0: return 0
        return s.ex[(s.lg[a]+s.lg[b]) % (s.q-1)]
    def inv(s, a): return s.ex[(-s.lg[a]) % (s.q-1)]
    def sqrt(s, a):
        if a == 0: return 0
        l = s.lg[a]
        return s.ex[l//2] if l % 2 == 0 else None

class Curve:
    def __init__(s, F, rng):
        s.F = F; q = F.q
        s.a = rng.randrange(1, q); s.b = rng.randrange(1, q)
        s.pts_by_x = {}
        n = 0
        for x in range(q):
            r = s.rhs(x); y = F.sqrt(r)
            if y is None: continue
            n += 1 if y == 0 else 2
            s.pts_by_x[x] = y
        s.N = n + 1
    def rhs(s, x):
        F = s.F; x2 = F.mul(x, x)
        return F.add(F.add(F.mul(x2, x), F.mul(s.a, x)), s.b)
    def padd(s, P, Q):
        F = s.F
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if F.add(y1, y2) == 0: return None
            three = F.add(F.add(1, 1), 1)
            num = F.add(F.mul(three, F.mul(x1, x1)), s.a)
            lam = F.mul(num, F.inv(F.add(y1, y1)))
        else:
            lam = F.mul(F.add(y2, F.neg(y1)), F.inv(F.add(x2, F.neg(x1))))
        x3 = F.add(F.add(F.mul(lam, lam), F.neg(x1)), F.neg(x2))
        y3 = F.add(F.mul(lam, F.add(x1, F.neg(x3))), F.neg(y1))
        return (x3, y3)
    def neg(s, P): return None if P is None else (P[0], s.F.neg(P[1]))

def covered(C, classes, m):
    """distinct group elements that are sums of m points from distinct x-classes, any signs."""
    seen = set(); count = 0
    cl = [(P, C.neg(P)) for P in classes]
    def rec(start, left, acc):
        nonlocal count
        if left == 0:
            seen.add(acc); count += 1; return
        for i in range(start, len(cl) - left + 1):
            for Q in (cl[i] if cl[i][0][1] != cl[i][1][1] else cl[i][:1]):
                rec(i+1, left-1, C.padd(acc, Q))
    rec(0, m, None)
    return len(seen), count

def classes_subfield(C):
    p = C.F.p
    return [(x, C.pts_by_x[x]) for x in range(p) if x in C.pts_by_x]
def classes_random(C, c, rng):
    xs = rng.sample(sorted(C.pts_by_x), c)
    return [(x, C.pts_by_x[x]) for x in xs]
def classes_ap(C, c, rng):  # positive control: multiples kP, structured, many collisions
    xs = sorted(C.pts_by_x); P = (xs[7], C.pts_by_x[xs[7]]); out = []; R = None; seen = set()
    while len(out) < c:
        R = C.padd(R, P)
        if R is None: break
        if R[0] not in seen: seen.add(R[0]); out.append(R)
    return out

def run_cell(p, n, m, seed, nrand):
    rng = random.Random(seed)
    F = Fld(p, n, rng); C = Curve(F, rng)
    sub = classes_subfield(C); c = len(sub)
    cs, N = covered(C, sub, m)
    nulls = []
    for _ in range(nrand):
        r, _n = covered(C, classes_random(C, c, rng), m); nulls.append(r)
    ap, _n = covered(C, classes_ap(C, c, rng), m)
    lam = N / C.N
    return dict(p=p, n=n, m=m, seed=seed, order=C.N, classes=c, sums=N, lam=round(lam, 5),
                sub=cs, null=nulls, ap=ap, exp_distinct=round(C.N*(1-math.exp(-lam)), 1))

def main():
    if SMOKE: cells = [(7, 3, 3, 11, 2)]
    else:
        cells = [(61, 3, 3, s, 3) for s in (101, 102, 103)] + [(499, 2, 2, s, 3) for s in (201, 202, 203)]
    rows = []
    for cell in cells:
        if time.time() - T0 > BUDGET: break
        rows.append(run_cell(*cell))
    # n=5 descriptive count only (lambda ~ 1/5!)
    n5 = None
    if not SMOKE and time.time() - T0 < BUDGET:
        n5 = run_cell(7, 5, 5, 301, 1)
    sub = sum(r['sub'] for r in rows); nul = sum(sum(r['null'])/len(r['null']) for r in rows)
    ap = sum(r['ap'] for r in rows)
    R = sub/nul if nul else float('nan'); Rap = ap/nul if nul else float('nan')
    # per-cell SD of null for noise floor (A/A: null vs null spread)
    aa = []
    for r in rows:
        if len(r['null']) > 1: aa.append((max(r['null'])-min(r['null']))/max(1, sum(r['null'])/len(r['null'])))
    if len(rows) < len(cells) or not rows: v = 'inconclusive'
    elif Rap > 0.9: v = 'inconclusive'          # detector blind: positive control failed
    elif abs(R-1) < 0.1: v = 'consistent_with_prediction'
    elif R > 1.25: v = 'falsified'
    else: v = 'inconclusive'
    print(json.dumps(dict(lead='LEAD-ICP-gfp5-odd-char-cover', smoke=SMOKE, statistic_R_sub_over_null=R,
        positive_control_R_ap_over_null=Rap, aa_relative_range=aa, thresholds=dict(consistent='|R-1|<0.1',
        falsified='R>1.25', detector_ok='Rap<=0.9'), n5_descriptive=n5, cells=rows, verdict=v,
        elapsed_s=round(time.time()-T0, 1))))
main()
