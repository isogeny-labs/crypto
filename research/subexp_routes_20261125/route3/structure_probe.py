#!/usr/bin/env python3
"""Structural probe: does the m=3 chain admit per-level pruning?

Route 1 showed the tower membership system is decided by an explicit tree at
Theta(2^T) field mults per decision (m=2).  For m=3 the chain is two S3 links
with a *free* intermediate R1 = P1 + P2.  Pruning would require x(R1) to live in
a small set, so that the second link is cheap for every intermediate.  That can
only happen if the factor base is (close to) a coset of a subgroup: then
V + V = 2R + H is again a coset of size |V|.

This script measures, on public 13-bit toy curves and with pure integer EC
arithmetic:
  A. a generic 18-abscissa base (the shape of prime-isogeny: 18 x-values on E
     with no group structure on E):  |x(V+V)|, |x(V+-V)|, |x(V+V+V)|;
  B. the coset control (V = R + H, |H| = 32):  the same set sizes;
  C. relation probability per target for m=2 and m=3 on the generic base
     (compare with the binary's F4 hit rate 0.128 at 18 abscissae, 13 bits).

Only counting is claimed; no timing.
"""
import itertools
import json
import random
import sys

P_GENERIC = 8191          # 2^13 - 1, prime
P_COSET = 7873            # = 32*246 + 1, prime, so a 32-torsion point can exist


def inv(a, p):
    return pow(a, p - 2, p)


def ec_add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if P == Q:
        m = (3 * x1 * x1 + a) * inv(2 * y1, p) % p
    else:
        m = (y2 - y1) * inv(x2 - x1, p) % p
    x3 = (m * m - x1 - x2) % p
    return (x3, (m * (x1 - x3) - y1) % p)


def ec_mul(k, P, a, p):
    R = None
    while k:
        if k & 1:
            R = ec_add(R, P, a, p)
        P = ec_add(P, P, a, p)
        k >>= 1
    return R


