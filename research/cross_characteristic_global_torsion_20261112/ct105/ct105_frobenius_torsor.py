#!/usr/bin/env python3
"""CT105: finite-module certificate for cyclic point-field torsor / Frobenius."""
import json
from collections import Counter
L=11
A59=((0,7),(1,5));A2=((0,9),(1,9));I=((1,0),(0,1))
def check(x,msg):
    if not x:raise ArithmeticError(msg)
def mm(a,b):
    return tuple(tuple(sum(a[i][k]*b[k][j] for k in range(2))%L for j in range(2)) for i in range(2))
def pw(a,n):
    z=I
    while n:
        if n&1:z=mm(z,a)
        a=mm(a,a);n//=2
    return z
def act(a,v):return tuple(sum(a[i][j]*v[j] for j in range(2))%L for i in range(2))
def normalize(v):
    if v[0]:
        inv=pow(v[0],-1,L)
        return (1,v[1]*inv%L)
    return (0,1)
def orbit_cycles(a,oriented=False):
    universe=([(x,y) for x in range(L) for y in range(L) if x or y] if oriented else
              [(0,1)]+[(1,k) for k in range(L)])
    unseen=set(universe); cycles=Counter()
    while unseen:
        first=min(unseen);v=first;n=0
        while v in unseen:
            unseen.remove(v);n+=1;v=act(a,v)
            if not oriented:v=normalize(v)
        check(v==first,'orbit not closed')
        cycles[n]+=1
    return [[n,cycles[n]] for n in sorted(cycles)]
check(pw(A2,4)==((7,0),(0,7)),'binary Frobenius-four not 7I')
check(pw(A2,20)==((10,0),(0,10)) and pw(A2,40)==I,'binary order40')
check(pw(A59,5)==I,'source order5')
check(len({pow(7,i,L) for i in range(10)})==10,'7 nonprimitive scalar')
src=orbit_cycles(A59);dst=orbit_cycles(A2)
check(src==[[1,2],[5,2]] and dst==[[4,3]],'projective splitting')
check(orbit_cycles(A59,True)==[[1,10],[5,22]],'source oriented cycles')
check(orbit_cycles(A2,True)==[[40,3]],'binary oriented cycles')
print(json.dumps({'binary_frobenius_four_scalar':7,'binary_oriented_cycles':[[40,3]],
    'binary_projective_cycles':dst,'binary_residue_degree':40,
    'cyclic_automorphism_order':10,'fixed_field_degree':12,
    'relative_binary_residue_degree':10,'relative_source_split_primes':10,
    'source_oriented_cycles':[[1,10],[5,22]],'source_projective_cycles':src},
    sort_keys=True,separators=(',',':')))
