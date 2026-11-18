#!/usr/bin/env python3
"""Independent integer-partition and original-prime31 point replay.

Usage: python3 verify_prime31_m6_formal_quotient.py result.json
The verifier is separate from the Rust producer, enumerates set partitions by
least-mask member rather than restricted-growth strings and uses Python BigInt
affine EC arithmetic (no dependency on the producer's Arithmetic).
"""
import copy
import itertools
import json
import math
import sys
from collections import Counter

P = 2_147_483_647
A = 847_514_846
B_CURVE = 2_087_444_724
N = 2_147_446_831
WEIGHTS = {
    "historical": (1, 3, 5, 7, 11, 13),
    "binary_unique_subset_sums": (1, 2, 4, 8, 16, 32),
    "odd_unique_subset_sums": (1, 3, 7, 15, 31, 63),
}


def partitions(mask):
    if mask == 0:
        yield ()
        return
    first = mask & -mask
    tail = mask ^ first
    sub = tail
    while True:
        first_block = sub | first
        remaining = mask ^ first_block
        for rest in partitions(remaining):
            yield (first_block,) + rest
        if sub == 0:
            break
        sub = (sub - 1) & tail


def patterns(weights):
    raw = [0] * 7
    signatures = [set() for _ in range(7)]
    for partition in partitions((1 << 6) - 1):
        sums = []
        for mask in partition:
            sums.append(sum(w for i, w in enumerate(weights) if mask & (1 << i)))
        key = tuple(sorted(sums))
        k = len(partition)
        raw[k] += 1
        signatures[k].add(key)
    assert sum(raw) == 203
    return raw, signatures


def falling(b, k):
    return math.prod(range(b - k + 1, b + 1)) if b >= k else 0


def quotient(b, signatures):
    by_k = [0] * 7
    for k in range(1, min(6, b) + 1):
        for sums in signatures[k]:
            multiplicities = Counter(sums)
            divisor = math.prod(math.factorial(n) for n in multiplicities.values())
            f = falling(b, k)
            assert f % divisor == 0
            by_k[k] += f // divisor
    return sum(by_k), by_k


def direct_coefficient_signatures(b, weights):
    # Deliberately small-B: brute over exactly 8^6 words at most.
    result = set()
    for word in itertools.product(range(b), repeat=6):
        c = [0] * b
        for index, weight in zip(word, weights):
            c[index] += weight
        result.add(tuple(c))
    return len(result)


def ec_add(u, v):
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, t = v
    if x == z and (y + t) % P == 0:
        return None
    if u == v:
        slope = (3 * x * x + A) * pow(2 * y, -1, P) % P
    else:
        slope = (t - y) * pow((z - x) % P, -1, P) % P
    new_x = (slope * slope - x - z) % P
    return (new_x, (slope * (x - new_x) - y) % P)


def ec_mul(k, point):
    out = None
    while k:
        if k & 1:
            out = ec_add(out, point)
        point = ec_add(point, point)
        k //= 2
    return out


def base_points():
    points = []
    for x in range(P):
        v = (x ** 3 + A * x + B_CURVE) % P
        y = pow(v, (P + 1) // 4, P)
        if (y * y) % P == v:
            assert y != 0
            points.append((x, min(y, P - y)))
        if len(points) == 31:
            break
    assert points[0][0] == 0 and points[-1][0] == 66
    return points


def check(data):
    assert data["scope"].startswith("exact integer formal-signature quotient")
    assert data["curve"] == dict(p=P, a=A, b=B_CURVE, n=N)
    assert data["planted_words"] == [[0, 1, 1, 0, 2, 3], [1, 0, 0, 1, 2, 3]]
    points = base_points()
    assert ec_mul(N, (625_037_715, 2_100_627_251)) is None
    sums = []
    for word in data["planted_words"]:
        acc = None
        for weight, idx in zip(WEIGHTS["historical"], word):
            acc = ec_add(acc, ec_mul(weight, points[idx]))
        sums.append(acc)
    assert sums[0] == sums[1] and sums[0] is not None
    assert data["planted_native_original_curve_sum"] == dict(x=sums[0][0], y=sums[0][1])

    assert len(data["results"]) == 3
    for item in data["results"]:
        weights = WEIGHTS[item["name"]]
        assert tuple(item["weights"]) == weights
        raw, pats = patterns(weights)
        assert item["raw_partitions_by_k_0_through_6"] == raw
        assert item["distinct_block_sum_patterns_by_k_0_through_6"] == [len(p) for p in pats]
        unique_sums = len({sum(weights[i] for i in range(6) if mask & (1 << i))
                           for mask in range(64)}) == 64
        assert item["subset_sums_distinct"] == unique_sums
        b31, _ = quotient(31, pats)
        assert item["formal_B31_support_upper"] == b31
        assert item["uniform_point_hit_upper_numerator"] == b31
        assert item["uniform_point_hit_upper_denominator"] == N
        for test in item["B_controls"]:
            b = test["B"]
            result, by_k = quotient(b, pats)
            assert test["formal_signatures"] == result
            assert test["by_support_size_k_0_through_6"] == by_k
            assert test["ordered_words"] == b ** 6
            assert result <= b ** 6
            if b <= 8:
                assert direct_coefficient_signatures(b, weights) == result
    assert data["results"][0]["formal_B31_support_upper"] == 884_350_051
    assert data["results"][1]["formal_B31_support_upper"] == 887_503_681
    assert data["results"][2]["formal_B31_support_upper"] == 887_503_681
    return True


if __name__ == "__main__":
    with open(sys.argv[1], encoding="utf-8") as f:
        source = json.load(f)
    assert check(source)
    mutation = copy.deepcopy(source)
    mutation["results"][0]["formal_B31_support_upper"] += 1
    try:
        check(mutation)
    except AssertionError:
        pass
    else:
        raise RuntimeError("mutation must be rejected")
    print("PASS: 203 partitions x 3; independent B2/B3/B4/B8 brute; original Prime31 EC collision; mutation rejected")
