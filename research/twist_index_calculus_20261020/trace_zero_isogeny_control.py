#!/usr/bin/env python3
"""Exact control where a degree-3 map transports ECDLP to a quadratic twist.

For p=2 mod 3, E: y^2=x^3+1 has trace zero.  The rational 3-isogeny
E -> y^2=x^3-27, followed by a rational scaling, reaches its nontrivial
quadratic twist d*y^2=x^3+1.  These are tiny research controls only.
"""

import json
from collections import Counter


PRIMES = (101, 233)
BASE_SIZES = (8, 16, 24)


def legendre(v, p):
    if v % p == 0:
        return 0
    z = pow(v % p, (p - 1) // 2, p)
    return -1 if z == p - 1 else z


def roots_table(p):
    roots = {}
    for y in range(p):
        roots.setdefault(y * y % p, []).append(y)
    return roots


def rhs(x, p):
    return (x * x * x + 1) % p


def add(P, Q, p, d):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if P == Q:
        slope = 3 * x1 * x1 * pow(2 * d * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow((x2 - x1) % p, -1, p) % p
    x3 = (d * slope * slope - x1 - x2) % p
    y3 = (slope * (x1 - x3) - y1) % p
    assert d * y3 * y3 % p == rhs(x3, p)
    return (x3, y3)


def mul(k, P, p, d):
    result = None
    while k:
        if k & 1:
            result = add(result, P, p, d)
        P = add(P, P, p, d)
        k >>= 1
    return result


def points(p, d, roots):
    inv_d = pow(d, -1, p)
    return [None] + [(x, y) for x in range(p)
                     for y in roots.get(rhs(x, p) * inv_d % p, [])]


def f3(x1, x2, x3, p):
    return ((x1 - x2) ** 2 * x3 ** 2
            - 2 * ((x1 + x2) * x1 * x2 + 2) * x3
            + (x1 * x2) ** 2 - 4 * (x1 + x2)) % p


def largest_prime_factor(n):
    largest = 1
    z = 2
    while z * z <= n:
        while n % z == 0:
            largest = z
            n //= z
        z += 1
    return max(largest, n)


def relation_yield(target_xs, base_xs, p, d, roots):
    hits = 0
    covered = 0
    inv_d = pow(d, -1, p)
    for rx in target_xs:
        n = 0
        for i, x1 in enumerate(base_xs):
            for x2 in base_xs[i:]:
                if f3(x1, x2, rx, p) != 0:
                    continue
                assert any(
                    (S is not None and S[0] == rx)
                    for y1 in roots[rhs(x1, p) * inv_d % p]
                    for y2 in roots[rhs(x2, p) * inv_d % p]
                    for S in [add((x1, y1), (x2, y2), p, d)]
                )
                n += 1
        hits += n
        covered += bool(n)
    return {"target_abscissas": len(target_xs),
            "polynomial_hits": hits, "covered_abscissas": covered,
            "candidate_pairs_per_target": len(base_xs) * (len(base_xs) + 1) // 2}


def experiment(p):
    assert p % 3 == 2
    d = next(z for z in range(2, p) if legendre(z, p) == -1)
    roots = roots_table(p)
    E = points(p, 1, roots)
    T = points(p, d, roots)
    assert len(E) == len(T) == p + 1
    target_coefficient = -27 * pow(d, -3, p) % p
    u = next(z for z in range(1, p) if pow(z, 6, p) == target_coefficient)
    inv_u2 = pow(u * u, -1, p)
    inv_u3 = pow(u * u * u, -1, p)
    inv_d = pow(d, -1, p)
    inv_d2 = pow(d * d, -1, p)

    def phi(P):
        if P is None or P[0] == 0:
            return None
        x, y = P
        x3 = pow(x, 3, p)
        X = (x3 + 4) * pow(x * x, -1, p) % p
        Y = y * (x3 - 8) * pow(x3, -1, p) % p
        result = (X * inv_u2 * inv_d % p, Y * inv_u3 * inv_d2 % p)
        assert d * result[1] * result[1] % p == rhs(result[0], p)
        return result

    image = {phi(P) for P in E}
    kernel = [P for P in E if phi(P) is None]
    assert len(kernel) == 3
    assert len(image) == (p + 1) // 3
    homomorphism_checks = 0
    for P in E:
        for Q in E:
            assert phi(add(P, Q, p, 1)) == add(phi(P), phi(Q), p, d)
            homomorphism_checks += 1

    r = largest_prime_factor(p + 1)
    assert r > 3
    G = next(R for P in E if P is not None
             for R in [mul((p + 1) // r, P, p, 1)] if R is not None)
    assert mul(r, G, p, 1) is None
    transported_G = phi(G)
    assert transported_G is not None
    assert mul(r, transported_G, p, d) is None
    scalar = 7
    Q = mul(scalar, G, p, 1)
    assert phi(Q) == mul(scalar, transported_G, p, d)

    target_e_xs = sorted({mul(k, G, p, 1)[0] for k in range(1, r)})
    target_t_xs = sorted({mul(k, transported_G, p, d)[0] for k in range(1, r)})
    assert len(target_e_xs) == len(target_t_xs) == (r - 1) // 2
    e_xs = sorted({P[0] for P in E if P is not None and P[1] != 0})
    t_xs = sorted({P[0] for P in T if P is not None and P[1] != 0})
    comparison = {}
    for base_size in BASE_SIZES:
        assert len(e_xs) >= base_size and len(t_xs) >= base_size
        comparison[str(base_size)] = {
            "E": relation_yield(target_e_xs, e_xs[:base_size], p, 1, roots),
            "transported_twist": relation_yield(target_t_xs,
                                                 t_xs[:base_size], p, d, roots),
        }
    return {"p": p, "nonsquare_d": d, "sixth_root_u": u,
            "E_order": len(E), "twist_order": len(T),
            "isogeny_degree": len(kernel), "image_size": len(image),
            "full_group_homomorphism_checks": homomorphism_checks,
            "target_prime_order": r, "ECDLP_test_scalar": scalar,
            "target_and_transport_verified": True,
            "width_two_yield_on_transported_prime_order_targets": comparison}


def main():
    print(json.dumps({"protocol": "exact p=2 mod 3 j=0 trace-zero isogeny"
                     " and fixed first-k abscissa factor bases; no timing claim",
                      "cases": [experiment(p) for p in PRIMES]},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
