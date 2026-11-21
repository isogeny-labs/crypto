#!/usr/bin/env python3
"""CT61: exact bounded binary elliptic trace census and fixed genus-two comparison."""
from itertools import product
POLY={1:3,2:7,3:11}
ELLS=(3,5,7,11,17,19,23,37,43,163,257)
CAP=256

def mul(a,b,k,poly):
    out=0
    while b:
        if b&1:out^=a
        b>>=1
        a<<=1
        if a&(1<<k):a^=poly
    return out

def pow2(a,n,k,poly):
    out=1
    while n:
        if n&1:out=mul(out,a,k,poly)
        a=mul(a,a,k,poly);n>>=1
    return out

def on(x,y,c,k,poly):
    a1,a2,a3,a4,a6=c
    x2=mul(x,x,k,poly)
    left=mul(y,y,k,poly)^mul(a1,mul(x,y,k,poly),k,poly)^mul(a3,y,k,poly)
    right=mul(x2,x,k,poly)^mul(a2,x2,k,poly)^mul(a4,x,k,poly)^a6
    return left==right

def smooth(c,k,poly):
    a1,a2,a3,a4,a6=c
    if not a1:return a3!=0
    inv=pow2(a1,(1<<k)-2,k,poly)
    x=mul(a3,inv,k,poly)
    y=mul(mul(x,x,k,poly)^a4,inv,k,poly)
    return not on(x,y,c,k,poly)

def census(k):
    q=1<<k;poly=POLY[k];result={}
    for c in product(range(q),repeat=5):
        if not smooth(c,k,poly):continue
        count=1+sum(on(x,y,c,k,poly) for x in range(q) for y in range(q))
        result.setdefault(q+1-count,c)
    return result

def first(q,t,ell):
    a,b=2,t%ell
    for f in range(1,CAP+1):
        if (pow(q,f,ell)+1-b)%ell==0:return f
        a,b=b,(t*b-q*a)%ell
    return None

def mm(a,b,m):
    return [[sum(a[i][h]*b[h][j] for h in range(4))%m for j in range(4)] for i in range(4)]

def genus2(ell):
    # CT56 smooth witness has C(F2)=6 and C(F4)=6.
    # Characteristic polynomial T^4+3T^3+5T^2+6T+4.
    F=[[0,0,0,-4],[1,0,0,-6],[0,1,0,-5],[0,0,1,-3]]
    P=[[int(i==j) for j in range(4)] for i in range(4)]
    for n in range(1,CAP+1):
        P=mm(P,F,ell)
        M=[[(int(i==j)-P[i][j])%ell for j in range(4)] for i in range(4)]
        rank=0
        for col in range(4):
            ix=next((i for i in range(rank,4) if M[i][col]),None)
            if ix is None:continue
            M[rank],M[ix]=M[ix],M[rank]
            z=pow(M[rank][col],-1,ell)
            for j in range(col,4):M[rank][j]=M[rank][j]*z%ell
            for i in range(rank+1,4):
                z=M[i][col]
                for j in range(col,4):M[i][j]=(M[i][j]-z*M[rank][j])%ell
            rank+=1
        if rank<4:return n
    return None

def main():
    data={k:census(k) for k in POLY}
    for k,v in data.items():
        print('field_degree',k,'traces',sorted(v),'witnesses',len(v))
    for ell in ELLS:
        candidates=[(k*f,k,f,t) for k,traces in data.items() for t in traces if (f:=first(1<<k,t,ell)) is not None]
        print('ell',ell,'best_binary_elliptic',min(candidates) if candidates else None,'fixed_g2_F2',genus2(ell))
    print('All searches capped at f<=',CAP,'; subgroup availability does not give cross-characteristic transport')
if __name__=='__main__':main()
