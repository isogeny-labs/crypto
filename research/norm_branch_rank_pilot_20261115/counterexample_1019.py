#!/usr/bin/env python3
"""Toy guarded rank counterexample at p=1019, with a planted public target."""
import itertools as it
import json
import math
import random
from counterexample import det
from run import coeffs, check_squared_vandermonde, elimination, matrix, norm_check, signed_sum

P,A,B=1019,46,158
POINTS=[(944,936),(243,525),(432,632),(225,527),(627,518),
        (387,878),(914,704),(764,816),(429,379)]
PLANTED=(1,1,1,1,1,1,1,-1)


def certificate():
    assert (4*A**3+27*B**2)%P and len({x for x,y in POINTS})==9
    assert all(y and (y*y-x*x*x-A*x-B)%P==0 for x,y in POINTS)
    assert signed_sum(POINTS,PLANTED+(1,),P,A) is None
    C=coeffs(POINTS,P)
    check_squared_vandermonde(POINTS,P,C)
    L=list(it.combinations(range(4),2));R=list(it.combinations(range(4,8),2))
    M=[[C[tuple(sorted(l+r))] for r in R] for l in L]
    assert len(elimination(M,P)[1])==4
    all_five=[det([[M[i][j] for j in js] for i in is_],P)
              for is_ in it.combinations(range(6),5)
              for js in it.combinations(range(6),5)]
    assert len(all_five)==36 and all(z==0 for z in all_five)
    four=det([row[:4] for row in M[:4]],P)
    assert four==126
    rng=random.Random(19)
    null=[[rng.randrange(1,P) for _ in range(6)] for _ in range(6)]
    while det(null,P)==0:
        null=[[rng.randrange(1,P) for _ in range(6)] for _ in range(6)]
    models=[]
    for s in it.product((-1,1),repeat=8):
        group=signed_sum(POINTS,s+(1,),P,A) is None
        singular=len(elimination(matrix(POINTS,s,P),P)[1])<9
        poly=sum(v*math.prod(s[i] for i in T) for T,v in C.items())%P==0
        assert group==singular==poly
        if group:
            norm_check(POINTS,s,P,A,B)
            models.append(s)
    assert models==[PLANTED]
    return dict(prime=P,a=A,b=B,points=POINTS,planted=PLANTED,models=len(models),
                block='t4 a2 b2',matrix=M,rank=4,all_5x5_minors_zero=36,
                leading_4x4_determinant=four,null_determinant=det(null,P))


if __name__=='__main__':
    print(json.dumps(certificate(),sort_keys=True,indent=2))
