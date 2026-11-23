#!/usr/bin/env python3
"""Independent producer-local exact EC and S3/S4 bridge validation.

Loads the frozen public natural-mask transcript; it does NOT solve a polynomial
system, recover the logarithm or independently certify a speed claim.
"""
import json
from pathlib import Path

P = 2**31 - 1
A, B, N = 847514846, 2087444724, 2147446831
G, Q = (625037715, 2100627251), (1547953785, 1667575595)
ROOT = Path(__file__).resolve().parent
TRANSCRIPT = ROOT / "M8_SPLIT_CONTROL.results.json"


def inv(x):
    return pow(x, -1, P)


def ec_add(u, v):
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, w = v
    if x == z:
        if (y + w) % P == 0:
            return None
        lam = (3*x*x + A) * inv(2*y % P) % P
    else:
        lam = (w-y) * inv((z-x) % P) % P
    xx = (lam*lam-x-z) % P
    return (xx, (lam*(x-xx)-y) % P)


def negate(q):
    return None if q is None else (q[0], (-q[1]) % P)


def ec_mul(k, q):
    result = None
    while k:
        if k & 1:
            result = ec_add(result, q)
        q = ec_add(q, q)
        k >>= 1
    return result


def sqrt_fp(x):
    x %= P
    r = pow(x, (P+1)//4, P)
    return r if r*r % P == x else None


def dickson_base(depth):
    for seed in range(2, 10_000):
        terminal = seed
        for _ in range(depth):
            terminal = (terminal*terminal-2) % P
        if terminal in (2, P-2):
            continue
        values = [terminal]
        for _ in range(depth):
            roots = []
            for value in values:
                s = sqrt_fp(value + 2)
                if s is not None:
                    roots.extend((s, (-s) % P))
            values = sorted(set(roots))
        if len(values) != 1 << depth:
            continue
        points = []
        for x in values:
            y = sqrt_fp((x*x*x + A*x + B) % P)
            if y is not None and y:
                points.append((x, min(y, P-y)))
        if len(points) >= 2:
            return terminal, points
    raise AssertionError("missing Dickson fibre")


def decode(code):
    return [((code >> (6*j)) & 63,
             -1 if ((code >> (24+j)) & 1) else 1)
            for j in range(4)]


def s3_coeff(x, y):
    sum_xy = (x+y) % P
    product = x*y % P
    return ((x-y)**2 % P,
            -2*(sum_xy*((product+A) % P) + 2*B) % P,
            ((product-A)**2 - 4*B*sum_xy) % P)


def s3(x, y, z):
    a, b, c = s3_coeff(x, y)
    return (a*z*z + b*z + c) % P


def s4(x, y, z, w):
    # Quadratic resultant Res_t(S3(x,y,t),S3(z,w,t)).
    a, b, c = s3_coeff(x, y)
    d, e, f = s3_coeff(z, w)
    return ((a*f-c*d)**2 - (a*e-b*d)*(b*f-c*e)) % P


def run():
    source = json.loads(TRANSCRIPT.read_text())
    assert source["curve"] == {"p": P, "a": A, "b": B, "n": N,
                               "g": list(G), "cofactor": 1}
    assert tuple(source["public_target"]) == Q
    terminal, points = dickson_base(6)
    assert terminal == 933677148 and len(points) == 31
    assert len(source["witnesses"]) == 32
    mutated = 0
    first_aux = None
    for aa, bb, left, right in source["witnesses"]:
        indices = decode(left) + decode(right)
        assert len({i for i, _ in indices}) == 8
        signed = [points[i] if sign == 1 else negate(points[i])
                  for i, sign in indices]
        u = ec_add(ec_add(signed[0], signed[1]), signed[2])
        v = ec_add(ec_add(signed[3], signed[4]), signed[5])
        w = ec_add(signed[6], signed[7])
        r = ec_add(ec_mul(aa, G), ec_mul(bb, Q))
        assert all(pt is not None for pt in (u, v, w, r))
        assert ec_add(ec_add(u, v), w) == r
        xs = [pt[0] for pt in signed]
        equations = [s4(xs[0], xs[1], xs[2], u[0]),
                     s4(xs[3], xs[4], xs[5], v[0]),
                     s3(xs[6], xs[7], w[0]),
                     s4(u[0], v[0], w[0], r[0])]
        assert equations == [0, 0, 0, 0]
        mutated += s4(u[0], v[0], w[0], (r[0] + 1) % P) != 0
        # A polynomial x-witness is not a signed point witness. Enumerate
        # all 256 orientations and require the full y-coordinate target.
        exact_signs = 0
        same_x_signs = 0
        distinct = set()
        for mask in range(256):
            total = None
            for j, pt in enumerate(signed):
                total = ec_add(total, negate(pt) if mask & (1 << j) else pt)
            distinct.add(total)
            exact_signs += total == r
            same_x_signs += total is not None and total[0] == r[0]
        assert (exact_signs, same_x_signs, len(distinct)) == (1, 2, 256)
        first_aux = first_aux or [u[0], v[0], w[0], r[0]]
    assert mutated == 32
    assert first_aux == [803189040, 898740481, 729813404, 903384438]
    return {"standing": "[pcc:unverified]",
            "bridge": "balanced S4+S3, eight Dickson depth-6 blocks",
            "p": P, "n": N, "B": 31,
            "free_variables": 3, "tower_variables": 48,
            "natural_witnesses_replayed": 32,
            "S4_zero_residuals": 96, "S3_zero_residuals": 32,
            "signed_public_group_equations_checked": 32,
            "corrupted_target_abscissae_rejected": mutated,
            "sign_orientations_per_tuple": 256,
            "unique_exact_y_signs_per_tuple": 1,
            "x_only_sign_solutions_per_tuple": 2,
            "first_aux": first_aux,
            "scope": "producer-only exact semantic replay; no algebraic oracle, F4 timing or speedup"}


if __name__ == "__main__":
    print(json.dumps(run(), indent=2, sort_keys=True))
