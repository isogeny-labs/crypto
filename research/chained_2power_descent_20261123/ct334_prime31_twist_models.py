#!/usr/bin/env python3
"""CT334: explicit Prime31 genus-two root-cycle and quadratic-twist fixtures.

Source-only finite-field factorization and 2-torsion subgroup-scheme census.
No isogeny, transfer, Cantor solver, timing experiment or PCC signature.
"""
import collections
import hashlib
import itertools
import json
import pathlib

P = (1 << 31) - 1

def trim(a):
    v = [x % P for x in a]
    while v and v[-1] == 0:
        v.pop()
    return v

def add(a, b, sign=1):
    return trim([(a[i] if i < len(a) else 0) +
                 sign * (b[i] if i < len(b) else 0)
                 for i in range(max(len(a), len(b)))])

def mul(a, b):
    if not a or not b:
        return []
    c = [0] * (len(a) + len(b) - 1)
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            c[i + j] = (c[i + j] + ai * bj) % P
    return trim(c)

def mod(a, b):
    a = trim(a)
    b = trim(b)
    if not b:
        raise ZeroDivisionError("zero polynomial")
    inv = pow(b[-1], P - 2, P)
    while a and len(a) >= len(b):
        c = a[-1] * inv % P
        shift = len(a) - len(b)
        for i, bi in enumerate(b):
            a[shift + i] = (a[shift + i] - c * bi) % P
        a = trim(a)
    return a

def pgcd(a, b):
    a, b = trim(a), trim(b)
    while b:
        a, b = b, mod(a, b)
    if not a:
        return []
    scale = pow(a[-1], P - 2, P)
    return trim([v * scale for v in a])

def ppow(base, exponent, modulus):
    v = [1]
    base = mod(base, modulus)
    while exponent:
        if exponent & 1:
            v = mod(mul(v, base), modulus)
        base = mod(mul(base, base), modulus)
        exponent >>= 1
    return v

def frobenius_powers(f):
    x = [0, 1]
    out = [mod(x, f)]
    for _ in range(len(f) - 1):
        out.append(ppow(out[-1], P, f))
    return out

def prime_divisors(n):
    divs = []
    for i in range(2, n + 1):
        if n % i == 0 and all(i % j != 0 for j in range(2, int(i ** .5) + 1)):
            divs.append(i)
    return divs

