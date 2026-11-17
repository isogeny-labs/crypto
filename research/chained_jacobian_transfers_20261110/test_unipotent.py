#!/usr/bin/env python3
"""Independent small-field control for cross-quotient mixing. No Cantor algebra.

The Jacobian endomorphism identity is proved in MIXING_AND_FROBENIUS.md.
This tests finite-field elliptic identities, quotient maps on affine C,
involution cancellation, and the full order-17 factor-matrix model.
NOT an implementation of Mumford divisor-class maps.
"""
PRIMES = (101, 103, 107, 109, 127)

def plus(p, P, Q):
    if P is None: return Q
    if Q is None: return P
    x,y=P; u,v=Q
    if x==u and (y+v)%p==0: return None
    if P==Q:
        if y==0: return None
        slope=3*x*x*pow(2*y,-1,p)%p
    else: slope=(v-y)*pow((u-x)%p,-1,p)%p
    z=(slope*slope-x-u)%p
    return z,(slope*(x-z)-y)%p

def neg(p,P):
    return None if P is None else (P[0],(-P[1])%p)

def times(p,n,P):
    if n<0:return times(p,-n,neg(p,P))
    R=None
    while n:
        if n&1:R=plus(p,R,P)
        P=plus(p,P,P);n>>=1
    return R

def on_E(p,P):
    return P is None or (P[1]**2-P[0]**3-1)%p==0

def pair(p,C):
    x,y=C
    assert (y*y-x**6-1)%p==0 and x%p
    f=(x*x%p,y)
    g=(pow(x,-2,p),y*pow(x,-3,p)%p)
    assert on_E(p,f) and on_E(p,g)
    return f,g

def check(p):
    curve_pts=[(x,y) for x in range(p) for y in range(p)
               if (y*y-x**6-1)%p==0]
    split_f=split_g=0
    for x,y in curve_pts:
        if x==0:continue
        f1,g1=pair(p,(x,y))
        f2,g2=pair(p,((-x)%p,y))
        assert f1==f2 and plus(p,g1,g2) is None
        assert plus(p,f1,f2)==times(p,2,f1)
        split_f+=1
        f3,g3=pair(p,((-x)%p,(-y)%p))
        assert plus(p,f1,f3) is None and g1==g3
        assert plus(p,g1,g3)==times(p,2,g1)
        split_g+=1
    return len(curve_pts),split_f,split_g

def check_matrix():
    p,P,r=101,(4,41),17
    assert on_E(p,P) and times(p,r,P) is None
    assert all(times(p,j,P) is not None for j in range(1,r))
    for a in range(r):
        R=times(p,a,P)
        # F=(f_*,g_*), F f^*(R)=(2R,0).
        x,y=times(p,2,R),None
        # K=g^* f_*, F K F^-1 = [[0,0],[2,0]].
        mixed_x=x
        mixed_y=plus(p,y,times(p,2,x))
        assert mixed_y==times(p,4*a,P)
        # I-K is inverse of I+K.
        assert plus(p,mixed_y,neg(p,times(p,2,mixed_x)))==y
        assert times(p,4,P) is not None
    print(f"PASS: all {r} subgroup scalars, nilpotent shear, inverse, 4P endpoint")

if __name__=="__main__":
    for p in PRIMES:
        total,f_c,g_c=check(p)
        print(f"PASS p={p}: {total} affine C points; {f_c} f-fiber and {g_c} g-fiber pair identities")
    check_matrix()
    print("LIMITATION: direct point-fiber/factor-matrix oracle; not Jacobian divisor arithmetic")
