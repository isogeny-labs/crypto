#!/usr/bin/env python3
"""CT67 exact two-prime orientation audit; public 11a1 mod-3 control.

Not a construction of a global algorithm to lift arbitrary encoded points.
"""
import json
from itertools import product

P=3
I=(1,0,0,1)
F=(0,1,1,1)       # Frobenius at 2 on 11a1[3], in a suitable basis
V=(1,0)

def matmul(a,b,p=P):
    return ((a[0]*b[0]+a[1]*b[2])%p,(a[0]*b[1]+a[1]*b[3])%p,
            (a[2]*b[0]+a[3]*b[2])%p,(a[2]*b[1]+a[3]*b[3])%p)

def det(a,p=P):return (a[0]*a[3]-a[1]*a[2])%p

def inverse(a,p=P):
    inv=pow(det(a,p),-1,p)
    return tuple(inv*x%p for x in (a[3],-a[1],-a[2],a[0]))

def point(a,v,p=P):return ((a[0]*v[0]+a[1]*v[1])%p,(a[2]*v[0]+a[3]*v[1])%p)

def powers(a,p=P):
    h=I;out=[]
    for n in range(1,100):
        h=matmul(h,a,p);out.append(h)
        if h==I:return n,out
    raise AssertionError("matrix order > 100")

def curve_points(p):
    total=1
    for x in range(p):
        rhs=(x*x*x-x*x-10*x-20)%p
        total+=sum((y*y+y-rhs)%p==0 for y in range(p))
    return total

def check_source_point():
    x,y=18,5;p=29
    rhs=(x*x*x-x*x-10*x-20)%p
    if not ((y*y+y-rhs)%p==0): raise AssertionError("CT67 check failed")
    # Standard generalized-Weierstrass tangent at P, followed by doubling.
    lam=((3*x*x-2*x-10)*pow(2*y+1,-1,p))%p
    nu=(y-lam*x)%p
    x2=(lam*lam+1-2*x)%p
    y2=(-lam*x2-nu-1)%p
    if not ((x2,y2)==(18,23) == (x,(-y-1)%p)): raise AssertionError("CT67 check failed")
    if not (curve_points(29)==30 and curve_points(2)==5): raise AssertionError("CT67 check failed")

def audit():
    check_source_point()
    G=[a for a in product(range(P),repeat=4) if det(a)]
    if not (len(G)==48): raise AssertionError("CT67 check failed")
    if not ((F[0]+F[3])%3==1 and det(F)==2): raise AssertionError("CT67 check failed")
    order,seq=powers(F)
    if not (order==8 and seq[3]==(2,0,0,2)): raise AssertionError("CT67 check failed")
    central=[g for g in G if matmul(F,g)==matmul(g,F)]
    if not (len(central)==8): raise AssertionError("CT67 check failed")
    orbit={point(g,V) for g in central}
    if not (len(orbit)==8 and (0,0) not in orbit): raise AssertionError("CT67 check failed")
    if not (len({matmul(matmul(g,F),inverse(g)) for g in G})==6): raise AssertionError("CT67 check failed")
    if not (all(sum(point(g,V)==u for g in central)==1 for u in orbit)): raise AssertionError("CT67 check failed")
    # In the full image, the pointwise stabilizer has size 6. The full
    # binary-Frobenius centralizer is smaller, but still gives all 8 U.
    if not (sum(point(g,V)==V for g in G)==6): raise AssertionError("CT67 check failed")
    if not (sum(point(g,V)==V for g in central)==1): raise AssertionError("CT67 check failed")
    # A fixed oriented generator image fixes the whole cyclic restriction,
    # but not a full basis extension. This is the important distinction.
    for k in range(3):
        for g in central:
            v=(k,0)
            u=point(g,V)
            if not (point(g,v)==(k*u[0]%3,k*u[1]%3)): raise AssertionError("CT67 check failed")
    if not (any(point(g,V)!=V for g in central)): raise AssertionError("CT67 check failed")
    # Enforce the counterexample: trace and determinant alone, and even the
    # literal Frobenius matrix, cannot select a specific oriented point.
    A=next(g for g in central if point(g,V)==(0,1))
    if not (matmul(A,F)==matmul(F,A) and point(A,V)!=(1,0)): raise AssertionError("CT67 check failed")
    record={"curve":"11a1", "ell":3,"p_source":29,"p_target":2,
            "source_point":[18,5], "source_curve_order":30, "binary_curve_order":5,
            "target_residue_degree":8,
            "frobenius_trace_mod3":1,"frobenius_det_mod3":2,
            "frobenius_order":order,"frobenius_fourth":"-I",
            "full_G_size":len(G),"fixed_generator_stabilizer":6,
            "conjugate_frobenius_matrices":6,
            "F_centralizer_size":len(central),"F_centralizer_orbit":len(orbit),
            "F_centralizer_vector_stabilizer":1,
            "same_F_different_image":True,"scalar_compatibility":True}
    print(json.dumps(record,sort_keys=True,separators=(',',':')))
if __name__=='__main__':audit()