def curve_order(a, b, p):
    """#E by counting x with a square RHS (p < 2^14, so this is cheap)."""
    n = 1
    for x in range(p):
        rhs = (x * x * x + a * x + b) % p
        if rhs == 0:
            n += 1
        elif pow(rhs, (p - 1) // 2, p) == 1:
            n += 2
    return n


def lift(x, a, b, p):
    rhs = (x * x * x + a * x + b) % p
    if rhs == 0:
        return (x, 0)
    if pow(rhs, (p - 1) // 2, p) != 1:
        return None
    y = pow(rhs, (p + 1) // 4, p) if p % 4 == 3 else None
    if y is None or y * y % p != rhs:
        # Tonelli-Shanks
        q, s = p - 1, 0
        while q % 2 == 0:
            q //= 2
            s += 1
        z = 2
        while pow(z, (p - 1) // 2, p) != p - 1:
            z += 1
        m, c, t, r = s, pow(z, q, p), pow(rhs, q, p), pow(rhs, (q + 1) // 2, p)
        while t != 1:
            i, t2 = 0, t
            while t2 != 1:
                t2 = t2 * t2 % p
                i += 1
            bb = pow(c, 1 << (m - i - 1), p)
            m, c, t, r = i, bb * bb % p, t * bb * bb % p, r * bb % p
        y = r
    return (x, y if y * y % p == rhs else p - y)


def generic_base(p, a, b, n_abscissae):
    xs = []
    x = 1
    while len(xs) < n_abscissae:
        if lift(x, a, b, p) is not None:
            xs.append(x)
        x += 1
    return [(x, lift(x, a, b, p)[1]) for x in xs]


def sumsets(points, a, p):
    """x-values of all +-pair sums (i != j and i == j) and of +-triples."""
    n = len(points)
    pairs = {}
    for i, j in itertools.combinations_with_replacement(range(n), 2):
        for sg in ((1, 1), (1, -1)):
            P = points[i]
            Q = points[j] if sg[1] == 1 else (points[j][0], (-points[j][1]) % p)
            R = ec_add(P, Q, a, p)
            if R is not None:
                pairs.setdefault(R[0], 0)
                pairs[R[0]] += 1
    triples = set()
    for i, j, k in itertools.combinations_with_replacement(range(n), 3):
        for s1, s2, s3 in itertools.product((1, -1), repeat=3):
            def signed(P, s):
                return P if s == 1 else (P[0], (-P[1]) % p)
            R = ec_add(ec_add(signed(points[i], s1), signed(points[j], s2), a, p),
                       signed(points[k], s3), a, p)
            if R is not None:
                triples.add(R[0])
    return pairs, triples


def main():
    out = {}
    rng = random.Random(20261185)

    # ---- A: generic 18-abscissa base on the 13-bit toy curve ----
    a, b = 1, 1
    p = P_GENERIC
    V = generic_base(p, a, b, 18)
    V = V + [(x, (-y) % p) for (x, y) in V]          # 18 columns x 2 signs = 36 points,
    # exactly the binary's "36 points / 18 columns / 2.0 pts per column" base shape
    pairs, triples = sumsets(V, a, p)
    # relation probabilities on random targets
    trials = 4000
    hit2 = hit3 = 0
    for _ in range(trials):
        x = rng.randrange(1, p)
        if x in pairs:
            hit2 += 1
        if x in triples:
            hit3 += 1
    out["generic_base"] = {
        "p": p, "curve": f"y^2=x^3+{a}x+{b}", "base_abscissae": len(V),
        "distinct_pair_sum_x": len(pairs),
        "ratio_pairs_to_abscissae2": len(pairs) / 18 ** 2,
        "distinct_triple_sum_x": len(triples),
        "ratio_triples_to_abscissae3": len(triples) / 18 ** 3,
        "note": "ratios are against the 18 *abscissae* (V is stored as 36 points)",
        "relation_prob_m2": hit2 / trials,
        "relation_prob_m3": hit3 / trials,
        "predicted_m2_generic": 2 * len(V) ** 2 / p,
        "predicted_m3_generic": min(1.0, 4 * len(V) ** 3 / p),
    }

    # ---- B: coset control, V = R + H with |H| = 32 ----
    p2 = P_COSET
    found = None
    for bb in range(1, 200):
        if (4 * 1 + 27 * bb * bb) % p2 == 0:
            continue
        n = curve_order(1, bb, p2)
        if n % 32 == 0:
            found = (1, bb, n)
            break
    if found:
        a2, b2, n2 = found
        # find a point of order 32
        def order_of(Q):
            o = n2
            for prime in (2, 31):
                while o % prime == 0 and ec_mul(o // prime, Q, a2, p2) is None:
                    o //= prime
            return o

        P32 = None
        R = None
        for x in range(1, 400):
            Q = lift(x, a2, b2, p2)
            if Q is None:
                continue
            o = order_of(Q)
            if o % 32 == 0:
                H = ec_mul(o // 32, Q, a2, p2)
                if H is not None and ec_mul(16, H, a2, p2) is not None:
                    P32 = H
                    R = ec_add(Q, Q, a2, p2) if R is None else R
                    break
            if R is None:
                R = Q
        coset = [ec_add(R, ec_mul(k, P32, a2, p2), a2, p2) for k in range(32)]
        coset = [Q for Q in coset if Q is not None]
        xs = {Q[0] for Q in coset}
        cpairs, ctriples = sumsets(coset, a2, p2)
        out["coset_control"] = {
            "p": p2, "curve": f"y^2=x^3 + x + {b2}", "curve_order": n2,
            "subgroup_order": len(coset), "distinct_coset_x": len(xs),
            "distinct_pair_sum_x": len(cpairs),
            "distinct_triple_sum_x": len(ctriples),
            "predicted_pair_sum_x_coset": len(xs),
        }

    print(json.dumps(out, indent=1, sort_keys=True))
    with open("structure_probe.json", "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)


if __name__ == "__main__":
    main()
