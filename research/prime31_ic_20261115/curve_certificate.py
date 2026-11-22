#!/usr/bin/env python3
"""Independent, standard-library mathematical curve-order certificate.

For nonzero G, verify a prime n kills G. Since n> #E/2 by Hasse,
#E=n. Curve was chosen by deterministic random search for prime group
order, not prescribed by a weak-order construction.
"""
import json
from math import isqrt
P=2**31-1
A=847514846
B=2087444724
N=2147446831
G=(625037715,2100627251)

def add(U,V):
    if U is None:return V
    if V is None:return U
    x,y=U;u,v=V
    if x==u and (y+v)%P==0:return None
    if x==u:
        assert y !=0
        slope=(3*x*x+A)*pow(2*y,-1,P)%P
    else:slope=(v-y)*pow(u-x,-1,P)%P
    xx=(slope*slope-x-u)%P
    return xx,(slope*(x-xx)-y)%P

def scalar(U,k):
    O=None
    while k:
        if k&1:O=add(O,U)
        U=add(U,U);k>>=1
    return O

def run():
    assert P%4==3 and P>3
    assert all(P%d for d in range(2,isqrt(P)+1))
    assert all(N%d for d in range(2,isqrt(N)+1))
    disc=(-16*(4*A**3+27*B*B))%P
    assert disc !=0 and A!=0 and B!=0
    assert G[1]**2%P==(G[0]**3+A*G[0]+B)%P
    assert scalar(G,N) is None and G is not None
    # True upper Hasse bound: #E <= p+1 + 2sqrt(p)
    assert 2*N > P+1+2*isqrt(P)+2
    small_embedding=[]
    x=P%N
    for k in range(1,1001):
        if x==1:small_embedding.append(k)
        x=x*P%N
    assert not small_embedding
    return dict(field_prime=P,curve_a=A,curve_b=B,discriminant=disc,
        generator=G,prime_group_order=N,cofactor=1,
        hasse_lower=P+1-2*isqrt(P)-2,
        hasse_upper=P+1+2*isqrt(P)+2,
        prime_checks='deterministic trial division through integer square roots',
        order_proof='[N]G=O, prime N, G nonzero, 2N>Hasse upper bound',
        no_embedding_degree_at_most_1000=True,
        not_special_j_0_or_1728=True)
if __name__=='__main__':
    print(json.dumps(run(),indent=2,sort_keys=True))
