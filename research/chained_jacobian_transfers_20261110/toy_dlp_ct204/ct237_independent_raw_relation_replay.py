#!/usr/bin/env python3
"""CT237 independent raw elliptic/Jacobian relation replay for CT235.

Uses independent Python Cantor arithmetic from CT223 instead of Rust's
prime_hyperelliptic; reconstructs the public deterministic relation stream,
verifies every accepted raw elliptic norm relation without logarithms, solves
the complete LP/FB system, then independently recovers the public Q73 target.
Public F101 toy only, not authenticated 695-record fixture or signed PCC.
"""
import json
import sys
from itertools import combinations
from pathlib import Path
import ct223_extension_norm_oracle as C

P=101
R=109
ID=C.ID
G=(2,37)
Q=(19,37)
BASES=(24,36,48,54)
MAX_OFFSETS=60
EXPECTED={24:(24,2882,27),36:(36,1596,15),48:(48,1421,14),54:(54,1421,14)}

def fail_if(condition,why):
    if condition:raise ValueError(why)

def inv(a):
    fail_if(a%P==0,'inverse zero')
    return pow(a%P,P-2,P)

def eadd(s,t):
    if s is None:return t
    if t is None:return s
    x,y=s;z,w=t
    if x==z and (y+w)%P==0:return None
    if x==z:
        slope=((3*x*x+1)*inv(2*y))%P
    else:
        slope=((w-y)*inv(z-x))%P
    xx=(slope*slope-x-z)%P
    return (xx,(slope*(x-xx)-y)%P)

def emul(k,t):
    k%=R
    acc=None
    while k:
        if k&1:acc=eadd(acc,t)
        t=eadd(t,t);k>>=1
    return acc

def ec_of_C(point):
    x,y=point
    return pow(x,3,P),y

def dpoint(pt):
    x,y=pt
    d=([(-x)%P,1],[y])
    fail_if(not C.valid(d),'invalid C rational input')
    return d

def dimage(e):
    if e is None:return ID
    x,y=e
    d=([(-x)%P,0,0,1],[y])
    fail_if(not C.valid(d),'invalid pullback of E')
    return d

def rational_roster():
    xs=[];first=[];y_for={}
    for x in range(P):
        y0=[y for y in range(P) if y*y%P == (pow(x,9,P)+pow(x,3,P)+46)%P]
        if y0:
            fail_if(len(y0)!=2,'ramified root not in model')
            xs.append(x);y_for[x]=y0
            if len(first)<8:first.append((x,y0[0]))
    fail_if(len(xs)!=54,'factor-base population drift')
    return xs,first,y_for

def offsets(first):
    return [list(tup) for n in (2,3,4) for tup in combinations(first,n)]

def support(div,xs):
    u,v=div
    if len(u)!=5:return None
    roots=[x for x in xs if C.pr(u,[(-x)%P,1])==[0]]
    if len(roots)!=4:return None
    signed={}
    for x in roots:
        yy=sum(z*pow(x,i,P) for i,z in enumerate(v))%P
        if yy==0 or (yy*yy-(pow(x,9,P)+pow(x,3,P)+46))%P!=0:
            raise ArithmeticError('invalid split support')
        signed[x]=1 if yy<P-yy else -1
    return signed

def point_for(x,s,y_for):
    ys=y_for[x]
    return (x,ys[0] if (ys[0]<P-ys[0])==(s==1) else ys[1])

def reduce_row(pivots,row,rhs):
    n=len(pivots)
    a=[z%R for z in row]+[rhs%R]
    for i in range(n):
        if not a[i]:continue
        if pivots[i] is not None:
            f=a[i]
            for j in range(i,n+1):a[j]=(a[j]-f*pivots[i][j])%R
        else:
            t=pow(a[i],-1,R)
            pivots[i]=[(z*t)%R for z in a]
            return True
    fail_if(a[n]!=0,'contradictory eliminated equation')
    return False

def solve(pivots):
    fail_if(any(z is None for z in pivots),'incomplete rank')
    ans=[0]*len(pivots);n=len(pivots)
    for i in range(n-1,-1,-1):
        ans[i]=(pivots[i][n]-sum(pivots[i][j]*ans[j] for j in range(i+1,n)))%R
    return ans

def lp_insert(pivots,row_b,row_lp,rhs):
    b=[z%R for z in row_b]
    lp=[z%R for z in row_lp];rhs%=R
    for i in range(len(lp)):
        f=lp[i]
        if not f:continue
        prev=pivots[i]
        if prev is not None:
            pb,pl,pr=prev
            b=[(z-f*t)%R for z,t in zip(b,pb)]
            lp=[(z-f*t)%R for z,t in zip(lp,pl)]
            rhs=(rhs-f*pr)%R
        else:
            invf=pow(f,-1,R)
            pivots[i]=([z*invf%R for z in b],[z*invf%R for z in lp],rhs*invf%R)
            return None
    fail_if(any(lp),'nonzero LP after elimination')
    return b,rhs

def recover_lp(pivots,fb):
    out=[None]*len(pivots)
    for i in range(len(pivots)-1,-1,-1):
        row=pivots[i]
        if row is None:continue
        b,lp,rhs=row
        val=(rhs-sum(x*y for x,y in zip(b,fb)))%R
        if any(a and out[j] is None for j,a in enumerate(lp) if j>i):continue
        val=(val-sum(lp[j]*out[j] for j in range(i+1,len(lp)) if lp[j]))%R
        out[i]=val
    return out

