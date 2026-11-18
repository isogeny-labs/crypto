#!/usr/bin/env python3
"""CT205 independent F31 toy-cover arithmetic oracle. No Rust/Cantor dependencies.

Public finite-field census, not an ECDLP benchmark. [pcc:unverified]
"""
from math import gcd

P = 31
B = 3
SQUARES = {x * x % P for x in range(P)}


def quadratic_character(x):
    x %= P
    return 0 if x == 0 else 1 if x in SQUARES else -1


def elliptic_order(b):
    return 1 + sum(1 + quadratic_character(x**3 + b) for x in range(P))


def extension_multiply(x, y, nonsquare):
    a, b = x
    c, e = y
    return ((a * c + nonsquare * b * e) % P, (a * e + b * c) % P)


def extension_power(x, n, nonsquare):
    out = (1, 0)
    while n:
        if n & 1:
            out = extension_multiply(out, x, nonsquare)
        x = extension_multiply(x, x, nonsquare)
        n //= 2
    return out


def genus_two_order_f_p():
    # Monic split sextic: both infinite points are F_p-rational.
    return 2 + sum(1 + quadratic_character(u**6 + B) for u in range(P))


def genus_two_order_f_p2(nonsquare, mode):
    assert quadratic_character(nonsquare) == -1
    # Independent direct-square and norm/Legendre character evaluations.
    squares = None
    if mode == "direct":
        squares = {
            extension_multiply((a, b), (a, b), nonsquare)
            for a in range(P) for b in range(P)
        }
    total = 2
    for a in range(P):
        for b in range(P):
            x = extension_power((a, b), 6, nonsquare)
            f = ((x[0] + B) % P, x[1])
            if mode == "direct":
                char = 0 if f == (0, 0) else (1 if f in squares else -1)
            else:
                # In F_{p^2}, z is square iff norm(z) is square in F_p.
                norm = (f[0] * f[0] - nonsquare * f[1] * f[1]) % P
                char = quadratic_character(norm)
            total += 1 + char
    return total


def elliptic_add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    x1, y1 = a
    x2, y2 = b
    if x1 == x2 and (y1 + y2) % P == 0:
        return None
    if a == b:
        assert y1 != 0
        slope = 3 * x1 * x1 * pow(2 * y1, -1, P) % P
    else:
        slope = (y2 - y1) * pow((x2 - x1) % P, -1, P) % P
    x3 = (slope * slope - x1 - x2) % P
    return x3, (slope * (x1 - x3) - y1) % P


def main():
    assert P % 3 == 1
    e_order, companion_order = elliptic_order(B), elliptic_order(B * B)
    assert (e_order, companion_order) == (43, 39)
    assert gcd(e_order, companion_order) == 1
    n1 = genus_two_order_f_p()
    n2_direct = genus_two_order_f_p2(3, "direct")
    n2_norm = genus_two_order_f_p2(6, "norm")
    assert (n1, n2_direct, n2_norm) == (50, 916, 916)

    # Coefficients of genus-two Weil polynomial from N1,N2.
    a1 = P + 1 - n1
    twice_a2 = n2_direct - P * P - 1 + a1 * a1
    assert twice_a2 % 2 == 0
    a2 = twice_a2 // 2
    jac_order = 1 - (P + 1) * a1 + a2 + P * P
    t1 = P + 1 - e_order
    t2 = P + 1 - companion_order
    assert (a1, a2) == (t1 + t2, t1 * t2 + 2 * P) == (-18, 139)
    assert jac_order == e_order * companion_order == 1677

    # Maps C:(u,v) -> E_3:(u²,v), E_9:(3/u²,3v/u³) for u != 0.
    affine = 0
    for u in range(P):
        for v in range(P):
            if (v * v - u**6 - B) % P:
                continue
            affine += 1
            assert (v * v - (u * u)**3 - B) % P == 0
            if u:
                x, y = B * pow(u, -2, P) % P, B * v * pow(u, -3, P) % P
                assert (y * y - x**3 - B * B) % P == 0
    assert affine == 48

    # Source E(F31) generator has 43 distinct multiples, including infinity.
    g, cur, orbit = (1, 2), None, []
    for _ in range(e_order):
        assert cur not in orbit
        orbit.append(cur)
        cur = elliptic_add(cur, g)
    assert cur is None and len(orbit) == e_order
    assert len(orbit) == len({None} | {
        (x, y) for x in range(P) for y in range(P)
        if (y * y - x**3 - B) % P == 0
    })
    print("CT205 F31 oracle PASS: E_3=43, E_9=39, C(F31)=50,")
    print("  C(F31²)=916 by 2 independent Fp2 models; #Jac(C)(F31)=1677=43*39")
    print("  traces=-11,-7, gcd=1; generator order=43; 48 affine cover points map correctly")
    print("  Complementary 39-part cannot receive nonzero group map from the 43-part")
    print("  [pcc:unverified] No native Rust DLP solve or speedup measured")


if __name__ == "__main__":
    main()
