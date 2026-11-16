#!/usr/bin/env python3
"""Independent exact arithmetic replay of the fixed S3 pair-root polynomial.

No Rust/C++ code, library EC implementation, discrete-log lookup or F4 oracle
is imported. Reconstruct the public ordinary prime31 curve, deterministically
derive the depth-six Dickson fibre, compute S3 roots and check them against an
independently written elliptic-curve group law. Producer-only [pcc:unverified].
"""
import hashlib
import json

P = 2**31 - 1
A, B, N = 847514846, 2087444724, 2147446831
EXPECTED_BRANCH_DEGREES = (16, 20, 62, 124, 240, 468, 31)
EXPECTED_ROOT_HASH = "f83e7ecdd585af6d9457b054e61d66c957624a7cc5a9719b62ca92e4afe7aca6"
EXPECTED_POLY_HASH = "a1b9f09503c9eb08ff70bca1f7cf6dd7b9c575c924bc4f5530580f7ca4852e3a"


def square_root(z):
    z %= P
    s = pow(z, (P + 1) // 4, P)
    return s if s * s % P == z else None


def f(x):
    return (x * x - 2) % P


def base():
    for seed in range(2, 10000):
        terminal = seed
        for _ in range(6):
            terminal = f(terminal)
        if terminal in (2, P-2):
            continue
        fibre = [terminal]
        for _ in range(6):
            new = []
            for value in fibre:
                r = square_root(value + 2)
                if r is not None:
                    new.extend((r, (-r) % P))
            fibre = sorted(set(new))
        if len(fibre) != 64:
            continue
        pts = []
        for x in fibre:
            y = square_root(x*x*x + A*x + B)
            if y not in (None, 0):
                pts.append((x, min(y, P-y)))
        if len(pts) >= 2:
            return seed, terminal, pts
    raise AssertionError("no deterministic Dickson base")


def ec_add(q, r):
    if q is None:
        return r
    if r is None:
        return q
    x, y = q
    z, w = r
    if x == z and (y + w) % P == 0:
        return None
    if x == z:
        slope = (3*x*x + A)*pow(2*y, -1, P) % P
    else:
        slope = (w - y)*pow((z - x) % P, -1, P) % P
    v = (slope*slope - x - z) % P
    return v, (slope*(x - v) - y) % P


def s3_roots(x, y):
    # These are the literal coefficient expressions of S3(x,y,w).
    aa = (x-y)**2 % P
    bb = (-2*((x+y)*(x*y+A)+2*B)) % P
    cc = ((x*y-A)**2-4*B*(x+y)) % P
    if aa == 0:
        assert bb
        return [(-cc*pow(bb, -1, P)) % P]
    d = (bb*bb-4*aa*cc) % P
    root = square_root(d)
    assert root is not None
    inv = pow(2*aa, -1, P)
    ans = sorted(set(((-bb+root)*inv % P, (-bb-root)*inv % P)))
    assert len(ans) == 2
    return ans


def collision_level(x, y):
    if x == y:
        return 6
    for j in range(6):
        if (x+y) % P == 0:
            return j
        x, y = f(x), f(y)
    raise AssertionError("same-fibre roots do not meet")


def multiply(a, b):
    out = [0]*(len(a)+len(b)-1)
    for i, v in enumerate(a):
        for j, w in enumerate(b):
            out[i+j] = (out[i+j] + v*w) % P
    return out


def poly_from_roots(roots):
    poly = [1]
    for w in roots:
        poly = multiply(poly, [(-w) % P, 1])
    return poly


def hash_le32(items):
    return hashlib.sha256(b"".join(x.to_bytes(4, "little") for x in items)).hexdigest()


def main():
    seed, terminal, pts = base()
    assert (seed, terminal, len(pts)) == (3, 933677148, 31)
    roots_by_arm = [[] for _ in range(7)]
    preimages = {}
    verified = 0
    for i, left in enumerate(pts):
        for j in range(i, len(pts)):
            right = pts[j]
            arm = collision_level(left[0], right[0])
            roots = s3_roots(left[0], right[0])
            ep = ec_add(left, right)
            assert ep is not None
            actual = [ep[0]]
            if i != j:
                em = ec_add(left, (right[0], -right[1] % P))
                assert em is not None
                actual.append(em[0])
            assert roots == sorted(actual)
            for k, w in enumerate(roots):
                assert w not in preimages
                preimages[w] = (i, j, k)
                roots_by_arm[arm].append(w)
                verified += 1
    ordered = sorted(preimages)
    assert verified == 961 == len(ordered)
    assert tuple(map(len, roots_by_arm)) == EXPECTED_BRANCH_DEGREES
    assert hash_le32(ordered) == EXPECTED_ROOT_HASH

    # Check the full monic polynomial and its seven first-collision factors
    # by a second, straightforward coefficient-convolution implementation.
    global_poly = poly_from_roots(ordered)
    parts = [poly_from_roots(sorted(xs)) for xs in roots_by_arm]
    product = [1]
    for part in parts:
        product = multiply(product, part)
    assert product == global_poly
    assert len(global_poly) == 962 and global_poly[-1] == 1
    assert hash_le32(global_poly) == EXPECTED_POLY_HASH
    assert global_poly[:6] == [
        1384764775, 1730047243, 2061394966,
        644849095, 1634675601, 1780857477,
    ]

    print(json.dumps({
        "standing": "[pcc:unverified]",
        "scope": "independent producer arithmetic/certificate replay, not a timed ECDLP solver",
        "p": P, "n": N, "seed": seed, "terminal": terminal,
        "liftable_base": len(pts),
        "verified_signed_S3_finite_abscissae": verified,
        "pair_preimage_unique": len(preimages) == 961,
        "branch_degrees": list(map(len, roots_by_arm)),
        "polynomial_degree": len(global_poly)-1,
        "root_sha256_le32": hash_le32(ordered),
        "poly_sha256_le32": hash_le32(global_poly),
        "seven_factor_product_exact": True,
    }, sort_keys=True))


if __name__ == "__main__":
    main()
