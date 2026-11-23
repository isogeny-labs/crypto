#!/usr/bin/env python3
"""Toy conservation test for split-graph factor bases (stdlib only, fixed seeds)."""
import sys, json, random, math, hashlib, time

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def curve(p, rng):
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        sq = {}
        for y in range(p): sq.setdefault(y*y % p, []).append(y)
        pts = []
        for x in range(p):
            for y in sq.get((x**3 + a*x + b) % p, []): pts.append((x, y))
        return a, b, pts

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

def coverage(base, a, p, N):
    seen = set()
    n = len(base)
    for i in range(n):
        for j in range(i, n):
            seen.add(add(base[i], base[j], a, p))
    return len(seen) / N

def graph_base(bypx, B, p, rng):
    k = max(1, math.ceil(math.log2(B)) + 1)
    c = rng.randrange(p); w = [rng.randrange(1, p) for _ in range(k)]
    xs = set()
    for m in range(1 << k):
        xs.add((c + sum(w[i] for i in range(k) if m >> i & 1)) % p)
    out = [P for x in sorted(xs) for P in bypx.get(x, [])]
    return out[:B], len(out)

def run(sizes, reps, seed):
    res = []
    for p0 in sizes:
        rng = random.Random(seed + p0)
        p = next_prime(p0)
        a, b, pts = curve(p, rng)
        N = len(pts) + 1
        B = int(math.sqrt(N)) + 1  # ~ N/2 unordered pairs -> coverage ~0.4
        bypx = {}
        for P in pts: bypx.setdefault(P[0], []).append(P)
        if len(pts) < 2 * B: continue
        null = []
        for r in range(reps):
            null.append(coverage(rng.sample(pts, B), a, p, N))
        mu = sum(null) / reps
        sd = max(math.sqrt(sum((v-mu)**2 for v in null)/(reps-1)), 1e-9)
        def z(c): return (c - mu) / sd
        g, avail = graph_base(bypx, B, p, rng)
        arms = {}
        arms["graph_boolean_offsets"] = coverage(g, a, p, N) if len(g) == B else None
        arms["low_x"] = coverage(sorted(pts)[:B], a, p, N)
        hb = sorted(pts, key=lambda P: hashlib.sha256(b"%d,%d" % P).digest())[:B]
        arms["hash_x"] = coverage(hb, a, p, N)
        aa_a = coverage(rng.sample(pts, B), a, p, N)  # A/A: fresh random base
        # positive control: first B multiples of a point (arithmetic progression, additive structure)
        P0 = pts[rng.randrange(len(pts))]
        prog, Q = [], None
        for _ in range(B):
            Q = add(Q, P0, a, p); 
            if Q is None: break
            prog.append(Q)
        arms["pos_control_progression"] = coverage(prog, a, p, N)
        row = {"p": p, "N": N, "B": B, "null_mean": mu, "null_sd": sd, "reps": reps,
               "coverage": arms, "aa_random": aa_a,
               "z": {k: (None if v is None else z(v)) for k, v in arms.items()},
               "z_aa": z(aa_a)}
        res.append(row)
    return res

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    sizes = [300] if smoke else [4000, 16000, 60000, 250000]
    reps = 8 if smoke else 300
    rows = run(sizes, reps, 20261110)
    gz = [abs(r["z"]["graph_boolean_offsets"]) for r in rows if r["z"]["graph_boolean_offsets"] is not None]
    pz = [abs(r["z"]["pos_control_progression"]) for r in rows]
    az = [abs(r["z_aa"]) for r in rows]
    if not rows or not gz: verdict = "inconclusive"
    elif min(pz) < 5 or max(az) > 3.5: verdict = "inconclusive"
    elif max(gz) >= 4: verdict = "falsified"
    elif max(gz) < 3: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICP-split-graph-secant-presentation", "smoke": smoke,
        "seed": 20261110, "statistic": "coverage z vs random same-size base null (m=2 sums)",
        "max_abs_z_graph": max(gz) if gz else None, "min_abs_z_poscontrol": min(pz) if pz else None,
        "max_abs_z_aa": max(az) if az else None, "rows": rows,
        "elapsed_s": round(time.time() - t0, 2), "verdict": verdict}))
main()
