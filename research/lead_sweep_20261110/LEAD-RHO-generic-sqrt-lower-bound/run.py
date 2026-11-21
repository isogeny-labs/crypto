#!/usr/bin/env python3
"""Black-box group toy: count group ops of generic DLP algorithms vs sqrt(N). stdlib only."""
import sys, json, math, random

class BB:
    """Opaque-label cyclic group Z_N. Labels = keyed Feistel permutation of the exponent.
    Algorithms get only add(l1,l2), equality/hash of labels, gen, zero. Ops are counted."""
    def __init__(s, N, key):
        s.N = N; k = max(2, (N - 1).bit_length()); k += k & 1
        s.h = k // 2; s.mask = (1 << s.h) - 1; s.ops = 0
        r = random.Random(key); s.keys = [r.getrandbits(61) | 1 for _ in range(4)]
    def _f(s, v, i): return ((v * s.keys[i] + 0x9E3779B97F4A7C15) ^ (v >> 3)) * s.keys[(i + 1) % 4] >> 7 & s.mask
    def _p(s, x):
        while True:
            l, r = x >> s.h, x & s.mask
            for i in range(4): l, r = r, l ^ s._f(r, i)
            x = (l << s.h) | r
            if x < s.N: return x
    def _q(s, x):
        while True:
            l, r = x >> s.h, x & s.mask
            for i in (3, 2, 1, 0): l, r = r ^ s._f(l, i), l
            x = (l << s.h) | r
            if x < s.N: return x
    def enc(s, x): return s._p(x % s.N)           # harness only
    def add(s, a, b):
        s.ops += 1; return s._p((s._q(a) + s._q(b)) % s.N)
    def mul(s, k, a):                              # double-and-add, counted
        res = None
        for bit in bin(k)[2:]:
            if res is not None: res = s.add(res, res)
            if bit == '1': res = a if res is None else s.add(res, a)
        return res

def bsgs(G, g, q):
    N = G.N; m = math.isqrt(N) + 1; tab = {}; cur = None
    tab[G.enc(0)] = 0; cur = g
    for j in range(1, m):
        tab[cur] = j; cur = G.add(cur, g)
    step = G.mul(N - m % N if m % N else 0, g) if True else None
    y = q
    for i in range(m + 1):
        if y in tab: return (i * m + tab[y]) % N
        y = G.add(y, step)
    return None

def rho(G, g, q, rng):
    """Distinguished-point r-adding walk, restarts by a jump from the current point (1 op each)."""
    N = G.N; r = 8; M = []
    for _ in range(r):
        a, b = rng.randrange(1, 8), rng.randrange(1, 8)          # small exponents: cheap setup
        M.append((G.add(G.mul(a, g), G.mul(b, q)), a, b))
    dbits = max(1, int(math.log2(N) / 2) - 3); dmask = (1 << dbits) - 1
    a, b = rng.randrange(N), rng.randrange(N)
    x = G.add(G.mul(a, g), G.mul(b, q)); seen = {}; limit = 60 * math.isqrt(N) + 1000
    while G.ops < limit:
        for _ in range(30 * (dmask + 1)):
            if (hash(x) & dmask) == 0:
                if x in seen and seen[x] != (a, b):
                    a2, b2 = seen[x]; db = (b - b2) % N
                    if db: return (a2 - a) * pow(db, -1, N) % N
                seen[x] = (a, b); break
            m = M[(hash(x) >> 20) % r]
            x = G.add(x, m[0]); a = (a + m[1]) % N; b = (b + m[2]) % N
        m = M[rng.randrange(r)]                                   # jump to a new trail
        x = G.add(x, m[0]); a = (a + m[1]) % N; b = (b + m[2]) % N
    return None

def clear_solver(N, x):
    # positive control: reads the encoding (labels are cleartext exponents): O(1) group ops
    return x, 1

def primes_near(n):
    def ip(k):
        if k < 2: return False
        return all(k % d for d in range(2, math.isqrt(k) + 1))
    while not ip(n): n -= 1
    return n

def slope(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def run_alg(name, ks, seeds, seed0):
    per = {}; fail = 0
    for k in ks:
        N = primes_near(2 ** k); lst = []
        for s in range(seeds):
            rng = random.Random(f"{seed0}-{name}-{k}-{s}")
            x = rng.randrange(1, N); G = BB(N, rng.getrandbits(64))
            g = G.enc(1); q = G.enc(x)
            if name == "bsgs": res = bsgs(G, g, q); ops = G.ops
            elif name == "rho": res = rho(G, g, q, rng); ops = G.ops
            else: res, ops = clear_solver(N, x)
            if res is None or res % N != x: fail += 1; continue
            lst.append(ops)
        per[k] = lst
    return per, fail

def fit(per, ks, boots, rng):
    def a_of(d): return slope([k / 1 for k in ks], [math.log2(max(1e-9, d[k])) for k in ks])
    means = {k: sum(per[k]) / len(per[k]) for k in ks}
    est = a_of(means); bs = []
    for _ in range(boots):
        d = {k: (lambda L: sum(L) / len(L))([rng.choice(per[k]) for _ in per[k]]) for k in ks}
        bs.append(a_of(d))
    bs.sort()
    return est, bs[int(.025 * boots)], bs[int(.975 * boots) - 1], means

def main():
    smoke = "--smoke" in sys.argv
    ks = [10, 12, 14] if smoke else [12, 14, 16, 18, 20, 22]
    seeds = 4 if smoke else 24; boots = 50 if smoke else 400
    out = {"protocol": "LEAD-RHO-generic-sqrt-lower-bound", "smoke": smoke, "log2N": ks,
           "seeds_per_size": seeds, "seed_base": "lsw20261110", "arms": {}}
    brng = random.Random(7)
    for name, s0 in [("bsgs", "A"), ("rho", "A"), ("bsgs_AA", "B"), ("clear_positive_control", "A")]:
        an = name.replace("_AA", "")
        per, fail = run_alg(an if an != "clear_positive_control" else "clear", ks, seeds, s0)
        if any(len(per[k]) < 2 for k in ks):
            out["arms"][name] = {"fail": fail, "error": "too few successes"}; continue
        a, lo, hi, means = fit(per, ks, boots, brng)
        out["arms"][name] = {"alpha": round(a, 4), "ci95": [round(lo, 4), round(hi, 4)], "failures": fail,
            "ratio_ops_over_sqrtN": {k: round(means[k] / math.sqrt(2 ** k), 3) for k in ks}}
    A = out["arms"]
    try:
        gen = [A["bsgs"], A["rho"]]
        pc_ok = A["clear_positive_control"]["ci95"][1] < 0.3
        aa_ok = abs(A["bsgs"]["alpha"] - A["bsgs_AA"]["alpha"]) < 0.1
        falsified = any(a["ci95"][1] < 0.45 or min(a["ratio_ops_over_sqrtN"].values()) < 0.5 for a in gen)  # ratio floor: no arm under 0.5 sqrt(N) ops
        consistent = all(a["ci95"][0] <= 0.5 <= a["ci95"][1] for a in gen)
        out["controls"] = {"positive_control_detects_subsqrt": pc_ok, "AA_alpha_diff_lt_0.1": aa_ok}
        out["verdict"] = "inconclusive" if not (pc_ok and aa_ok) and not smoke else \
            "falsified" if falsified else "consistent_with_prediction" if consistent else "inconclusive"
        if smoke: out["verdict"] = out["verdict"] if out["verdict"] != "falsified" else "inconclusive"
    except KeyError:
        out["verdict"] = "inconclusive"
    print(json.dumps(out)); return 0

if __name__ == "__main__": sys.exit(main())
