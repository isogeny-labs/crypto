#!/usr/bin/env python3
"""CT343 public synthetic coefficient arithmetic; no new curve points or timings."""
import json, random, sys
from collections import Counter
P=2**31-1
C=-120
MUT=set(sys.argv[1:])
COUNTS=Counter()

def need(ok,msg):
    if not ok: raise ValueError(msg)

def euler(x):
    r=pow(x%P,(P-1)//2,P)
    need(r in (1,P-1),'zero squareclass')
    return int(r==P-1)

def fold(x):
    # Inputs to this routine are nonnegative, at most 6*P+1.
    need(0<=x<=6*P+1,'fold domain')
    r=(x&P)+(x>>31)
    if r>=P:r-=P
    need(0<=r<P,'fold range')
    return r

def jacobi(x):
    need(0<x<P,'zero squareclass')
    y=P;b=0;steps=0
    while x!=1:
        s=(x&-x).bit_length()-1
        x>>=s
        COUNTS['two_adic_strips']+=1
        if s&1 and y%8 in (3,5):b^=1
        if x==1:break
        if x%4==y%4==3:b^=1
        x,y=y%x,x;steps+=1
        need(x!=0,'noncoprime')
    COUNTS['remainders']+=steps
    COUNTS['calls']+=1
    COUNTS['max_remainders']=max(COUNTS['max_remainders'],steps)
    return b

def evaluate(u,a):
    x=0
    for v in reversed(u):x=(x*a+v)%P
    return x

def reference(u,j):
    a=pow(j,-1,P);n=len(u)-1
    q=evaluate(u,a)
    if q:return euler(pow(-C,n,P)*q)
    derivative=(-j)%P
    for k in range(1,6):
        if k!=j:derivative=derivative*(1-k*a)%P
    other=1 if n==1 else (u[1]+2*a)%P
    need(other!=0,'repeated ramified support')
    return euler(pow(C,3,P)*derivative*pow(-C,n-1,P)*other)

def fast(u,j):
    n=len(u)-1
    need(n in (0,1,2) and u[-1]==1 and all(0<=x<P for x in u),'monic canonical domain')
    if n==0:return 0
    if n==1:q=fold(1+(u[0] if j==1 else u[0]<<1))
    else:q=fold(1+u[1]+u[0] if j==1 else 1+(u[1]<<1)+(u[0]<<2))
    if '--bad-scale' in MUT and j==2 and n==2:q=fold(1+(u[1]<<1)+(u[0]<<1))
    if q:return jacobi(q)
    # With j=2, multiplying the degree-one quotient value by 2 is square-neutral.
    branch=int(j==1)
    if '--bad-branch' in MUT:branch^=1
    if n==1:return branch
    other=fold(u[1]+2) if j==1 else fold((u[1]<<1)+2)
    need(other!=0,'repeated ramified support')
    return branch^jacobi(other)

def main():
    need(euler(-C)==euler(2)==0,'scale square')
    for j in (1,2):
        a=pow(j,-1,P)
        need(reference([(-a)%P,1],j)==int(j==1),'branch constant')
    rng=random.Random(3433101)
    cohort=[[1]]+[[rng.randrange(P),1] for _ in range(1024)]+[[rng.randrange(P),rng.randrange(P),1] for _ in range(1024)]
    for j in (1,2):
        a=pow(j,-1,P)
        cohort.append([(-a)%P,1])
        for b in (0,3,7,P-1):cohort.append([a*b%P,(-a-b)%P,1])
    for u in cohort:
        for j in (1,2):need(fast(u,j)==reference(u,j),'character disagreement')
    bad=0
    for j in (1,2):
        a=pow(j,-1,P);u=[a*a%P,-2*a%P,1]
        for fn in (fast,reference):
            try:fn(u,j)
            except ValueError as e:
                need(str(e)=='repeated ramified support','wrong rejection');bad+=1
            else:raise ValueError('accepted repeated ramified support')
    print(json.dumps({'schema':'ct343-prime31-homogeneous/v1','polynomial_entries':len(cohort),'character_agreements':2*len(cohort),'repeated_ramified_rejections':bad,'jacobi_counts':dict(COUNTS),'modular_multiplications_online':0,'scope':'synthetic monic coefficients, not Jacobian membership or timings'},sort_keys=True,separators=(',',':')))

if __name__=='__main__':main()
