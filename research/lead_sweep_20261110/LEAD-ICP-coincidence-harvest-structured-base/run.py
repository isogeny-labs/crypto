#!/usr/bin/env python3
"""Toy break-even ladder for kappa_rel of a window base (stdlib only)."""
import json, math, random, sys
from collections import Counter

def curve(p, rng):
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p:
            break
    sq = {}
    for y in range(p):
        sq.setdefault(y*y % p, y)
    pts = []
    for x in range(p):
        r = (x**3 + a*x + b) % p
        if r in sq:
            pts.append((x, sq[r]))
            if sq[r]:
                pts.append((x, p - sq[r]))
    return a, b, len(pts) + 1, pts

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l*l - x1 - x2) % p
    return (x3, (l*(x1 - x3) - y1) % p)

def coll(base, a, p):
    c = Counter()
    n = len(base)
    for i in range(n):
        for j in range(i + 1, n):
            S = add(base[i], base[j], a, p)
            if S is not None: c[S[0]] += 1
    return sum(v*(v-1)//2 for v in c.values())

def one(p, nb, seed):
    rng = random.Random(seed)
    a, b, N, pts = curve(p, rng)
    xs = sorted({P[0] for P in pts})
    byx = {}
    for P in pts: byx.setdefault(P[0], P)
    window = [byx[x] for x in xs[:nb]]
    rnd = lambda: [byx[x] for x in rng.sample(xs, nb)]
    r1, r2 = rnd(), rnd()
    g = rng.choice(pts)
    gap, Q = [], None
    for _ in range(nb):
        Q = add(Q, g, a, p)
        if Q is None: break
        gap.append(Q)
    cw, c1, c2, cg = (coll(window, a, p), coll(r1, a, p), coll(r2, a, p), coll(gap, a, p))
    return N, cw, c1, c2, cg

def primes_near(k, m):
    out, c = [], (1 << k) + 1
    while len(out) < m:
        if all(c % q for q in range(2, int(c**.5) + 1)): out.append(c)
        c += 2
    return out

def slope(xs, ys):
    mx, my = sum(xs)/len(xs), sum(ys)/len(ys)
    return sum((x-mx)*(y-my) for x, y in zip(xs, ys)) / sum((x-mx)**2 for x in xs)

def main():
    smoke = "--smoke" in sys.argv
    bits, reps = ([8, 9, 10], 2) if smoke else ([12, 13, 14, 15, 16, 17], 6)
    rows, per_rep = [], {}
    for k in bits:
        p = primes_near(k, 1)[0]
        nb = max(8, int(math.sqrt(p)) * (1 if smoke else 2))
        sw = s1 = s2 = sg = 0; lns = []
        for r in range(reps):
            N, cw, c1, c2, cg = one(p, nb, 1000003*k + r)
            sw += cw; s1 += c1; s2 += c2; sg += cg
            if c1 > 0 and cw > 0: per_rep.setdefault(r, []).append((math.log(N), math.log(cw/c1)))
        rows.append(dict(bits=k, p=p, base=nb, kappa_window=sw/max(s1,1), AA_random_vs_random=s1/max(s2,1),
                         kappa_logGAP_positive=sg/max(s1,1), coll_window=sw, coll_random=s1, coll_random2=s2))
    sl = [slope([a for a, _ in v], [b for _, b in v]) for v in per_rep.values() if len(v) >= 3]
    ms = sum(sl)/len(sl) if sl else float('nan')
    se = (sum((s-ms)**2 for s in sl)/max(len(sl)-1, 1) / max(len(sl), 1))**.5 if len(sl) > 1 else float('nan')
    kw = [r['kappa_window'] for r in rows]; aa = [r['AA_random_vs_random'] for r in rows]
    pos = min(r['kappa_logGAP_positive'] for r in rows)
    controls_ok = pos >= 3 and all(abs(x-1) <= 0.15 for x in aa)
    if smoke: verdict = "inconclusive"
    elif not controls_ok: verdict = "inconclusive"
    elif ms == ms and se == se and ms > 0.05 and ms/max(se,1e-9) > 3: verdict = "falsified"
    elif all(abs(x-1) <= 0.1 for x in kw) and abs(ms) < 0.03: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps(dict(protocol="LEAD-ICP-coincidence-harvest-structured-base", smoke=smoke, seeds="1000003*bits+rep",
        bits=bits, reps=reps, rows=rows, slope_dlnkappa_dlnN=ms, slope_se=se, n_slopes=len(sl),
        positive_control_min_kappa=pos, controls_ok=controls_ok, verdict=verdict)))
main()
