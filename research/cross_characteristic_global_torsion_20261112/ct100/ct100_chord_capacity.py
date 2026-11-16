#!/usr/bin/env python3
"""CT100: independent finite-field chord incidence and source-BSGS controls.
Proof of universal bound is algebraic; this is a finite-model falsifier.
"""
import json
from math import isqrt

FIXTURES = ((5,7,10,(5,1)), (7,37,35,(9,4)), (11,59,55,(46,28)),
            (13,139,130,(81,134)), (17,811,850,(27,85)),
            (19,193,190,(84,73)))

def check(ok,msg):
    if not ok:
        raise ArithmeticError(msg)

def rhs(x,p): return (x*x*x-x*x-10*x-20)%p

def neg(P,p):return None if P is None else (P[0],(-1-P[1])%p)

def add(P,Q,p):
    if P is None:return Q
    if Q is None:return P
    x,y=P; u,v=Q
    if x==u and (y+v+1)%p==0:return None
    if P==Q:
        n=(3*x*x-2*x-10)%p
        d=(2*y+1)%p
    else:
        n=(v-y)%p
        d=(u-x)%p
    check(d%p!=0,'nonunit slope')
    slope=n*pow(d,-1,p)%p
    xx=(slope*slope+1-x-u)%p
    yy=(-slope*(xx-x)-y-1)%p
    return (xx,yy)

def mul(P,k,p):
    R=None
    while k:
        if k&1:R=add(R,P,p)
        P=add(P,P,p); k//=2
    return R

def point_count(p):
    # Euler criterion and the discriminant (2y+1)^2=1+4f(x).
    n=1
    for x in range(p):
        delta=(1+4*rhs(x,p))%p
        n+=1 if delta==0 else 2 if pow(delta,(p-1)//2,p)==1 else 0
    return n

def bsgs(P,Q,ell,p):
    # Cold BSGS with baby-table construction, giant-step evaluation and final point proof.
    m=isqrt(ell)
    if m*m<ell:m+=1
    B={None:0}; V=None
    for i in range(1,m):
        V=add(V,P,p)
        B[V]=i
    step=neg(mul(P,m,p),p)
    W=Q
    for j in range(m+1):
        if W in B:
            result=(j*m+B[W])%ell
            check(mul(P,result,p)==Q,'BSGS unrecovered log')
            return result
        W=add(W,step,p)
    raise ArithmeticError('BSGS failed')

def verify(ell,p,n,P):
    check(p not in (2,11,ell) and point_count(p)==n,'wrong curve size')
    check((P[1]*P[1]+P[1]-rhs(P[0],p))%p==0,'P off curve')
    check(mul(P,ell,p) is None and P is not None,'wrong point order')
    pts=[]; W=None
    for k in range(1,ell):
        W=add(W,P,p)
        check(W is not None,'premature identity')
        pts.append(W)
    check(add(W,P,p) is None,'noncyclic generated subgroup')
    check(all(pts[ell-k-1]==neg(pts[k-1],p) for k in range(1,ell)),
          'inverse mismatch')
    classes={pt[0] for pt in pts}
    check(len(classes)==(ell-1)//2,'x-coordinate class count')
    P2=pts[1]
    check(P2[0]!=P[0],'bad source chord')
    slope=(P2[1]-P[1])*pow((P2[0]-P[0])%p,-1,p)%p
    intercept=(P[1]-slope*P[0])%p
    hit_indices=[k for k,(x,y) in enumerate(pts,1)
                 if (y-slope*x-intercept)%p==0]
    check(hit_indices==sorted(set((1,2,ell-3))),'wrong chord incidence')
    hit_x={pts[k-1][0] for k in hit_indices}
    x_classes_covered=len(hit_x)
    expected_classes=min(3,(ell-1)//2)
    check(x_classes_covered==expected_classes,'line covers unexpected roots')
    for k in range(1,ell):
        check(bsgs(P,pts[k-1],ell,p)==k,'public DLP baseline')
    m=(ell-1)//2
    return dict(ell=ell,prime=p,group_order=n,x_classes=m,
                chord_x_classes=x_classes_covered,chord_oriented_indices=hit_indices,chord_slope=slope,chord_intercept=intercept,
                missing_x_classes=m-x_classes_covered,
                general_line_lower_bound=(m+2)//3,
                fixed_basepoint_line_lower_bound=m//2,
                bsgs_cases=ell-1)

rows=[verify(*row) for row in FIXTURES]
check(rows[1]['chord_x_classes']==rows[1]['x_classes']
      and all(r['missing_x_classes']>0 for r in rows[2:]),
      'CT99 saturation must stop beyond ell=7')
print(json.dumps(dict(experiment='CT100-chord-incidence-capacity',
                      fixtures=rows,source_bsgs_verified=True,
                      global_bound='single affine line meets a nonsingular plane cubic in <=3 distinct x-values',
                      pcc='unverified'),sort_keys=True,separators=(',',':')))
