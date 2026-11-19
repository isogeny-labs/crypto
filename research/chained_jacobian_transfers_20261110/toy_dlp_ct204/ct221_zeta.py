#!/usr/bin/env python3
"""CT221: independently count F101^2 points and derive exact split genus-4 order.

Uses two nonsquare F101^2 models and square tables versus Euler characters.
The L-polynomial deduction uses the known degree-3 C -> E map, Weil's
functional equation, and E's rational order 109. Not a performance result.
"""
import argparse
import json
from collections import Counter
from math import gcd

Q, R = 101, 109


def check(ok, why):
    if not ok:
        raise ValueError(why)


def census(d, euler):
    check(pow(d, 50, Q) == Q-1, 'reducible quadratic extension')
    def mul(x, y):
        a,b=x; c,e=y
        return ((a*c+d*b*e)%Q, (a*e+b*c)%Q)
    def power(x, k):
        out=(1,0)
        while k:
            if k & 1: out=mul(out,x)
            x=mul(x,x); k>>=1
        return out
    xs=[(a,b) for a in range(Q) for b in range(Q)]
    squares=Counter(mul(y,y) for y in xs) if not euler else None
    def nroots(v):
        if squares is not None: return squares[v]
        if v == (0,0): return 1
        ch=power(v, (Q*Q-1)//2)
        check(ch in ((1,0),(Q-1,0)), 'not a quadratic character')
        return 2 if ch == (1,0) else 0
    totals=[]
    for cover in (False, True):
        count=1
        for x in xs:
            cube=mul(mul(x,x),x)
            y=mul(mul(cube,cube),cube) if cover else cube
            z=cube if cover else x
            v=((y[0]+z[0]+46)%Q,(y[1]+z[1])%Q)
            count+=nroots(v)
        totals.append(count)
    return totals


def derive():
    check(Q % 3 == 2 and gcd(3,Q-1) == 1, 'cube bijection failed')
    try: census(1,False)
    except ValueError: pass
    else: raise ValueError('reducible negative control passed')
    counts=[census(d,euler) for d in (2,3) for euler in (False,True)]
    check(all(z == [10355,10947] for z in counts), 'inconsistent Fq^2 counts')
    ne,nc=counts[0]
    # q == -1 mod 3, so cubing permutes F_{q^n} for odd n.
    # Hence S1(A)=S3(A)=0 for dim(A)=3, and S2(A)=ne-nc.
    # Newton + Weil symmetry: LA(T)=1+a*T^2+q*a*T^4+q^3*T^6.
    a=(nc-ne)//2
    check(2*a == nc-ne, 'a not integral')
    la=[1,0,a,0,Q*a,0,Q**3]
    le=[1,7,Q] # #E(Fq)=109 implies trace -7.
    lj=[sum(le[i]*la[k-i] for i in range(3) if 0<=k-i<7)
        for k in range(9)]
    h=sum(la); order=sum(lj)
    check((h,order)==(1060494,115593846), 'unexpected order')
    check(order==R*h and all(lj[8-i]==Q**(4-i)*lj[i] for i in range(4)),
          'Jacobian zeta symmetry mismatch')
    check(gcd(h,R)==1 and h%R==33, '109-primary overlap')
    inv=pow(h,-1,R); projector=h*inv
    check(inv==76 and projector==80597544 and projector%R==1
          and projector%h==0 and (projector*projector-projector)%order==0,
          'CRT projector invalid')
    factors=[2,3,17,37,109,281]
    prod=1
    for f in factors: prod*=f
    check(prod==order and all(all(f%d for d in range(2,int(f**.5)+1))
                             for f in factors), 'factor certificate invalid')
    return {'schema':'ct221-f101-exact-composite-jacobian-v1',
            'counts_fp2':{'E':ne,'C':nc},'nonresidues':[2,3],
            'count_methods':['square-table','Euler-character'],
            'a2_complement':a,'L_A_ascending':la,'L_J_ascending':lj,
            'order_A':h,'order_J':order,'order_J_prime_factors':factors,
            'source_sylow_109_multiplicity':1,'cofactor_h_mod_109':h%R,
            'cofactor_inverse_mod_109':inv,'cofactor_projector_scalar':projector,
            'standing':'exact toy zeta arithmetic, not complete-cost evidence',
            'pcc':'unverified'}


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--self-test',action='store_true')
    args=ap.parse_args();out=derive()
    if args.self_test:
        print('CT221_SELFTEST_PASS',out['order_J'],out['source_sylow_109_multiplicity'])
    else: print(json.dumps(out,sort_keys=True,indent=2))
