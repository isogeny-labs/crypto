#!/usr/bin/env python3
"""Independent mathematical replay for the original-Rust Prime31 x-only [16] certificate.

A validator only: not an ECDLP solver, extractor, or measured speed claim.
"""
import json
import sys

P=2147483647
N=2147446831
NT=2147520465
A=847514846
B=2087444724
D=3
MASK=(1<<64)-1
SEED=0x4b554d4d45523136

def mul(x,y):
    return x*y%P

def rhs(x,a=A,b=B):
    return (x*x*x+a*x+b)%P

def chi(x):
    y=rhs(x)
    assert y!=0, ("rational 2-torsion",x)
    z=pow(y,(P-1)//2,P)
    assert z in (1,P-1)
    return 1 if z==1 else -1

def twice_x(x):
    f=rhs(x)
    num=(x**4-2*A*x*x-8*B*x+A*A)%P
    return num*pow(4*f,-1,P)%P

def add_points(p,q,a):
    if p is None:return q
    if q is None:return p
    x,y=p
    z,w=q
    if x==z and (y+w)%P==0: return None
    if x==z:
        num=(3*x*x+a)%P
        den=2*y%P
    else:
        num=(w-y)%P
        den=(z-x)%P
    lam=num*pow(den,-1,P)%P
    xx=(lam*lam-x-z)%P
    yy=(lam*(x-xx)-y)%P
    return (xx,yy)

def scalar(q,k,a):
    r=None
    while k:
        if k&1:r=add_points(r,q,a)
        q=add_points(q,q,a)
        k//=2
    return r

def splitmix64(s):
    s=(s+0x9e3779b97f4a7c15)&MASK
    z=s
    z=(z^(z>>30))*0xbf58476d1ce4e5b9&MASK
    z=(z^(z>>27))*0x94d049bb133111eb&MASK
    return s, (z^(z>>31))&MASK

def inspect(j):
    assert j["schema"]=="prime31.kummer16.rational_fibre.v1"
    assert j["curve"]=={"p":P,"a":A,"b":B,"order":N,"cofactor":1}
    at=(A*D*D)%P
    bt=(B*D*D*D)%P
    assert j["twist"]=={"order":NT,"nonsquare":D,"a":at,"b":bt}
    assert j["inverse16"]=={"curve":pow(16,-1,N),"twist":pow(16,-1,NT)}
    assert (2*P+2-N)==NT
    assert pow(D,(P-1)//2,P)==P-1
    assert j["sample_seed"]==str(SEED)
    assert j["all_chain_and_inverse_assertions_passed"] is True
    assert j["generic_sum_oracle_used"] is False
    assert j["public_only_ecdlp_extractor"] is False
    assert j["complete_ecdlp_solved"] is False
    assert j["pcc_verified"] is False
    assert j["sample_counts"]=={"curve":512,"twist":512,"factor_base":31}
    base=[x for x in range(10000) if chi(x)==1][:31]
    assert j["factor_base_x"]==base
    assert base[:8]==[0,1,2,3,5,6,8,9]
    cert=j["certificates"]
    assert len(cert)==64
    byclass={"curve":[],"twist":[]}
    for row in cert:
        byclass[row["kind"]].append(row)
    assert len(byclass["curve"])==len(byclass["twist"])==32
    counts={"curve":0,"twist":0}
    s=SEED
    draws=0
    while counts["curve"]<512 or counts["twist"]<512:
        s,raw=splitmix64(s)
        x=raw%P
        draws+=1
        cls="curve" if chi(x)==1 else "twist"
        if counts[cls]==512:continue
        ordinal=counts[cls]
        if ordinal<32:
            row=byclass[cls][ordinal]
            assert (row["ordinal"],row["x"])==(ordinal,x)
        counts[cls]+=1
    assert j["random_x_draws"]==draws
    # Independent full elliptic-curve reconstruction, including twist via x'=3x.
    for cls, rows in byclass.items():
        for row in rows:
            x=row["x"]
            tx=x if cls=="curve" else x*D%P
            a=A if cls=="curve" else at
            b=B if cls=="curve" else bt
            r=rhs(tx,a,b)
            y=pow(r,(P+1)//4,P)
            assert y*y%P==r
            p0=(tx,y)
            x16=x
            for _ in range(4):
                x16=twice_x(x16)
                assert chi(x16)==chi(x)
            assert row["x16"]==x16
            q16=scalar(p0,16,a)
            assert q16 is not None
            assert q16[0]*(1 if cls=="curve" else pow(D,-1,P))%P==x16
            inv=pow(16,-1,N if cls=="curve" else NT)
            assert scalar(q16,inv,a)==p0
    return {"schema":"prime31.kummer16.independent_replay.v1",
            "status":"passed", "random_public_x":1024,
            "factor_base_x":31,"independent_full_ec_certificates":64,
            "verified_curve_certificates":32,
            "verified_twist_certificates":32,
            "random_x_draws":draws,
            "complete_ecdlp_solved":False,"pcc_verified":False}

def main():
    assert len(sys.argv)==2, "usage: validator path-to-producer.json"
    with open(sys.argv[1],encoding="utf-8") as f:
        j=json.load(f)
    print(json.dumps(inspect(j),sort_keys=True))
if __name__=="__main__":
    main()
