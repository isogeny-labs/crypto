#!/usr/bin/env python3
"""Toy closure-degree-vs-l study for Weil-descended Semaev S_3 over GF(2^n), m=2. stdlib only."""
import sys, json, random, itertools
POLY = {7: 0b10000011, 9: 0b1000010001, 11: 0b100000000101, 13: 0b10000000011011}

def gmul(a, b, n):
    r = 0; mod = POLY[n]
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> n & 1: a ^= mod
    return r

def pmul(p, q, n):
    r = {}
    for m1, c1 in p.items():
        for m2, c2 in q.items():
            m = m1 | m2; c = gmul(c1, c2, n)
            r[m] = r.get(m, 0) ^ c
    return {m: c for m, c in r.items() if c}

def padd(p, q):
    r = dict(p)
    for m, c in q.items():
        r[m] = r.get(m, 0) ^ c
        if not r[m]: del r[m]
    return r

def psq(p, n):  # Frobenius: linear on bits, multilinear reduction
    return {m: gmul(c, c, n) for m, c in p.items()}

def semaev_system(n, l, rng):
    """n Boolean quadratics in 2l vars: S3(x1,x2,xR)=0, x1,x2 in random l-dim subspace V."""
    while True:
        V = [rng.randrange(1, 1 << n) for _ in range(l)]
        span = {0}
        for v in V: span |= {s ^ v for s in span}
        if len(span) == 1 << l: break
    b = rng.randrange(1, 1 << n); xR = rng.randrange(1, 1 << n)
    X1 = {1 << i: V[i] for i in range(l)}
    X2 = {1 << (l + i): V[i] for i in range(l)}
    R = {0: xR}
    x12 = pmul(X1, X2, n)
    s = padd(padd(psq(x12, n), psq(pmul(X1, R, n), n)), psq(pmul(X2, R, n), n))
    s = padd(s, pmul(x12, R, n)); s = padd(s, {0: b})
    eqs = []
    for j in range(n):
        eqs.append({m for m, c in s.items() if c >> j & 1})
    return eqs, span, xR, b

def sat_instance(n, l, rng):
    """Same shape but xR chosen so a solution exists (positive control): solve for b."""
    eqs, span, xR, b = semaev_system(n, l, rng)
    return eqs

def has_solution(eqs, nv):
    for x in range(1 << nv):
        ok = True
        for e in eqs:
            v = 0
            for m in e:
                if m & x == m: v ^= 1
            if v: ok = False; break
        if ok: return True
    return False

def monos_upto(nv, d):
    out = []
    for k in range(d + 1):
        for c in itertools.combinations(range(nv), k):
            m = 0
            for i in c: m |= 1 << i
            out.append(m)
    return out

def refutes(eqs, nv, D):
    """Degree-D XL (multilinear): is 1 in the span of {mono * f : deg(mono*f) <= D}?"""
    mons = monos_upto(nv, D)
    idx = {m: i for i, m in enumerate(mons)}
    pc = lambda m: bin(m).count("1")
    deg = [max([pc(m) for m in e], default=0) for e in eqs]
    basis = {}  # pivot bit -> row
    one = 1 << idx[0]
    for e, de in zip(eqs, deg):
        for t in monos_upto(nv, max(0, D - de)):
            row = 0
            for m in e:
                row ^= 1 << idx[m | t]
            while row:
                p = row.bit_length() - 1
                if p in basis: row ^= basis[p]
                else: basis[p] = row; break
    # 1 in span iff reducing 'one' gives 0 (with pivots chosen by top bit)
    row = one
    while row:
        p = row.bit_length() - 1
        if p in basis: row ^= basis[p]
        else: return False
    return True

def closure_degree(eqs, nv, dmax):
    for D in range(2, dmax + 1):
        if refutes(eqs, nv, D): return D
    return None

