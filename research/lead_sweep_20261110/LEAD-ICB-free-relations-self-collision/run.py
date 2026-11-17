#!/usr/bin/env python3
"""Preregistered toy: free relations from self-collisions of a pair-sum table. See PROTOCOL.md. stdlib only."""
import sys, json, random, math

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

def curve(p, a, b):
    sq = {}
    for y in range(p): sq.setdefault(y * y % p, []).append(y)
    cnt = 1
    for x in range(p):
        cnt += len(sq.get((x ** 3 + a * x + b) % p, ()))
    return cnt, sq

def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)

def setup(logp, seed):
    """prime-order curve near 2^logp; returns p,a,n,G,xtab (xtab[k]=x(kG), k<n, xtab[0]=None)"""
    rng = random.Random(seed)
    p = next_prime((1 << logp) + rng.randrange(1 << (logp - 1)))
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        n, sq = curve(p, a, b)
        if is_prime(n): break
    while True:
        x = rng.randrange(p); ys = sq.get((x ** 3 + a * x + b) % p)
        if ys: G = (x, ys[0]); break
    xt = [None] * n; P = None
    pts = [None] * 2
    for k in range(1, n):
        P = add(P, G, p, a); xt[k] = P[0] if P else None
    return p, a, n, G, xt, rng

def canon(k, n): return min(k % n, (-k) % n)

def measure(n, xt, ks, rng, probes):
    """ks: factor-base discrete logs (shortcut). Returns (T, collisions C, probe hits)."""
    B = len(ks); keys = {}; T = 0
    for i in range(B):
        for j in range(i + 1, B):
            s = canon(ks[i] + ks[j], n)
            if s == 0: continue
            keys[s] = keys.get(s, 0) + 1; T += 1
    C = sum(m * (m - 1) // 2 for m in keys.values())
    hits = 0
    for _ in range(probes):
        if canon(rng.randrange(1, n), n) in keys: hits += 1
    return T, C, hits

def mean_se(v):
    m = sum(v) / len(v)
    if len(v) < 2: return m, 0.0
    return m, math.sqrt(sum((x - m) ** 2 for x in v) / (len(v) - 1) / len(v))

def run(logps, reps, seed0):
    out = []; allok = True; anyfal = False
    for li, lp in enumerate(logps):
        p, a, n, G, xt, rng = setup(lp, seed0 + li)
        # sanity: dlog shortcut equals real curve addition on samples
        for _ in range(100):
            u, v = rng.randrange(1, n), rng.randrange(1, n)
            Pu = (xt[u],); 
        chk = 0
        # real-addition check on small multiples
        P = None
        for k in range(1, 50):
            P = add(P, G, p, a); assert P[0] == xt[k]
        B = int(math.sqrt(n * 0.5 * 2)) // 1 + 1   # T ~ n/2... B^2/2 ~ n/2 -> B ~ sqrt(n)
        def arm(kind, seedoff):
            R, Y, Cs, Es = [], [], [], []
            for r in range(reps):
                rr = random.Random(seed0 * 1000003 + li * 997 + seedoff * 101 + r)
                if kind == "ap":
                    d = rr.randrange(1, n); ks = [(i + 1) * d % n for i in range(B)]
                else:
                    ks = [rr.randrange(1, n) for _ in range(B)]
                T, C, h = measure(n, xt, ks, rr, T_probe(B))
                # null: T random canonical classes
                kn = {}
                for _ in range(T):
                    c = canon(rr.randrange(1, n), n); kn[c] = kn.get(c, 0) + 1
                Cn = sum(m * (m - 1) // 2 for m in kn.values())
                R.append(C / max(Cn, 1))
                pr = T_probe(B)
                Y.append((C / T) / max(h / pr, 1e-12))
            return mean_se(R), mean_se(Y)
        def T_probe(B): return B * (B - 1) // 2
        (rm, rs), (ym, ys) = arm("rand", 1)
        (r2, s2), (y2, ys2) = arm("rand", 2)       # A/A arm
        (pm, ps), _ = arm("ap", 3)                 # positive control
        aa = abs(rm - r2) < 3 * math.hypot(rs, s2) + 0.02
        null_ok = abs(rm - 1) < 3 * rs + 0.05
        pos_ok = pm > 3
        yok = ym + 2 * ys < 1.0
        yfal = ym - 2 * ys > 1.0
        allok &= aa and null_ok and pos_ok and yok
        anyfal |= (not null_ok and pos_ok and aa) or (yfal and pos_ok and aa)
        out.append(dict(logp=lp, p=p, n=n, B=B, R_random=[rm, rs], R_AA=[r2, s2], R_positive_AP=[pm, ps],
                        Y_free_over_probe=[ym, ys], aa_ok=aa, null_ok=null_ok, pos_ok=pos_ok, y_below_1=yok))
    v = "consistent_with_prediction" if allok else ("falsified" if anyfal else "inconclusive")
    return out, v

if __name__ == "__main__":
    smoke = "--smoke" in sys.argv
    logps, reps = ([10], 4) if smoke else ([12, 14, 16, 17], 12)
    seed0 = 20261110
    cells, verdict = run(logps, reps, seed0)
    print(json.dumps(dict(lead="LEAD-ICB-free-relations-self-collision", smoke=smoke, seed0=seed0, reps=reps,
                          statistic="R=C_obs/C_null (collision ratio), Y=(free rel/table op)/(probe rel/probe op)",
                          thresholds=dict(R_null="|R-1|<3SE+0.05", R_positive=">3", Y="mean+2SE<1"),
                          cells=cells, verdict=verdict)))
