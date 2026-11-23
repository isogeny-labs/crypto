#!/usr/bin/env python3
"""Preregistered toy: op-count of batched-affine rho step (Montgomery trick). Stdlib only."""
import sys, json, random
SMOKE = "--smoke" in sys.argv
cnt = {"M": 0, "S": 0, "I": 0}

def inv(a, p):
    cnt["I"] += 1
    return pow(a, -1, p)

def find_curve(p, rng):
    # y^2=x^3+ax+b with prime group order; count by Legendre sum
    sq = [0]*p
    for y in range(p): sq[y*y % p] += 1
    for _ in range(400):
        a, b = rng.randrange(p), rng.randrange(1, p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        n = 1 + sum(sq[(x**3 + a*x + b) % p] for x in range(p))
        if n > 7 and all(n % q for q in range(2, int(n**.5)+1)):
            return a, b, n
    return None

def sqrtmod(c, p):
    for y in range(p):
        if y*y % p == c: return y

def rand_pt(a, b, p, rng):
    while True:
        x = rng.randrange(p); c = (x**3 + a*x + b) % p
        if pow(c, (p-1)//2, p) == 1 and c:
            return (x, sqrtmod(c, p))

def add_naive(P, Q, a, p):  # reference, uncounted
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0] and (P[1]+Q[1]) % p == 0: return None
    if P == Q: l = (3*P[0]**2 + a)*pow(2*P[1], -1, p) % p
    else: l = (Q[1]-P[1])*pow(Q[0]-P[0], -1, p) % p
    x = (l*l - P[0] - Q[0]) % p
    return (x, (l*(P[0]-x) - P[1]) % p)

def mul_naive(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add_naive(R, P, a, p)
        P = add_naive(P, P, a, p); k >>= 1
    return R

def batch_inv(vals, p):
    n = len(vals); cnt["M"] += 3*(n-1)
    if n == 1: return [inv(vals[0], p)]
    pre = [vals[0]]
    for v in vals[1:]: pre.append(pre[-1]*v % p)
    t = inv(pre[-1], p); out = [0]*n
    for i in range(n-1, 0, -1):
        out[i] = t*pre[i-1] % p; t = t*vals[i] % p
    out[0] = t
    return out

def rho(p, a, b, N, G, H, W, seed, dpbits):
    """r-adding walk, W walks in lockstep, one shared batch inversion per round."""
    rng = random.Random(seed); R = 16
    tab = []
    for _ in range(R):
        u, v = rng.randrange(1, N), rng.randrange(1, N)
        tab.append((add_naive(mul_naive(u, G, a, p), mul_naive(v, H, a, p), a, p), u, v))
    walks = []
    for _ in range(W):
        u, v = rng.randrange(N), rng.randrange(N)
        Pt = add_naive(mul_naive(u, G, a, p), mul_naive(v, H, a, p), a, p)
        walks.append([Pt, u, v])
    mask = (1 << dpbits) - 1; dps = {}; steps = 0; fallback = 0
    while steps < 40*N**.5*W + 1000:
        dens = []; idx = []
        for i, w in enumerate(walks):
            if w[0] is None: continue
            j = w[0][0] % R
            dx = (tab[j][0][0] - w[0][0]) % p
            if dx == 0: fallback += 1; dens.append(1); idx.append(i); continue
            dens.append(dx); idx.append(i)
        invs = batch_inv(dens, p) if dens else []
        for k, i in enumerate(idx):
            w = walks[i]; j = w[0][0] % R; Q, u, v = tab[j]
            if dens[k] == 1 and (Q[0] - w[0][0]) % p == 0:  # degenerate: uncounted exact fallback
                w[0] = add_naive(w[0], Q, a, p)
            else:
                l = (Q[1]-w[0][1])*invs[k] % p; cnt["M"] += 1
                x = (l*l - w[0][0] - Q[0]) % p; cnt["S"] += 1
                y = (l*(w[0][0]-x) - w[0][1]) % p; cnt["M"] += 1
                w[0] = (x, y)
            w[1] = (w[1]+u) % N; w[2] = (w[2]+v) % N; steps += 1
            if w[0] is None: w[1] = w[2] = 0; w[0] = None; continue
            if w[0][0] & mask == 0:
                key = w[0]
                if key in dps and dps[key][1:] != (w[1], w[2]):
                    u0, v0 = dps[key][1:]
                    if (v0 - w[2]) % N:
                        d = (w[1]-u0) * pow(v0-w[2], -1, N) % N
                        if mul_naive(d, G, a, p) == H: return d, steps, fallback
                dps[key] = (key, w[1], w[2])
        walks = [w for w in walks if w[0] is not None] or walks
    return None, steps, fallback

def trial(p, W, seed, dpbits=3):
    rng = random.Random(seed); c = find_curve(p, rng)
    if not c: return None
    a, b, N = c; G = rand_pt(a, b, p, rng); x = rng.randrange(1, N); H = mul_naive(x, G, a, p)
    for k in cnt: cnt[k] = 0
    d, steps, fb = rho(p, a, b, N, G, H, W, seed*7+W, dpbits)
    return dict(N=N, W=W, solved=d == x, steps=steps, fallback=fb, M=cnt["M"], S=cnt["S"], I=cnt["I"])

def main():
    primes = [1009, 1013] if SMOKE else [16411, 32771, 65537, 131101]  # |E| ~ 2^10..2^17
    Ws = [1, 16] if SMOKE else [1, 4, 16, 64, 256]
    seeds = [11, 12] if SMOKE else [101, 102, 103, 104, 105, 106]
    rows = []
    for p in primes:
        for W in Ws:
            for s in seeds:
                r = trial(p, W, s)
                if r: r["p"] = p; r["seed"] = s; rows.append(r)
    ok = [r for r in rows if r["solved"]]
    # per-step counted cost; formula: 2M+1S + 3(W-1)/W M + I/W  (round-level; degenerate fallbacks excluded by slack)
    summary = {}; max_dev = 0.0; below = 0
    for W in Ws:
        rr = [r for r in ok if r["W"] == W]
        if not rr: continue
        ms = sum(r["M"] for r in rr)/sum(r["steps"] for r in rr)
        ss = sum(r["S"] for r in rr)/sum(r["steps"] for r in rr)
        is_ = sum(r["I"] for r in rr)/sum(r["steps"] for r in rr)
        fm = 2 + 3*(W-1)/W
        dev = abs(ms - fm)/fm
        max_dev = max(max_dev, dev)
        # cost in M-equivalents (S=M) at I=20,50,100; jacobian mixed add = 11 M-eq (8M+3S)
        costs = {str(c): ms+ss+c*is_ for c in (20, 50, 100)}
        if any(v < 5.0 + 1.0 for v in [ms+ss]): below += 1  # inversion-free part below 6 M-eq: impossible
        summary[W] = dict(M_per_step=ms, S_per_step=ss, I_per_step=is_, formula_M=fm, rel_dev=dev, cost_M_eq=costs,
                          beats_jacobian_mixed_at_I={c: costs[str(c)] < 11 for c in (20, 50, 100)})
    sol_rate = len(ok)/max(1, len(rows))
    steps_by_W = {W: sum(r["steps"] for r in ok if r["W"] == W)/max(1, sum(1 for r in ok if r["W"] == W)) for W in Ws}
    ratio_aa = None
    if 1 in steps_by_W and Ws[-1] in steps_by_W and steps_by_W[1]:
        ratio_aa = steps_by_W[Ws[-1]]/steps_by_W[1]
    w1 = summary.get(1); wl = summary.get(Ws[-1])
    pos_ctrl = bool(w1 and w1["I_per_step"] > 0.99 and wl and wl["I_per_step"] < 0.1)  # trick works at all
    neg_ctrl = bool(w1 and not w1["beats_jacobian_mixed_at_I"][50])  # unbatched at I=50 must lose
    cond = max_dev < 0.02 and below == 0 and sol_rate > 0.9 and pos_ctrl and neg_ctrl
    verdict = "consistent_with_prediction" if cond else ("inconclusive" if sol_rate <= 0.9 or not summary else "falsified")
    print(json.dumps(dict(lead="LEAD-RHO-batched-inversion-shared-pm", smoke=SMOKE, primes=primes, Ws=Ws, seeds=seeds,
        n_rows=len(rows), solve_rate=sol_rate, summary=summary, max_rel_dev_M=max_dev, steps_by_W=steps_by_W,
        walk_count_steps_ratio_Wmax_over_1=ratio_aa, positive_control_ok=pos_ctrl, negative_control_ok=neg_ctrl,
        threshold=dict(rel_dev=0.02, min_inversion_free_cost_M_eq=6.0, solve_rate=0.9), verdict=verdict)))
main()
