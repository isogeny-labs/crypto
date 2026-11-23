#!/usr/bin/env python3
"""Producer-independent-language prime31 S4 triple-root replay.

Independent Python field and elliptic-curve formulas. No Rust result file,
secret discrete logarithm or generic group lookup drives the algebraic roots.
This is producer-local cross-check [pcc:unverified], not signed PCC evidence.
"""
import hashlib
import itertools
import json
from collections import Counter

P = 2**31 - 1
A = 847514846
B = 2087444724
ORDER = 2147446831

def root(v):
    v %= P
    s = pow(v, (P + 1)//4, P)
    return s if s*s % P == v else None

def s3_rational(x, y):
    xy = x*y % P
    a = (x-y)**2 % P
    b = -2*((x+y)*(xy+A)+2*B) % P
    c = ((xy-A)**2 - 4*B*(x+y)) % P
    assert a != 0, (x,y)
    s = root((b*b-4*a*c) % P)
    assert s is not None and s != 0, (x,y)
    denominator = pow((2*a) % P, P-2, P)
    rr = sorted({((-b+s)*denominator) % P,
                 ((-b-s)*denominator) % P})
    assert len(rr)==2
    assert all((a*r*r+b*r+c) % P == 0 for r in rr)
    return rr

def ec_add(Q, R):
    if Q is None: return R
    if R is None: return Q
    x,y=Q; z,w=R
    if x==z and (y+w)%P==0: return None
    num = (3*x*x+A) if x==z else w-y
    den = (2*y) if x==z else z-x
    assert den%P
    t = (num * pow(den%P, P-2, P)) % P
    u = (t*t-x-z)%P
    return (u,(t*(x-u)-y)%P)

def neg(Q):
    return None if Q is None else (Q[0],(-Q[1])%P)

def main():
    c=3
    for _ in range(6): c=(c*c-2)%P
    assert c==933677148
    fibre=[c]
    for _ in range(6):
        ss=[]
        for v in fibre:
            r=root(v+2)
            assert r is not None and r
            ss.extend((r,(-r)%P))
        fibre=sorted(set(ss))
    assert len(fibre)==64
    base=[]
    for x in fibre:
        y=root((x*x*x+A*x+B)%P)
        if y is not None and y:
            base.append((x,min(y,P-y)))
    assert len(base)==31

    pair=dict()
    for i,j in itertools.combinations(range(len(base)),2):
        pair[(i,j)]=s3_rational(base[i][0],base[j][0])
    hit=Counter()
    checked=0
    for i,j,k in itertools.combinations(range(len(base)),3):
        x=base[k][0]
        rr=sorted(r for w in pair[(i,j)] for r in s3_rational(w,x))
        assert len(rr)==4 and len(set(rr))==4
        for r in rr: hit[r]+=1
        # Standalone group-law cross-check, performed only after S3 roots.
        s=ec_add(base[i],base[j])
        d=ec_add(base[i],neg(base[j]))
        gs=[ec_add(t,pk) for t in (s,d)
            for pk in (base[k],neg(base[k]))]
        assert all(Q is not None for Q in gs)
        assert rr==sorted(Q[0] for Q in gs)
        checked+=4
    assert checked==17980 and len(hit)==17980
    assert set(hit.values())=={1}
    values=sorted(hit)
    blob=b"".join(int(x).to_bytes(4,"little") for x in values)
    assert hashlib.sha256(blob).hexdigest()==(
        "a71a99940794aead48642690575a37be72ef9994a93c9bbace47b538307d126d")
    h=0xcbf29ce484222325
    for b in blob:
        h=((h^b)*0x100000001b3)&((1<<64)-1)
    assert h==0xcc15d719e3eaf8c2
    assert values[:6]==[88176,222548,287120,464420,492148,1073855]
    assert values[-6:]==[2146230119,2146244992,2146280773,2146873560,2146899553,2147179629]
    assert not (set(range(256)) & set(values))
    print(json.dumps({
        "standing":"[pcc:unverified]",
        "p":P,"a":A,"b":B,"n":ORDER,
        "base_points":len(base),
        "triple_count":4495,
        "nested_s3_quadratic_calls":465+2*4495,
        "signed_triple_x_roots":checked,
        "distinct_roots":len(hit),
        "cross_triple_x_collisions":sum(c-1 for c in hit.values()),
        "field_roots_sha256_le32":hashlib.sha256(blob).hexdigest(),
        "field_roots_fnv1a_le32":f"{h:016x}",
        "ec_semantic_checks":checked,
        "claim":"stage-only; no unknown scalar or complete index calculus solved"
    },sort_keys=True))
if __name__=="__main__": main()
