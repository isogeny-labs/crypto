#!/usr/bin/env python3
"""LEAD-EXO-coordinate-revealing-oracles toy: coordinate-aware m=2 x-membership IC vs rho, counted group ops."""
import json, math, random, sys, time
OPS = [0]

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

class Curve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b
    def add(s, A, B):
        OPS[0] += 1
        if A is None: return B
        if B is None: return A
        p = s.p
        x1, y1 = A; x2, y2 = B
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            l = (3 * x1 * x1 + s.a) * pow(2 * y1, -1, p) % p
        else:
            l = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (l * l - x1 - x2) % p
        return (x3, (l * (x1 - x3) - y1) % p)
    def neg(s, A): return None if A is None else (A[0], (-A[1]) % s.p)
    def mul(s, k, A):
        R = None
        while k:
            if k & 1: R = s.add(R, A)
            A = s.add(A, A); k >>= 1
        return R

def order(p, a, b):
    sq = [0] * p
    for y in range(p): sq[y * y % p] += 1
    n = 1
    for x in range(p): n += sq[(x * x * x + a * x + b) % p]
    return n

def make_instance(bits, seed):
    rng = random.Random(seed)
    p = next_prime((1 << bits) + rng.randrange(1 << (bits - 2)))
    while True:
        a, b = rng.randrange(p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        N = order(p, a, b)
        for h in (1, 2, 3, 4, 6, 8):
            if N % h == 0 and is_prime(N // h) and N // h > 50:
                r = N // h; break
        else: continue
        C = Curve(p, a, b)
        while True:
            x = rng.randrange(p); t = (x ** 3 + a * x + b) % p
            if pow(t, (p - 1) // 2, p) == 1 and t:
                y = pow(t, (p + 1) // 4, p) if p % 4 == 3 else None
                if y is None:
                    for y in range(p):
                        if y * y % p == t: break
                T = C.mul(h, (x, y))
                if T is not None: break
        # T has order r since r prime and T != O
        d = rng.randrange(2, r - 1)
        return C, T, C.mul(d, T), r, d

def rho(C, P, Q, r, d, rng):
    total = 0
    for _ in range(50):
        K = 16
        M = []
        for _ in range(K):
            u, v = rng.randrange(r), rng.randrange(r)
            M.append((u, v, C.add(C.mul(u, P), C.mul(v, Q))))
        a, b = rng.randrange(r), rng.randrange(r)
        R = C.add(C.mul(a, P), C.mul(b, Q))
        OPS[0] = 0
        seen = {}
        while True:
            if R is None: break
            if R in seen:
                a2, b2 = seen[R]
                if (b2 - b) % r:
                    dd = (a - a2) * pow(b2 - b, -1, r) % r
                    if dd == d: return OPS[0] + total
                break
            seen[R] = (a, b)
            u, v, Mi = M[R[0] % K]
            R = C.add(R, Mi); a = (a + u) % r; b = (b + v) % r
        total += OPS[0]
    return float('nan')

def solve(rows, f, r):
    n = f + 1
    M = [row[:] for row in rows]
    piv = 0
    for c in range(n):
        s = None
        for i in range(piv, len(M)):
            if M[i][c] % r: s = i; break
        if s is None: return None
        M[piv], M[s] = M[s], M[piv]
        inv = pow(M[piv][c], -1, r)
        M[piv] = [v * inv % r for v in M[piv]]
        for i in range(len(M)):
            if i != piv and M[i][c] % r:
                m = M[i][c]; M[i] = [(vi - m * vp) % r for vi, vp in zip(M[i], M[piv])]
        piv += 1
    return M[n - 1][n] % r  # d (last unknown)

def ic(C, P, Q, r, d, f, rng, fb_mode, free_oracle=False):
    p = C.p
    pts = []
    if fb_mode == "small_x":
        x = 0
        while len(pts) < f:
            t = (x ** 3 + C.a * x + C.b) % p
            if t == 0 or pow(t, (p - 1) // 2, p) == 1:
                y = next(y for y in range(p) if y * y % p == t)
                if C.mul(r, (x, y)) is None: pts.append((x, y))
            x += 1
    else:  # random x-subset (hash-null: no coordinate structure)
        used = set()
        while len(pts) < f:
            x = rng.randrange(p)
            if x in used: continue
            used.add(x); t = (x ** 3 + C.a * x + C.b) % p
            if t == 0 or pow(t, (p - 1) // 2, p) == 1:
                y = next(y for y in range(p) if y * y % p == t)
                if C.mul(r, (x, y)) is None: pts.append((x, y))
    fbx = {pt[0]: (i, pt[1]) for i, pt in enumerate(pts)}
    T = [(u, v, C.add(C.mul(u, P), C.mul(v, Q))) for u, v in
         ((rng.randrange(r), rng.randrange(r)) for _ in range(8))]
    OPS[0] = 0
    a, b = rng.randrange(r), rng.randrange(r)
    R = C.add(C.mul(a, P), C.mul(b, Q))
    rows, need = [], f + 6
    cost = None; sol = None
    trials = 0
    while trials < 400000:
        if len(rows) >= need:
            if cost is None: cost = OPS[0]
            sol = solve(rows, f, r)
            if sol is not None or need > 8 * f + 40: break
            need += 5
            cost = None
        trials += 1
        u, v, Ti = rng.choice(T)
        R = C.add(R, Ti); a = (a + u) % r; b = (b + v) % r
        if R is None: continue
        found = None
        if free_oracle:
            OPS[0] += 0  # oracle charged 1 op per trial (the R update above)
            for i, F in enumerate(pts):
                for s1 in (1, -1):
                    S = C.add(R, C.neg(F) if s1 == 1 else F); OPS[0] -= 1  # uncharged local work
                    if S is not None and S[0] in fbx:
                        j, yj = fbx[S[0]]; found = (i, s1, j, 1 if S[1] == yj else -1); break
                if found: break
        else:
            for i, F in enumerate(pts):
                for s1 in (1, -1):
                    S = C.add(R, C.neg(F) if s1 == 1 else F)
                    if S is not None and S[0] in fbx:
                        j, yj = fbx[S[0]]; found = (i, s1, j, 1 if S[1] == yj else -1); break
                if found: break
        if found:
            i, s1, j, s2 = found
            row = [0] * (f + 2)
            row[i] = (row[i] + s1) % r; row[j] = (row[j] + s2) % r
            row[f] = (-b) % r; row[f + 1] = a % r
            rows.append(row)
    if sol is None or sol != d or cost is None: return float("nan")
    return cost

def med(v):
    v = sorted(x for x in v if x == x)
    return v[len(v) // 2] if v else float('nan')

def main():
    smoke = "--smoke" in sys.argv
    sizes = [10] if smoke else [12, 14, 16, 18]
    ncurve = 1 if smoke else 3
    nrep = 2 if smoke else 5
    t0 = time.time()
    out = {"lead": "LEAD-EXO-coordinate-revealing-oracles", "seeds": "20261110+bits*100+curve", "sizes_bits": sizes,
           "curves_per_size": ncurve, "reps": nrep, "cost_unit": "counted group additions", "rows": []}
    for bits in sizes:
        rr, c1, c2, nl, fr = [], [], [], [], []
        for ci in range(ncurve):
            seed = 20261110 + bits * 100 + ci
            C, P, Q, r, d = make_instance(bits, seed)
            f = max(8, round(C.p ** (1 / 3)) * 2)
            rng = random.Random(seed * 7 + 1)
            for k in range(nrep):
                rr.append(rho(C, P, Q, r, d, rng))
                c1.append(ic(C, P, Q, r, d, f, rng, "small_x"))
                c2.append(ic(C, P, Q, r, d, f, rng, "small_x"))  # A/A arm: same arm, fresh randomness
                nl.append(ic(C, P, Q, r, d, f, rng, "random_x"))
                fr.append(ic(C, P, Q, r, d, f, rng, "small_x", free_oracle=True))
        rho_m = med(rr)
        out["rows"].append({"bits": bits, "f": f, "rho_med": rho_m, "ic_med": med(c1), "ic_AA_med": med(c2),
            "ic_null_randomx_med": med(nl), "ic_free_oracle_med": med(fr),
            "ratio_ic_over_rho": med(c1) / rho_m, "AA_ratio": med(c2) / med(c1),
            "null_ratio": med(nl) / med(c1), "pos_ctrl_reduction": med(c1) / med(fr),
            "ic_fail_frac": sum(x != x for x in c1) / len(c1)})
        if time.time() - t0 > 500: out["truncated"] = True; break
    R = out["rows"]
    b = [x["bits"] for x in R]; y = [math.log2(x["ratio_ic_over_rho"]) for x in R if x["ratio_ic_over_rho"] == x["ratio_ic_over_rho"]]
    slope = float('nan')
    if len(y) >= 2 and len(y) == len(b):
        mb, my = sum(b) / len(b), sum(y) / len(y)
        slope = sum((u - mb) * (v - my) for u, v in zip(b, y)) / sum((u - mb) ** 2 for u in b)
    out["slope_log2_ratio_per_bit"] = slope
    out["thresholds"] = {"ratio_gt_1_all": True, "slope_gt": 0.2, "AA_within": 0.25, "null_within": 0.35, "pos_ctrl_reduction_ge": 4}
    ok_ratio = all(x["ratio_ic_over_rho"] > 1 for x in R)
    ctrl = all(abs(x["AA_ratio"] - 1) < 0.25 and abs(x["null_ratio"] - 1) < 0.35 and x["pos_ctrl_reduction"] >= 4 for x in R)
    if smoke:
        out["verdict"] = "inconclusive"
    elif R and R[-1]["ratio_ic_over_rho"] < 1 and not out.get("truncated"):
        out["verdict"] = "falsified"
    elif ok_ratio and slope == slope and slope > 0.2 and ctrl and not out.get("truncated"):
        out["verdict"] = "consistent_with_prediction"
    else:
        out["verdict"] = "inconclusive"
    out["controls_ok"] = ctrl
    out["elapsed_s"] = round(time.time() - t0, 2)
    print(json.dumps(out))

main()
