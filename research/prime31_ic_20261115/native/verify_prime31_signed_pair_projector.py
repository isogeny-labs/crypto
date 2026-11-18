#!/usr/bin/env python3
"""Independent Python-integer replay of original-curve signed-pair projection.

No Rust imports, discrete-log solver, or sum-lookup table. Separately checks
the RFC-like membership formula, signed group law and emitted Rust counts.
"""
import json
import sys

P = 2_147_483_647
A = 847_514_846
B = 2_087_444_724
N = 2_147_446_831
G = (625_037_715, 2_100_627_251)
SEED = 0xDEC0_DE12_34AB_CDEF
MASK = (1 << 64) - 1


def inv(x):
    return pow(x % P, P - 2, P)


def neg(q):
    return None if q is None else (q[0], (-q[1]) % P)


def add(q, r):
    if q is None:
        return r
    if r is None:
        return q
    x, y = q
    z, v = r
    if x == z and (y + v) % P == 0:
        return None
    slope = ((3*x*x + A) * inv(2*y) if q == r
             else (v-y) * inv(z-x)) % P
    xx = (slope*slope-x-z) % P
    return xx, (slope*(x-xx)-y) % P


def scalar(k, q):
    out = None
    while k:
        if k & 1:
            out = add(out, q)
        q = add(q, q)
        k >>= 1
    return out


def lifts():
    out = []
    for x in range(10_000):
        rhs = (x*x*x + A*x + B) % P
        y = pow(rhs, (P+1)//4, P)
        if y*y % P == rhs and y:
            out.append((x, min(y, P-y)))
        if len(out) == 31:
            break
    assert len(out) == 31
    assert [pt[0] for pt in out[:8]] == [0,1,2,3,5,6,8,9]
    assert scalar(N, G) is None
    return out


def targets(base):
    k = min(8, len(base))
    out = []
    for i in range(k):
        for j in range(k):
            out += [add(base[i], base[j]), add(base[i], neg(base[j]))]
    out += [None] + base[:k] + [neg(v) for v in base[:k]]
    state = SEED
    for _ in range(128):
        state ^= (state << 13) & MASK
        state ^= state >> 7
        state ^= (state << 17) & MASK
        scalar_seed = state % N or 1
        out.append(scalar(scalar_seed, G))
    return out


def algebraic(q, base):
    if q is None:
        return True
    u, v = q
    for x, y in base:
        if u == x:
            if v == y:
                continue
            assert v == -y % P
            d = (4*y*y) % P
            nu = ((3*x*x + A)**2 - 2*x*d) % P
        else:
            d = (u-x)**2 % P
            nu = ((v+y)**2 - (u+x)*d) % P
        assert d
        x_result = nu * inv(d) % P
        if any(x_result == pt[0] for pt in base):
            return True
    return False


def group(q, base):
    if q is None:
        return True
    for pt in base:
        diff = add(q, neg(pt))
        if diff is not None and any(diff in (p, neg(p)) for p in base):
            return True
    return False


def main(path):
    with open(path, encoding="utf-8") as handle:
        record = json.load(handle)
    base_all = lifts()
    assert len(record) == 3
    assert [r["b"] for r in record] == [4, 8, 31]
    expected_targets = [169, 273, 273]
    expected_positives = [33, 129, 129]
    expected_exception_counts = [8, 16, 16]
    for r, b, target_count, positive_count, exceptional_count in zip(
        record, [4,8,31], expected_targets, expected_positives,
        expected_exception_counts
    ):
        base = base_all[:b]
        qs = targets(base)
        assert len(qs) == target_count
        positives = 0
        exceptions = 0
        for q in qs:
            actual = algebraic(q, base)
            ref = group(q, base)
            assert actual == ref, (b, q, actual, ref)
            positives += actual
            if q is not None and any(q[0] == p[0] for p in base):
                exceptions += 1
        assert r["targets"] == len(qs)
        assert r["positives"] == positives == positive_count
        assert r["exceptional_target_abscissae"] == exceptions == exceptional_count
        assert r["algebraic"]["mul"] > 0
        assert r["algebraic"]["homogeneous_evals"] > 0
        assert r["control_ec"]["group_additions"] > 0
        assert r["interpretation"].startswith("component diagnostic")
    # At -P0 the naive denominator-cleared expression vanishes identically,
    # but the correct tangent double is not in the B31 x base.
    x,y=base_all[0]
    bad_q=neg((x,y))
    assert not algebraic(bad_q,base_all)
    print("PASS independent BigInt original-curve replay: 715 test cases; "
          "291 positive; vertical/tangent exceptions checked")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: python3 verify_prime31_signed_pair_projector.py result.json")
    main(sys.argv[1])
