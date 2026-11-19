#!/usr/bin/env python3
"""Independent Python-bigint original Prime31 F3 gate replay. No Rust source import."""
import json
import sys
from pathlib import Path

P=2147483647
A=847514846
B=2087444724
N=2147446831
G=(625037715,2100627251)
SEED=0x4633524f4f543331
MASK=(1<<64)-1

def rhs(x): return (x*x*x+A*x+B)%P
def neg(Q): return None if Q is None else (Q[0],(-Q[1])%P)
def ecadd(U,V):
    if U is None: return V
    if V is None: return U
    x,y=U
    z,w=V
    if x==z and (y+w)%P==0: return None
    if x==z:
        m=((3*x*x+A)*pow(2*y,-1,P))%P
    else:
        m=((w-y)*pow((z-x)%P,-1,P))%P
    xx=(m*m-x-z)%P
    yy=(m*(x-xx)-y)%P
    return xx,yy
def scalar(k,Q):
    out=None
    while k:
        if k&1: out=ecadd(out,Q)
        Q=ecadd(Q,Q)
        k>>=1
    return out
def splitmix(seed):
    seed=(seed+0x9e3779b97f4a7c15)&MASK
    z=seed
    z=((z^(z>>30))*0xbf58476d1ce4e5b9)&MASK
    z=((z^(z>>27))*0x94d049bb133111eb)&MASK
    return seed,z^(z>>31)
def draw_scalar(seed):
    threshold=(1<<64)%N
    while True:
        seed,raw=splitmix(seed)
        if raw>=threshold and raw%N:
            return seed,raw%N
def base31():
    out=[]
    for x in range(10000):
        y=pow(rhs(x),(P+1)//4,P)
        if (y*y)%P==rhs(x) and y:
            out.append((x,min(y,P-y)))
            if len(out)==31: return out
    raise AssertionError("missing factor base")
def sample_panel(base):
    seed=SEED
    cases=[]
    while len(cases)<64:
        seed,v=splitmix(seed); i=v%31
        seed,v=splitmix(seed); j=v%31
        seed,v=splitmix(seed); si=-1 if v&1 else 1
        seed,v=splitmix(seed); sj=-1 if v&1 else 1
        q=ecadd(base[i] if si==1 else neg(base[i]),
                base[j] if sj==1 else neg(base[j]))
        if q is not None: cases.append(("blinded_planted",q))
    for _ in range(256):
        seed,k=draw_scalar(seed)
        cases.append(("natural_unlabelled",scalar(k,G)))
    for q in base: cases.append(("degenerate_x",q))
    return cases
def f3_coeffs(u,r):
    aa=(r-u)**2%P
    bb=(-2*((r+u)*(r*u+A)+2*B))%P
    cc=((r*u-A)**2-4*B*(r+u))%P
    return aa,bb,cc
def recover(q,base,xlookup):
    found=set()
    r,s=q
    for i,(x,y) in enumerate(base):
        aa,bb,cc=f3_coeffs(x,r)
        discr=(bb*bb-4*aa*cc)%P
        ds=4*s*y%P
        assert discr==ds*ds%P
        if aa:
            inv=pow(2*aa,-1,P)
            roots=[(-bb+ds)*inv%P,(-bb-ds)*inv%P]
        else:
            assert bb!=0
            roots=[-cc*pow(bb,-1,P)%P]
        # Strong independent oracle: recompute the two actual group sums
        # with different Python affine formulas, excluding infinity.
        sums=[ecadd(q,base[i]),ecadd(q,neg(base[i]))]
        group_x={t[0] for t in sums if t is not None}
        assert set(roots)==group_x, (q,i,roots,group_x)
        for t in roots:
            if t not in xlookup: continue
            j=xlookup[t]
            for si in (-1,1):
                for sj in (-1,1):
                    pi=base[i] if si==1 else neg(base[i])
                    pj=base[j] if sj==1 else neg(base[j])
                    if ecadd(pi,pj)==q: found.add((i,si,j,sj))
    return sorted(found)

def main(path):
    j=json.loads(Path(path).read_text())
    assert j["schema"]=="prime31.semaev.f3.batch_root.v1"
    assert (j["curve"]["p"],j["curve"]["a"],j["curve"]["b"],j["curve"]["n"])==(P,A,B,N)
    base=base31()
    assert [list(x) for x in base]==j["base"]
    cases=sample_panel(base)
    assert len(cases)==351
    assert [[c[0],list(c[1])] for c in cases]==[
        [c["kind"],c["q"]] for c in j["samples"]]
    xlookup={q[0]:i for i,q in enumerate(base)}
    match=[]
    for k,(kind,q) in enumerate(cases):
        actual=recover(q,base,xlookup)
        expected=[tuple(t) for t in j["full_signed_witnesses"][k]]
        assert actual==expected, (k,expected,actual)
        match.append(actual)
    for name,lo,hi in (("planted",0,64),("natural",64,320),("degenerate",320,351)):
        cohort=j["cohorts"][name]
        assert cohort["hits"]==sum(bool(a) for a in match[lo:hi])
        assert cohort["misses"]==sum(not a for a in match[lo:hi])
        assert cohort["ordered_signed_witnesses"]==sum(len(a) for a in match[lo:hi])
    assert all(match[:64])
    assert j["field_ops_f3"]["inversions"]==351
    assert j["direct_batch_field_ops"]["inversions"]==351
    assert j["direct_batch_degenerate_linear_roots"]>=31
    assert j["existing_prime_fast_direct_batch_method"] is True
    assert j["f3_degenerate_linear_roots"]>=31
    assert j["forbidden_group_sum_tables"]==0
    assert not j["complete_ecdlp_solved"] and not j["pcc_verified"]
    print(json.dumps({"independent_python_replay":"PASS",
          "cases":351,"planted_verified":64,"natural_hits":j["cohorts"]["natural"]["hits"],
          "degenerate_cases":31,"all_signed_fibres_exact":True,
          "field_inversions":j["field_ops_f3"]["inversions"],
          "direct_batch_inversions":j["direct_batch_field_ops"]["inversions"],
          "f3_median_ns":j["f3_median_ns"],
          "direct_batch_median_ns":j["direct_batch_median_ns"],
          "negative_queries_fully_checked":j["cohorts"]["natural"]["misses"]},sort_keys=True))

if __name__=="__main__":
    if len(sys.argv)!=2: raise SystemExit("usage: verify_prime31_f3_batch_roots.py <raw-json>")
    main(sys.argv[1])
