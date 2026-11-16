#!/usr/bin/env python3
"""Matched toy rank-defect vs ordinary liftable-base relation-yield census.

All observations are producer-only [pcc:unverified]. No target-adaptive search,
no cold-cost claim, and no extrapolation to growing k or cryptographic p.
"""
import itertools
import json
import random
from signed_support import P, A, B, POINTS, add, neg, group_size

SEED = 4188
CONTROLS = 64
FIXED_X = [x for x, _ in POINTS[:7]]
CONDITIONED_Z = 764
TARGET = (429, 379)


def delta2(xs):
    d = 1
    for i in range(len(xs)):
        for j in range(i + 1, len(xs)):
            d = d * (xs[j] - xs[i]) ** 2 % P
    return d


def rank(matrix):
    m = [row[:] for row in matrix]
    rows, cols = len(m), len(m[0])
    t = 0
    for j in range(cols):
        pivot = next((i for i in range(t, rows) if m[i][j] % P), None)
        if pivot is None:
            continue
        m[t], m[pivot] = m[pivot], m[t]
        inv = pow(m[t][j] % P, -1, P)
        m[t] = [(v * inv) % P for v in m[t]]
        for i in range(rows):
            if i != t and m[i][j] % P:
                scale = m[i][j]
                m[i] = [(u - scale * v) % P for u, v in zip(m[i], m[t])]
        t += 1
        if t == rows:
            break
    return t


def block_rank(xs):
    left = list(itertools.combinations(range(4), 2))
    right = list(itertools.combinations(range(4, 8), 2))
    return rank([[delta2([xs[i] for i in a + b]) for b in right] for a in left])


def lift(z):
    rhs = (z ** 3 + A * z + B) % P
    if rhs == 0:
        return None
    y = pow(rhs, (P + 1) // 4, P)  # P=1019 = 3 mod 4
    if y * y % P != rhs:
        return None
    return (z, min(y, P - y))


def signed_counts(points):
    counts = {None: 1}
    for point in points:
        nxt = {}
        for state, count in counts.items():
            for branch in (point, neg(point)):
                q = add(state, branch)
                nxt[q] = nxt.get(q, 0) + count
        counts = nxt
    assert sum(counts.values()) == 256
    return counts


def measure(z, y=None):
    point = lift(z) if y is None else (z, y)
    assert point is not None and z not in FIXED_X
    assert point[1] and point[1] ** 2 % P == (z ** 3 + A * z + B) % P
    counts = signed_counts(POINTS[:7] + [point])
    return dict(z=z, rank=block_rank(FIXED_X + [z]),
                support=len(counts), target_witnesses=counts.get(neg(TARGET), 0),
                total_branches=sum(counts.values()))


def main():
    assert P % 4 == 3 and group_size() > 0
    conditioned = measure(CONDITIONED_Z, POINTS[7][1])
    assert conditioned["rank"] == 4 and conditioned["target_witnesses"] >= 1
    candidates = [z for z in range(P) if z not in FIXED_X + [CONDITIONED_Z] and lift(z)]
    rng = random.Random(SEED)
    chosen = rng.sample(candidates, min(CONTROLS, len(candidates)))
    controls = [measure(z) for z in chosen]
    orders = sorted({c["rank"] for c in controls})
    result = dict(protocol="fixed-seven-x; fixed-seven-y; one variable liftable eighth point",
                  seed=SEED, p=P, curve=[A, B], fixed_x=FIXED_X,
                  conditioned=conditioned, conditioned_is_planted=True,
                  controls=controls, control_n=len(controls),
                  control_ranks={str(k):sum(x["rank"]==k for x in controls) for k in orders},
                  control_mean_support=sum(x["support"] for x in controls)/len(controls),
                  control_positive_targets=sum(x["target_witnesses"]>0 for x in controls),
                  exact_uniform_target_probability=f'{conditioned["support"]}/{group_size()}',
                  limitation="target is planted for conditioned case; controls do not have matched target generation; diagnostic only",
                  standing="[pcc:unverified]")
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
