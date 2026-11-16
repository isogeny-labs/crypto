#!/usr/bin/env python3
"""LEAD-EXO-methodology-instruments: calibration of the character-sum instrument
with planted signals on toy elliptic-curve DL labellings. stdlib only."""
import sys, json, math, random

SMOKE = "--smoke" in sys.argv
K = 64            # frequencies scanned
QS = [0.02, 0.05, 0.1, 0.2]

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def make_curve(p0):
    p = p0
    while not (is_prime(p) and p % 4 == 3): p += 1
    b = 1
    while True:
        a = 1
        # group order by Legendre sum
        sq = {}
        for y in range(p): sq.setdefault(y*y % p, []).append(y)
        pts = []
        for x in range(p):
            for y in sq.get((x**3 + a*x + b) % p, []): pts.append((x, y))
        n = len(pts) + 1
        # find point of large order
        for P in pts[:50]:
            order, Q = 1, P
            def add(P1, P2):
                if P1 is None: return P2
                if P2 is None: return P1
                x1, y1 = P1; x2, y2 = P2
                if x1 == x2 and (y1 + y2) % p == 0: return None
                if P1 == P2: l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
                else: l = (y2 - y1) * pow(x2 - x1, -1, p) % p
                x3 = (l*l - x1 - x2) % p
                return (x3, (l*(x1 - x3) - y1) % p)
            d = {}; Q = P; k = 1
            while Q is not None:
                d.setdefault(Q[0], k); Q = add(Q, P); k += 1
            N = k
            if N >= n // 2 and N > 50:
                return p, N, d
        b += 1

def stat(labels, N, cs):
    n = len(labels)
    best = 0.0
    w = 2 * math.pi / N
    for k in range(1, K + 1):
        s = 0.0
        for d in labels: s += math.cos(w * k * d)
        s = abs(s)
        if s > best: best = s
    return best / math.sqrt(n)

def plant(labels, N, fam, q, rng):
    out = []
    for d in labels:
        if rng.random() < q:
            if fam == "A": d = rng.randrange(N // 8)
            elif fam == "B":
                while True:
                    t = rng.randrange(N)
                    if rng.random() * 2 < 1 + math.cos(2*math.pi*17*t/N): d = t; break
            else:
                while True:
                    t = rng.randrange(N)
                    if (41 * t) % N < N / 20 or N - (41 * t) % N < N / 20: d = t; break
        out.append(d)
    return out

def main():
    ps = [1500, 2600, 4100] if not SMOKE else [300]
    nnull = 200 if not SMOKE else 12
    ntr = 60 if not SMOKE else 4
    qs = QS if not SMOKE else [0.2]
    rows = []; fpr_all = []; power = {f: {str(q): [] for q in qs} for f in "ABC"}
    nulls_ctl = []
    for ci, p0 in enumerate(ps):
        p, N, dmap = make_curve(p0)
        lab = [min(d, N - d) for x, d in dmap.items() if x < p // 2]
        rng = random.Random(1000 + ci)
        def shuf():
            l = lab[:]; rng.shuffle(l); return l
        # per-class: random-uniform labelling is the symmetric null
        nulls = sorted(stat([rng.randrange(N) for _ in lab], N, None) for _ in range(nnull))
        thr = nulls[int(0.95 * len(nulls)) - 1]
        # negative control / A-A: real labels (shuffled) vs thr; and 2 independent null sets
        aa = [stat(shuf(), N, None) > thr for _ in range(ntr)]
        fp = [stat([rng.randrange(N) for _ in lab], N, None) > thr for _ in range(ntr)]
        fpr_all += fp
        for f in "ABC":
            for q in qs:
                hits = [stat(plant(shuf(), N, f, q, rng), N, None) > thr for _ in range(ntr)]
                power[f][str(q)].append(sum(hits) / ntr)
        rows.append({"p": p, "N": N, "n_labels": len(lab), "thr95": round(thr, 4),
                     "real_label_reject_rate": sum(aa) / ntr, "null_fpr": sum(fp) / ntr})
    fpr = sum(fpr_all) / len(fpr_all)
    pw = {f: {q: round(sum(v) / len(v), 3) for q, v in d.items()} for f, d in power.items()}
    qmin = {}
    for f in "ABC":
        ok = [float(q) for q, v in pw[f].items() if v >= 0.8]
        qmin[f] = min(ok) if ok else None
    pos_ok = all(pw[f][str(qs[-1])] >= 0.8 for f in "ABC")
    fpr_ok = 0.01 <= fpr <= 0.10
    sens_ok = all(qmin[f] is not None and qmin[f] <= 0.1 for f in "ABC")
    if not pos_ok or not fpr_ok: verdict = "falsified"
    elif sens_ok: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    if SMOKE: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-EXO-methodology-instruments", "smoke": SMOKE,
        "statistic": "max_k<=64 |sum cos(2pi k d/N)|/sqrt(n)", "seeds": [1000 + i for i in range(len(ps))],
        "params": {"primes_start": ps, "K": K, "null_draws": nnull, "trials": ntr, "q_grid": qs},
        "curves": rows, "null_fpr": round(fpr, 4), "power": pw, "min_q_80pct_power": qmin,
        "controls": {"negative": "uniform-label null", "positive": "planted A/B/C at q=%s" % qs[-1], "aa": "real shuffled labels vs threshold"},
        "verdict": verdict}))
main()
