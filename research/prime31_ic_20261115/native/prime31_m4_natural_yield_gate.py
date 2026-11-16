#!/usr/bin/env python3
"""Exact prime31 m4 signed natural-target coverage / collision controls.

GENERIC GROUP ENUMERATION ONLY. Does not discover algebraic IC relations or
recover a discrete log. Reuses the independent y_fiber_probe.py arithmetic.
"""
import hashlib
import json
from math import comb

from y_fiber_probe import P, A, B, base, add, sqrt

N = 2147446831
ROWS = (
    ((5,-1),(9,1),(16,-1),(17,-1),(18,-1),(19,-1),(29,-1),(31,1)),
    ((1,-1),(5,1),(16,-1),(20,1),(22,1),(25,-1),(26,1),(30,1)),
    ((5,-1),(11,-1),(15,1),(16,-1),(22,-1),(23,-1),(25,1),(28,-1)),
    ((4,-2),(9,-1),(11,-1),(12,-1),(16,-1),(29,1),(30,1)),
)


def upper_bound(k):
    return 1 + 4*k + 16*comb(k,2) + 24*comb(k,3) + 16*comb(k,4)


def census(points):
    signed = sorted({q for x,y in points for q in ((x,y),(x,(-y)%P))})
    assert len(signed) == 2*len(points)
    pair = {None}
    for i,x in enumerate(signed):
        for y in signed[i:]:
            pair.add(add(x,y))
    nonzero = sorted(v for v in pair if v is not None)
    four = set(pair)
    for i,x in enumerate(nonzero):
        for y in nonzero[i:]:
            four.add(add(x,y))
    assert len(pair) == 2*len(points)**2+1
    assert len(four) <= upper_bound(len(points))
    sha = hashlib.sha256(
        ''.join(f'{x},{y}\n' for x,y in sorted(v for v in four if v is not None)).encode()
    ).hexdigest()
    return {
        "base_size":len(points),
        "signed_base_size":len(signed),
        "two_support":len(pair),
        "four_support":len(four),
        "combinatorial_max":upper_bound(len(points)),
        "exact_uniform_target_probability":len(four)/N,
        "expected_uniform_queries_per_hit":N/len(four),
        "two_support_pair_additions":len(signed)*(len(signed)+1)//2,
        "four_support_pair_additions":len(nonzero)*(len(nonzero)+1)//2,
        "support_sha256":sha,
    }


def check_relation(points, row):
    acc = None
    for index, coeff in row:
        x,y = points[index-1]
        q = (x,y if coeff > 0 else (-y)%P)
        for _ in range(abs(coeff)):
            acc = add(acc,q)
    assert acc is None


def rank(rows):
    mat = [[0]*31 for _ in rows]
    for k,row in enumerate(rows):
        for i,v in row:
            mat[k][i-1] = (mat[k][i-1]+v)%N
    r = 0
    for c in range(31):
        pivot = next((k for k in range(r,len(mat)) if mat[k][c]),None)
        if pivot is None:continue
        mat[pivot],mat[r] = mat[r],mat[pivot]
        inv = pow(mat[r][c],-1,N)
        mat[r] = [v*inv%N for v in mat[r]]
        for k in range(len(mat)):
            if k != r and mat[k][c]:
                f=mat[k][c]
                mat[k]=[(x-f*y)%N for x,y in zip(mat[k],mat[r])]
        r += 1
        if r==len(mat):break
    return r


def run():
    seed,points=base()
    assert seed==3 and len(points)==31
    result = {"standing":"[pcc:unverified]",
              "method":"generic exact group-support enumeration — NEGATIVE CONTROL, not index calculus",
              "curve_p":P,"curve_a":A,"curve_b":B,"order":N,
              "four_term_coefficient_bound":"1+4B+16*C(B,2)+24*C(B,3)+16*C(B,4)",
              "canonical":[]}
    frozen=[(4,225,"3194dc2f56a420ae37052d7d69c020a6e9d5920439f45e1499cb3ab3fc45d9d3"),
            (8,2945,"5ed3b3754ff1cec8a63e803b509b55888fa6fa87b5beeb9cb139c1eedb43bafa"),
            (16,44545,"9a09d07322773e30c365d13cccf4fe343ae2cbb150fad5294c5fc36ef72bc0d0"),
            (31,618885,"8cb1a70c7b96983b7cfc17e809959a93248db98298e38748719537d26468df89")]
    for k,count,sha in frozen:
        item=census(points[:k])
        assert item["four_support"]==count and item["support_sha256"]==sha
        result["canonical"].append(item)
    seed7,points7=base(7)
    assert seed7==3 and len(points7)==64
    yunique=[q for q in points7 if sqrt((-3*q[0]*q[0]-4*A)%P) is None]
    assert len(yunique)==39
    injective=yunique[:31]
    item=census(injective)
    assert item["four_support"]==618625
    assert item["support_sha256"]=="6a962ddc5a03c26086a0eeaf1012e49a2b15a56edf3b64ae31a06ba6284aef4c"
    for row in ROWS:check_relation(injective,row)
    assert rank(ROWS)==4
    item.update({"selection":"first31 unique-y of depth7 Dickson fiber",
        "support_deficit_vs_max":upper_bound(31)-item["four_support"],
        "four_direct_replayed_zero_relations":len(ROWS),
        "independent_rank_of_frozen_relations":rank(ROWS)})
    result["injective_y_depth7"]=item
    return result


if __name__=="__main__":
    print(json.dumps(run(),sort_keys=True,indent=2))
