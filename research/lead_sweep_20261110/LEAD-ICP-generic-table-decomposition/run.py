#!/usr/bin/env python3
"""Preregistered toy: cost exponent of pair-table (arity 2) decomposition IC, relation phase lower bound.
stdlib only. See PROTOCOL.md."""
import sys, json, random, math

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def add(P, Q, p, a, cnt):
    cnt[0] += 1
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def neg(P, p): return None if P is None else (P[0], (-P[1]) % p)

def mul(k, P, p, a, cnt):
    R = None
    while k:
        if k & 1: R = add(R, P, p, a, cnt)
        P = add(P, P, p, a, cnt); k >>= 1
    return R

def make_curve(bits, rng):
    while True:
        p = rng.getrandbits(bits) | (1 << (bits - 1)) | 3
        if p % 4 != 3 or not isprime(p): continue
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a**3 + 27 * b * b) % p == 0: continue
        sq = bytearray(p)
        for x in range(p): sq[x * x % p] = 1
        N = p + 1
        for x in range(p):
            f = (x**3 + a * x + b) % p
            if f == 0: N += 0
            elif sq[f]: N += 1
            # chi(f)= +1 -> two points -> contributes +1 to sum chi
            else: N -= 1
        # N = p+1+sum chi(f)
        if not isprime(N): continue
        while True:
            x = rng.randrange(p); f = (x**3 + a * x + b) % p
            if f and sq[f]:
                return p, a, b, N, (x, pow(f, (p + 1) // 4, p))

def rho_cost(p, a, N, P, Q, rng):
    cnt = [0]; seen = set()
    pts = []
    for _ in range(16):
        u, v = rng.randrange(1, N), rng.randrange(1, N)
        pts.append(add(mul(u, P, p, a, [0]), mul(v, Q, p, a, [0]), p, a, [0]))
    R = add(mul(rng.randrange(1, N), P, p, a, [0]), mul(rng.randrange(1, N), Q, p, a, [0]), p, a, [0])
    while R is not None and R not in seen and cnt[0] < 50 * N:
        seen.add(R)
        R = add(R, pts[R[0] & 15], p, a, cnt)
    return cnt[0]

def ic_cost(p, a, N, P, Q, B, rng, free):
    """cost = table build (unless free) + walk steps until B+1 distinct relations R=+-Fj+-Fk."""
    cnt = [0]
    F = []
    for _ in range(B):
        F.append(add(mul(rng.randrange(1, N), P, p, a, [0]), mul(rng.randrange(1, N), Q, p, a, [0]), p, a, [0]))
    fx = {f[0] for f in F if f}
    tab = set(); bc = [0]
    for j in range(B):
        for k in range(j, B):
            S = add(F[j], F[k], p, a, bc)
            if S: tab.add(S[0])
            if k > j:
                D = add(F[j], neg(F[k], p), p, a, bc)
                if D: tab.add(D[0])
    build = 0 if free else bc[0]
    R = add(mul(rng.randrange(1, N), P, p, a, [0]), mul(rng.randrange(1, N), Q, p, a, [0]), p, a, [0])
    got = set(); walk = 0; steps = 0; cap = 40 * N; period = max(8, int(math.isqrt(N)))
    while len(got) < B + 1 and walk < cap:
        walk += 1; steps += 1
        if R is None: R = F[0]
        if R[0] in tab and R[0] not in fx: got.add(R)
        R = add(R, F[R[0] % B], p, a, [0])
        if steps % period == 0:
            R = add(mul(rng.randrange(1, N), P, p, a, [0]), mul(rng.randrange(1, N), Q, p, a, [0]), p, a, [0])
            walk += 3 * N.bit_length()  # charge the restart
    return build + walk, len(got) >= B + 1

def slope(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def boot(xs, ys, rng, n=300):
    s = []
    for _ in range(n):
        idx = [rng.randrange(len(xs)) for _ in xs]
        if len({xs[i] for i in idx}) < 2: continue
        s.append(slope([xs[i] for i in idx], [ys[i] for i in idx]))
    s.sort()
    return [s[int(0.025 * len(s))], s[int(0.975 * len(s)) - 1]]

def main():
    smoke = "--smoke" in sys.argv
    bits_list = [9, 10] if smoke else list(range(12, 19))
    reps = 2 if smoke else 5
    egrid = [0.3, 0.4, 0.5] if smoke else [0.25, 0.3, 0.35, 0.4, 0.45, 0.5]
    rng = random.Random(20261110)
    data = {k: ([], []) for k in ("rho", "ic", "ic_aa", "ic_free")}
    incomplete = 0
    for bits in bits_list:
        p, a, b, N, P = make_curve(bits, random.Random(1000 + bits))
        Q = mul(random.Random(7).randrange(1, N), P, p, a, [0])
        lx = math.log2(N)
        for r in range(reps):
            data["rho"][0].append(lx); data["rho"][1].append(math.log2(rho_cost(p, a, N, P, Q, random.Random(100 + r + 1000 * bits))))
            for key, free, seed0 in (("ic", False, 5000), ("ic_aa", False, 9000), ("ic_free", True, 5000)):
                best = None
                for gi, e in enumerate(egrid):
                    B = max(4, int(round(N ** e)))
                    c, ok = ic_cost(p, a, N, P, Q, B, random.Random(seed0 + 97 * r + 13 * gi + 100000 * bits), free)
                    if not ok: incomplete += 1; continue
                    best = c if best is None else min(best, c)
                if best is None: best = 40 * N
                data[key][0].append(lx); data[key][1].append(math.log2(best))
    out = {"lead": "LEAD-ICP-generic-table-decomposition", "smoke": smoke, "bits": bits_list, "reps": reps,
           "B_exponent_grid": egrid, "seeds": "curve 1000+bits; walks 100/5000/9000 + derived", "incomplete_runs": incomplete, "arms": {}}
    for k, (xs, ys) in data.items():
        out["arms"][k] = {"slope": round(slope(xs, ys), 4), "ci95": [round(v, 4) for v in boot(xs, ys, random.Random(1))]}
    ic, rho, aa, fr = (out["arms"][k] for k in ("ic", "rho", "ic_aa", "ic_free"))
    aa_diff = abs(ic["slope"] - aa["slope"])
    out["controls"] = {"aa_slope_diff": round(aa_diff, 4), "aa_ok": aa_diff < 0.1,
        "null_rho_ci_contains_0.5": rho["ci95"][0] <= 0.5 <= rho["ci95"][1],
        "positive_control_free_table_gap": round(ic["slope"] - fr["slope"], 4),
        "positive_control_ok": ic["slope"] - fr["slope"] > 0.08}
    c = out["controls"]
    controls_ok = c["aa_ok"] and c["null_rho_ci_contains_0.5"] and c["positive_control_ok"]
    if not controls_ok or incomplete > 0.2 * len(bits_list) * reps * len(egrid) * 3: v = "inconclusive"
    elif ic["ci95"][0] > 0.55: v = "consistent_with_prediction"
    elif ic["ci95"][1] < 0.55: v = "falsified"
    else: v = "inconclusive"
    out["verdict"] = v
    print(json.dumps(out))

main()
