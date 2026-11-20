#!/usr/bin/env python3
"""Independent BigInt original-curve replay of B8 windowed generic oracle results.

Producer-only validation: not an independent signed PCC attestation.
Unlike the Rust producer, this Python implementation reconstructs every
public curve addition, all training linear equations and each target scalar.
"""
import json
import sys

P=2147483647
A=847514846
B=2087444724
N=2147446831
G=(625037715,2100627251)
W_ODD=[1,3,5,7,11,13,17,19]
W_POW2=[1,2,4,8,16,32,64,128]
WINDOWS=[
  list(range(0,8)), list(range(8,16)),list(range(16,24)),list(range(23,31))
]

def neg(q):
    if q is None:
        return None
    return q[0],(-q[1])%P

def add(u,v):
    if u is None: return v
    if v is None: return u
    x,y=u; z,w=v
    if x==z and (y+w)%P==0: return None
    if x==z:
        assert y!=0
        m=((3*x*x+A)*pow(2*y,P-2,P))%P
    else:
        m=((w-y)*pow(z-x,P-2,P))%P
    xx=(m*m-x-z)%P
    yy=(m*(x-xx)-y)%P
    return xx,yy

def scalar(k,q):
    result=None
    while k:
        if k&1: result=add(result,q)
        q=add(q,q)
        k>>=1
    return result

def point(q):
    return None if q is None else tuple(q)

def canonical():
    result=[]
    for x in range(10000):
        r=(x*x*x+A*x+B)%P
        y=pow(r,(P+1)//4,P)
        if y and y*y%P==r:
            result.append((x,min(y,P-y)))
            if len(result)==31: break
    assert len(result)==31
    assert [q[0] for q in result[:8]]==[0,1,2,3,5,6,8,9]
    return result

def slots(base,wi,weights):
    result=[]
    for j,w in enumerate(weights):
        plus=[scalar(w,base[i]) for i in WINDOWS[wi]]
        assert all(q is not None for q in plus)
        result.append(plus+[neg(q) for q in plus])
    abscissae=[q[0] for row in result for q in row[:8]]
    assert len(set(abscissae))==64
    return result

def replay(ss,word):
    assert len(word)==8
    result=None
    for j,i in enumerate(word):
        assert 0<=i<16
        result=add(result,ss[j][i])
    return result

def row(wi,word,weights):
    out=[0]*31
    for j,i in enumerate(word):
        idx=WINDOWS[wi][i%8]
        sign=1 if i<8 else -1
        out[idx]=(out[idx]+sign*weights[j])%N
    return out

def independent_elimination(eqs):
    a=[(list(coeffs)+[rhs]) for coeffs,rhs in eqs]
    pivots=[]
    rank=0
    for j in range(31):
        pivot=next((k for k in range(rank,len(a)) if a[k][j]),None)
        if pivot is None: continue
        a[rank],a[pivot]=a[pivot],a[rank]
        inverse=pow(a[rank][j],N-2,N)
        a[rank]=[(v*inverse)%N for v in a[rank]]
        for i in range(len(a)):
            if i==rank: continue
            factor=a[i][j]
            if factor:
                a[i]=[(a[i][k]-factor*a[rank][k])%N for k in range(32)]
        pivots.append(j)
        rank+=1
    assert rank==31
    for i in range(rank,len(a)):
        assert a[i]==[0]*32
    result=[None]*31
    for i,j in enumerate(pivots):
        assert a[i][:31]==[1 if k==j else 0 for k in range(31)]
        result[j]=a[i][31]
    assert all(x is not None for x in result)
    return result

def verify(data):
    assert data["schema"]=="prime31.w8.windowed-generic-oracle-full-log.v1"
    assert data["generic_four_plus_four_grader_used"] is True
    assert data["genuine_nongeneric_rr9_extractors"]==0
    assert data["claim_of_cheaper_nongeneric_solver"] is False
    c=data["curve"]
    assert (c["p"],c["a"],c["b"],c["n"],c["cofactor"])==(P,A,B,N,1)
    assert point(c["g"])==G
    assert data["weights"] in (W_ODD,W_POW2)
    weights=data["weights"]
    assert data["windows"]==WINDOWS
    assert data["ordered_words_per_window"]==16**8
    base=canonical()
    assert [point(q) for q in data["base_points"]]==base
    ss=[slots(base,wi,weights) for wi in range(4)]

    eqs=[]
    ranks=[]
    for rec in data["accepted_natural_training_relations"]:
        wi=rec["window"]
        scalar_k=rec["known_offset_scalar"]
        public_r=point(rec["public_r"])
        assert public_r==scalar(scalar_k,G)
        assert replay(ss[wi],rec["word"])==public_r
        eqs.append((row(wi,rec["word"],weights),scalar_k))
        ranks.append(rec["rank"])
    assert ranks==list(range(1,32))
    logs=independent_elimination(eqs)
    assert logs==data["factor_base_logs"]
    for i,q in enumerate(base):
        assert scalar(logs[i],G)==q,("base log",i)

    stats=data["window_training"]
    assert len(stats)==4
    assert [v["cumulative_rank"] for v in stats]==[8,16,24,31]
    assert sum(v["independent"] for v in stats)==31
    assert sum(v["trials"] for v in stats)>=31
    assert sum(v["trials"] for v in stats)<=512

    targets=data["recovered_targets"]
    assert data["target_count"]==len(targets)==data["verified_target_count"]==1024
    assert data["online_queries"]==len(targets)+data["failed_online_queries"]
    for t,rec in enumerate(targets):
        assert rec["target_index"]==t
        wi=rec["window"]
        secret=rec["test_secret"]
        recovered=rec["log"]
        q=point(rec["public_q"])
        shift=rec["offset"]
        assert q==scalar(secret,G)==scalar(recovered,G)
        assert secret==recovered
        assert replay(ss[wi],rec["word"])==add(q,scalar(shift,G))
        coeff=row(wi,rec["word"],weights)
        candidate=(sum(x*y for x,y in zip(coeff,logs))-shift)%N
        assert candidate==secret,("recovered wrong",t)
        assert rec["failed_prior_queries"]<32

    assert data["rank"]==31
    assert len(targets)==1024
    print(json.dumps({"independent_python_bigint":"PASS",
        "train_rank":31,"learned_base_logs":31,
        "public_targets_verified":len(targets),
        "all_training_witnesses_replayed":len(eqs),
        "generic_oracle_only":True,
        "signed_pcc":"unverified"}))

if __name__=="__main__":
    if len(sys.argv)!=2:
        raise SystemExit("usage: verify_prime31_w8_windowed_rank.py JSON_FILE")
    with open(sys.argv[1],encoding="utf-8") as handle:
        verify(json.load(handle))
