#!/usr/bin/env python3
"""Independent bitmask-partition/BigInt replay for original-curve signed W8."""
import functools
import itertools
import json
import math
import sys
from collections import Counter

P = 2_147_483_647
A = 847_514_846
B_CURVE = 2_087_444_724
N = 2_147_446_831
W = (1, 3, 5, 7, 11, 13, 17, 19)
M = len(W)

@functools.lru_cache(None)
def partitions(mask):
    if not mask:
        return ((),)
    pivot = mask & -mask
    answer = []
    s = mask
    while s:
        if s & pivot:
            for tail in partitions(mask ^ s):
                answer.append((s,) + tail)
        s = (s - 1) & mask
    return tuple(answer)

def pattern_map(signed):
    ptn = partitions((1 << M) - 1)
    assert len(ptn) == 4140
    out = {}
    total = 0
    for sm in range(1 << M if signed else 1):
        ws = tuple(-w if sm & (1 << i) else w for i, w in enumerate(W))
        for blocks in ptn:
            total += 1
            sums = []
            for blk in blocks:
                ss = sum(ws[i] for i in range(M) if blk & (1 << i))
                if ss:
                    sums.append(ss)
            signature = tuple(sorted(sums))
            if signature not in out or len(blocks) < out[signature]:
                out[signature] = len(blocks)
    return out, total

def count(b, pat):
    by = [0] * (M + 1)
    for sig, h in pat.items():
        k = len(sig)
        if h > b or k > b:
            continue
        ways = math.factorial(b) // math.factorial(b-k)
        denom = math.prod(math.factorial(n) for n in Counter(sig).values())
        assert ways % denom == 0
        by[k] += ways // denom
    return sum(by), by

def brute_two():
    vals = set()
    for ix in itertools.product(range(2), repeat=M):
        for mask in range(1 << M):
            c = [0, 0]
            for j, idx in enumerate(ix):
                c[idx] += (-W[j] if mask & (1 << j) else W[j])
            vals.add(tuple(c))
    return len(vals)

def first_points():
    pts = []
    for x in range(100):
        rhs = (x**3 + A*x + B_CURVE) % P
        y = pow(rhs, (P + 1)//4, P)
        if y*y % P != rhs:
            continue
        assert y != 0
        pts.append((x, min(y, P-y)))
        if len(pts) == 31:
            break
    assert len(pts) == 31 and pts[0][0] == 0 and pts[-1][0] == 66
    return pts

def ec_add(q, r):
    if q is None:
        return r
    if r is None:
        return q
    x1, y1 = q
    x2, y2 = r
    if x1 == x2 and (y1 + y2) % P == 0:
        return None
    if x1 == x2:
        numerator = (3*x1*x1+A) % P
        denominator = (2*y1) % P
    else:
        numerator = (y2-y1) % P
        denominator = (x2-x1) % P
    lam = numerator*pow(denominator, P-2, P) % P
    x3 = (lam*lam-x1-x2) % P
    y3 = (lam*(x1-x3)-y1) % P
    return (x3,y3)

def ec_mul(k,p):
    r = None
    while k:
        if k&1:
            r=ec_add(r,p)
        p=ec_add(p,p)
        k//=2
    return r

def replay_curve():
    pts=first_points()
    sums=[]
    for word in ((0,1,1,0,2,3,4,5),(1,0,0,1,2,3,4,5)):
        total=None
        for j, idx in enumerate(word):
            total=ec_add(total,ec_mul(W[j],pts[idx]))
        sums.append(total)
    assert sums[0] == sums[1] and sums[0] is not None
    assert all(y*y % P == (x*x*x + A*x+B_CURVE)%P for x,y in pts)
    return dict(x=sums[0][0],y=sums[0][1])

def verify(obj):
    assert obj['curve']==dict(p=P,a=A,b=B_CURVE,n=N)
    assert obj['signed_weights'] == list(W)
    signed,pc=pattern_map(True)
    positive,uc=pattern_map(False)
    assert pc==1059840 and uc==4140
    assert len(signed)==528104
    assert obj['signed_distinct_patterns']==len(signed)
    assert obj['positive_distinct_patterns']==len(positive)
    assert obj['signed_partition_leaves']==pc
    assert obj['positive_partition_leaves']==uc
    assert len(obj['b_controls'])==8
    for entry,b in zip(obj['b_controls'],(1,2,3,4,8,12,16,31)):
        sn,by=count(b,signed)
        un,uby=count(b,positive)
        assert entry['B']==b
        assert entry['signed_formal']==sn, ('signed',b)
        assert entry['positive_only_formal']==un, ('positive',b)
        assert entry['signed_words']==256*b**8
        assert entry['positive_only_words']==b**8
        assert entry['signed_by_k_0_through_8']==by
        assert entry['positive_only_by_k_0_through_8']==uby
        assert entry['uniform_hit_ceiling_numerator']==min(sn,N)
        assert entry['uniform_hit_ceiling_denominator']==N
    assert count(2,signed)[0]==brute_two()==5625
    assert count(4,signed)[0]==5491889
    assert count(8,signed)[0]==3240125537
    assert count(31,signed)[0]==214117016830955
    assert obj['original_curve_planted_sum']==replay_curve()
    # Control that an off-by-one published formal count is caught.
    mutated = count(4,signed)[0] + 1
    assert mutated != obj['b_controls'][3]['signed_formal']
    print('PASS: independent bitmask partition census, brute B2, B1..31 strata, original Prime31 BigInt signed collision, mutation check')

if __name__ == '__main__':
    assert len(sys.argv)==2, 'pass Rust-produced json record'
    with open(sys.argv[1],encoding='utf-8') as f:
        verify(json.load(f))
