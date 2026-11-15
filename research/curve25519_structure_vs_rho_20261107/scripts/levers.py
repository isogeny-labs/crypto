#!/usr/bin/env python3
"""Exact checks C1-C8 of PROTOCOL.md on the k = 20 Curve25519-family miniature.

Pure Python, exact arithmetic. Writes data/levers.json. Seed 20261107.
The Monte Carlo (C5) draws uniform group elements as a*G + b*Q through a table of the
enumerated subgroup (an oracle shortcut for computing a*G + b*Q); the class key is computed
from real coordinates, and every recovered logarithm is checked against Q.
"""
import json, math, random, sys, time
from pathlib import Path

p, A, r = 1048573, 926, 131203
SEED = 20261107
OUT = Path(__file__).resolve().parent.parent / "data" / "levers.json"


def inv(a):
    return pow(a, p - 2, p)


def chi(a):
    a %= p
    if a == 0:
        return 0
    return 1 if pow(a, (p - 1) // 2, p) == 1 else -1


def sqrt_mod(a):  # p = 5 mod 8 (Atkin)
    a %= p
    if a == 0:
        return 0
    b = pow(2 * a, (p - 5) // 8, p)
    i = 2 * a * b * b % p
    s = a * b * (i - 1) % p
    return s if s * s % p == a else None


def count(a2, a4):  # y^2 = x^3 + a2 x^2 + a4 x
    return p + 1 + sum(chi(x * (x * x + a2 * x + a4)) for x in range(p))


O = None


def add(P, Q, a2=A):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        l = (3 * x1 * x1 + 2 * a2 * x1 + 1) * inv(2 * y1) % p
    else:
        l = (y2 - y1) * inv(x2 - x1) % p
    x3 = (l * l - a2 - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)


def neg(P):
    return None if P is None else (P[0], (-P[1]) % p)


def mul(k, P):
    R = None
    while k:
        if k & 1:
            R = add(R, P)
        P = add(P, P)
        k >>= 1
    return R


def rand_point(rng):
    while True:
        x = rng.randrange(1, p)
        y = sqrt_mod(x * (x * x + A * x + 1))
        if y is not None and y != 0:
            return (x, y)


def factor(n):
    f, d = {}, 2
    while d * d <= n:
        while n % d == 0:
            f[d] = f.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        f[n] = f.get(n, 0) + 1
    return f


def is_prime(n):
    return n > 1 and factor(n) == {n: 1}


def main():
    rng = random.Random(SEED)
    res = {"curve": {"p": p, "A": A, "r": r}, "seed": SEED, "checks": {}}
    t0 = time.time()

    # C1 point counts
    nE = count(A, 1)
    nT = 2 * p + 2 - nE
    two_tors = [x for x in range(p) if x * (x * x + A * x + 1) % p == 0]
    v2 = (nE & -nE).bit_length() - 1
    res["checks"]["C1"] = {
        "#E": nE, "#E==8r": nE == 8 * r, "r_prime": is_prime(r),
        "rational_2torsion_x": two_tors, "v2(#E)": v2,
        "two_part_cyclic": len(two_tors) == 1,
        "#twist": nT, "twist/4": nT // 4, "twist/4_prime": nT % 4 == 0 and is_prime(nT // 4),
        "gcd(r, #twist)": math.gcd(r, nT),
    }
    T0 = (0, 0)

    # C8 j-invariant
    jE = 256 * pow(A * A - 3, 3, p) * inv(A * A - 4) % p
    res["checks"]["C8"] = {"j(E)": jE, "j_not_0_1728": jE not in (0, 1728 % p)}

    # C2 x(P+T0) = 1/x(P)
    ok = 0
    for _ in range(10000):
        P = rand_point(rng)
        S = add(P, T0)
        ok += S is not None and S[0] == inv(P[0])
    res["checks"]["C2"] = {"samples": 10000, "x(P+T0)==1/x(P)": ok}

    # generator of the order-r subgroup, and of the 2-part
    while True:
        G = mul(8, rand_point(rng))
        if G is not None and mul(r, G) is None:
            break
    while True:
        T8 = mul(r, rand_point(rng))
        if mul(4, T8) is not None:
            break  # order exactly 8
    E8 = [None]
    for _ in range(7):
        E8.append(add(E8[-1], T8))
    assert E8[4] == T0 or E8[4] == (0, 0)

    # enumerate <G>
    pts = [None, G]
    for _ in range(r - 2):
        pts.append(add(pts[-1], G))
    assert add(pts[-1], G) is None
    X = {P[0] for P in pts[1:]}
    Xinv = {inv(x) for x in X}
    # images under translation by T0, computed by real additions (not by assuming C2)
    XT0 = {add(P, T0)[0] for P in pts[1:]}
    res["checks"]["C3"] = {
        "|X(<G>)|": len(X), "|X ∩ 1/X|": len(X & Xinv), "|X ∩ x(<G>+T0)|": len(X & XT0),
        "x(<G>+T0) == 1/X": XT0 == Xinv,
        "torsion_x_in_X": [x for x in (0, 1, p - 1) if x in X],
    }
    # other torsion translations: x(P+T) for T of order 4 and 8, against X(<G>)
    tr = {}
    for i in (1, 2, 3):
        Xi = {add(P, E8[i])[0] for P in pts[1:2000]}
        tr["T8^%d" % i] = len(Xi & X)
    res["checks"]["C3"]["sample_2000_|x(P+T)∩X| for T in E[8]"] = tr

    # C4 exact class counts
    neg_classes = len(X)  # nonzero classes of {±P} on <G>
    xi_GT = {min(x, inv(x)) for x in X}  # {±P, ±P+T0}, P in <G>\{O}
    res["checks"]["C4"] = {
        "neg_on_G_nonzero_classes": neg_classes, "(r-1)/2": (r - 1) // 2,
        "xi_on_G+T0_classes_of_size4": len(xi_GT),
        "xi_classes_on_G_alone": len({min(x, inv(x)) for x in X}),
        "note": "xi = min(x, 1/x): on <G> alone it splits nothing and merges nothing (C3)",
    }

    # C5 Monte Carlo: draws to the first usable key collision
    d = rng.randrange(1, r)
    Q = pts[d]

    def elem(a, b):
        return pts[(a + b * d) % r]  # oracle shortcut for a*G + b*Q

    def key_plain(P):
        return P

    def key_neg(P):
        return P[0]

    def key_xi(P):
        x = P[0]
        return min(x, inv(x)) if x else ("T0",)

    def key_e8(P):
        best = None
        for T in E8:
            S = add(P, T)
            if S is None:
                continue
            best = S[0] if best is None else min(best, S[0])
        return best

    def trial(kind):
        seen = {}
        n = 0
        while True:
            a, b = rng.randrange(r), rng.randrange(r)
            P = elem(a, b)
            if P is None:
                continue
            if kind == "xi":
                if rng.getrandbits(1):
                    P = add(P, T0)
                k = key_xi(P)
            elif kind == "e8":
                P = add(P, E8[rng.randrange(8)])
                k = key_e8(P)
            elif kind == "neg":
                k = key_neg(P)
            else:
                k = key_plain(P)
            n += 1
            if k in seen:
                a2, b2 = seen[k]
                for s in (1, -1):  # a + b d = s (a2 + b2 d)
                    den = (b - s * b2) % r
                    if den:
                        dd = (s * a2 - a) * pow(den, -1, r) % r
                        if pts[dd] == Q:  # [dd]G = Q check
                            return n
                continue  # useless collision (same element drawn twice)
            seen[k] = (a, b)

    mc = {}
    for kind, trials in (("plain", 2000), ("neg", 2000), ("xi", 2000), ("e8", 600)):
        v = [trial(kind) for _ in range(trials)]
        m = sum(v) / len(v)
        sd = math.sqrt(sum((x - m) ** 2 for x in v) / (len(v) - 1))
        mc[kind] = {"trials": trials, "mean_over_sqrt_r": m / math.sqrt(r),
                    "se_over_sqrt_r": sd / math.sqrt(len(v)) / math.sqrt(r)}
    mc["model"] = {"plain": math.sqrt(math.pi / 2), "folded_r/2_classes": math.sqrt(math.pi / 4)}
    res["checks"]["C5"] = mc

    # C6 endomorphisms
    t = p + 1 - nE
    D = t * t - 4 * p
    lam = (1 - t // 2) * pow(22, -1, r) % r  # omega = (pi - t/2)/22, pi acts as 1 on <G>
    rf = factor(r - 1)

    def order(x):
        o = r - 1
        for q in rf:
            while o % q == 0 and pow(x, o // q, r) == 1:
                o //= q
        return o

    gen = next(g for g in range(2, r) if order(g) == r - 1)
    # E[2] is not rational (C1: one rational 2-torsion point), so pi is not scalar mod 2 and
    # (pi + 525)/2 is not an endomorphism: End(E) lies in Z + 2*Z*sqrt(-1597). step = 1 grants
    # O_K (impossible, kept as the most optimistic bound), 2 the largest order End can be,
    # 22 is Z[pi].
    small = []
    for step in (1, 2, 22):
        for m in sorted(m for m in range(3, 13) if (r - 1) % m == 0):
            best = None
            for k in range(1, m):
                if math.gcd(k, m) != 1:
                    continue
                mu = pow(gen, (r - 1) // m * k, r)
                for sgn in (1, -1):
                    bb = step
                    while best is None or 1597 * bb * bb < best[0]:
                        a = (mu - sgn * bb * lam) % r
                        a = a - r if a > r // 2 else a
                        nrm = a * a + 1597 * bb * bb
                        if best is None or nrm < best[0]:
                            best = (nrm, a, sgn * bb, mu)
                        bb += step
            small.append({"b_multiple_of": step, "m": m, "min_degree": best[0], "a": best[1],
                          "b": best[2], "min_degree/sqrt_r": best[0] / math.sqrt(r),
                          "degree_factored": factor(best[0])})
    res["checks"]["C6"] = {
        "t": t, "t^2-4p": D, "D/(4*22^2)": D / (4 * 22 * 22),
        "lambda_omega": lam, "lambda^2+1597 mod r": (lam * lam + 1597) % r,
        "order(lambda_omega)": order(lam), "r-1": r - 1, "r-1_factored": rf,
        "least_nonscalar_degree_lower_bound": 1597,
        "least_nonscalar_degree/sqrt_r": 1597 / math.sqrt(r),
        "small_order_eigenvalues": small,
        "note": "degree = a^2 + 1597 b^2 of a + b*sqrt(-1597); b_multiple_of 1 = O_K (excluded, cyclic 2-part), 2 = largest possible End, 22 = Z[pi]",
    }

    # C7 2-isogenous curve E' = E/<T0>: Y^2 = X(X^2 - 2A X + A^2 - 4)
    a2, a4 = (-2 * A) % p, (A * A - 4) % p
    nE2 = count(a2, a4)
    tors2 = [x for x in range(p) if x * (x * x + a2 * x + a4) % p == 0]
    j2 = 256 * pow(a2 * a2 - 3 * a4, 3, p) * inv(a4 * a4 * (a2 * a2 - 4 * a4)) % p
    res["checks"]["C7"] = {"#E'": nE2, "#E'==8r": nE2 == 8 * r, "rational_2torsion_x": tors2,
                           "full_2torsion": len(tors2) == 3, "j(E')": j2,
                           "j_not_0_1728": j2 not in (0, 1728 % p)}
    res["seconds"] = round(time.time() - t0, 1)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(res, indent=1, default=str))
    json.dump(res, sys.stdout, indent=1, default=str)


if __name__ == "__main__":
    main()
