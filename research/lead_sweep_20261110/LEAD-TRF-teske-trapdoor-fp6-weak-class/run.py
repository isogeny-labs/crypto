#!/usr/bin/env python3
"""Toy for LEAD-TRF-teske-trapdoor-fp6-weak-class: cost of a public blind 2-isogeny walk
to a weak set vs trapdoor (graph-informed) navigation, on real ordinary 2-isogeny graphs over F_p.
Stdlib only, deterministic."""
import sys, json, random, math
from collections import deque, defaultdict

def phi2(x, y, p):
    x2, y2 = x*x % p, y*y % p
    return (x*x2 + y*y2 - x2*y2 + 1488*(x2*y + x*y2) - 162000*(x2+y2)
            + 40773375*x*y + 8748000000*(x+y) - 157464000000000) % p

def is_prime(n):
    return n > 1 and all(n % q for q in range(2, int(n**.5)+1))

def build_graph(p):
    leg = [0]*p
    for y in range(1, p): leg[y*y % p] = 1
    sym = [0 if v == 0 else (1 if leg[v] else -1) for v in range(p)]
    sym[0] = 0
    cls = defaultdict(list)
    for j in range(2, p):
        if j == 1728 % p: continue
        k = j*pow((1728-j) % p, -1, p) % p
        a, b = 3*k % p, 2*k % p
        s = sum(sym[(x*x*x + a*x + b) % p] for x in range(p))
        t = abs(s)  # |trace| up to sign
        if t % p and t*t < 4*p: cls[t].append(j)
    t = max(cls, key=lambda t: len(cls[t]))
    V = cls[t]
    adj = {j: set() for j in V}
    for i, a in enumerate(V):
        for b in V[i:]:
            if phi2(a, b, p) == 0:
                adj[a].add(b); adj[b].add(a)
    # largest connected component
    seen, best = set(), []
    for s in V:
        if s in seen: continue
        comp, dq = [s], deque([s]); seen.add(s)
        while dq:
            u = dq.popleft()
            for v in adj[u]:
                if v not in seen: seen.add(v); comp.append(v); dq.append(v)
        if len(comp) > len(best): best = comp
    comp = set(best)
    return t, sorted(comp), {u: sorted(adj[u] & comp) for u in comp}

def walk_cost(adj, W, start, rng, cap):
    prev, cur, n = None, start, 0
    while n < cap:
        nb = adj[cur]
        c = [v for v in nb if v != prev] or nb
        if not c: return cap
        prev, cur = cur, rng.choice(c); n += 1
        if cur in W: return n
    return cap

def bfs_dist(adj, W):
    d = {u: 0 for u in W}; dq = deque(W)
    while dq:
        u = dq.popleft()
        for v in adj[u]:
            if v not in d: d[v] = d[u]+1; dq.append(v)
    return d

def main():
    smoke = "--smoke" in sys.argv
    primes = [211] if smoke else [1009, 1511, 2003, 2503, 3001]
    reps, starts, w = (20, 10, 0.1) if smoke else (300, 40, 0.05)
    out = {"lead": "LEAD-TRF-teske-trapdoor-fp6-weak-class", "smoke": smoke, "seed_base": 20261110,
           "weak_fraction": w, "reps": reps, "starts": starts, "per_prime": []}
    tot = {"A": [0.0, 0], "B": [0.0, 0], "inf": [0.0, 0], "ball": [0.0, 0]}
    for p in primes:
        assert is_prime(p)
        t, V, adj = build_graph(p)
        n = len(V); m = max(1, round(w*n)); cap = int(50/w)
        rng = random.Random(20261110 + p)
        row = {"p": p, "trace_abs": t, "n_vertices": n, "weak_size": m}
        for rep in range(reps):
            arm = "A" if rep % 2 == 0 else "B"   # A/A halves: independent null weak sets
            W = set(rng.sample(V, m))
            nonW = [v for v in V if v not in W]
            for _ in range(starts):
                tot[arm][0] += walk_cost(adj, W, rng.choice(nonW), rng, cap); tot[arm][1] += 1
            d = bfs_dist(adj, W)
            for v in nonW: tot["inf"][0] += d.get(v, cap); tot["inf"][1] += 1
        # graph-aligned weak set: BFS ball of size m around a random vertex (descriptive)
        for rep in range(max(1, reps//5)):
            c = rng.choice(V); d0 = bfs_dist(adj, [c])
            W = set(sorted(V, key=lambda v: (d0.get(v, 10**9), v))[:m])
            nonW = [v for v in V if v not in W]
            for _ in range(starts):
                tot["ball"][0] += walk_cost(adj, W, rng.choice(nonW), rng, cap); tot["ball"][1] += 1
        out["per_prime"].append(row)
    mean = {k: v[0]/v[1] for k, v in tot.items()}
    inv = 1/w
    r = {k: mean[k]/inv for k in mean}
    pub = (tot["A"][0]+tot["B"][0])/(tot["A"][1]+tot["B"][1])/inv
    aa = abs(mean["A"]-mean["B"])/((mean["A"]+mean["B"])/2)
    out.update({"mean_steps": mean, "ratio_to_1_over_w": r, "public_ratio_pooled": pub,
                "aa_relative_diff": aa, "positive_control_informed_ratio": r["inf"],
                "negative_control": "independent uniformly random weak sets (public walk, expected ratio ~1)",
                "thresholds": {"consistent": "0.85<=pub<=1.15 and aa<=0.08 and informed<0.5",
                               "falsified": "pub<0.85 or pub>1.15 with controls valid"}})
    controls = aa <= 0.08 and r["inf"] < 0.5
    if smoke: v = "inconclusive"
    elif not controls: v = "inconclusive"
    elif 0.85 <= pub <= 1.15: v = "consistent_with_prediction"
    else: v = "falsified"
    out["verdict"] = v
    print(json.dumps(out))

main()
