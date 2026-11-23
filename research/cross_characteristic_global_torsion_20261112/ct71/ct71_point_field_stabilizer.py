#!/usr/bin/env python3
"""CT71: exhaustive GL2 stabilizer, normalizer, and fixed-torsion audit.
All computations are over small F_ell, not a large-curve DLP solver.
"""
import json
from math import comb

def require(ok, msg):
    if not ok:
        raise AssertionError(msg)

# CT69 order-three field: ascending polynomial coefficient order.
H=(-101306819,89655,267096,349907,162543,-14817,-4813,108,27)
def ev(coeff,x,mod=None):
    acc=0
    for c in reversed(coeff):
        acc=acc*x+c
        if mod is not None: acc%=mod
    return acc
def concrete_control():
    transformed=[sum(H[j]*comb(j,i)*(-1)**j for j in range(i,len(H)))
                 for i in range(len(H))]
    require(tuple(transformed)==H, "negation beta -> -beta-1 is not a Q-automorphism")
    require(ev(H,5,29)==ev(H,23,29)==0,"bad mod-29 source roots")
    require((-5-1)%29==23,"bad source negation")

PRIMES=(3,5,7,11)

def mul(A,B,p):
    a,b,c,d=A; e,f,g,h=B
    return ((a*e+b*g)%p,(a*f+b*h)%p,(c*e+d*g)%p,(c*f+d*h)%p)

def inv(A,p):
    a,b,c,d=A
    D=(a*d-b*c)%p
    if not D: raise ValueError('singular matrix')
    z=pow(D,-1,p)
    return (d*z%p,-b*z%p,-c*z%p,a*z%p)

def vector(A,v,p):
    a,b,c,d=A; x,y=v
    return ((a*x+b*y)%p,(c*x+d*y)%p)

def main():
    concrete_control()
    for ell in PRIMES:
        G=[(a,b,c,d) for a in range(ell) for b in range(ell)
           for c in range(ell) for d in range(ell) if (a*d-b*c)%ell]
        H={(1,a,0,b) for a in range(ell) for b in range(1,ell)}
        require(len(G)==(ell*ell-1)*(ell*ell-ell), "GL2 count")
        require(len(H)==ell*(ell-1), "H size")
        fixed=sorted((x,y) for x in range(ell) for y in range(ell)
                     if all(vector(h,(x,y),ell)==(x,y) for h in H))
        require(fixed==[(x,0) for x in range(ell)], "H-invariants")
        # Full conjugation against EVERY H-element, not just generating matrices.
        normalizer=[]
        for g in G:
            gi=inv(g,ell)
            if all(mul(mul(g,h,ell),gi,ell) in H for h in H):
                normalizer.append(g)
        require(len(normalizer)==ell*(ell-1)**2,"normalizer size")
        require(set(vector(g,(1,0),ell) for g in normalizer)=={(x,0) for x in range(1,ell)}, "normalizer images")
        orbit={vector(g,(1,0),ell) for g in G}
        require(len(orbit)==ell*ell-1, "orbit degree")
        record={'ell':ell,'gl2_size':len(G),'stabilizer_size':len(H),
                'point_field_degree':len(orbit),
                'K_rational_ell_torsion_size':len(fixed),
                'K_rational_nonzero_points':len(fixed)-1,
                'normalizer_size':len(normalizer),
                'K_field_automorphisms':len(normalizer)//len(H),
                'automorphism_images':sorted({v[0] for v in
                    (vector(g,(1,0),ell) for g in normalizer)})}
        require(record['K_field_automorphisms']==ell-1, 'K-aut count')
        print(json.dumps(record,separators=(',',':')))
    print('ct69_negation_verified=true')
    print('ct71_verified=true')
if __name__=='__main__':main()
