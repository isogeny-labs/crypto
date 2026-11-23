#!/usr/bin/env python3
"""Independent affine check of the *Rust*-generated Prime31 W8 blockscale census.
Not a decomposition solver, not a generic grader and not used by the measured arm.
It reconstructs blinded public targets, 20 witness equations and all summary totals.
"""
import json
import sys

P, A, B = 2147483647, 847514846, 2087444724
N = 2147446831
G = (625037715, 2100627251)
K = [4, 5, 6, 7, 8]
SEED = 0x424c4f434b534341
MASK = (1 << 64) - 1
WINS = [
    list(range(0,8)),list(range(8,16)),list(range(16,24)),
    list(range(23,31)),
]

def add(u, v):
    if u is None: return v
    if v is None: return u
    x,y=u
    z,w=v
    if x==z and (y+w)%P==0: return None
    slope = ((3*x*x+A) * pow(2*y,-1,P) if x==z
             else (w-y)*pow(z-x,-1,P)) % P
    xr=(slope*slope-x-z)%P
    return (xr,(slope*(x-xr)-y)%P)

def scalar(q,k):
    r=None
    while k:
        if k&1: r=add(r,q)
        q=add(q,q)
        k>>=1
    return r

def good(q):
    return q is None or (
        0<=q[0]<P and 0<=q[1]<P and
        (q[1]*q[1]-q[0]**3-A*q[0]-B)%P==0)

def splitmix64(state):
    state=(state+0x9e3779b97f4a7c15)&MASK
    z=state
    z=((z^(z>>30))*0xbf58476d1ce4e5b9)&MASK
    z=((z^(z>>27))*0x94d049bb133111eb)&MASK
    return state,z^(z>>31)

def nonzero(state):
    threshold=(1<<64)%N
    while True:
        state,r=splitmix64(state)
        if r>=threshold and r%N!=0: return state,r%N

def point(v):
    return None if v is None else tuple(v)

def word_sum(word, indices, base):
    out=None
    for weight in [1,2,4,8]:
        choice=word&15
        word>>=4
        p=base[indices[choice&7]]
        if choice>=8: p=(p[0],(-p[1])%P)
        out=add(out,scalar(p,weight))
    assert word==0
    return out

def stat(v):
    hist=[0]*7
    for m in v:
        assert isinstance(m,int) and m>=0
        b=0 if m==0 else 1 if m==1 else 2 if m==2 else 3 if m==3 \
            else 4 if m<=7 else 5 if m<=15 else 6
        hist[b]+=1
    return {"hits":len(v)-hist[0],"misses":hist[0],
            "total_witnesses":sum(v),
            "witness_square_sum":str(sum(x*x for x in v)),
            "max":max(v),"hist_0_1_2_3_4to7_8to15_16plus":hist}

def verify(path):
    with open(path,encoding="utf-8") as f: d=json.load(f)
    assert d["schema"]=="prime31.w8.blockscale.v1"
    assert [d["curve"][k] for k in ("p","a","b","n","cofactor")] == [P,A,B,N,1]
    assert point(d["curve"]["generator"])==G
    assert scalar(G,N) is None
    assert d["k_values"]==K
    assert d["seed"]==f"{SEED:016x}"
    assert d["generic_four_plus_four_grading_only"] is True
    assert d["nonenumerative_index_extraction"] is False
    assert d["complete_ecdlp_solved"] is False
    assert d["pcc_verified"] is False
    per=d["samples_per_window"]
    assert 1<=per<=128
    base=[point(q) for q in d["base"]]
    assert len(base)==31 and all(good(q) and q is not None for q in base)
    state=SEED
    targets=[]
    for _ in range(4*per):
        state,k=nonzero(state)
        targets.append(scalar(G,k))
    pooled=[[] for _ in K]
    paired=[[0]*5 for _ in K]
    for wi,row in enumerate(d["per_window"]):
        assert row["window"]==wi and row["indices"]==WINS[wi]
        assert [point(x) for x in row["targets"]]==targets[wi*per:(wi+1)*per]
        assert row["shared_four_slot_words"]==65536
        unique=row["shared_four_slot_unique"]
        assert 0 < unique<=65536
        assert len(row["arms"])==len(row["multiplicity_per_target"])==5
        for ai,k in enumerate(K):
            arm=row["arms"][ai]
            values=row["multiplicity_per_target"][ai]
            assert arm["k"]==k
            assert arm["weights"]==[1,2,4,8,1<<k,2<<k,4<<k,8<<k]
            assert arm["four_slot_unique"]==arm["right_unique"]==unique
            assert len(values)==per
            assert arm["stats"]==stat(values)
            pooled[ai].extend(values)
        for a in range(5):
            for b in range(5):
                paired[a][b]+=sum(u==0 and v>0 for u,v in
                                    zip(row["multiplicity_per_target"][a],
                                        row["multiplicity_per_target"][b]))
    assert d["pairwise_a_miss_b_hit"]==paired
    assert len(d["pooled"])==5
    for ai,item in enumerate(d["pooled"]):
        assert item["k"]==K[ai]
        assert item["stats"]==stat(pooled[ai])
    assert len(d["certificates"])==20
    seen=set()
    for cert in d["certificates"]:
        w,k=cert["window"],cert["k"]
        assert (w,k) not in seen and w in range(4) and k in K
        seen.add((w,k))
        qi=cert["q_index"]
        assert 0<=qi<per and cert["window_indices"]==WINS[w]
        q=targets[w*per+qi]
        assert point(cert["target"])==q and good(q)
        u=word_sum(cert["left_word"],WINS[w],base)
        v=word_sum(cert["right_word"],WINS[w],base)
        assert add(u,scalar(v,1<<k))==q
        assert d["per_window"][w]["multiplicity_per_target"][K.index(k)][qi]>0
    assert len(seen)==20
    return {"curve":"prime31 original", "target_points":len(targets),
            "arms":K,"certificates":len(seen),
            "hits":{str(k):stat(pooled[i])["hits"] for i,k in enumerate(K)},
            "validated":"target construction, Fp EC witness, full-fibre summaries, paired outcomes",
            "nonenumerative_solver":False}

if __name__=="__main__":
    if len(sys.argv)!=2: raise SystemExit("Usage: python3 verify_prime31_w8_blockscale.py result.json")
    print(json.dumps(verify(sys.argv[1]),sort_keys=True))
