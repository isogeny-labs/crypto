#!/usr/bin/env python3
"""Independent integer/EC arithmetic replay of the Rust M6 fixed-curve gate.

No unknown logarithm is recovered; no new solver runtime is measured.
"""
import json
import sys

P = 2147483647
A = 847514846
B = 2087444724
N = 2147446831
WEIGHTS = (1, 3, 5, 7, 11, 13)

def plus(U, V):
    if U is None: return V
    if V is None: return U
    x, y = U
    u, v = V
    if x == u:
        if (y + v) % P == 0: return None
        assert y == v and y != 0
        slope = ((3*x*x + A) * pow(2*y, P-2, P)) % P
    else:
        slope = ((v-y) * pow((u-x) % P, P-2, P)) % P
    X = (slope*slope-x-u) % P
    return X, (slope*(x-X)-y) % P

def multiple(k, pt):
    acc = None
    while k:
        if k & 1: acc = plus(acc, pt)
        pt = plus(pt, pt)
        k >>= 1
    return acc

def first_x_basis():
    out = []
    for x in range(1000):
        rhs = (x*x*x + A*x + B) % P
        y = pow(rhs, (P+1)//4, P)
        if y*y % P == rhs and y != 0:
            out.append((x, min(y, P-y)))
            if len(out) == 31: break
    assert len(out) == 31 and out[0][0] == 0 and out[-1][0] == 66
    assert all((y*y - x*x*x - A*x - B) % P == 0 for x, y in out)
    return out

def triple_image(slots):
    out = set()
    for x in slots[0]:
        for y in slots[1]:
            xy = plus(x, y)
            for z in slots[2]:
                out.add(plus(xy, z))
    return out

def ceil_expected_calls(numerator):
    words = 31**6
    return (numerator*N + words - 1)//words

def main():
    base = first_x_basis()
    assert all(multiple(N, base[i]) is None for i in (0, 8, 30))
    slots = [[multiple(w, pt) for pt in base] for w in WEIGHTS]
    left, right = triple_image(slots[:3]), triple_image(slots[3:])
    assert len(left) == len(right) == 31**3 == 29791
    words = 31**6
    assert words == 887503681
    assert 31*N <= 87*words
    assert 1024*N <= 3143*words
    assert ceil_expected_calls(31) == 76
    assert ceil_expected_calls(1024) == 2478
    if len(sys.argv) == 2:
        with open(sys.argv[1], encoding="utf-8") as inp:
            rust = json.load(inp)
        assert rust["ordered_six_slot_words"] == words
        assert rust["maximum_uniform_positive_probability"] == {
            "numerator": words, "denominator": N
        }
        half = rust["three_slot_half_support"]
        assert half["left_distinct"] == len(left)
        assert half["right_distinct"] == len(right)
        assert rust["archived_training"]["ideal_uniform_calls_for_expected_31_hits_ceil"] == 76
        assert rust["archived_online"]["ideal_uniform_calls_for_expected_1024_hits_ceil"] == 2478
    print(json.dumps({"status": "independent_exact_integer_and_EC_replay_passed",
        "n": N, "six_words": words, "left_support": len(left),
        "right_support": len(right), "uniform_probability_bound": words/N,
        "ideal_training_calls_ceil": ceil_expected_calls(31),
        "ideal_online_calls_ceil": ceil_expected_calls(1024),
        "training_yield_only_ratio": 87*words/(31*N),
        "online_yield_only_ratio": 3143*words/(1024*N)}, sort_keys=True))

if __name__ == "__main__":
    main()
