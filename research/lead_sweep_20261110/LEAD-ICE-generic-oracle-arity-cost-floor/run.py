#!/usr/bin/env python3
"""Generic decomposition-oracle arity cost floor: counted-group-op model vs Pollard rho.
stdlib only, deterministic. --smoke = tiny sizes."""
import sys, json, math, random

LG = math.lgamma
def lC(B, k):
    if k > B: return -1e300
    return (LG(B+1)-LG(k+1)-LG(B-k+1))
def C(B, k): return math.exp(lC(B, k))

def cost(N, m, kind, B, LA=True):
    """expected counted group ops for a full IC solve; kind in ENUM, MITM, FREE."""
    k = m//2
    p = 1-math.exp(-C(B, m)/N)
    if p <= 0: return float('inf')
    targets = (B+1)/p
    if kind == 'ENUM': build, look = 0.0, C(B, m-1)
    elif kind == 'MITM': build, look = C(B, k) if k > 1 else 0.0, C(B, m-k)
    else: build, look = 0.0, 0.0
    tot = B + build + targets*(1+look)
    if LA: tot += m*B*B
    return tot

def best(N, m, kind, LA=True, bmax=1<<22):
    bs, B = [], 2
    while B <= min(N, bmax):
        bs.append(B); B = max(B+1, int(B*1.12))
    return min((cost(N, m, kind, b, LA), b) for b in bs)

def rho_once(N, g, rng):
    """Pollard rho (Brent-free, Floyd) on Z_N, 16-way adding walk; counts adds."""
    h = rng.randrange(1, N)
    T = h*g % N
    r = 16
    ca = [rng.randrange(N) for _ in range(r)]; cb = [rng.randrange(N) for _ in range(r)]
    st = [(ca[i]*g+cb[i]*T) % N for i in range(r)]
    def step(x, a, b):
        j = x % r
        return (x+st[j]) % N, (a+ca[j]) % N, (b+cb[j]) % N
    x1 = (3*g+5*T) % N; a1, b1 = 3, 5
    x2, a2, b2 = x1, a1, b1
    ops = 0
    while True:
        x1, a1, b1 = step(x1, a1, b1); x2, a2, b2 = step(x2, a2, b2)
        x2, a2, b2 = step(x2, a2, b2); ops += 3
        if x1 == x2:
            if (b1-b2) % N == 0: return ops, False
            return ops, True

def rho_mean(N, trials, seed):
    rng = random.Random(seed); tot = 0
    for _ in range(trials):
        o, _ = rho_once(N, 1, rng); tot += o
    return tot/trials

def prime_below(n):
    n -= 1
    while True:
        if all(n % q for q in range(2, int(n**0.5)+1)): return n
        n -= 1

def sim(N, m, kind, B, seed):
    """real counted simulation (no LA): returns ops to collect B+1 relations."""
    rng = random.Random(seed)
    fb = rng.sample(range(1, N), B)
    k = m//2
    from itertools import combinations
    ops = B; table = {}
    if kind == 'MITM':
        if k > 1:
            for c in combinations(range(B), k):
                ops += 1; table.setdefault(sum(fb[i] for i in c) % N, []).append(c)
        else:
            for i in range(B): table.setdefault(fb[i], []).append((i,))
        lk = m-k
    else:
        for i in range(B): table.setdefault(fb[i], []).append((i,))
        lk = m-1
    rel = 0
    while rel < B+1:
        t = rng.randrange(N); ops += 1; hit = False
        for c in combinations(range(B), lk):
            ops += 1
            r = (t-sum(fb[i] for i in c)) % N
            if r in table and not hit:
                for tc in table[r]:
                    if not set(tc) & set(c): hit = True; break
        rel += hit
    return ops

