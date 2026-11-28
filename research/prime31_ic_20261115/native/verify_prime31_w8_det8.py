#!/usr/bin/env python3
"""Independent Python BigInt verification of the ORIGINAL Prime31 W8 det8 Rust record.
Never consumes producer witness points or producer elliptic arithmetic code.
This verifies supplied-leaf predicates, not an unknown-ECDLP attack.
"""
import json
import sys

P=2147483647
A=847514846
B=2087444724
N=2147446831
G=(625037715,2100627251)
W=(1,3,5,7,11,13,17,19)

def neg(q):
    return None if q is None else (q[0], (-q[1]) % P)

def ec_add(u,v):
    if u is None: return v
    if v is None: return u
    x,y=u; z,t=v
    if x==z and (y+t)%P==0: return None
    if x==z:
        m=((3*x*x+A)*pow(2*y,-1,P))%P
    else:
        m=((t-y)*pow((z-x)%P,-1,P))%P
    rx=(m*m-x-z)%P
    return (rx,(m*(x-rx)-y)%P)

def scalar(k,u):
    out=None
    while k:
        if k&1: out=ec_add(out,u)
        u=ec_add(u,u); k>>=1
    return out

def base31():
    out=[]
    for x in range(10000):
        rhs=(x*x*x+A*x+B)%P
        if rhs==0: continue
        y=pow(rhs,(P+1)//4,P)
        if y*y%P==rhs:
            out.append((x,min(y,P-y)))
        if len(out)==31:break
    assert len(out)==31
    return out

def det(rows):
    a=[list(row) for row in rows]
    n=len(a); out=1
    for c in range(n):
        k=next((i for i in range(c,n) if a[i][c]!=0),None)
        if k is None: return 0
        if k!=c:
            a[c],a[k]=a[k],a[c]; out=(-out)%P
        pivot=a[c][c]; out=out*pivot%P
        ip=pow(pivot,-1,P)
        for i in range(c+1,n):
            q=a[i][c]*ip%P
            if q:
                for j in range(c+1,n):
                    a[i][j]=(a[i][j]-q*a[c][j])%P
            a[i][c]=0
    return out

def row8(q,r):
    x,y=q;rx,ry=r
    pw=[1,x,x*x%P,x*x*x%P]
    return [((x-rx)*z)%P for z in pw]+[((y+ry)*z)%P for z in pw]

def row9(q):
    x,y=q;x2=x*x%P;x3=x2*x%P
    return [1,x,y,x2,x*y%P,x3,x2*y%P,x3*x%P,x3*y%P]

def pair(pts,r):
    return det([row8(q,r) for q in pts]),det([row9(q) for q in list(pts)+[neg(r)]])

def main(path):
    data=json.load(open(path,encoding="utf8"))
    assert data["schema"]=="prime31.w8.target-reduced-det8.v1"
    assert (data["p"],data["a"],data["b"],data["n"],data["cofactor"])==(P,A,B,N,1)
    assert data["weights"]==list(W)
    base=base31()
    assert data["base_x"]==[p[0] for p in base]
    slots=[[scalar(w,q) for q in base] for w in W]
    assert len({q[0] for row in slots for q in row})==248
    assert len(data["cases"])==48
    true_count=false_count=0
    for t in range(16):
        original=[]
        for j in range(8):
            idx=(3*t+7*j+j*j)%31
            q=slots[j][idx]
            original.append(q if (t+3*j)%4<2 else neg(q))
        r=None
        for q in original:r=ec_add(r,q)
        assert r is not None
        flip=original[:];flip[0]=neg(flip[0])
        shifted=ec_add(r,G)
        for k,(name,pts,target,expect_zero) in enumerate((
            ("positive",original,r,True),
            ("first_sign_flipped",flip,r,False),
            ("residual_plus_g",original,shifted,False)
        )):
            assert target is not None
            assert len(set(pts+[neg(target)]))==9
            s=None
            for q in pts:s=ec_add(s,q)
            assert (s==target)==expect_zero
            d8,d9=pair(pts,target)
            assert d8==d9
            assert (d8==0)==expect_zero
            item=data["cases"][3*t+k]
            assert (item["fixture"],item["kind"])==(t,name)
            assert item["target"]==list(target)
            assert item["det8"]==d8 and item["det9"]==d9
            assert all(isinstance(item[field],int) and item[field]>=0 for field in
                ("fp8_multiplications","fp8_inversions",
                 "fp9_multiplications","fp9_inversions"))
            true_count+=int(expect_zero)
            false_count+=int(not expect_zero)
    assert (true_count,false_count)==(16,32)
    assert data["exact_identity_checks"]==48
    assert data["positive_zero_checks"]==true_count
    assert data["negative_nonzero_checks"]==false_count
    pts=[slots[j][(7*j+j*j)%31] if (3*j)%4<2 else neg(slots[j][(7*j+j*j)%31]) for j in range(8)]
    r=None
    for q in pts:r=ec_add(r,q)
    wrong=ec_add(r,G)
    duplicate=pts[:]; duplicate[1]=duplicate[0]
    assert len(set(duplicate+[neg(wrong)]))<9
    assert pair(duplicate,wrong)==(0,0)
    negative=pts[:];negative[0]=neg(wrong)
    assert len(set(negative+[neg(wrong)]))<9
    assert pair(negative,wrong)==(0,0)
    assert all(data[k] is True for k in
        ("duplicate_false_zero_rejected","target_point_false_zero_rejected"))
    assert data["unknown_chart_extractions"]==0
    assert data["complete_unknown_ecdlp_solves"]==0
    print("PASS: 48/48 exact original-curve det8=det9 residues, 16 positive, "
          "32 negative, two explicit rejected degeneracies")

if __name__=="__main__":
    if len(sys.argv)!=2:
        sys.exit("usage: verify_prime31_w8_det8.py RUST_RECORD.json")
    main(sys.argv[1])
