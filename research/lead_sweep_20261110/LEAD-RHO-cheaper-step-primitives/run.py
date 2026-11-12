#!/usr/bin/env python3
"""LEAD-RHO-cheaper-step-primitives toy: counted field-op cost of batched-affine vs
tag-tracing (inversion-free Jacobian) r-adding walks; plus a function-of-point check.
stdlib only, deterministic."""
import json, sys, random

SMOKE = "--smoke" in sys.argv
P_LIST = [2053] if SMOKE else [2053, 4099, 8209, 16411]
R = 16                      # r-adding table size
WS = [8, 16, 64, 256, 1024]
IM = [10, 20, 40]           # inversion cost in M (band)
SM = [1.0, 0.8]             # S cost in M
SEED = 20261110

class C:
    def __init__(s): s.M = s.S = s.I = 0

def count_order(p, a, b):
    sq = {}
    for y in range(p): sq[y*y % p] = sq.get(y*y % p, 0) + 1
    return 1 + sum(sq.get((x*x*x + a*x + b) % p, 0) for x in range(p))

def is_prime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def affine_add(p, A, B, c, inv=None):
    (x1, y1), (x2, y2) = A, B
    if inv is None:
        c.I += 1; inv = pow((x2-x1) % p, -1, p)
    l = (y2-y1)*inv % p; c.M += 1
    x3 = (l*l - x1 - x2) % p; c.S += 1
    y3 = (l*(x1-x3) - y1) % p; c.M += 1
    return (x3, y3)

def affine_dbl(p, a, A):
    x, y = A
    l = (3*x*x + a) * pow(2*y, -1, p) % p
    x3 = (l*l - 2*x) % p
    return (x3, (l*(x-x3) - y) % p)

def scal(p, a, k, A):
    R_ = None
    while k:
        if k & 1:
            R_ = A if R_ is None else (None if R_[0] == A[0] and (R_[1]+A[1]) % p == 0 else
                  (affine_dbl(p, a, R_) if R_ == A else affine_add(p, R_, A, C())))
        A = affine_dbl(p, a, A) if A[1] else A
        k >>= 1
    return R_

def jac_madd(p, X1, Y1, Z1, x2, y2, c):
    # mixed Jacobian addition: 8M + 3S (generic, counted)
    Z1Z1 = Z1*Z1 % p; U2 = x2*Z1Z1 % p; S2 = y2*Z1*Z1Z1 % p
    H = (U2-X1) % p; HH = H*H % p; I4 = 4*HH % p; J = H*I4 % p
    r = 2*(S2-Y1) % p; V = X1*I4 % p
    X3 = (r*r - J - 2*V) % p; Y3 = (r*(V-X3) - 2*Y1*J) % p; Z3 = ((Z1+H)**2 - Z1Z1 - HH) % p
    c.M += 8; c.S += 3
    return X3, Y3, Z3

def cost(c, im, sm): return c.M + sm*c.S + im*c.I

def make_curve(p):
    rng = random.Random(SEED + p)
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        n = count_order(p, a, b)
        # largest prime factor cofactor-1 not required; need a point of order >= n/4
        return a, b, n

