#!/usr/bin/env python3
"""CT62 exact F32 elliptic trace witness/control: 2,016 standard models.

Ordinary: y²+xy=x³+a2*x²+a6, a6 != 0.
Supersingular: y²+y=x³+a4*x+a6.
These cover elliptic F32 isomorphism classes by standard char-two changes of
Weierstrass coordinates; parameter duplicates are intentionally retained.
"""
from collections import Counter
P=0b100101
N=32
mul=[[0]*N for _ in range(N)]
for a in range(N):
    for b in range(N):
        x,y=a,b;z=0
        while y:
            if y&1:z^=x
            y>>=1;x<<=1
            if x&N:x^=P
        mul[a][b]=z
if not all(set(mul[a])==set(range(N)) for a in range(1,N)):
    raise AssertionError('field polynomial not irreducible')
S=[[0]*N for _ in range(N)]
for h in range(N):
    for y in range(N):S[h][mul[y][y]^mul[h][y]]+=1
T=Counter();ORD=Counter();SUP=Counter()
for a2 in range(N):
    for a6 in range(1,N):
        ct=1
        for x in range(N):
            xx=mul[x][x];rhs=mul[xx][x]^mul[a2][xx]^a6
            ct+=S[x][rhs]
        t=N+1-ct;T[t]+=1;ORD[t]+=1
for a4 in range(N):
    for a6 in range(N):
        ct=1
        for x in range(N):
            xx=mul[x][x];rhs=mul[xx][x]^mul[a4][x]^a6
            ct+=S[1][rhs]
        t=N+1-ct;T[t]+=1;SUP[t]+=1
if T[2]!=0 or sum(T.values())!=2016 or set(SUP)!={-8,0,8}:
    raise AssertionError(('unexpected F32 traces',T,SUP))
print('models',sum(T.values()),'ordinary',sum(ORD.values()),'supersingular',sum(SUP.values()))
print('ordinary traces',sorted(ORD))
print('supersingular traces',sorted(SUP))
print('all traces',sorted(T))
print('models with order31', T[2])