def main():
    smoke = '--smoke' in sys.argv
    SEED = 20261110
    ns = [12, 14] if smoke else [12, 14, 16, 18, 20, 22]
    ext = [24, 32] if smoke else [24, 32, 40, 56, 80, 128]
    arities = [2, 3] if smoke else [2, 3, 4, 5]
    ntr = 10 if smoke else 300
    out = {"lead": "LEAD-ICE-generic-oracle-arity-cost-floor", "seed": SEED, "smoke": smoke,
           "limits": "stdlib; counted group adds; LA=m*B^2; rho = real Floyd walk on Z_N"}
    # rho reference and A/A
    rho = {}; aa = {}
    for n in ns:
        N = prime_below(1 << n)
        a = rho_mean(N, ntr, SEED+n); b = rho_mean(N, ntr, SEED+1000+n)
        rho[n] = (a+b)/2; aa[n] = a/b
    # analytic rho constant check
    rho_an = {n: math.sqrt(math.pi*(1 << n)/2) for n in ns}
    rho_ratio_model = {n: rho[n]/rho_an[n] for n in ns}
    # rho for extension sizes: scale measured constant at largest measured n
    def rho_ref(n): return math.sqrt(math.pi*(2.0**n)/2)  # ideal birthday rho, favourable to IC; measured Floyd rho reported as sanity
    cells = []
    for kind in ['ENUM', 'MITM', 'FREE']:
        for m in arities:
            if kind == 'FREE' and m < 3: continue
            for n in ns+ext:
                N = (1 << n) if n > 30 else prime_below(1 << n)
                c, B = best(N, m, kind)
                cells.append({"kind": kind, "m": m, "n": n, "B": B, "log2cost": math.log2(c),
                              "ratio_to_rho": c/rho_ref(n), "toy": n in ns})
    # slopes vs n over ext range
    slopes = {}
    for kind in ['ENUM', 'MITM', 'FREE']:
        for m in arities:
            if kind == 'FREE' and m < 3: continue
            pts = [(x["n"], x["log2cost"]) for x in cells if x["kind"] == kind and x["m"] == m and x["n"] in ext[-2:]]
            (x0, y0), (x1, y1) = pts[0], pts[-1]
            slopes[f"{kind}_m{m}"] = (y1-y0)/(x1-x0)/1.0
    # positive control: FREE oracle, LA not charged, m=4 should beat rho at n=22
    pc = []
    for m in [4]:
        n = ns[-1]; N = prime_below(1 << n)
        c, B = best(N, m, 'FREE', LA=False); pc.append({"m": m, "n": n, "ratio_to_rho": c/rho_ref(n)})
    pc_ok = all(x["ratio_to_rho"] < 1 for x in pc)
    # model validation vs real counted simulation (no LA)
    vn = 10 if smoke else 12; N = prime_below(1 << vn); val = []
    for kind, m in [('MITM', 2), ('MITM', 3), ('ENUM', 3)]:
        c, B = best(N, m, kind, LA=False); B = min(B, 40 if smoke else 90)
        reps = 3 if smoke else 8
        meas = sum(sim(N, m, kind, B, SEED+r) for r in range(reps))/reps
        mod = cost(N, m, kind, B, LA=False)
        val.append({"kind": kind, "m": m, "B": B, "measured": meas, "model": mod, "ratio": meas/mod})
    val_ok = all(0.5 <= v["ratio"] <= 2.0 for v in val)
    aa_ok = all(0.8 <= v <= 1.25 for v in aa.values()) if not smoke else True
    # decision statistic: min ratio over ENUM/MITM cells (all m) and FREE m=3, toy sizes
    dec = [x for x in cells if x["toy"] and (x["kind"] != 'FREE' or x["m"] == 3)]
    minratio = min(x["ratio_to_rho"] for x in dec)
    sub = [x for x in dec if x["ratio_to_rho"] <= 1.0]
    if not (pc_ok and val_ok and aa_ok): verdict = "inconclusive"
    elif sub: verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    out.update({"statistic_min_ratio_to_rho_decision_cells": minratio, "n_sub_rho_cells": len(sub),
                "sub_rho_cells": sub[:10], "threshold": "ratio<=1 in any ENUM/MITM(m=2..5)/FREE(m=3) toy cell falsifies",
                "positive_control": pc, "positive_control_ok": pc_ok,
                "model_validation": val, "model_validation_ok": val_ok,
                "aa_rho_ratio": aa, "aa_ok": aa_ok, "rho_vs_analytic": rho_ratio_model,
                "slopes_log2cost_per_bit_n24_to_max": slopes,
                "cells": cells if not smoke else cells[:4], "verdict": verdict})
    print(json.dumps(out))
    return 0
if __name__ == "__main__": sys.exit(main())
