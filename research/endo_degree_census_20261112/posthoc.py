#!/usr/bin/env python3
"""POST HOC (written after the census data were seen; not part of the frozen protocol).

Exact probabilities for a uniformly random curve over F_p (same derivation as predict.py):
  P(End(E) = O_K, d_K = d)   : the c = 1 term only,        2*h(d)/(w(d)/2) per (t,f);
  P(|D_E| <= 44), D_E = d c^2: orders O of conductor c | f with |d| c^2 <= 44,
        2*h(O)/(w(O)/2) per (t,f), h(O) = h(d) c prod_{q|c}(1-(d/q)/q) / (w(d)/2)  (c > 1).
Compares the crater probability for the class-number-one d_K with the census's
observed crater counts (decidable only for h = 1).
"""
import json, math, random, sys
sys.path.insert(0, ".")
import predict as P

H1 = [-3, -4, -7, -8, -11, -19, -43]
def term(d, c, hd, w2):
    t = hd / w2 if c == 1 else hd * c
    if c > 1:
        n, q = c, 2
        fs = []
        while q * q <= n:
            if n % q == 0:
                fs.append(q)
                while n % q == 0: n //= q
            q += 1
        if n > 1: fs.append(n)
        for q in fs: t *= 1 - P.kron(d, q) / q
        t /= w2   # unit index w(d)/2 for the order of conductor c > 1 (w(O)/2 = 1)
    return t

def probs(p):
    crater_all = crater_h1 = de44 = 0.0
    for d in P.DISCS:
        hd = P.h(d); w2 = 3 if d == -3 else (2 if d == -4 else 1)
        f = 1
        while -d * f * f < 4 * p:
            s = 4 * p + d * f * f
            t = math.isqrt(s)
            if t * t == s and t > 0:
                crater_all += 2 * term(d, 1, hd, w2)
                if d in H1: crater_h1 += 2 * term(d, 1, hd, w2)
                c = 1
                while -d * c * c <= 44:
                    if f % c == 0:
                        de44 += 2 * term(d, c, hd, w2)
                    c += 1
            f += 1
    n = 2 * p
    return crater_all / n, crater_h1 / n, de44 / n

if __name__ == "__main__":
    census = json.load(open("census.json"))
    obs = {int(x["name"].split(":")[2]): x for x in census["populations"] if x["name"].startswith("c:null:")}
    rng = random.Random(20261113)
    out = {}
    for bits, nprimes in ((12, 300), (16, 300), (20, 150), (24, 60), (28, 30), (32, 12)):
        acc = [0.0, 0.0, 0.0]
        for _ in range(nprimes):
            while True:
                p = rng.randrange(1 << (bits - 1), 1 << bits) | 1
                if P.is_prime(p): break
            for i, v in enumerate(probs(p)): acc[i] += v / nprimes
        o = obs[bits]
        n = o["ordinary"]
        row = {"P_crater_all_15_d": acc[0], "P_crater_h1": acc[1], "P_DE_le_44": acc[2],
               "observed_crater_h1": o["crater_true"], "n_ordinary": n,
               "expected_crater_h1_hits": acc[1] * n, "P_class_le_44_census_expected": o["expected_frac44"]}
        out[bits] = row
        print(bits, {k: (round(v, 8) if isinstance(v, float) else v) for k, v in row.items()}, flush=True)
    json.dump(out, open("posthoc.json", "w"), indent=1)
