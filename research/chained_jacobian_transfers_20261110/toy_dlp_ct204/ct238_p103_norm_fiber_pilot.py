#!/usr/bin/env python3
"""CT238 post-hoc F103 norm-fiber pilot, public known-answer toy.

This is an exploratory Python arithmetic screen, not native Rust timing,
independent PCC, a general prime-field advantage, or a cryptographic attack.
Reuse only CT223's rational polynomial/Cantor code (not its quadratic routines)
after setting a separate fresh-process prime/curve for this one toy.
"""
import json
import sys
from itertools import combinations
import ct223_extension_norm_oracle as C
import ct237_independent_raw_relation_replay as T

P=103
B=24
R=109
G=(1,51)
Q=(42,71)
C.P=P; C.F=[B,0,0,1,0,0,0,0,0,1]
T.P=P; T.C=C; T.G=G; T.Q=Q

def emul_raw(k,g):
    out=None
    while k:
        if k&1:out=T.eadd(out,g)
        g=T.eadd(g,g)
        k>>=1
    return out

def roster():
    xs=[];first=[];ys={};norms=set()
    for u in range(P):
        values=[y for y in range(P) if y*y%P==(pow(u,9,P)+pow(u,3,P)+B)%P]
        if values:
            if len(values)!=2:raise ArithmeticError('non-paired rational support')
            xs.append(u);ys[u]=values;norms.add(pow(u,3,P))
            if len(first)<8:first.append((u,values[0]))
    if (len(xs),len(norms))!=(66,22):raise ArithmeticError('norm-roster drift')
    for x in norms:
        fibers=[u for u in xs if pow(u,3,P)==x]
        if len(fibers)!=3:raise ArithmeticError('nontriple rational norm fiber')
    return xs,first,ys,sorted(norms)

def support(div,all_u):
    u,v=div
    if len(u)!=5:return None
    roots=[x for x in all_u if C.pr(u,[(-x)%P,1])==[0]]
    if len(roots)!=4:return None
    signed={}
    for x in roots:
        y=sum(z*pow(x,i,P) for i,z in enumerate(v))%P
        if y==0 or (y*y-pow(x,9,P)-pow(x,3,P)-B)%P:
            raise ArithmeticError('invalid split support')
        signed[x]=1 if y<P-y else -1
    return signed

def run():
    if (4+27*B*B)%P==0:raise ArithmeticError('singular E curve')
    nE=1
    for x in range(P):
        rhs=(pow(x,3,P)+x+B)%P
        nE+=1 if rhs==0 else 2 if pow(rhs,(P-1)//2,P)==1 else 0
    if nE!=R or emul_raw(R,G) is not None or emul_raw(73,G)!=Q:
        raise ArithmeticError('source group/certificate drift')
    all_u,first,ys,norm_x=roster()
    offsets=T.offsets(first)
    pivots=[None]*len(norm_x)
    gdiv=T.dimage(G)
    rank=0; candidates=0; smooth=0; used_offsets=0
    raw_verified=0
    for pts in offsets[:60]:
        used_offsets+=1
        cur=C.ID
        for pt in pts:cur=C.cantor(cur,T.dpoint(pt))
        off={x:0 for x in norm_x}
        for u,y in pts:off[pow(u,3,P)]+=1 if y<P-y else -1
        for k in range(R):
            candidates+=1
            signed=support(cur,all_u)
            if signed is not None:
                smooth+=1
                row={x:0 for x in norm_x}
                for u,s in signed.items():row[pow(u,3,P)]+=s
                T.validate_raw(signed,pts,k,ys)
                raw_verified+=1
                coeff=[row[x]-off[x] for x in norm_x]
                if T.reduce_row(pivots,coeff,3*k):rank+=1
                if rank==len(norm_x):break
            if k<R-1:cur=C.cantor(cur,gdiv)
        if rank==len(norm_x):break
    if (rank,candidates,smooth,used_offsets)!=(22,381,27,4):
        raise ArithmeticError(f'rank/yield drift: {(rank,candidates,smooth,used_offsets)}')
    logs=T.solve(pivots)
    for x,ell in zip(norm_x,logs):
        y=next(y for y in range(P) if y*y%P==(pow(x,3,P)+x+B)%P)
        if T.emul(ell,G)!=(x,y):raise ArithmeticError('wrong norm factor log')
    target_candidates=0;answer=None
    for pts in offsets[:60]:
        cur=T.dimage(Q)
        for pt in pts:cur=C.cantor(cur,T.dpoint(pt))
        off={x:0 for x in norm_x}
        for u,y in pts:off[pow(u,3,P)]+=1 if y<P-y else -1
        for a in range(R):
            target_candidates+=1
            signed=support(cur,all_u)
            if signed is not None:
                row={x:0 for x in norm_x}
                for u,s in signed.items():row[pow(u,3,P)]+=s
                total=sum((row[x]-off[x])*ell for x,ell in zip(norm_x,logs))
                scalar=(total*pow(3,-1,R)-a)%R
                if emul_raw(scalar,G)!=Q:raise ArithmeticError('wrong target log')
                answer=scalar
                break
            if a<R-1:cur=C.cantor(cur,gdiv)
        if answer is not None:break
    if (answer,target_candidates)!=(73,27):raise ArithmeticError('target drift')
    return dict(schema='ct238-f103-posthoc-public-norm-fiber-pilot-v1',
                p=P,b=B,order=nE,source_g=list(G),target_q=list(Q),
                rational_c_signed_u=len(all_u),source_norm_variables=len(norm_x),
                three_to_one_fibers=len(norm_x),rank=rank,candidates=candidates,
                accepted_raw_group_relations=raw_verified,used_offsets=used_offsets,
                target_attempts=target_candidates,recovered_log=answer,
                timing='not_measured',pcc='unverified')

if __name__=='__main__':
    print('CT238_PYTHON_PILOT_PASS',json.dumps(run(),sort_keys=True))