def validate_raw(signed,offs,k,ys):
    # Source elliptic point equality, without any logarithm lookup.
    lhs=None
    for x,s in signed.items():lhs=eadd(lhs,ec_of_C(point_for(x,s,ys)))
    for x,y in offs:
        pt=ec_of_C((x,y));lhs=eadd(lhs,(pt[0],(-pt[1])%P))
    fail_if(lhs!=emul(3*k,G),'raw elliptic norm relation incorrect')

def test_basis(basis,roster,xs,ys):
    selected=xs[:basis]
    excluded=xs[basis:]
    pivots=[None]*basis;lp_pivots=[None]*(54-basis)
    accepted=[];eqs=[]
    rank=0
    counts=dict(candidates=0,offsets=0,full=0,single_lp=0,double_lp=0,
                multi_lp_discard=0,b_only_rows=0,verified_relations=0)
    Gdiv=dimage(G)
    for pts in roster[:MAX_OFFSETS]:
        counts['offsets']+=1
        div=ID
        for pt in pts:div=C.cantor(div,dpoint(pt))
        off={x:(1 if y<P-y else -1) for x,y in pts}
        for k in range(R):
            counts['candidates']+=1
            signed=support(div,xs)
            if signed is not None:
                out=[(x,s) for x,s in signed.items() if x in excluded]
                if len(out)<=2:
                    validate_raw(signed,pts,k,ys)
                    tag={0:'full',1:'single_lp',2:'double_lp'}[len(out)]
                    counts[tag]+=1
                    rb=[signed.get(x,0)-off.get(x,0) for x in selected]
                    rl=[signed.get(x,0) for x in excluded]
                    rhs=3*k%R
                    elim=lp_insert(lp_pivots,rb,rl,rhs)
                    if elim is not None:
                        brow,erhs=elim
                        counts['b_only_rows']+=1
                        if reduce_row(pivots,brow,erhs):rank+=1
                        accepted.append((brow,erhs))
                    eqs.append((rb,rl,rhs))
                else:counts['multi_lp_discard']+=1
                if rank==basis:break
            if k<R-1:div=C.cantor(div,Gdiv)
        if rank==basis:break
    fail_if(rank!=basis,'rank not reached')
    logs=solve(pivots)
    lp_logs=recover_lp(lp_pivots,logs)
    for x,ell in zip(selected,logs):
        fail_if(emul(ell,G)!=ec_of_C(point_for(x,1,ys)),'wrong factor-base logarithm')
    for x,ell in zip(excluded,lp_logs):
        if ell is not None:fail_if(emul(ell,G)!=ec_of_C(point_for(x,1,ys)),'wrong LP logarithm')
    for rb,rl,rhs in eqs:
        if all(not a or lp_logs[i] is not None for i,a in enumerate(rl)):
            counts['verified_relations']+=1
            lhs=sum(a*b for a,b in zip(rb,logs))+sum(a*lp_logs[i] for i,a in enumerate(rl) if a)
            fail_if(lhs%R!=rhs,'solved equation mismatch')
    target_attempts=0;recovered=None
    for pts in roster[:MAX_OFFSETS]:
        div=dimage(Q)
        for pt in pts:div=C.cantor(div,dpoint(pt))
        off={x:(1 if y<P-y else -1) for x,y in pts}
        for a in range(R):
            target_attempts+=1
            signed=support(div,xs)
            if signed is not None and all(lp_logs[j] is not None for j,x in enumerate(excluded) if signed.get(x,0)):
                total=sum((signed.get(x,0)-off.get(x,0))*ell for x,ell in zip(selected,logs))
                total+=sum(signed.get(x,0)*lp_logs[j] for j,x in enumerate(excluded) if signed.get(x,0))
                candidate=(total*pow(3,-1,R)-a)%R
                fail_if(emul(candidate,G)!=Q,'wrong independently recovered target')
                recovered=candidate
                break
            if a<R-1:div=C.cantor(div,Gdiv)
        if recovered is not None:break
    fail_if(recovered is None,'target recovery failed')
    counts.update(rank=rank,lp_pivot_rank=sum(x is not None for x in lp_pivots),
                  known_lp=sum(x is not None for x in lp_logs),
                  target73=recovered,target_candidates73=target_attempts)
    fail_if(counts['candidates']>MAX_OFFSETS*R,'over budget')
    return counts

def run(records_path=None):
    xs,first,ys=rational_roster()
    roster=offsets(first)
    fail_if(len(roster)<MAX_OFFSETS,'not enough offsets')
    reports={}
    for b in BASES:
        counts=test_basis(b,roster,xs,ys)
        expect=EXPECTED[b]
        fail_if((counts['rank'],counts['candidates'],counts['offsets'])!=expect,
                f'independent replay drift B{b}: {counts} vs {expect}')
        reports[b]=counts
    if records_path:
        rows=[json.loads(x) for x in Path(records_path).read_text().splitlines() if x.strip()]
        measurements=[z for z in rows if 'basis' in z]
        fail_if(len(measurements)!=16,'exactly 16 native panel records required')
        for rec in measurements:
            b=rec['basis']
            fail_if(b not in reports,'unregistered factor base')
            for fld,val in reports[b].items():
                if fld in rec:fail_if(rec[fld]!=val,
                    f'raw replay mismatch B{b}, round {rec["round"]}: {fld} {rec[fld]} != {val}')
    return reports

if __name__=='__main__':
    data=run(sys.argv[1] if len(sys.argv)>1 else None)
    print('CT237_INDEPENDENT_RAW_RELATION_REPLAY_PASS',json.dumps(data,sort_keys=True))