def rand_pt(p, a, b, rng):
    while True:
        x = rng.randrange(p); t = (x**3 + a*x + b) % p
        if pow(t, (p-1)//2, p) == 1:
            y = pow(t, (p+1)//4, p) if p % 4 == 3 else None
            if y is not None and y*y % p == t: return (x, y)
            for y in range(p):
                if y*y % p == t: return (x, y)

def measure_steps(p, a, b, W, steps, rng):
    """W parallel walks; count ops for `steps` steps of plain batched-affine and of tag-tracing."""
    table = [rand_pt(p, a, b, rng) for _ in range(R)]
    pts = [rand_pt(p, a, b, rng) for _ in range(W)]
    ca = C(); ct = C()
    # plain batched affine: Montgomery trick, 3 M per element + 1 inversion per batch
    cur = list(pts)
    done = 0
    while done < steps:
        idx = [q[0] % R for q in cur]
        dx = [(table[i][0]-q[0]) % p for i, q in zip(idx, cur)]
        if any(d == 0 for d in dx):
            cur = [rand_pt(p, a, b, rng) for _ in range(W)]; continue
        done += 1
        pref = [1]*W; acc = 1
        for j in range(W): pref[j] = acc; acc = acc*dx[j] % p
        ca.M += 3*W; ca.I += 1
        inv = pow(acc, -1, p); out = [None]*W
        for j in range(W-1, -1, -1):
            out[j] = pref[j]*inv % p; inv = inv*dx[j] % p
        cur = [affine_add(p, q, table[i], ca, inv=o) for q, i, o in zip(cur, idx, out)]
    # tag tracing: Jacobian mixed add, no inversion; step index must come from a tag
    # (here: a counter-free tag = low bits of X as an *unnormalised* label)
    cj = [(q[0], q[1], 1) for q in pts]
    for _ in range(steps):
        nxt = []
        for X, Y, Z in cj:
            i = X % R                      # representation-dependent tag (cost-free)
            if (table[i][0]*Z*Z - X) % p == 0: nxt.append((X, Y, Z)); continue
            nxt.append(jac_madd(p, X, Y, Z, table[i][0], table[i][1], ct))
        cj = nxt
    return ca, ct, steps*W

def rep_dependence(p, a, b, rng, trials=300):
    """Fraction of random points whose tag X%R changes when the SAME point is rescaled in Jacobian
    coordinates (tag not a function of the point => walks of equal points need not merge)."""
    diff = 0
    for _ in range(trials):
        x, y = rand_pt(p, a, b, rng); l = rng.randrange(2, p)
        if x % R != (x*l*l % p) % R: diff += 1
    return diff / trials

def rho_merge_test(p, a, b, n, rng, normalised, walks=150):
    """Two walks from the same start point must merge (same orbit). Count merges within budget
    when the step index is a function of the normalised point vs of the Jacobian X."""
    table = [rand_pt(p, a, b, rng) for _ in range(R)]
    merged = 0; budget = 20
    for _ in range(walks):
        s = rand_pt(p, a, b, rng); l = rng.randrange(2, p)
        seen = []; A = s
        for _ in range(budget):
            seen.append(A); i = A[0] % R
            if A[0] == table[i][0]: break
            A = affine_add(p, A, table[i], C())
        # second walk: Jacobian with random scaling, index from either normalised x or raw X
        X, Y, Z = s[0]*l*l % p, s[1]*l**3 % p, l; ok = True
        for k in range(budget):
            zi = pow(Z, -1, p); x = X*zi*zi % p; y = Y*zi**3 % p
            if k < len(seen) and (x, y) != seen[k]: ok = False; break
            i = (x if normalised else X) % R
            if x == table[i][0]: break
            X, Y, Z = jac_madd(p, X, Y, Z, table[i][0], table[i][1], C())
        merged += ok
    return merged / walks

def main():
    rng = random.Random(SEED)
    rows = []; worst = None
    for p in P_LIST:
        while not is_prime(p) or p % 4 != 3: p += 1
        a, b, n = make_curve(p)
        for W in ([16] if SMOKE else WS):
            ca, ct, st = measure_steps(p, a, b, W, 2 if SMOKE else 6, rng)
            for im in IM:
                for sm in SM:
                    ratio = (cost(ct, im, sm)/st) / (cost(ca, im, sm)/st)
                    rows.append({"p": p, "n": n, "W": W, "I_in_M": im, "S_in_M": sm,
                                 "plain_per_step": round(cost(ca, im, sm)/st, 3),
                                 "tag_per_step": round(cost(ct, im, sm)/st, 3),
                                 "ratio_tag_over_plain": round(ratio, 4)})
    sel = [r for r in rows if r["W"] >= 16]
    min_ratio = min(r["ratio_tag_over_plain"] for r in sel)
    pr = P_LIST[0]; pr = next(q for q in range(pr, pr+500) if is_prime(q) and q % 4 == 3)
    a, b, n = make_curve(pr)
    repdep = rep_dependence(pr, a, b, rng)
    walks = 20 if SMOKE else 150
    m_norm = rho_merge_test(pr, a, b, n, rng, True, walks)      # positive control (index = function of point)
    m_raw = rho_merge_test(pr, a, b, n, rng, False, walks)      # tag = raw Jacobian X (negative control)
    # A/A: same instrument, plain vs plain with different seed
    rng2 = random.Random(SEED+1); ca1, _, s1 = measure_steps(pr, a, b, 16, 3, rng2)
    ca2, _, s2 = measure_steps(pr, a, b, 16, 3, random.Random(SEED+2))
    aa = (cost(ca1, 20, 1.0)/s1)/(cost(ca2, 20, 1.0)/s2)
    controls_ok = m_norm >= 0.9 and m_raw <= 0.5*m_norm and repdep > 0.8 and abs(aa-1) < 0.01
    if SMOKE and not controls_ok: verdict = "inconclusive"
    elif not controls_ok: verdict = "inconclusive"
    elif min_ratio < 0.95: verdict = "falsified"
    elif min_ratio >= 1.0: verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-RHO-cheaper-step-primitives", "smoke": SMOKE, "seed": SEED,
        "params": {"R": R, "W": WS, "I_in_M": IM, "S_in_M": SM, "primes": P_LIST},
        "statistic": "min over W>=16, I in 10..40 M, S in {1,0.8} M of tag/plain counted cost per step",
        "min_ratio": min_ratio, "thresholds": {"falsified_below": 0.95, "consistent_at_or_above": 1.0},
        "controls": {"positive_merge_rate_normalised_index": m_norm,
                     "negative_merge_rate_raw_X_tag": m_raw,
                     "tag_changes_under_rescaling_fraction": repdep, "A_A_ratio": aa,
                     "controls_ok": controls_ok},
        "rows": rows, "verdict": verdict}))
main()
