#!/usr/bin/env python3
"""Certified rational-root odd-degree models for the frozen Smith holdout.

This is an arithmetic adapter only; it does not screen odd torsion or build an
isogeny. Coefficients throughout are ascending powers over F_2147483647.
"""

import json
from math import comb
from pathlib import Path

P = 2147483647
HERE = Path(__file__).resolve().parent


def trim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return a


def rem(a, b):
    a, b = trim(a), trim(b)
    inv = pow(b[-1], -1, P)
    while len(a) >= len(b):
        c = a[-1] * inv % P
        d = len(a) - len(b)
        for i, v in enumerate(b):
            a[d + i] = (a[d + i] - c * v) % P
        a = trim(a)
    return a


def mulmod(a, b, modulus):
    if not a or not b:
        return []
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = (out[i + j] + x * y) % P
    return rem(out, modulus)


def powmod(a, n, modulus):
    out = [1]
    while n:
        if n & 1:
            out = mulmod(out, a, modulus)
        a = mulmod(a, a, modulus)
        n >>= 1
    return out


def gcd(a, b):
    while b:
        a, b = b, rem(a, b)
    return [(x * pow(a[-1], -1, P)) % P for x in a] if a else []


def sub(a, b):
    out = [0] * max(len(a), len(b))
    for i in range(len(out)):
        out[i] = ((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % P
    return trim(out)


def eval_poly(f, x):
    out = 0
    for c in reversed(f):
        out = (out * x + c) % P
    return out


def linear_roots(f):
    linear = gcd(f, sub(powmod([0, 1], P, f), [0, 1]))
    roots = []
    pending = [linear]
    while pending:
        g = pending.pop()
        if len(g) == 1:
            continue
        if len(g) == 2:
            roots.append((-g[0] * pow(g[1], -1, P)) % P)
            continue
        # Deterministic split schedule; never label an unsplit factor rootless.
        for seed in range(1, 1025):
            a = [seed % P, 1]
            h = sub(powmod(a, (P - 1) // 2, g), [1])
            d = gcd(g, h)
            if 1 < len(d) < len(g):
                quotient = trim(g)
                inv = pow(d[-1], -1, P)
                q = [0] * (len(g) - len(d) + 1)
                while len(quotient) >= len(d):
                    c = quotient[-1] * inv % P
                    shift = len(quotient) - len(d)
                    q[shift] = c
                    for j, value in enumerate(d):
                        quotient[shift + j] = (quotient[shift + j] - c * value) % P
                    quotient = trim(quotient)
                assert not quotient
                pending.extend([d, trim(q)])
                break
        else:
            raise RuntimeError('deterministic split budget exhausted')
    return sorted(roots)


def odd_model(f, r):
    derivative = sum(k * f[k] * pow(r, k - 1, P) for k in range(1, 9)) % P
    assert derivative != 0 and eval_poly(f, r) == 0
    out = []
    for j in range(9):
        m = 8 - j
        value = sum(f[k] * comb(k, m) * pow(r, k - m, P) for k in range(m, 9))
        out.append(value * pow(derivative, -m, P) % P)
    assert out[8] == 0 and out[7] == 1
    # Independent direct substitution check at nonzero coordinates.
    for z in (1, 2, 3, 17, 257, P - 1):
        x = (r + pow(derivative * z % P, -1, P)) % P
        assert eval_poly(out[:8], z) == pow(z, 8, P) * eval_poly(f, x) % P
    # The two old infinities, when rational, become the two finite z=0 points.
    old_infinities = []
    if pow(f[8], (P - 1) // 2, P) == 1:
        root_lead = pow(f[8], (P + 1) // 4, P)
        old_infinities = sorted({root_lead * pow(derivative, -4, P) % P, -root_lead * pow(derivative, -4, P) % P})
        assert len(old_infinities) == 2
        assert all(w * w % P == out[0] for w in old_infinities)
    assert bool(old_infinities) == (pow(out[0], (P - 1) // 2, P) == 1)
    return derivative, out[:8], old_infinities


def main():
    rows = [json.loads(line) for line in (HERE / 'correctness/smith-negative-materialized-v1.jsonl').read_text().splitlines()]
    result = []
    for row in rows:
        f = row['f_ascending']
        derivative_poly = [(k * f[k]) % P for k in range(1, len(f))]
        assert gcd(f, derivative_poly) == [1], row['index']
        roots = linear_roots(f)
        assert len(roots) == row['cycle_type'].split('+').count('1')
        models = []
        for r in roots:
            derivative, h, old_infinities = odd_model(f, r)
            assert gcd(h, [(k * h[k]) % P for k in range(1, len(h))]) == [1]
            models.append({'root': r, 'derivative': derivative, 'h_ascending': h, 'old_infinity_images_w_at_z0': old_infinities})
        result.append({'index': row['index'], 'cycle_type': row['cycle_type'], 'rational_roots': roots, 'odd_models': models, 'odd_kernel_screen_executed': False})
    print(json.dumps({'schema': 'smith-odd-model-gate/v1', 'p': P, 'curve_count': len(result), 'convertible_count': sum(bool(r['rational_roots']) for r in result), 'rows': result}, sort_keys=True))


if __name__ == '__main__':
    main()
