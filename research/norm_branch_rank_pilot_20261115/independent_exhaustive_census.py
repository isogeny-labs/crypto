#!/usr/bin/env python3
"""Independent arithmetic replay of the fixed-seven-point rank/coverage census.

Full enumeration of every allowable eighth abscissa over F_1019. No imports
from the producer census. Producer-local replay only: [pcc:unverified].
"""
import collections
import itertools
import json
import random

P, A, B = 1019, 46, 158
POINTS = [(944,936),(243,525),(432,632),(225,527),(627,518),
          (387,878),(914,704),(764,816)]
FIXED_X = [x for x, _ in POINTS[:7]]
TARGET = (429,379)


def negative(v):
    return None if v is None else (v[0], (-v[1]) % P)


def plus(v, w):
    if v is None:
        return w
    if w is None:
        return v
    x, y = v
    u, z = w
    if x == u and (y + z) % P == 0:
        return None
    slope = ((3*x*x + A)*pow(2*y, -1, P) if x == u
             else (z-y)*pow(u-x, -1, P)) % P
    xx = (slope*slope-x-u) % P
    return xx, (slope*(x-xx)-y) % P


def lift(x):
    rhs = (x**3+A*x+B) % P
    if rhs == 0:
        return None
    y = pow(rhs, (P+1)//4, P)
    return (x, min(y, P-y)) if y*y % P == rhs else None


def squared_vandermonde(values):
    out = 1
    for i in range(len(values)):
        for j in range(i+1,len(values)):
            out = out*(values[i]-values[j])**2 % P
    return out


def rank(matrix):
    rows = [row[:] for row in matrix]
    pivots = 0
    for j in range(len(rows[0])):
        i = next((i for i in range(pivots,len(rows)) if rows[i][j]),None)
        if i is None:
            continue
        rows[pivots], rows[i] = rows[i], rows[pivots]
        scale = pow(rows[pivots][j], -1, P)
        rows[pivots] = [v*scale % P for v in rows[pivots]]
        for k in range(pivots+1,len(rows)):
            coefficient = rows[k][j]
            rows[k] = [(u-coefficient*v) % P
                       for u,v in zip(rows[k], rows[pivots])]
        pivots += 1
        if pivots == len(rows):
            break
    return pivots


def block_rank(z):
    x = FIXED_X+[z]
    pairs_l = list(itertools.combinations(range(4),2))
    pairs_r = list(itertools.combinations(range(4,8),2))
    return rank([[squared_vandermonde([x[i] for i in left+right])
                  for right in pairs_r] for left in pairs_l])


def seven_sums():
    counts = collections.Counter()
    for signs in itertools.product((-1,1),repeat=7):
        state = None
        for sign, point in zip(signs, POINTS):
            state = plus(state, point if sign == 1 else negative(point))
        counts[state] += 1
    assert sum(counts.values()) == 128
    return counts


def score(z, seven):
    point = lift(z)
    assert point is not None
    counts = collections.Counter()
    for s, n in seven.items():
        counts[plus(s,point)] += n
        counts[plus(s,negative(point))] += n
    assert sum(counts.values()) == 256
    return dict(z=z,rank=block_rank(z),support=len(counts),
                target_witnesses=counts[negative(TARGET)])


def main():
    assert len(set(FIXED_X)) == 7
    assert all(y*y % P == (x**3+A*x+B) % P for x,y in POINTS+[TARGET])
    seven = seven_sums()
    group_order = 1
    for x in range(P):
        rhs = (x**3+A*x+B) % P
        group_order += (1 if rhs == 0 else
                        2 if pow(rhs,(P-1)//2,P) == 1 else 0)
    assert group_order == 977
    conditioned = score(764,seven)
    assert conditioned == dict(z=764,rank=4,support=256,target_witnesses=1)
    eligible = [z for z in range(P)
                if z not in FIXED_X+[764] and lift(z) is not None]
    controls = [score(z,seven) for z in eligible]
    sampled_z = random.Random(4188).sample(eligible,64)
    sampled = [score(z,seven) for z in sampled_z]
    assert len(controls) == 480 and len(sampled) == 64
    assert all(c['rank'] == 6 for c in controls)
    assert sum(c['support'] for c in controls) == 115249
    assert sum(c['support'] for c in sampled) == 15358
    assert sum(c['target_witnesses']>0 for c in controls) == 118
    assert sum(c['target_witnesses']>0 for c in sampled) == 9
    report = dict(standing='[pcc:unverified]',p=P,curve=[A,B],
                  group_order=group_order,seven_support=len(seven),
                  conditioned=conditioned,control_count=len(controls),
                  control_ranks=dict(collections.Counter(c['rank'] for c in controls)),
                  control_min_support=min(c['support'] for c in controls),
                  control_max_support=max(c['support'] for c in controls),
                  control_mean_support=sum(c['support'] for c in controls)/len(controls),
                  control_positive_target_count=sum(c['target_witnesses']>0 for c in controls),
                  sample_n=len(sampled),
                  sample_mean_support=sum(c['support'] for c in sampled)/len(sampled),
                  sample_positive_target_count=sum(c['target_witnesses']>0 for c in sampled),
                  conditioned_uniform_target_hit_probability='256/977',
                  caution='Planted conditioned target; coefficient rank != relation yield or full ECDLP solving cost')
    print(json.dumps(report,indent=2,sort_keys=True))


if __name__ == '__main__':
    main()
