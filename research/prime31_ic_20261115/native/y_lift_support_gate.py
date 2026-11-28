#!/usr/bin/env python3
"""Exact prime31 y-fiber and two-sum supports, independent arithmetic control.

All EC additions here are generic *support enumeration* and are NOT an
algebraic IC oracle or a complete ECDLP solver. Standing [pcc:unverified].
"""
import json
import hashlib
from collections import Counter
from y_fiber_probe import P, A, B, base, add, sqrt


def neg(q):
    return (q[0], (-q[1]) % P)


def oriented_y_fiber(q):
    x, y = q
    # (X-x)(X^2+xX+x^2+a) = X^3+aX+b-y^2.
    disc = (-3*x*x-4*A) % P
    root = sqrt(disc)
    xs = {x}
    if root is not None:
        inv2 = (P+1)//2
        xs.add(((-x+root)*inv2) % P)
        xs.add(((-x-root)*inv2) % P)
    fiber = sorted((xx, y) for xx in xs)
    assert len(fiber) in (1, 3)
    assert all((xx**3+A*xx+B-y*y) % P == 0 for xx, _ in fiber)
    return fiber


def support(arr):
    out = set()
    identity = 0
    for i, p in enumerate(arr):
        for q in arr[i:]:
            z = add(p, q)
            if z is None:
                identity += 1
            else:
                out.add(z)
    return out, identity


def run():
    seed, points = base()
    assert seed == 3 and len(points) == 31
    fibers = [oriented_y_fiber(q) for q in points]
    hist = Counter(len(v) for v in fibers)
    assert hist == {1:14, 3:17}, hist
    projection_hist = Counter()
    for i, fi in enumerate(fibers):
        for fj in fibers[i+1:]:
            values = {add(a,b)[1] for a in fi for b in fj}
            expected = len(fi)*len(fj)
            assert len(values) == expected
            projection_hist[len(values)] += 1
    assert projection_hist == {1:91, 3:238, 9:136}
    lifted = sorted({p for fiber in fibers for p in fiber})
    unique = sorted(p for p, fiber in zip(points, fibers) if len(fiber) == 1)
    assert len(lifted) == 65 and len(unique) == 14
    assert set(points).issubset(lifted)
    def signed(points):
        return sorted({q for p in points for q in (p, neg(p))})
    _, base7 = base(7)
    assert len(base7) == 64
    injective7 = [q for q in base7 if len(oriented_y_fiber(q)) == 1]
    assert len(injective7) == 39
    depth7_support, depth7_identities = support(signed(injective7[:31]))
    assert len(depth7_support) == 1922 and depth7_identities == 31
    so, si = support(signed(points))
    sl, sli = support(signed(lifted))
    su, sui = support(signed(unique))
    oo, oi = support(points)
    ol, oli = support(lifted)
    assert so.issubset(sl)
    def digest(rows):
        raw = "".join(f"{x},{y}\n" for x, y in sorted(rows)).encode()
        return hashlib.sha256(raw).hexdigest()
    fingerprints = {"canonical1922":digest(so), "lifted8450":digest(sl)}
    assert fingerprints == {
        "canonical1922":"4fcd2507f91d2c80707e5b8b34f45dad1ac69c67d32d51e9c5a186b0fe795716",
        "lifted8450":"7638f8d3de56842d1a715d98b860a0c12f7df59d4d841e8741b32c92dc8d4974"
    }
    assert (len(so), len(sl), len(su), len(oo), len(ol)) == (1922, 8450, 392, 496, 2145)
    assert (si, sli, sui, oi, oli) == (31, 65, 14, 0, 0)
    return {
        "standing": "[pcc:unverified]",
        "method": "generic group-support enumeration: NEGATIVE CONTROL, not algebraic IC",
        "p": P, "a": A, "b": B, "n": 2147446831, "seed": seed,
        "canonical_base": len(points), "y_fiber_size_histogram": dict(sorted(hist.items())),
        "lifted_rational_points": len(lifted),
        "distinct_y_pair_output_root_histogram":dict(sorted(projection_hist.items())),
        "depth7_full_base": len(base7),
        "depth7_injective_y_base": len(injective7),
        "depth7_first31_injective_y_signed_pair_support": len(depth7_support),
        "oriented_pair_support_canonical": len(oo),
        "oriented_pair_support_y_lifted": len(ol),
        "signed_pair_support_canonical": len(so),
        "signed_pair_support_y_lifted": len(sl),
        "signed_pair_support_injective_y_subset": len(su),
        "y_lift_spurious_signed_targets_against_original_factor_base": len(sl-so),
        "support_sha256": fingerprints,
        "extra_fraction_of_lifted_support": len(sl-so)/len(sl),
        "identity_exception_counts": {"canonical_signed":si, "lifted_signed":sli,
                                      "injective_signed":sui}
    }


if __name__ == "__main__":
    print(json.dumps(run(), indent=2, sort_keys=True))
