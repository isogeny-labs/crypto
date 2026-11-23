#!/usr/bin/env python3
"""Assessor: independent GSp_6(F_l) reference by a symplectic-transvection random walk
(not the producer's basis construction), charpoly by Hessenberg reduction, criterion via
'no self-reciprocal irreducible factor' found by brute-force root/divisor search.
seed 20261116; 20000 samples per cell, 80 transvections each. [pcc:unverified]"""
import random, sys, json
import os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from assessor_check import fire  # brute-force criterion written by the assessor
def om(x, y, l): return (x[0]*y[3]+x[1]*y[4]+x[2]*y[5]-x[3]*y[0]-x[4]*y[1]-x[5]*y[2]) % l
def charpoly(A, l):
    n=len(A); A=[r[:] for r in A]
    for k in range(n-2):  # Hessenberg
        piv=next((i for i in range(k+1,n) if A[i][k]),None)
        if piv is None: continue
        if piv!=k+1:
            A[piv],A[k+1]=A[k+1],A[piv]
            for r in A: r[piv],r[k+1]=r[k+1],r[piv]
        inv=pow(A[k+1][k],l-2,l)
        for i in range(k+2,n):
            c=A[i][k]*inv%l
            if c:
                for j in range(n): A[i][j]=(A[i][j]-c*A[k+1][j])%l
                for r in A: r[k+1]=(r[k+1]+c*r[i])%l
    P=[[1]]
    def pm(a,b):
        r=[0]*(len(a)+len(b)-1)
        for i,x in enumerate(a):
            for j,y in enumerate(b): r[i+j]=(r[i+j]+x*y)%l
        return r
    for m in range(1,n+1):
        cur=pm([(-A[m-1][m-1])%l,1],P[m-1]); t=1
        for i in range(1,m):
            t=t*A[m-i][m-i-1]%l
            term=[x*t*A[m-i-1][m-1]%l for x in P[m-i-1]]
            for j,x in enumerate(term): cur[j]=(cur[j]-x)%l
        P.append(cur)
    return P[n]
rnd=random.Random(20261116)
out=[]
for (l,m) in [(3,1),(3,2),(5,2),(7,3)]:
    N=20000; fi=0; nz=0; finz=0
    for _ in range(N):
        G=[[int(i==j) for j in range(6)] for i in range(6)]
        for _ in range(80):
            v=[rnd.randrange(l) for _ in range(6)]; a=rnd.randrange(1,l)
            # x -> x + a*om(x,v) v ; apply to columns of G
            for c in range(6):
                col=[G[r][c] for r in range(6)]; s=a*om(col,v,l)%l
                for r in range(6): G[r][c]=(G[r][c]+s*v[r])%l
        for r in range(6):
            for c in range(3,6): G[r][c]=G[r][c]*m%l
        cp=charpoly(G,l)  # low->high, monic deg 6; as 'chi' integers with q=m
        f=fire(cp, m, l)>0  # fire() uses p mod l = m
        fi+=f; z1=sum(cp)%l!=0; nz+=z1; finz+=f and z1
    out.append({'l':l,'m':m,'samples':N,'fire':fi,'nz1':nz,'fire_nz1':finz})
print(json.dumps(out))
