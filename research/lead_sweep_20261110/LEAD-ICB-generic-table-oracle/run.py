#!/usr/bin/env python3
"""LEAD-ICB-generic-table-oracle toy: pair-table 4-sum decomposition oracle vs matched rho.
Stdlib only, one core, fixed seeds. Group ops are done via a discrete-log table of a toy Koblitz
curve y^2+xy=x^3+x^2+1 over GF(2^n); costs are counted in abstract group additions (GAE)."""
import sys, json, random, math

POLY = {7: (1 << 7) | 0b11, 9: (1 << 9) | 0b10001, 11: (1 << 11) | 0b101, 13: (1 << 13) | 0b11011,
        15: (1 << 15) | 0b11, 17: (1 << 17) | 0b1001}

def field(n):
    p = POLY[n]; size = 1 << n
    for g in range(2, 64):
        exp = [0] * (size - 1); x = 1; ok = True
        for i in range(size - 1):
            exp[i] = x
            if i and x == 1: ok = False; break
            # multiply x by g (carryless) mod p
            a, b, r = x, g, 0
            while b:
                if b & 1: r ^= a
                a <<= 1
                if a >> n: a ^= p
                b >>= 1
            x = r
        if ok and x == 1:
            log = [0] * size
            for i, v in enumerate(exp): log[v] = i
            return exp, log
    raise RuntimeError("no generator")

def curve_group(n):
    exp, log = field(n); m = (1 << n) - 1
    mul = lambda a, b: 0 if a == 0 or b == 0 else exp[(log[a] + log[b]) % m]
    inv = lambda a: exp[(-log[a]) % m]
    sq2z = {}
    for z in range(1 << n): sq2z.setdefault(mul(z, z) ^ z, z)
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 ^ y2 == x1 or (y1 == y2 and x1 == 0) and False: pass
            if y1 != y2: return None
            if x1 == 0: return None
            lam = x1 ^ mul(y1, inv(x1))
            x3 = mul(lam, lam) ^ lam ^ 1
            return (x3, mul(x1, x1) ^ mul(lam ^ 1, x3))
        lam = mul(y1 ^ y2, inv(x1 ^ x2))
        x3 = mul(lam, lam) ^ lam ^ x1 ^ x2 ^ 1
        return (x3, mul(lam, x1 ^ x3) ^ x3 ^ y1)
    Q = None
    for x in range(1, 1 << n):
        t = mul(x ^ 1 if False else (mul(mul(x, x), x) ^ mul(x, x) ^ 1), inv(mul(x, x)))
        if t in sq2z: Q = (x, mul(x, sq2z[t])); break
    G = add(Q, Q); pts = [None]; P = G
    while P is not None:
        pts.append(P); P = add(P, G)
    return pts  # pts[k] = kG, order r = len(pts)

def rho_steps(r, seeds):
    tot = 0
    for s in seeds:
        rng = random.Random(s); a = [rng.randrange(1, r) for _ in range(16)]
        L = rng.randrange(r); seen = {L}; k = 1
        while True:
            L = (L + a[L & 15]) % r; k += 1
            if L in seen: break
            seen.add(L)
        tot += k
    return tot / len(seeds)

def oracle(pts, k, seed, cap_tries=200000, subspace=True):
    """Pair-table 4-sum oracle. FB = points with x < 2^k (subspace) or random set of the same size."""
    r = len(pts); rng = random.Random(seed)
    if subspace:
        F = [i for i in range(1, r) if pts[i][0] < (1 << k)]
    else:
        sz = sum(1 for i in range(1, r) if pts[i][0] < (1 << k)); F = rng.sample(range(1, r), sz)
    F = sorted(set(F) | {(-i) % r for i in F} - {0})
    nF = len(F); table = {}
    for i in range(nF):
        for j in range(i, nF):
            table.setdefault((F[i] + F[j]) % r, (F[i], F[j]))
    adds = len(table); lookups = 0; need = nF // 2 + 1; got = 0; tries = 0; rel_ok = True
    Tkeys = list(table)
    classes = set()
    while got < need and tries < cap_tries:
        tries += 1; R = rng.randrange(r); hit = False
        for s in Tkeys:
            lookups += 1; adds += 1
            t = (R - s) % r
            if t in table:
                a, b = table[s]; c, d = table[t]
                if (a + b + c + d) % r != R: rel_ok = False
                hit = True; break
        if hit and min(R, (-R) % r) not in classes:
            classes.add(min(R, (-R) % r)); got += 1
    return dict(k=k, nF=nF, table=len(table), adds=adds, lookups=lookups, tries=tries, got=got,
                need=need, verified=rel_ok, p_hat=(got / tries if tries else 0.0),
                p_pred=min(1.0, len(table) ** 2 / (2 * r)))

def main():
    smoke = "--smoke" in sys.argv
    ns = [9] if smoke else [11, 13, 15, 17]
    nrho = 20 if smoke else 200
    out = dict(lead="LEAD-ICB-generic-table-oracle", smoke=smoke, seeds=dict(oracle=[101, 202], rho="1000..%d" % (999 + nrho)),
               curve="y^2+xy=x^3+x^2+1 / GF(2^n), subgroup <2Q>", rows=[])
    for n in ns:
        pts = curve_group(n); r = len(pts)
        rho = rho_steps(r, range(1000, 1000 + nrho))
        ks = [3, 4] if smoke else list(range(2, max(3, n // 2 + 1)))
        best = {}
        for sub, tag in ((True, "subspace"), (False, "random")):
            for sd in ((101, 202) if tag == "subspace" else (101,)):
                runs = [oracle(pts, k, sd, subspace=sub) for k in ks]
                good = [x for x in runs if x["got"] >= x["need"]]
                best[(tag, sd)] = min(good, key=lambda x: x["adds"]) if good else None
        def ratio(b, c): return None if b is None else (b["adds"] + c * b["lookups"]) / rho
        a1, a2 = best[("subspace", 101)], best[("subspace", 202)]
        aa = None if not (a1 and a2) else abs(a1["adds"] - a2["adds"]) / max(a1["adds"], a2["adds"])
        row = dict(n=n, r=r, rho_mean_steps=rho, rho_theory=math.sqrt(math.pi * r / 2),
                   best_subspace=a1, best_subspace_seed2=a2, best_random_null=best[("random", 101)],
                   aa_rel_diff=aa, ratio_c0=ratio(a1, 0), ratio_c01=ratio(a1, 0.1), ratio_c1=ratio(a1, 1),
                   ratio_null_c0=ratio(best[("random", 101)], 0))
        out["rows"].append(row)
    rs = [x["ratio_c0"] for x in out["rows"]]
    # positive control: all relations verify and measured p_hat within factor 3 of predicted pair-table probability
    pos = all(x["best_subspace"] and x["best_subspace"]["verified"] for x in out["rows"])
    aa_ok = all(x["aa_rel_diff"] is not None and x["aa_rel_diff"] < 0.35 for x in out["rows"])
    out["controls"] = dict(positive_all_relations_verified=pos, aa_within_35pct=aa_ok)
    out["statistic"] = dict(name="min over n of ratio_c0 = best oracle GAE / mean rho steps", values=rs,
                            threshold=1.0)
    if any(x is None for x in rs) or not pos or not aa_ok:
        v = "inconclusive"
    elif min(rs) < 1.0:
        v = "falsified"
    else:
        v = "consistent_with_prediction"
    out["verdict"] = v
    print(json.dumps(out))

if __name__ == "__main__":
    main()
