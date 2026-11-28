#!/usr/bin/env python3
"""Independent prime-order EC fixture for the exact Z[z] locus certificate.

All 991 public toy DLP targets are recovered and rechecked. Reports raw
addition-call accounting for baseline BSGS versus a fixed-eight-sign table.
The comparison excludes both algorithms' point-verification costs, and
excludes locus discovery from the sign method (favorable to the sign method).
One cold target is not amortized across the 991-target correctness census.
"""
from math import isqrt
import json
from exact_integer_locus import matrix, rank_mod, P

A, B, ORDER = 447, 997, 991
POINTS = ((0, 911), (1, 528), (2, 620), (3, 731),
          (4, 555), (5, 518), (6, 687), (338, 592))


def add(p, q):
    if p is None: return q
    if q is None: return p
    x, y = p
    z, w = q
    if x == z and (y + w) % P == 0: return None
    if x == z:
        assert y % P
        slope = (3*x*x + A) * pow(2*y, -1, P) % P
    else:
        slope = (w-y)*pow(z-x, -1, P) % P
    x3 = (slope*slope - x - z) % P
    return x3, (slope*(x-x3)-y) % P


def neg(p): return None if p is None else (p[0], (-p[1]) % P)


class Meter:
    def __init__(self): self.count = 0
    def add(self, p, q):
        self.count += 1
        return add(p, q)


def multiply(p, n):
    acc = None
    while n:
        if n & 1: acc = add(acc, p)
        p = add(p, p)
        n >>= 1
    return acc


def prepare_bsgs(g):
    meter = Meter()
    m = isqrt(ORDER) + 1
    point = None
    baby = {}
    for i in range(m):
        baby.setdefault(point, i)
        point = meter.add(point, g)
    mg = None
    n = m
    base = g
    while n:
        if n & 1: mg = meter.add(mg, base)
        base = meter.add(base, base)
        n >>= 1
    return baby, neg(mg), m, meter.count


def bsgs(g, target, baby, minus_mg, m):
    meter = Meter()
    x = target
    for j in range(m + 1):
        if x in baby:
            k = (j*m + baby[x]) % ORDER
            assert multiply(g, k) == target
            return k, meter.count
        x = meter.add(x, minus_mg)
    raise AssertionError('missing log')


def build_signed_table(pts, logs):
    meter = Meter()
    cur = None
    cur_log = 0
    for pt, lg in zip(pts, logs):
        cur = meter.add(cur, pt)
        cur_log = (cur_log + lg) % ORDER
    doubles = [meter.add(pt, pt) for pt in pts]
    out = {cur: cur_log}
    prev = 0
    for index in range(1, 256):
        gray = index ^ (index >> 1)
        delta = gray ^ prev
        j = delta.bit_length() - 1
        assert delta == 1 << j
        if gray & delta:
            cur = meter.add(cur, neg(doubles[j]))
            cur_log = (cur_log - 2*logs[j]) % ORDER
        else:
            cur = meter.add(cur, doubles[j])
            cur_log = (cur_log + 2*logs[j]) % ORDER
        if cur in out: assert out[cur] == cur_log
        out[cur] = cur_log
        prev = gray
    return out, meter.count


def signed_recover(g, target, table):
    meter = Meter()
    t = target
    for j in range(ORDER):
        if t in table:
            k = (table[t]-j) % ORDER
            assert multiply(g, k) == target
            return k, meter.count
        t = meter.add(t, g)
    raise AssertionError('missing log')


def run():
    assert (4*A*A*A + 27*B*B) % P
    assert all((y*y - x*x*x - A*x - B) % P == 0 and y != 0 for x,y in POINTS)
    assert rank_mod(matrix(POINTS[-1][0]), P) == 4
    residues = {y*y % P for y in range(P)}
    group_order = 1 + sum((1 if rhs == 0 else 2 if rhs in residues else 0)
                          for x in range(P) for rhs in [(x*x*x + A*x + B) % P])
    assert group_order == ORDER and all(ORDER % d for d in range(2, isqrt(ORDER)+1))
    g = POINTS[0]
    assert multiply(g, ORDER) is None
    baby, neg_step, m, bsgs_setup = prepare_bsgs(g)
    # G's logarithm is free to the signed solver; other seven are BSGS-derived.
    tail = [bsgs(g, pt, baby, neg_step, m) for pt in POINTS[1:]]
    logs = [1] + [k for k,c in tail]
    base_log_adds = sum(c for k,c in tail)
    table, signed_table_adds = build_signed_table(POINTS, logs)
    assert len(table) == 252
    targets=[]
    pt=None
    for i in range(ORDER):
        assert pt not in targets
        targets.append(pt)
        pt = add(pt, g)
    assert pt is None
    baseline_online=0
    signed_online=0
    max_shift=0
    for i, pt in enumerate(targets):
        k1,c1 = bsgs(g,pt,baby,neg_step,m)
        k2,c2 = signed_recover(g,pt,table)
        assert k1 == k2 == i
        baseline_online += c1
        signed_online += c2
        max_shift = max(max_shift,c2)
    sign_setup = bsgs_setup + base_log_adds + signed_table_adds
    baseline_cold_mean = bsgs_setup + baseline_online / ORDER
    sign_cold_mean = sign_setup + signed_online / ORDER
    return dict(prime=P,curve_a=A,curve_b=B,order=ORDER,
        rank_defect=4, signed_support=len(table),
        signed_support_ceiling=256, all_targets_recovered=ORDER,
        matched_bsgs_baby_setup_add_calls=bsgs_setup,
        bsgs_online_total_add_calls=baseline_online,
        signed_extra_base_log_add_calls=base_log_adds,
        signed_table_add_calls=signed_table_adds,
        signed_online_total_add_calls=signed_online,
        signed_online_max_add_calls=max_shift,
        bsgs_mean_cold_add_calls=round(baseline_cold_mean,6),
        signed_mean_cold_add_calls=round(sign_cold_mean,6),
        accounting='one-cold-target setup plus mean target online add calls; excludes curve/locus selection, inversions and verification, favors signed method',
        standing='producer-local toy exact arithmetic, not independently PCC verified')


if __name__ == '__main__': print(json.dumps(run(),indent=2,sort_keys=True))
