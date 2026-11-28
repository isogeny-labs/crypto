#!/usr/bin/env python3
"""CT89: exact Q-linear basis closure and F2 Frobenius reduction (stdlib)."""
from fractions import Fraction as Q
import json
G=(-2102915,3828478,-6423347,-1696464,-39690,-29862,-840,-8,1)
A=[[0]*8 for _ in range(8)]
for j in range(4): A[j][j]=4
for j,col in enumerate(((1,3,0,0,1,0,0,0),(0,1,3,0,0,1,0,0),(0,0,1,3,0,0,1,0),(1,3,0,1,0,0,0,1)),4):
    for i in range(8): A[i][j]=col[i]
B=[[Q(A[i][j],4) for j in range(8)] for i in range(8)]
def inverse(M):
    T=[[M[i][j] for j in range(8)]+[Q(i==j) for j in range(8)] for i in range(8)]
    for k in range(8):
        p=next(i for i in range(k,8) if T[i][k]); T[k],T[p]=T[p],T[k]
        z=T[k][k]; T[k]=[v/z for v in T[k]]
        for i in range(8):
            if i!=k:
                z=T[i][k]; T[i]=[v-z*w for v,w in zip(T[i],T[k])]
    return [row[8:] for row in T]
BI=inverse(B)
def multiply(a,b):
    c=[Q(0) for i in range(15)]
    for i,x in enumerate(a):
        for j,y in enumerate(b): c[i+j]+=x*y
    for k in range(14,7,-1):
        z=c[k]
        for j in range(8): c[k-8+j]-=z*G[j]
    return c[:8]
def coords(v): return [sum(BI[i][j]*v[j] for j in range(8)) for i in range(8)]
basis=[[B[i][j] for i in range(8)] for j in range(8)]
for i in range(8):
    for j in range(i+1):
        v=coords(multiply(basis[i],basis[j]))
        assert all(x.denominator==1 for x in v),(i,j,v)
F=[]
for a in basis:
    v=coords(multiply(a,a))
    assert all(x.denominator==1 for x in v)
    F.append([int(x)%2 for x in v])
def apply(v): return [sum(F[j][i]*v[j] for j in range(8))%2 for i in range(8)]
def rank(cols):
    rows=[sum(cols[j][i]<<j for j in range(8)) for i in range(8)]
    k=0
    for j in range(8):
        p=next((i for i in range(k,8) if rows[i]>>j&1),None)
        if p is None: continue
        rows[k],rows[p]=rows[p],rows[k]
        for i in range(k+1,8):
            if rows[i]>>j&1: rows[i]^=rows[k]
        k+=1
    return k
assert rank(F)==8
v1=[apply([int(i==j) for i in range(8)]) for j in range(8)]
assert v1==F
v2=[apply(q) for q in v1]
v4=[apply(apply(q)) for q in v2]
def fixednull(cols): return 8-rank([[c[i]^(i==j) for i in range(8)] for j,c in enumerate(cols)])
fixed=[fixednull(q) for q in (v1,v2,v4)]
assert fixed==[2,4,8],fixed
idempotents=[m for m in range(256) if apply([(m>>i)&1 for i in range(8)])==[(m>>i)&1 for i in range(8)]]
assert idempotents==[0,1,240,241]
print(json.dumps({"idempotents":idempotents,"test":"ct89-2-maximal-basis",
    "index_2part":256,"basis_denominator":4,"products_checked":36,
    "frob_rank":8,"frob_fixed_dimensions":fixed,"disc_basis_v2":0},
    sort_keys=True,separators=(',',':')))
