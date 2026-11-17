#!/usr/bin/env python3
"""Toy Cheon aux-input reproduction (generic group Z_r, G=1). stdlib only."""
import sys, json, math, random

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13):
        if n % p == 0: return n == p
    i = 17
    while i*i <= n:
        if n % i == 0: return False
        i += 2
    return True

def factor(n):
    f, p = [], 2
    while p*p <= n:
        if n % p == 0:
            f.append(p)
            while n % p == 0: n //= p
        p += 1
    if n > 1: f.append(n)
    return f

def prime_with(bits, d, rng):
    """prime r near 2^bits with r = 1 mod d"""
    k = (1 << bits) // d + rng.randrange(1, 64)
    while True:
        r = k*d + 1
        if is_prime(r): return r
        k += 1

def prim_root(r):
    fs = factor(r-1)
    g = 2
    while any(pow(g, (r-1)//q, r) == 1 for q in fs): g += 1
    return g

def bsgs(n, step, target, cnt, gen_scalar_cost):
    """find x in [0,n) with step^x * 1 == target-in-orbit.
    Elements are ints mod r; 'step' is a scalar (known multiplier) applied
    to the group element; here orbit element x -> start*step^x. Returns
    (x, mults, entries). Caller passes closure pieces via lists."""
    raise NotImplementedError

def solve_orbit(r, n, scal, start, target, ctr):
    """find x in [0,n): start*scal^x = target (mod r), BSGS, count scalar mults."""
    m = math.isqrt(n-1) + 1
    tab = {}
    cur = start
    for j in range(m):
        tab.setdefault(cur, j)
        cur = cur*scal % r; ctr[0] += 1
    ctr[2] += len(tab)
    giant = pow(scal, -m, r)
    cur = target
    for i in range(m+1):
        j = tab.get(cur)
        if j is not None and (i*m + j) < n:
            return i*m + j
        cur = cur*giant % r; ctr[0] += 1
    return None

def cheon(r, d, g, alpha):
    ctr = [0, 0, 0]  # scalar mults, group adds, table entries
    dp = (r-1)//d
    h = pow(g, d, r)
    x0 = solve_orbit(r, dp, h, 1, pow(alpha, d, r), ctr)  # aux input a^d G
    if x0 is None: return None, ctr
    ctr[0] += 1
    R = alpha * pow(g, -x0, r) % r      # = zeta^t G
    z = pow(g, dp, r)
    t = solve_orbit(r, d, z, 1, R, ctr)
    if t is None: return None, ctr
    a = pow(g, x0 + dp*t, r)
    return (a if a == alpha else None), ctr

def plain_bsgs(r, alpha):
    """baseline: DLP of aG with G, only group additions (no aux)."""
    m = math.isqrt(r-1)+1
    tab = {}
    cur = 0
    for j in range(m):
        tab.setdefault(cur, j); cur = (cur+1) % r
    ops = m
    giant = (-m) % r; cur = alpha
    for i in range(m+1):
        j = tab.get(cur)
        if j is not None: return i*m+j, ops, len(tab)
        cur = (cur+giant) % r; ops += 1
    return None, ops, len(tab)

def mean(v): return sum(v)/len(v)

def cell(bits, d, trials, seed, ctrl_wrong=False):
    rng = random.Random(seed)
    r = prime_with(bits, d, rng)
    g = prim_root(r)
    dd = d + 1 if ctrl_wrong else d
    if ctrl_wrong:  # supply d' not dividing r-1 (negative control)
        while (r-1) % dd == 0: dd += 1
    raw, base, ok, ent = [], [], 0, []
    for _ in range(trials):
        alpha = rng.randrange(2, r)
        if ctrl_wrong:
            # algorithm run with a wrong divisor: stage structure invalid
            dp = (r-1)//dd
            ctr = [0,0,0]
            h = pow(g, dd, r)
            x0 = solve_orbit(r, dp, h, 1, pow(alpha, dd, r), ctr)
            res = None
            if x0 is not None:
                z = pow(g, dp, r); R = alpha*pow(g,-x0,r) % r
                t = solve_orbit(r, dd, z, 1, R, ctr)
                if t is not None:
                    a = pow(g, x0+dp*t, r); res = a if a == alpha else None
        else:
            res, ctr = cheon(r, d, g, alpha)
        ok += res is not None
        raw.append(ctr[0]); ent.append(ctr[2])
        x, ops, _ = plain_bsgs(r, alpha); base.append(ops)
    dp = (r-1)//d
    pred = 1.5*(math.sqrt(dp)+math.sqrt(d))
    pbase = 1.5*math.sqrt(r)
    c = 1.5*math.log2(r)  # scalar mult charged in group ops
    return dict(bits=bits, d=d, r=r, trials=trials, success=ok/trials,
        mean_cheon_mults=mean(raw), mean_plain_ops=mean(base),
        pred_cheon=pred, pred_plain=pbase, mean_table_entries=mean(ent),
        ratio_meas_over_pred=mean(raw)/pred,
        raw_gain=mean(base)/mean(raw),
        charged_gain=mean(base)/(mean(raw)*c),
        theory_gain=pbase/pred)

def main():
    smoke = "--smoke" in sys.argv
    trials = 5 if smoke else 120
    grid = [(12, 8)] if smoke else [(16,16),(16,64),(20,16),(20,64),(20,256),(22,64),(22,256),(22,1024)]
    seed = 20261110
    cells = [cell(b, d, trials, seed + 7*i) for i, (b, d) in enumerate(grid)]
    aa = [cell(b, d, trials, seed + 1000 + 7*i) for i, (b, d) in enumerate(grid[:3] if not smoke else grid)]
    neg = [cell(b, d, trials, seed + 2000, ctrl_wrong=True) for (b, d) in (grid[:2] if not smoke else grid)]
    pos_ok = all(c["success"] == 1.0 for c in cells)
    ratio_ok = all(0.8 <= c["ratio_meas_over_pred"] <= 1.25 for c in cells)
    gain_ok = all(c["raw_gain"] >= 0.7*c["theory_gain"] for c in cells)
    chg_lt1 = [c["charged_gain"] < 1 for c in cells]
    aa_dev = [abs(a["mean_cheon_mults"]/c["mean_cheon_mults"]-1) for a, c in zip(aa, cells)]
    aa_ok = all(x <= 0.10 for x in aa_dev)
    neg_ok = all(n["success"] < 0.05 for n in neg)
    if not (pos_ok and aa_ok and neg_ok):
        verdict = "inconclusive"
    elif ratio_ok and gain_ok:
        verdict = "consistent_with_prediction"
    else:
        verdict = "falsified"
    print(json.dumps(dict(lead="LEAD-RHO-cheon-toy-reproduction", smoke=smoke, seed=seed,
        cells=cells, aa_cells=aa, neg_control_cells=neg, aa_rel_dev=aa_dev,
        checks=dict(positive_all_recovered=pos_ok, ratio_in_0p8_1p25=ratio_ok,
            raw_gain_ge_0p7_theory=gain_ok, charged_gain_below_1_per_cell=chg_lt1,
            aa_within_10pct=aa_ok, neg_control_fails=neg_ok),
        verdict=verdict)))
main()
