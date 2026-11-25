#!/usr/bin/env python3
"""Exact combinatorial counts, no random draws or elliptic arithmetic."""
from math import comb
from fractions import Fraction as F
B, r = 31, 2147446831
C, H = comb(B,4), 16*comb(B,4)
q = F(H,r-1)
a = [F(comb(B-j,4),C) if B-j>=4 else F(0) for j in range(B+1)]
mean = B+sum((-1)**(j+1)*comb(B,j)*a[j]**B/(1-a[j]) for j in range(1,B+1))
assert F(14514356,100) < mean/q < F(14514357,100)
counts = [0]*(B+1)
counts[0] = 1
cov = [F(0)]
den = 1
for m in range(1,257):
    nxt = [0]*(B+1)
    for c,v in enumerate(counts):
        if not v:
            continue
        for k in range(5):
            if k<=B-c and 4-k<=c:
                nxt[c+k] += v*comb(B-c,k)*comb(c,4-k)
    den *= C
    counts = nxt
    assert sum(counts)==den
    cov.append(F(counts[B],den))
    # independent inclusion-exclusion cross-check
    assert cov[m] == sum((-1)**j*comb(B,j)*a[j]**m for j in range(B+1))
S=10**60
def ceil(n,d):
    return (n+d-1)//d
def power_interval(num,den,n):
    a=num*S//den
    b=ceil(num*S,den)
    lo=hi=S
    while n:
        if n&1:
            lo=lo*a//S
            hi=ceil(hi*b,S)
        a=a*a//S
        b=ceil(b*b,S)
        n//=2
    return lo,hi
def cdf(n):
    lo,hi=power_interval(r-1-H,r-1,n)
    slo=shi=masslo=0
    for m in range(min(n,256)+1):
        masslo+=lo
        if m>=B:
            f=cov[m]
            slo+=lo*f.numerator//f.denominator
            shi+=ceil(hi*f.numerator,f.denominator)
        if m==n:
            break
        u=(n-m)*H
        d=(m+1)*(r-1-H)
        lo=lo*u//d
        hi=ceil(hi*u,d)
    return slo,min(S,shi+max(0,S-masslo))
for level,n in [(95,211709),(99,265406)]:
    before=cdf(n-1)
    after=cdf(n)
    assert 100*before[1]<level*S<=100*after[0]
    print(level,n,float(F(before[1],S)),float(F(after[0],S)))
print("expected positive floor",float(mean))
print("expected blind floor",float(mean/q))
print("coverage at 31/40",float(cov[31]),float(cov[40]))
