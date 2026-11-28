#!/usr/bin/env python3
"""CT119: finite-residue witness for the invariant-field root-selection gate.
No external packages, no precomputed source scalar decoder, no use of assert.
This is a structural control, NOT a DLP transfer or a speed measurement.
"""
import json

SOURCE_PRIME = 59
SOURCE_ROOTS = (19, 20, 43, 46, 22)
SOURCE_POLY = (27, 19, 45, 31, 27, 1)
BINARY_POLYS = ((9, 6, 3, 1, 2, 1),
                (4, 10, 15, 7, 2, 1),
                (15, 15, 13, 6, 2, 1))
BINARY_ROOTS = (2, 17, 4956, 65536, 4697)


def require(ok, message):
    if not ok:
        raise RuntimeError(message)


def gf2n_mul(a, b, modulus, degree):
    limit = 1 << degree
    r = 0
    while b:
        if b & 1:
            r ^= a
        b >>= 1
        a <<= 1
        if a & limit:
            a ^= modulus
    return r


def gf2n_pow(a, n, modulus, degree):
    r = 1
    while n:
        if n & 1:
            r = gf2n_mul(r, a, modulus, degree)
        a = gf2n_mul(a, a, modulus, degree)
        n >>= 1
    return r


def fmul(a, b):
    return gf2n_mul(a, b, 0b10011, 4)


def trim(p):
    t = list(p)
    while t and t[-1] == 0:
        t.pop()
    return t


def poly_mod(a, b):
    a = trim(a)
    b = trim(b)
    require(bool(b), "polynomial zero divisor")
    scale = gf2n_pow(b[-1], 14, 0b10011, 4)
    while len(a) >= len(b):
        t = fmul(a[-1], scale)
        off = len(a) - len(b)
        for i, v in enumerate(b):
            a[off + i] ^= fmul(t, v)
        a = trim(a)
    return a


def poly_mul_mod(a, b, modulus):
    c = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            c[i + j] ^= fmul(x, y)
    return poly_mod(c, modulus)


def poly_pow_mod(a, n, modulus):
    acc = [1]
    while n:
        if n & 1:
            acc = poly_mul_mod(acc, a, modulus)
        a = poly_mul_mod(a, a, modulus)
        n >>= 1
    return acc


def poly_gcd(a, b):
    while b:
        a, b = b, poly_mod(a, b)
    return trim(a)


def irreducible_degree_five(p):
    # Rabin's criterion: 5 is prime, so only proper prime divisor is 5.
    # gcd(f,X^(q)-X)=1 and X^(q^5) == X modulo f (q=16).
    x = [0, 1]
    f16 = poly_pow_mod(x, 16, p)
    test = [0] * max(2, len(f16))
    for i, c in enumerate(f16):
        test[i] ^= c
    test[1] ^= 1
    return len(poly_gcd(p, trim(test))) == 1 and poly_pow_mod(x, 16**5, p) == x


def source_poly_from_roots():
    c = [1]
    for root in SOURCE_ROOTS:
        next_c = [0] * (len(c) + 1)
        for i, value in enumerate(c):
            next_c[i] = (next_c[i] - value * root) % SOURCE_PRIME
            next_c[i + 1] = (next_c[i + 1] + value) % SOURCE_PRIME
        c = next_c
    return c


def binary_poly_from_roots():
    c = [1]
    for root in BINARY_ROOTS:
        next_c = [0] * (len(c) + 1)
        for i, value in enumerate(c):
            next_c[i] ^= gf2n_mul(value, root, 0x130027, 20)
            next_c[i + 1] ^= value
        c = next_c
    return c


def sum_xor(items):
    out = 0
    for item in items:
        out ^= item
    return out


def main():
    require(len(set(SOURCE_ROOTS)) == 5, "distinct source roots")
    require(source_poly_from_roots() == list(SOURCE_POLY), "source relative quintic")
    require(all(irreducible_degree_five(p) for p in BINARY_POLYS), "3 binary quintics")
    mutated = (0,) + BINARY_POLYS[0][1:]
    require(not irreducible_degree_five(mutated), "reducible control was accepted")
    mask = 0x130027
    s = 0
    for x in BINARY_ROOTS:
        s ^= x
    require(gf2n_pow(s, 4, mask, 20) ^ s ^ 1 == 0, "trace generator s")
    require(gf2n_pow(s, 16, mask, 20) == s, "s is F16-valued")

    def embed(v):
        return sum_xor(gf2n_pow(s, i, mask, 20) for i in range(4) if (v >> i) & 1)

    actual = binary_poly_from_roots()
    expected = [embed(v) for v in BINARY_POLYS[0]]
    require(actual == expected, "fixed binary residue coefficients")
    orbit = [gf2n_pow(2, 16**j, mask, 20) for j in range(5)]
    require(len(set(orbit)) == 5 and set(orbit) == set(BINARY_ROOTS), "Frobenius orbit")
    require(gf2n_pow(2, 16**5, mask, 20) == 2, "Frobenius period")
    record = {
        "binary_F16_polynomials": [list(v) for v in BINARY_POLYS],
        "binary_quintic_irreducible": [True, True, True],
        "fixed_binary_F16_generator_mask": s,
        "fixed_binary_orbit_masks": orbit,
        "fixed_binary_product_coeff_masks": actual,
        "mutation_rejected": True,
        "source_modulus": SOURCE_PRIME,
        "source_quintic_coeff": source_poly_from_roots(),
        "source_root_count": 5,
        "scope": "invariant-base-field root exclusion only; not a cost bound or DLP transfer"
    }
    print(json.dumps(record, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
