#!/usr/bin/env python3
"""Independent simple affine/BigInt replay of W1 collision certificates.
No Rust arithmetic, no factor-base discrete logarithms, no generic grading.
Usage: python3 verify_prime31_w8_heldout_collision.py evidence.json
"""
import json
import sys

P=2147483647
A=847514846
B=2087444724
N=2147446831
G=(625037715,2100627251)
W=(1,2,4,8,16,32,64,128)
WIN=(8,9,10,11,12,13,14,15)

def add(a,b):
    if a is None: return b
    if b is None: return a
    x1,y1=a; x2,y2=b
    if x1==x2 and (y1+y2)%P==0: return None
    if a==b:
        if y1==0: return None
        slope=((3*x1*x1+A)*pow(2*y1,-1,P))%P
    else:
        slope=((y2-y1)*pow(x2-x1,-1,P))%P
    x3=(slope*slope-x1-x2)%P
    return (x3,(slope*(x1-x3)-y1)%P)

def mul(k,q):
    out=None
    while k:
        if k&1: out=add(out,q)
        q=add(q,q)
        k>>=1
    return out

def base_points():
    out=[]
    x=0
    while len(out)<31:
        yy=(x*x*x+A*x+B)%P
        y=pow(yy,(P+1)//4,P)
        if (y*y)%P==yy and y!=0:
            y=min(y,P-y)
            out.append((x,y))
        x+=1
    return out

def word_summary(word, half, points):
    v=[0]*8
    out=None
    for j in range(half*4,half*4+4):
        digit=word&15
        word >>= 4
        i=digit&7
        sign=1 if digit<8 else -1
        v[i]+=sign*W[j]
        q=mul(W[j],points[WIN[i]])
        if sign<0:q=(q[0],(-q[1])%P)
        out=add(out,q)
    return v,out

def main(path):
    doc=json.load(open(path))
    assert doc["schema"]=="prime31.w8.heldout-collision.v1"
    assert doc["curve"]["p"]==P and doc["curve"]["n"]==N
    assert doc["targets_seed"]=="48454c444f555431"
    assert doc["target_count"]==1024
    assert mul(N,G) is None
    pts=base_points()
    assert [x for x,y in pts[:8]]==[0,1,2,3,5,6,8,9]
    assert [x for x,y in pts]==doc["base_x"]
    verified=0
    for i,h in enumerate(doc["w1_power2_collision_certificates"]["halves"]):
        assert h["half"]==i and h["unique_sums"]==65496
        assert h["formal_duplicate_words"]==0 and h["accidental_duplicate_words"]==40
        for c in h["first_12_certificates"]:
            coeff1,q1=word_summary(c["first_word"],i,pts)
            coeff2,q2=word_summary(c["colliding_word"],i,pts)
            assert coeff1==c["first_coeff"] and coeff2==c["colliding_coeff"]
            assert coeff1 != coeff2
            assert q1==q2
            assert list(q1) == c["sum"]
            verified+=1
    counts=doc["paired_total"]
    assert counts["old_hits"]==counts["both"]+counts["old_only"]
    assert counts["powers_two_hits"]==counts["both"]+counts["new_only"]
    assert sum(counts[k] for k in ("both","old_only","new_only","neither"))==1024
    print(json.dumps({"independent_affine_verified_certificates":verified,
        "heldout_paired":counts,"status":"PASS"},sort_keys=True))

if __name__=="__main__":
    main(sys.argv[1])
