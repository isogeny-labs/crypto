#!/usr/bin/env python3
"""Independent BigInt check of four-prefix interpolation on pinned Prime31 RR9.
Uses the pre-existing independent group-law verifier, never the Rust chart-reduction
formula. The Jacobian is checked by symmetric finite differences, not copied from
the producer's analytic derivative. No public ECDLP solve or PCC signature.
"""
import json
import sys
import verify_prime31_rr9_weighted_resultants as prior

p=prior.P

def lagrange(nodes, values):
    assert len(nodes)==4 and len(set(nodes))==4
    ans=[0]*4
    for i,xi in enumerate(nodes):
        basis=[1]
        denom=1
        for j,xj in enumerate(nodes):
            if i!=j:
                basis=prior.polymul(basis,[(-xj)%p,1])
                denom=denom*(xi-xj)%p
        w=values[i]*prior.inv(denom)%p
        for k in range(4):
            ans[k]=(ans[k]+w*basis[k])%p
    return ans

def reconstruct(r, chosen, v):
    rx,sy=r
    nodes=[q[0] for q in chosen[:4]]
    assert rx not in nodes
    vals=[]
    for x,y in chosen[:4]:
        V=(x*x*x+v[0]+v[1]*x+v[2]*x*x)%p
        vals.append(-(y+sy)*V*prior.inv(x-rx)%p)
    return lagrange(nodes,vals)

def rank(matrix):
    a=[row[:] for row in matrix]
    k=0
    for j in range(len(a[0])):
        pos=next((i for i in range(k,len(a)) if a[i][j]%p),None)
        if pos is None:
            continue
        a[k],a[pos]=a[pos],a[k]
        f=prior.inv(a[k][j])
        a[k]=[z*f%p for z in a[k]]
        for i in range(len(a)):
            if i==k:continue
            f=a[i][j]
            a[i]=[(z-f*b)%p for z,b in zip(a[i],a[k])]
        k+=1
    return k

def run(record):
    prior.validate(record)
    base_points=prior.base()
    slots=[[prior.scalar(w,q) for q in base_points] for w in prior.W]
    good=0
    negatives=0
    full_rank=0
    for row in record["fixtures"]:
        r=tuple(row["target"])
        c=row["chart"]
        chosen=[]
        for j,(idx,sgn) in enumerate(zip(row["root_indices"],row["root_signs"])):
            x,y=slots[j][idx]
            chosen.append((x,y if sgn==1 else -y%p))
        assert len({q[0] for q in chosen[:4]})==4
        v=c[4:7]
        assert reconstruct(r,chosen,v)==c[:4]
        origin=prior.norm_m(r,c)
        assert all(prior.value(origin,q[0])==0 for q in chosen[4:])
        good+=1
        grads=[]
        for q in chosen[4:]:
            x=q[0]
            gradient=[]
            for j in range(3):
                vp=v[:];vp[j]=(vp[j]+1)%p
                vm=v[:];vm[j]=(vm[j]-1)%p
                fp=reconstruct(r,chosen,vp)+vp
                fm=reconstruct(r,chosen,vm)+vm
                dp=prior.value(prior.norm_m(r,fp),x)
                dm=prior.value(prior.norm_m(r,fm),x)
                gradient.append((dp-dm)*prior.inv(2)%p)
            grads.append(gradient)
        assert rank(grads)==3
        full_rank+=1
        vv=v[:];vv[0]=(vv[0]+1)%p
        bad=prior.norm_m(r,reconstruct(r,chosen,vv)+vv)
        hits=[sum(prior.value(bad,q[0])==0 for q in slots[j]) for j in range(4,8)]
        assert any(count==0 for count in hits), (row["fixture"],hits)
        negatives+=1
    assert (good,negatives,full_rank)==(16,16,16)
    print("PASS independent Prime31 RR9 four-prefix: 16/16 exact A, "
          "16/16 rank-three 4x3 finite-difference Jacobians, "
          "16/16 v0+1 negatives; no unknown-x extraction")

if __name__=="__main__":
    with open(sys.argv[1],encoding="utf-8") as f:
        run(json.load(f))
