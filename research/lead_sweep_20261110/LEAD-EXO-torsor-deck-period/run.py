#!/usr/bin/env python3
"""LEAD-EXO-torsor-deck-period toy: cheap selectors as sections of the cover G -> G/<T>. stdlib only."""
import sys, json, random, math

def primes_in(lo, hi):
    return [n for n in range(lo, hi) if n > 2 and all(n % d for d in range(2, int(n**.5) + 1))]

class Curve:
    def __init__(s, p, a, b):
        s.p, s.a, s.b = p, a, b
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            l = (3 * x1 * x1 + s.a) * pow(2 * y1, -1, p) % p
        else:
            l = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (l * l - x1 - x2) % p
        return (x3, (l * (x1 - x3) - y1) % p)

def leg(v, p):
    v %= p
    if v == 0: return 0
    return 1 if pow(v, (p - 1) // 2, p) == 1 else -1

def make_curves(p, rng, want, tries=4000):
    """want: dict residue-class-of-N-mod-4 -> count. Curves with exactly one root of the cubic."""
    out = {2: [], 0: []}
    sq = {}
    for y in range(p):
        sq.setdefault(y * y % p, []).append(y)
    for _ in range(tries):
        if all(len(out[k]) >= want[k] for k in want): break
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a**3 + 27 * b * b) % p == 0: continue
        roots = [x for x in range(p) if (x**3 + a * x + b) % p == 0]
        if len(roots) != 1: continue
        N = 1 + sum(len(sq.get((x**3 + a * x + b) % p, [])) for x in range(p))
        k = 2 if N % 4 == 2 else (0 if N % 4 == 0 else None)
        if k is None or len(out[k]) >= want[k]: continue
        pts = [(x, y) for x in range(p) for y in sq.get((x**3 + a * x + b) % p, []) if y]
        out[k].append((Curve(p, a, b), roots[0], N, pts))
    return out

def oracle_dl(C, N, pts, rng):
    """dl table w.r.t. a generator of order N, or None if group not cyclic."""
    for _ in range(40):
        g = rng.choice(pts)
        Q, ok = g, True
        for q in {2} | {r for r in range(3, 200, 2) if N % r == 0 and all(r % d for d in range(2, r))}:
            if N % q: continue
            R = None; Pm = g
            m = N // q
            while m:
                if m & 1: R = C.add(R, Pm)
                Pm = C.add(Pm, Pm); m >>= 1
            if R is None: ok = False; break
        if not ok: continue
        # full-order check by walking
        T, n = g, 1
        dl = {g: 1}
        while T is not None and n <= N:
            T = C.add(T, g); n += 1
            if T is None: break
            dl[T] = n
        if n == N: return dl
    return None

def selectors(p, x0, salt):
    h = random.Random(salt)
    mult = h.randrange(1, p) | 1; add = h.randrange(p)
    return {
        "y_parity": lambda x, y: y & 1,
        "y_high": lambda x, y: 1 if y > p // 2 else 0,
        "x_parity": lambda x, y: x & 1,
        "x_high": lambda x, y: 1 if x > p // 2 else 0,
        "leg_y": lambda x, y: 1 if leg(y, p) == 1 else 0,
        "leg_x_minus_x0": lambda x, y: 1 if leg(x - x0, p) == 1 else 0,
        "hash_null": lambda x, y: ((x * mult + add) % p * 2654435761 >> 7) & 1,
    }

def trial(C, x0, pts, bit, M, rng):
    p = C.p; T = (x0, 0)
    ok = tot = skip = 0
    def norm(Q):
        if bit(*Q) == 0:
            Q2 = C.add(Q, T)
            return Q if (Q2 is None or bit(*Q2) == 1) else None
        Q2 = C.add(Q, T)
        return Q2 if (Q2 is not None and Q2[1] and bit(*Q2) == 0) else None
    while tot < M and skip < 20 * M:
        Q1, Q2 = norm(rng.choice(pts)), norm(rng.choice(pts))
        if Q1 is None or Q2 is None: skip += 1; continue
        S = C.add(Q1, Q2)
        S2 = C.add(S, T) if S is not None else None
        if S is None or S2 is None or S[1] == 0 or bit(*S) == bit(*S2): skip += 1; continue
        tot += 1; ok += (bit(*S) == 0)
    if tot < M // 4: return None
    f = ok / tot
    return {"n": tot, "skipped": skip, "hom_frac": round(f, 4), "z": round((f - .5) / math.sqrt(.25 / tot), 2)}

