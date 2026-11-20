#!/usr/bin/env python3
"""Independent BigInt group replay for the original-Rust signed W8 RR9 grading oracle.

This script is deliberately a checker of the GENERIC 4+4 grader, not an
alternative solver. It independently recomputes 16 public targets and all
8 planted targets for each B4/B8 panel; the other 48 public rows per panel
are checked for schema, consistency and scalar-derived target, not full count.
"""
import itertools
import json
import sys
from collections import Counter
from verify_prime31_rr9_weighted_resultants import (
    P, A, B, N, G, W, add, scalar, base, curve
)

MASK64=(1<<64)-1
SEED=0x7738423442383231

def neg(q):
    return None if q is None else (q[0], -q[1] % P)

def splitmix64(s):
    s=(s+0x9e3779b97f4a7c15)&MASK64
    z=s
    z=((z^(z>>30))*0xbf58476d1ce4e5b9)&MASK64
    z=((z^(z>>27))*0x94d049bb133111eb)&MASK64
    return s,(z^(z>>31))&MASK64

def next_scalar(s):
    # threshold == (2^64 mod N), matching the Rust rejection scheme.
    threshold=(1<<64)%N
    while True:
        s,z=splitmix64(s)
        if z>=threshold and z%N:
            return s,z%N

def signed_slots(b):
    points=base()[:b]
    slots=[]
    for w in W:
        plus=[scalar(w,p) for p in points]
        assert all(curve(q) and q is not None for q in plus)
        slots.append(plus+[neg(q) for q in plus])
    xs=[q[0] for slot in slots for q in slot[:b]]
    # Plus and minus share an x by design; require disjoint abscissae
    # only across the eight distinct positive weighted slot images.
    assert len(xs)==len(set(xs)), "weighted x root sets must be disjoint"
    return points,slots

def half(slots,offset,b):
    m=2*b
    words=[]
    sums=Counter()
    formal=set()
    # Recompute all 4-slot EC words with independent Python arithmetic.
    for i in range(m):
        for j in range(m):
            s2=add(slots[offset][i],slots[offset+1][j])
            for k in range(m):
                s3=add(s2,slots[offset+2][k])
                for l in range(m):
                    q=add(s3,slots[offset+3][l])
                    w=(i,j,k,l)
                    words.append((q,w))
                    sums[q]+=1
                    c=[0]*b
                    for jj,index in enumerate(w):
                        c[index%b] += W[offset+jj]*(1 if index<b else -1)
                    formal.add(tuple(c))
    assert len(words)==m**4
    assert len(sums)==len(formal)
    return words,sums,len(formal)

def replay(slots,w):
    q=None
    for j,i in enumerate(w):
        q=add(q,slots[j][i])
    return q

def oracle(left,right,slots,r):
    total=0
    first=None
    for q,w in left:
        complement=add(r,neg(q))
        hits=right.get(complement,0)
        if hits:
            total+=hits
            if first is None:
                first=w # first 4 only; first full witness not required to match producer
    return total

def readpoint(record):
    return None if record is None else tuple(record)

def verify_one(row,slots,r,total):
    assert readpoint(row["target"])==r
    assert row["multiplicity"]==total
    w=row["witness"]
    assert (w is None)==(total==0)
    if w is not None:
        assert len(w)==8
        assert all(0<=z<2*len(slots[0])//2 for z in w)
        assert replay(slots,w)==r, "invalid original-group witness"

def check_case(case):
    b=case["base_size"]
    assert b in (4,8)
    points,slots=signed_slots(b)
    assert case["base_x"]==[q[0] for q in points]
    left,leftmap,lf=half(slots,0,b)
    right,rightmap,rf=half(slots,4,b)
    expected=3921 if b==4 else 64737
    for f,val in [
        ("half_words",(2*b)**4),
        ("left_group_support",len(leftmap)),
        ("right_group_support",len(rightmap)),
        ("left_formal_support",lf),
        ("right_formal_support",rf),
        ("unavoidable_formal_redundancy_per_half",(2*b)**4-expected),
        ("full_eight_word_count",(2*b)**8),
        ("full_group_first_moment_numerator",(2*b)**8),
        ("full_group_first_moment_denominator",N)
    ]:
        assert case[f]==val,(b,f,val,case[f])
    assert lf==rf==expected
    u=case["uniform_trials"]
    t=case["planted_trials"]
    assert len(u)==64 and len(t)==8
    assert sum(row["multiplicity"]>0 for row in u)==case["uniform_hits"]
    assert sum(row["multiplicity"] for row in u)==case["uniform_multiplicity_sum"]
    state=SEED^b
    for j,row in enumerate(u):
        state,k=next_scalar(state)
        r=scalar(k,G)
        assert readpoint(row["target"])==r,(b,j,"wrong target")
        if j<16:
            verify_one(row,slots,r,oracle(left,rightmap,slots,r))
    for j,row in enumerate(t):
        indices=[(3*j+7*z+z*z)%b for z in range(8)]
        word=[i+(0 if (j+3*z)%4<2 else b)
              for z,i in enumerate(indices)]
        r=replay(slots,word)
        assert r is not None
        total=oracle(left,rightmap,slots,r)
        assert total>0
        verify_one(row,slots,r,total)
    assert int(case["original_curve_ec_additions_total"])>0
    print(f"PASS B{b}: {len(leftmap)}/{(2*b)**4} distinct half sums, "
          "all 64 public target identities, 16 exact counts, "
          "8 planted exact counts and witness replay")

def validate(record):
    assert record["schema"]=="prime31.w8.signed-4plus4-grader.v1"
    assert record["curve"]=={"p":P,"a":A,"b":B,"n":N,"cofactor":1}
    assert scalar(N,G) is None and curve(G)
    assert record["oracle_only"] is True
    assert record["generic_four_plus_four_tables"] is True
    assert record["unknown_rr9_chart_extractions"]==0
    assert record["complete_unknown_ecdlp_solves"]==0
    assert record["pcc"]=="unverified"
    assert [c["base_size"] for c in record["cases"]]==[4,8]
    for c in record["cases"]:
        check_case(c)

if __name__=="__main__":
    with open(sys.argv[1],encoding="utf8") as f:
        validate(json.load(f))
