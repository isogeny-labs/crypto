#!/usr/bin/env python3
"""Independent native-small genus-two Cantor composition/reduction over F4.

Checks all 31 classes of y²+y=x⁵+w*x³ and ramified branch of
y²+x*y=x⁵+1, with group identities and nonsplit quadratic divisor cases.
"""
from itertools import product
from ct62_genus2_census import M4,I4,trim,npoints,jac_order
import json

def require(condition, context=None):
    if not condition: raise AssertionError(context)

def norm(a):return tuple(trim(list(a)))
def add(a,b):
    z=[0]*max(len(a),len(b))
    for i,t in enumerate(a):z[i]^=t
    for i,t in enumerate(b):z[i]^=t
    return norm(z)
def mul(a,b):
    if not a or not b:return ()
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):z[i+j]^=M4[x][y]
    return norm(z)
def scale(a,c):return norm([M4[x][c] for x in a])
def divmodp(a,b):
    a=list(norm(a));b=norm(b)
    if not b:raise ValueError('division by zero')
    out=[0]*max(1,len(a)-len(b)+1)
    while a and len(a)>=len(b):
        d=len(a)-len(b);c=M4[a[-1]][I4[b[-1]]]
        out[d]^=c
        for i,t in enumerate(b):a[i+d]^=M4[c][t]
        trim(a)
    return norm(out),norm(a)
def mod(a,b):return divmodp(a,b)[1]
def exact(a,b):
    q,r=divmodp(a,b)
    if r:raise ArithmeticError(f'nonexact {a} / {b}: {r}')
    return q
def monic(a):
    if not a:return ()
    return scale(a,I4[a[-1]])
def xgcd(a,b):
    r0,r1=norm(a),norm(b);s0,s1=(1,),();t0,t1=(),(1,)
    while r1:
        q,r=divmodp(r0,r1)
        r0,r1=r1,r;s0,s1=s1,add(s0,mul(q,s1));t0,t1=t1,add(t0,mul(q,t1))
    if not r0:return (),(),()
    v=I4[r0[-1]]
    return scale(r0,v),scale(s0,v),scale(t0,v)

def canonical(u,v,f,h):
    u=monic(u)
    if not u:return ((1,),())
    v=mod(v,u)
    while len(u)-1>2:
        u2=exact(add(f,add(mul(v,v),mul(h,v))),u)
        u2=monic(u2)
        if not u2:raise ArithmeticError('zero reduction')
        v=mod(add(h,v),u2);u=u2
    if mod(add(f,add(mul(v,v),mul(h,v))),u):raise AssertionError(('bad divisor',u,v))
    return u,v

def compose(A,B,f,h):
    u1,v1=A;u2,v2=B
    if len(u1)==1:return B
    if len(u2)==1:return A
    d1,e1,e2=xgcd(u1,u2)
    d,g1,g2=xgcd(d1,add(add(v1,v2),h))
    s1=mul(g1,e1);s2=mul(g1,e2);s3=g2
    U=exact(mul(u1,u2),mul(d,d))
    t=add(add(mul(mul(s1,u1),v2),mul(mul(s2,u2),v1)),mul(s3,add(mul(v1,v2),f)))
    V=mod(exact(t,d),U)
    return canonical(U,V,f,h)

def neg(A,h):
    u,v=A
    return (u,mod(add(h,v),u))

def classes(f,h):
    ans=[((1,),())]
    for d in (1,2):
        for c in product(range(4),repeat=d):
            u=tuple(c)+(1,)
            if d==2 and u[1]==0:
                root=next(r for r in range(4) if M4[r][r]==u[0])
                if h[0]^M4[h[1]][root]^M4[h[2]][M4[root][root]]==0:continue
            for v in product(range(4),repeat=d):
                if not mod(add(f,add(mul(v,v),mul(h,v))),u):ans.append((u,norm(v)))
    return ans

def power(A,k,f,h):
    out=((1,),());step=A
    while k:
        if k&1:out=compose(out,step,f,h)
        step=compose(step,step,f,h);k>>=1
    return out

def bsgs(P,Q,n,f,h):
    # Target-only BSGS; not a cross-characteristic transfer.
    from math import isqrt
    m=isqrt(n)+1
    identity=((1,),())
    baby={};R=identity;adds=0
    for j in range(m):
        baby.setdefault(R,j)
        R=compose(R,P,f,h);adds+=1
    negative_stride=neg(R,h)
    giant=Q
    for i in range(m+1):
        j=baby.get(giant)
        if j is not None:
            k=(i*m+j)%n
            if power(P,k,f,h)!=Q:raise AssertionError('invalid BSGS candidate')
            return k,adds
        giant=compose(giant,negative_stride,f,h);adds+=1
    raise AssertionError('BSGS no solution')

def check(name,f,h,order=None,branch=None):
    group=classes(f,h);setg=set(group)
    N1=npoints(list(f),list(h),M4,list(range(4)))
    from ct62_genus2_census import M16,EMBED
    N2=npoints(list(f),list(h),M16,EMBED)
    n=jac_order(4,N1,N2)
    require(n==len(group) and len(setg)==n,(name,n,len(setg),len(group)))
    if order is not None:require(order==n)
    typ={'nonsplit_quadratic':0,'repeated_unramified':0,'ramified_linear':0}
    for u,v in group:
        require(neg(neg((u,v),h),h)==(u,v))
        require(compose((u,v),neg((u,v),h),f,h)==((1,),()))
        require(compose((u,v),((1,),()),f,h)==(u,v))
        if len(u)==3:
            if u[1]==0:typ['repeated_unramified']+=1
            elif not any(M4[r][r]^M4[u[1]][r]^u[0]==0 for r in range(4)):
                typ['nonsplit_quadratic']+=1
        if len(u)==2 and h[0]^M4[h[1]][u[0]]^M4[h[2]][M4[u[0]][u[0]]]==0:
            typ['ramified_linear']+=1
    for A in group:
        for B in group:
            C=compose(A,B,f,h)
            require(C in setg,(name,A,B,C))
            require(C==compose(B,A,f,h),(name,A,B))
    for A in group:
        for B in group:
            for C in group:
                require(compose(compose(A,B,f,h),C,f,h)==compose(A,compose(B,C,f,h),f,h))
    if order is not None and all(order%d for d in range(2,int(order**.5)+1)):
        for A in group[1:]:require(power(A,order,f,h)==((1,),()))
    if order==31:
        solved=0;max_add=0
        for P in group[1:]:
            log_by_point={power(P,k,f,h):k for k in range(31)}
            require(len(log_by_point)==31)
            for Q in group:
                k,adds=bsgs(P,Q,31,f,h)
                require(k==log_by_point[Q])
                solved+=1;max_add=max(max_add,adds)
        require(solved==930)
    if branch:
        ramified=((0,1),(1,))
        require(ramified in setg and power(ramified,2,f,h)==((1,),()))
    return {'name':name,'N1':N1,'N2':N2,'J':n,'classes':n,'divisor_types':typ,
            'checked_all_pairs':n*n,'checked_all_triples':n*n*n,'ramified_order2':bool(branch),
            'independent_target_bsgs_pairs':(solved if order==31 else 0),
            'bsgs_max_baby_plus_giant_additions':(max_add if order==31 else None)}

def main():
    witness=check('prime31_F4', (0,0,0,2,0,1,0),(1,0,0,0),31)
    ramified=check('ramified_x0_F4',(1,0,0,0,0,1,0),(0,1,0,0),branch=True)
    print(json.dumps({'tests':[witness,ramified]},sort_keys=True,separators=(',',':')))
if __name__=='__main__':main()