def random_null(n, l, rng, template):
    """Null: n random quadratics, same per-equation monomial count as the Semaev system."""
    nv = 2 * l; allq = [m for m in monos_upto(nv, 2)]
    return [set(rng.sample(allq, min(len(e), len(allq)))) for e in template]

def run_arm(kind, l, draws, seed, dmax):
    n = 2 * l + 1; rng = random.Random(seed * 1000 + l); res = []
    for _ in range(draws):
        eqs, span, xR, b = semaev_system(n, l, rng)
        if kind == "null": eqs = random_null(n, l, rng, eqs)
        # restrict to unsatisfiable instances only where checkable (2l<=12 -> brute force 4096)
        if has_solution(eqs, 2 * l): continue
        res.append(closure_degree(eqs, 2 * l, dmax))
    return res

def summ(r, cap):
    v = [cap + 1 if x is None else x for x in r]
    return {"n": len(v), "mean": round(sum(v) / len(v), 3) if v else None, "max": max(v) if v else None, "raw": v}

def main():
    smoke = "--smoke" in sys.argv
    ls = [3] if smoke else [3, 4, 5, 6]
    draws = 3 if smoke else 24
    dmax = 4 if smoke else 6
    out = {"lead": "LEAD-ICE-descent-closure-degree-at-scale", "smoke": smoke, "ls": ls,
           "draws": draws, "dmax": dmax, "seeds": {"A": 11, "A_prime": 12, "null": 13, "pos": 14},
           "model": "Semaev S3 (char 2, assumed form) descended, m=2, n=2l+1, x1,x2 in random l-dim F2-subspace"}
    arms = {}
    for name, kind, seed in [("A", "ic", 11), ("A_prime", "ic", 12), ("null", "null", 13)]:
        arms[name] = {str(l): summ(run_arm(kind, l, draws, seed, dmax), dmax) for l in ls}
    # positive control: planted-solution (satisfiable) systems must NOT be refuted; detector check
    rng = random.Random(14); pos_ok = True; pos_n = 0
    for l in ls:
        for _ in range(2 if smoke else 6):
            n = 2 * l + 1
            for _ in range(200):
                eqs, *_ = semaev_system(n, l, rng)
                if has_solution(eqs, 2 * l): break
            else: continue
            pos_n += 1
            if closure_degree(eqs, 2 * l, dmax) is not None: pos_ok = False
    # positive control 2: overdetermined random quadratics must refute at small D
    rng = random.Random(15); l = ls[0]; nv = 2 * l
    allq = monos_upto(nv, 2); od = [set(rng.sample(allq, len(allq) // 2)) for _ in range(4 * nv)]
    od_deg = None if has_solution(od, nv) else closure_degree(od, nv, dmax)
    out.update(arms=arms, pos_control={"sat_never_refuted": pos_ok, "n": pos_n, "overdetermined_degree": od_deg})
    means = [arms["A"][str(l)]["mean"] for l in ls if arms["A"][str(l)]["mean"] is not None]
    nm = [arms["null"][str(l)]["mean"] for l in ls if arms["null"][str(l)]["mean"] is not None]
    aa = [abs((arms["A"][str(l)]["mean"] or 0) - (arms["A_prime"][str(l)]["mean"] or 0)) for l in ls]
    amax = max([arms["A"][str(l)]["max"] or 0 for l in ls])
    slope = (means[-1] - means[0]) / max(1, len(means) - 1) if len(means) > 1 else 0.0
    out["stat"] = {"max_closure_degree": amax, "slope_per_l": round(slope, 3),
                   "null_means": nm, "aa_max_mean_gap": round(max(aa), 3)}
    if not pos_ok or od_deg is None or len(means) < len(ls): v = "inconclusive"
    elif smoke: v = "inconclusive"
    elif max(aa) > 0.5: v = "inconclusive"
    elif amax <= 4 and slope <= 0.2: v = "consistent_with_prediction"
    elif amax >= 5 and slope > 0.2: v = "falsified"
    else: v = "inconclusive"
    out["verdict"] = v
    print(json.dumps(out))
main()
