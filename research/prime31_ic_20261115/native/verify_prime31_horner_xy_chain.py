#!/usr/bin/env python3
"""Independent BigInt affine EC replay of original-Rust Horner witness controls.

This script is a verifier only, not a candidate public-only relation extractor
or an ECDLP implementation.
"""
import json, sys
P = 2147483647
A = 847514846
B = 2087444724
N = 2147446831
G = (625037715,2100627251)

def rhs(x): return (x*x*x+A*x+B)%P
def neg(q): return None if q is None else (q[0],(-q[1])%P)
def add(p,q):
    if p is None: return q
    if q is None: return p
    x,y=p
    u,v=q
    assert 0<=x<P and 0<=y<P and y*y%P==rhs(x)
    assert 0<=u<P and 0<=v<P and v*v%P==rhs(u)
    if x==u and (y+v)%P==0: return None
    if x==u:
        assert y!=0
        k=(3*x*x+A)*pow(2*y,-1,P)%P
    else:
        k=(v-y)*pow(u-x,-1,P)%P
    z=(k*k-x-u)%P
    w=(k*(x-z)-y)%P
    assert w*w%P==rhs(z)
    return z,w

def times(k,p):
    acc=None
    while k:
        if k&1: acc=add(acc,p)
        p=add(p,p)
        k>>=1
    return acc

def base31():
    out=[]
    for x in range(10000):
        r=rhs(x)
        y=pow(r,(P+1)//4,P)
        if y*y%P==r and y:
            out.append((x,min(y,P-y)))
            if len(out)==31:break
    assert len(out)==31
    return out

def replay(path):
    d=json.load(open(path))
    assert d["schema"]=="prime31.horner_xy_chain.v1"
    assert d["curve"]["p"]==P and d["curve"]["n"]==N
    assert d["tested"]==d["ordinary"]+d["exceptional"]
    assert d["max_polynomial_identity_residual"]==0
    assert d["mutated_slope_rejected"]==d["ordinary"]
    assert d["mutated_target_same_witness_rejected"]==d["ordinary"]
    assert d["planted_witness_controls_only"] is True
    assert d["public_only_unknown_index_extraction"] is False
    assert d["complete_ecdlp_solved"] is False
    assert d["includes_generic_table_oracle"] is False
    assert d["polynomial_equations"]==74
    base=base31()
    checked=0
    for cert in d["certificates"]:
        idx=d["per_window"][cert["window"]]["indices"]
        leaves=[tuple(q) for q in cert["leaves"]]
        assert len(leaves)==8
        for x,y in leaves:
            assert x in {base[i][0] for i in idx}
            assert y*y%P==rhs(x)
        q=None
        for i,point in enumerate(leaves):
            q=add(q,times(1<<i,point))
        assert q==tuple(cert["target"])==tuple(cert["terminal"])
        t=leaves[7]
        for point in reversed(leaves[:7]):
            t=add(point,add(t,t))
        assert t==q
        c=cert["first_step"]
        t0=tuple(c["tail"])
        leaf=tuple(c["leaf"])
        dbl=add(t0,t0)
        assert dbl==tuple(c["doubled"])
        assert add(leaf,dbl)==tuple(c["next"])
        x,y=t0
        den=(2*y)%P
        assert den and den*c["rho"]%P==1
        lam=c["lambda"]
        assert (den*lam-3*x*x-A)%P==0
        xp,yp=leaf
        den2=(xp-dbl[0])%P
        assert den2 and den2*c["sigma"]%P==1
        assert (den2*c["mu"]-(yp-dbl[1]))%P==0
        assert add(q,G)!=q
        checked+=1
    assert checked==len(d["certificates"])>=4
    print(json.dumps({"schema":"prime31.horner_xy_independent.v1",
        "certificates_replayed":checked,"full_group_order_checked":times(N,G) is None,
        "all_checks_passed":True},indent=2))
if __name__=="__main__":
    if len(sys.argv)!=2: sys.exit("usage: verifier output.json")
    replay(sys.argv[1])