def irreducible_rabin(f):
    f = trim(f)
    d = len(f) - 1
    if d < 1 or f[-1] != 1:
        return False
    if d == 1:
        return True
    x = mod([0, 1], f)
    powers = frobenius_powers(f)
    if powers[d] != x:
        return False
    return all(pgcd(f, add(powers[d // q], x, -1)) == [1]
               for q in prime_divisors(d))

def irreducible_all_gcd(f):
    f = trim(f)
    d = len(f) - 1
    if d < 1 or f[-1] != 1:
        return False
    if d == 1:
        return True
    x = mod([0, 1], f)
    z = x
    for k in range(1, d + 1):
        z = ppow(z, P, f)
        if k <= d // 2 and pgcd(f, add(z, x, -1)) != [1]:
            return False
    return z == x

def derivative(f):
    return trim([i * f[i] for i in range(1, len(f))])

def partitions(n, least=1):
    if n == 0:
        yield ()
    else:
        for i in range(least, n + 1):
            for rest in partitions(n - i, i):
                yield (i,) + rest

def matchings(points):
    if not points:
        yield ()
        return
    x = points[0]
    for y in points[1:]:
        remain = tuple(z for z in points[1:] if z != y)
        for tail in matchings(remain):
            yield tuple(sorted(((x, y),) + tail))

MATCHINGS = tuple(sorted(set(matchings(tuple(range(6))))))

def permutation(part):
    out = list(range(6))
    at = 0
    for size in part:
        for i in range(size):
            out[at + i] = at + (i + 1) % size
        at += size
    return out

def map_matching(m, perm):
    return tuple(sorted(tuple(sorted((perm[a], perm[b]))) for a, b in m))

def kernel_profile(part):
    perm = permutation(part)
    unseen = set(MATCHINGS)
    lengths = []
    fixed = 0
    while unseen:
        seed = min(unseen)
        current = seed
        count = 0
        while current in unseen:
            unseen.remove(current)
            count += 1
            current = map_matching(current, perm)
        if current != seed:
            raise ValueError("noncyclic matching action")
        lengths.append(count)
        if count == 1:
            fixed += 1
    return fixed, min(lengths), sorted(lengths)

EXPECTED = {
 (1,1,1,1,1,1): 15, (1,1,1,1,2): 3, (1,1,2,2): 3,
 (1,1,1,3): 0, (2,2,2): 7, (1,2,3): 0,
 (1,1,4): 1, (3,3): 3, (2,4): 1, (1,5): 0, (6,): 1,
}

def product(factors):
    f = [1]
    for h in factors:
        f = mul(f, h)
    return f

def validate_model(factors, part, claimed_fixed=None):
    if len(factors) != len(part) or tuple(sorted(len(h) - 1 for h in factors)) != part:
        raise ValueError("factor degree partition")
    if len(set(tuple(h) for h in factors)) != len(factors):
        raise ValueError("repeated factor")
    if any(not irreducible_rabin(h) or not irreducible_all_gcd(h) for h in factors):
        raise ValueError("reducible factor")
    f = product(factors)
    if len(f) != 7 or f[-1] != 1 or pgcd(f, derivative(f)) != [1]:
        raise ValueError("not squarefree monic sextic")
    fixed, extension, lengths = kernel_profile(part)
    if claimed_fixed is not None and claimed_fixed != fixed:
        raise ValueError("wrong Frobenius kernel count")
    if fixed != EXPECTED[part]:
        raise ValueError("CT333/A1 disagreement")
    return f, fixed, extension, lengths

def choose_factors():
    pool = {1: [[(-a) % P, 1] for a in range(1, 7)]}
    needs = {2:3, 3:2, 4:1, 5:1, 6:1}
    for degree, need in needs.items():
        selected = []
        for a in range(1, 65):
            for b in range(1, 65):
                f = [b, a] + [0] * (degree - 2) + [1]
                if irreducible_rabin(f) and irreducible_all_gcd(f):
                    selected.append(f)
                    if len(selected) == need:
                        break
            if len(selected) == need:
                break
        if len(selected) != need:
            raise ValueError("bounded irreducible search failed at degree " + str(degree))
        pool[degree] = selected
    return pool

def rejects(test):
    try:
        test()
    except ValueError:
        return True
    raise ValueError("mutation accepted")

def main():
    if len(MATCHINGS) != 15 or pow(P-1, (P-1)//2, P) != P-1:
        raise ValueError("model field or matching count incorrect")
    pool = choose_factors()
    parts = tuple(partitions(6))
    if len(parts) != 11 or set(parts) != set(EXPECTED):
        raise ValueError("partition count")
    rows = []
    for part in parts:
        indices = collections.Counter()
        factors = []
        for degree in part:
            factors.append(pool[degree][indices[degree]])
            indices[degree] += 1
        f, fixed, extension, lengths = validate_model(factors, part)
        if part in ((1,1,1,1,1,1),(1,5),(3,3)):
            if product(factors) != f:
                raise ValueError("product changed")
        rows.append({
            "cycle_type": list(part),
            "f_coefficients_ascending": f,
            "twist_minus_f_coefficients_ascending": [(-v) % P for v in f],
            "factors_coefficients_ascending": factors,
            "all_factor_rabin_pass": True,
            "all_factor_every_lower_gcd_pass": True,
            "squarefree": True,
            "rational_richelot_kernels": fixed,
            "minimum_kernel_extension_degree": extension,
            "kernel_orbit_lengths": lengths,
            "twist_rational_kernels": fixed,
        })
    good_part = (1,1,2,2)
    good_factors = [pool[1][0], pool[1][1], pool[2][0], pool[2][1]]
    validate_model(good_factors, good_part)
    negative = {
        "repeated_irreducible_factor": rejects(lambda: validate_model(
            [pool[1][0], pool[1][0], pool[2][0], pool[2][1]], good_part)),
        "reducible_quadratic": rejects(lambda: validate_model(
            [pool[1][0], pool[1][1], mul(pool[1][2], pool[1][3]), pool[2][1]], good_part)),
        "corrupted_kernel_count": rejects(lambda: validate_model(
            good_factors, good_part, claimed_fixed=kernel_profile(good_part)[0]+1)),
    }
    print(json.dumps({
        "scope": "Actual smooth genus-two Prime31 source/twist equations and 2-torsion Frobenius kernels only; producer-unverified",
        "p": P, "quadratic_twist_d": P-1,
        "minus_one_legendre": P-1,
        "root_cycle_types": 11, "models": 22, "geometric_kernels_each": 15,
        "mutation_checks": negative, "positive_control": True,
        "source_sha256": hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest(),
        "rows": rows,
        "not_checked": "No actual isogeny codomain, descended morphism, Cantor arithmetic, Jacobian order, DLP, timings, 695/701 original fixture, or independent PCC",
    }, sort_keys=True, indent=2))

if __name__ == "__main__":
    main()
