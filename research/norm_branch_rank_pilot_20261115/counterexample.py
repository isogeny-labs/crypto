#!/usr/bin/env python3
"""Certified toy full-rank counterexample for the signed norm tensor, over F19."""
import itertools as it
import json
import math
import random
from run import add, coeffs, check_squared_vandermonde, elimination, matrix, norm_check, signed_sum

P=19
A,B=0,5
POINTS=[(0,9),(1,5),(5,4),(7,5),(10,6),(12,2),(15,6),(17,4),(18,2)]


def det(M,p):
    a=[[v%p for v in row] for row in M]
    d=1
    for j in range(len(a)):
        t=next((i for i in range(j,len(a)) if a[i][j]), None)
        if t is None: return 0
        if t!=j: a[t],a[j]=a[j],a[t];d=-d%p
        pivot=a[j][j]
        d=d*pivot%p
        iv=pow(pivot,-1,p)
        for i in range(j+1,len(a)):
            z=a[i][j]*iv%p
            for k in range(j,len(a)): a[i][k]=(a[i][k]-z*a[j][k])%p
    return d


def certificate():
    assert (4*A**3+27*B**2)%P and len({x for x,y in POINTS})==9
    assert all(y and (y*y-x*x*x-A*x-B)%P==0 for x,y in POINTS)
    C=coeffs(POINTS,P)
    check_squared_vandermonde(POINTS,P,C)
    L=list(it.combinations(range(4),2));R=list(it.combinations(range(4,8),2))
    M=[[C[tuple(sorted(l+r))] for r in R] for l in L]
    assert len(elimination(M,P)[1])==4
    # Independent minor characterization: all 5x5 minors vanish, but a 4x4 minor survives.
    five=[det([[M[i][j] for j in js] for i in is_],P)
          for is_ in it.combinations(range(6),5) for js in it.combinations(range(6),5)]
    assert len(five)==36 and all(v==0 for v in five)
    witness=next((dict(rows=list(is_),cols=list(js),det=val)
                  for is_ in it.combinations(range(6),4)
                  for js in it.combinations(range(6),4)
                  if (val:=det([[M[i][j] for j in js] for i in is_],P))!=0),None)
    assert witness is not None
    rng=random.Random(19)
    null=[[rng.randrange(1,P) for _ in range(6)] for _ in range(6)]
    while det(null,P)==0:
        null=[[rng.randrange(1,P) for _ in range(6)] for _ in range(6)]
    models=[]
    for signs in it.product((-1,1),repeat=8):
        grp=signed_sum(POINTS,signs+(1,),P,A) is None
        singular=len(elimination(matrix(POINTS,signs,P),P)[1])<9
        poly=sum(v*math.prod(signs[i] for i in T) for T,v in C.items())%P==0
        assert grp==singular==poly
        if grp:
            norm_check(POINTS,signs,P,A,B)
            models.append(''.join('+' if x==1 else '-' for x in signs))
    assert len(models)==9
    return dict(prime=P,a=A,b=B,points=POINTS,block='t4 a2 b2',shape=[6,6],
                matrix=M,rank=4,all_5x5_minors_zero=len(five),nonzero_4x4=witness,
                random_null_determinant=det(null,P),models=len(models),model_strings=models)


if __name__=='__main__':
    print(json.dumps(certificate(),sort_keys=True,indent=2))
