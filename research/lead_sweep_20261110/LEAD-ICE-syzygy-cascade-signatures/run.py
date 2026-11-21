#!/usr/bin/env python3
"""Planted-syzygy unit-slope calibration of a Macaulay-rank syzygy detector (stdlib only)."""
import itertools, json, random, sys

P = 10007
def monos(n, d):
    return [c for c in itertools.product(range(d + 1), repeat=n) if sum(c) == d] if n <= 6 else None
def mons(n, d):
    res = []
    def rec(i, left, cur):
        if i == n - 1:
            res.append(tuple(cur + [left])); return
        for a in range(left, -1, -1):
            rec(i + 1, left - a, cur + [a])
    rec(0, d, []); return res
def rand_form(rng, n, d, ms):
    return {m: rng.randrange(1, P) for m in ms}
def mul(f, m):
    return {tuple(a + b for a, b in zip(k, m)): v for k, v in f.items()}
def pmul(f, g):
    r = {}
    for k, v in f.items():
        for k2, v2 in g.items():
            kk = tuple(a + b for a, b in zip(k, k2)); r[kk] = (r.get(kk, 0) + v * v2) % P
    return r
def rank(rows, ncols):
    piv = {}; r = 0
    for row in rows:
        row = row[:]
        for c in range(ncols):
            if row[c]:
                if c in piv:
                    pr = piv[c]; f = row[c]
                    row = [(x - f * y) % P for x, y in zip(row, pr)]
                else:
                    inv = pow(row[c], P - 2, P); row = [x * inv % P for x in row]
                    piv[c] = row; r += 1; break
    return r
def deficit(n, gens, d):
    """gens: list of (poly, degree). rows - rank of Macaulay matrix in degree d."""
    cols = mons(n, d); idx = {m: i for i, m in enumerate(cols)}; rows = []
    for f, df in gens:
        if df > d: continue
        for m in mons(n, d - df):
            g = mul(f, m); row = [0] * len(cols)
            for k, v in g.items(): row[idx[k]] = v
            rows.append(row)
    return len(rows) - rank(rows, len(cols))
def system(rng, n, m, k):
    q = mons(n, 2); lin = mons(n, 1)
    quads = [rand_form(rng, n, 2, q) for _ in range(m)]
    gens = [(f, 2) for f in quads]
    for _ in range(k):  # planted degree-3 generator in the ideal: one non-Koszul syzygy at degree 3
        gens.append((pmul(rand_form(rng, n, 1, lin), quads[0]), 3))
    return gens
def arm(seed, n, m, k, trials, degs):
    rng = random.Random(seed); out = {d: [] for d in degs}
    for _ in range(trials):
        g = system(rng, n, m, k)
        for d in degs: out[d].append(deficit(n, g, d))
    return out
def main():
    smoke = "--smoke" in sys.argv
    n, m = (3, 3) if smoke else (5, 5)
    trials = 2 if smoke else 20
    ks = [0, 1, 2] if smoke else [0, 1, 2, 3, 4]
    degs = [3] if smoke else [3, 4]
    koszul4 = m * (m - 1) // 2
    res = {"n": n, "m": m, "p": P, "trials": trials, "ks": ks, "seeds": {}}
    A = arm(1001, n, m, 0, trials, degs); B = arm(2002, n, m, 0, trials, degs)
    res["null_A"], res["null_B_AA"] = A, B
    res["seeds"]["null_A"], res["seeds"]["null_B"] = 1001, 2002
    pos = {}
    for k in ks:
        pos[k] = arm(3000 + k, n, m, k, trials, degs)[3]; res["seeds"]["planted_k%d" % k] = 3000 + k
    res["planted_deficit_d3"] = pos
    means = [sum(pos[k]) / trials for k in ks]
    kb = sum(ks) / len(ks); mb = sum(means) / len(ks)
    slope = sum((k - kb) * (x - mb) for k, x in zip(ks, means)) / sum((k - kb) ** 2 for k in ks)
    res["slope_d3"] = slope
    null_ok = all(x == 0 for x in A[3] + B[3])
    kosz_ok = True if smoke else all(x == koszul4 for x in A[4] + B[4])
    exact = all(x == k for k in ks for x in pos[k])
    res["checks"] = {"null_d3_zero": null_ok, "null_d4_equals_koszul": kosz_ok, "unit_slope_exact": exact}
    if null_ok and kosz_ok and exact and abs(slope - 1) < 1e-9: v = "consistent_with_prediction"
    elif not null_ok or not kosz_ok: v = "falsified"
    else: v = "inconclusive"
    res["verdict"] = v
    print(json.dumps(res)); return 0
if __name__ == "__main__":
    sys.exit(main())