def main():
    smoke = "--smoke" in sys.argv
    SEED = 20261110
    p_range = (1000, 1100) if smoke else (9000, 16000)
    nc = 1 if smoke else 10
    M = 200 if smoke else 3000
    rng = random.Random(SEED)
    ps = primes_in(*p_range)
    rows = []
    for ci in range(nc * 2):
        p = rng.choice(ps)
        cs = make_curves(p, random.Random(SEED + ci), {2: 1, 0: 1})
        for k in (2, 0):
            for (C, x0, N, pts) in cs[k][:1]:
                sels = selectors(p, x0, SEED + ci)
                res = {}
                for name, f in sels.items():
                    r = trial(C, x0, pts, f, M, random.Random(SEED * 7 + ci))
                    r2 = trial(C, x0, pts, f, M, random.Random(SEED * 13 + ci))  # A/A: independent resample
                    res[name] = {"main": r, "aa": r2}
                if k == 2:
                    dl = oracle_dl(C, N, pts, random.Random(SEED + ci))
                    if dl:
                        res["ORACLE_dl_parity"] = {"main": trial(C, x0, pts, lambda x, y: dl[(x, y)] & 1, M, random.Random(5)), "aa": None}
                rows.append({"p": p, "a": C.a, "b": C.b, "N": N, "N_mod4": N % 4, "res": res})
    # statistics
    Z = 5.0
    names = sorted({n for r in rows for n in r["res"]})
    summ = {}
    for n in names:
        zs = [(r["N_mod4"], r["res"][n]["main"]["z"]) for r in rows if n in r["res"] and r["res"][n]["main"]]
        summ[n] = {"cells": len(zs), "strong_cells_N2": sum(1 for k, z in zs if k == 2 and abs(z) > Z),
                   "strong_cells_N0": sum(1 for k, z in zs if k == 0 and abs(z) > Z),
                   "max_abs_z": max([abs(z) for _, z in zs], default=None)}
    aa = [abs(r["res"][n]["aa"]["z"]) for r in rows for n in r["res"] if r["res"][n].get("aa") and n not in ("leg_x_minus_x0",)]
    aa_exceed = sum(1 for z in aa if z > Z)
    hash_strong = summ.get("hash_null", {}).get("strong_cells_N2", 0) + summ.get("hash_null", {}).get("strong_cells_N0", 0)
    pos = summ.get("ORACLE_dl_parity", {})
    pos_ok = pos.get("cells", 0) > 0 and pos.get("strong_cells_N2", 0) == pos.get("cells", 0)
    cheap = [n for n in names if n not in ("ORACLE_dl_parity", "hash_null", "leg_x_minus_x0")]
    leaks = [n for n in cheap if summ[n]["strong_cells_N2"] + summ[n]["strong_cells_N0"] >= 3]
    if smoke: verdict = "inconclusive"
    elif not pos_ok or hash_strong > 0 or aa_exceed > 0: verdict = "inconclusive"
    elif leaks: verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    print(json.dumps({"lead": "LEAD-EXO-torsor-deck-period", "smoke": smoke, "seed": SEED,
        "params": {"p_range": p_range, "curves_per_class": nc, "pairs_per_cell": M, "z_threshold": Z},
        "statistic": "z of fraction b(Q1+Q2)==0 vs 0.5 over normalised pairs (section-homomorphism test)",
        "summary": summ, "positive_control_ok": pos_ok, "negative_control_hash_strong": hash_strong,
        "aa_exceed": aa_exceed, "aa_n": len(aa), "leaking_cheap_selectors": leaks,
        "ncells": len(rows), "rows": rows if smoke else None, "verdict": verdict}))

main()
