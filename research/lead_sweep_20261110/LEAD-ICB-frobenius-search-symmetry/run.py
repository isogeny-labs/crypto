import sys, json, random
POLY = {7: 0b10000011, 11: (1 << 11) | 0b101, 13: (1 << 13) | 0b11011}
A_COEF, B_COEF = 1, 1

class F:
    def __init__(s, n):
        s.n, s.q, s.poly = n, 1 << n, POLY[n]
        s.exp = [0] * (2 * s.q); s.log = [0] * s.q
        for g in range(2, s.q):
            x, ok, seen = 1, True, 0
            for i in range(s.q - 1):
                s.exp[i] = x; s.log[x] = i
                x = s.mulslow(x, g)
                if x == 1 and i < s.q - 2: ok = False; break
            if ok: break
        for i in range(s.q - 1, 2 * s.q): s.exp[i] = s.exp[i - (s.q - 1)]
    def mulslow(s, a, b):
        r = 0
        while b:
            if b & 1: r ^= a
            b >>= 1; a <<= 1
            if a >> s.n: a ^= s.poly
        return r
    def mul(s, a, b):
        return 0 if not a or not b else s.exp[s.log[a] + s.log[b]]
    def inv(s, a): return s.exp[(s.q - 1) - s.log[a]]
    def sq(s, a): return s.mul(a, a)

def build(n):
    f = F(n); q = f.q
    z2 = {}
    for z in range(q): z2.setdefault(f.sq(z) ^ z, []).append(z)
    pts = []
    for x in range(q):
        rhs = f.mul(f.sq(x), f.mul(x, 1)) ^ (f.sq(x) if A_COEF else 0) ^ B_COEF
        if x == 0:
            pts.append((0, f.exp[(f.log[B_COEF] * (q // 2)) % (q - 1)] if False else None)); pts.pop()
            # y^2 = b -> y = sqrt(b) = b^(2^(n-1))
            y = B_COEF
            for _ in range(n - 1): y = f.sq(y)
            pts.append((0, y)); continue
        t = f.mul(rhs, f.inv(f.sq(x)))
        for z in z2.get(t, []): pts.append((x, f.mul(x, z)))
    return f, pts

def add(f, P, Q):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if y1 != y2 or x1 == 0: return None
        l = x1 ^ f.mul(y1, f.inv(x1))
        x3 = f.sq(l) ^ l ^ A_COEF
    else:
        l = f.mul(y1 ^ y2, f.inv(x1 ^ x2))
        x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ A_COEF
    y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1
    return (x3, y3)

def frob(f, P): return None if P is None else (f.sq(P[0]), f.sq(P[1]))

def counts(f, fb):
    N = {}
    for i in range(len(fb)):
        for j in range(i, len(fb)):
            R = add(f, fb[i], fb[j]); N[R] = N.get(R, 0) + 1
    return N

def run_n(n, norb, seeds):
    f, pts = build(n); order = len(pts) + 1
    allp = pts + [None]
    out = {"n": n, "order": order, "seeds": seeds, "norb": norb}
    ratios, stab_d, rnd_d, dtau, drnd = [], 0.0, 0.0, [], []
    for sd in seeds:
        rng = random.Random(sd)
        # stable FB: union of Frobenius orbits
        st = set()
        while len(st) < norb * n:
            P = rng.choice(pts)
            for _ in range(n): st.add(P); P = frob(f, P)
        st = sorted(st)
        ns = set(rng.sample(pts, len(st))); ns = sorted(ns)
        res = {}
        for name, fb in (("stable", st), ("random", ns)):
            N = counts(f, fb)
            d_t = sum(abs(N.get(R, 0) - N.get(frob(f, R), 0)) for R in allp) / order
            perm = allp[:]; rng.shuffle(perm)
            d_r = sum(abs(N.get(R, 0) - N.get(S, 0)) for R, S in zip(allp, perm)) / order
            res[name] = (d_t, d_r)
        dtau.append(res["random"][0]); drnd.append(res["random"][1]); stab_d += res["stable"][0]
    out["stable_fb_mean_abs_diff_tau"] = stab_d / len(seeds)
    out["random_fb_mean_abs_diff_tau"] = sum(dtau) / len(dtau)
    out["random_fb_mean_abs_diff_randompair"] = sum(drnd) / len(drnd)
    out["tau_over_randompair"] = out["random_fb_mean_abs_diff_tau"] / out["random_fb_mean_abs_diff_randompair"]
    fixed = sum(1 for P in allp if frob(f, P) == P)
    out["frobenius_fixed_points"] = fixed
    out["fixed_fraction"] = fixed / order
    return out

def main():
    smoke = "--smoke" in sys.argv
    cfg = [(7, 4, [1, 2])] if smoke else [(11, 40, [101, 102, 103, 104, 105]), (13, 40, [201, 202, 203, 204, 205])]
    rows = [run_n(*c) for c in cfg]
    ok = all(r["stable_fb_mean_abs_diff_tau"] == 0 for r in rows)  # positive control
    if not ok: v = "inconclusive"
    else:
        good = all(0.9 <= r["tau_over_randompair"] <= 1.1 and r["fixed_fraction"] < 0.01 + (0.5 if smoke else 0) for r in rows)
        bad = any(r["tau_over_randompair"] < 0.8 or r["tau_over_randompair"] > 1.25 for r in rows)
        v = "consistent_with_prediction" if good else ("falsified" if bad else "inconclusive")
    print(json.dumps({"lead": "LEAD-ICB-frobenius-search-symmetry", "smoke": smoke, "rows": rows,
                      "positive_control_stable_fb_exact_invariance": ok, "verdict": v}))
main()
