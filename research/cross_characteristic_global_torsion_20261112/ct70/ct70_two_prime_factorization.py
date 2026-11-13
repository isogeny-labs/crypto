#!/usr/bin/env python3
"""CT70 exact two-prime torsion point-field Frobenius-cycle certificate.

Independently checks CT69's degree-eight H(Y) by modular gcd and
matrix permutations. Only a bounded, unverified mathematical fixture.
"""
import json
from collections import Counter

H=(-101306819,89655,267096,349907,162543,-14817,-4813,108,27)


def trim(a):
    a=list(a)
    while a and a[-1]==0:a.pop()
    return tuple(a)


def reduce_coeff(a,p):return trim([v%p for v in a])


def divmod_poly(a,b,p):
    a=list(reduce_coeff(a,p));b=reduce_coeff(b,p)
    if not b:raise ZeroDivisionError('polynomial divisor zero')
    quotient=[0]*max(0,len(a)-len(b)+1)
    inv=pow(b[-1],-1,p)
    while len(a)>=len(b):
        shift=len(a)-len(b);coef=a[-1]*inv%p
        quotient[shift]=coef
        for j,v in enumerate(b):a[shift+j]=(a[shift+j]-coef*v)%p
        while a and a[-1]==0:a.pop()
    return trim(quotient),tuple(a)


def mulmod(a,b,f,p):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):c[i+j]=(c[i+j]+x*y)%p
    return divmod_poly(c,f,p)[1]


def powmod_poly(a,e,f,p):
    result=(1,)
    while e:
        if e&1:result=mulmod(result,a,f,p)
        a=mulmod(a,a,f,p);e>>=1
    return result


def gcd_poly(a,b,p):
    a=reduce_coeff(a,p);b=reduce_coeff(b,p)
    while b:a,b=b,divmod_poly(a,b,p)[1]
    if not a:return ()
    return tuple(v*pow(a[-1],-1,p)%p for v in a)


def profile(p):
    f=reduce_coeff(H,p)
    d=reduce_coeff([i*v for i,v in enumerate(H)][1:],p)
    assert gcd_poly(f,d,p)==(1,), 'bad prime / repeated factors'
    z=(0,1);counts={}
    for k in (1,2,4,8):
        z=powmod_poly((0,1),p**k,f,p)
        z_minus_x=list(z)+[0]*max(0,2-len(z))
        z_minus_x[1]=(z_minus_x[1]-1)%p
        counts[k]=len(gcd_poly(f,z_minus_x,p))-1
    # Only degrees dividing 8 can occur because F_2 is irreducible
    # or F_29 is completely split into degrees 1 and 2; these facts
    # are verified from the N_8 count, not assumed.
    assert counts[8]==8
    n1=counts[1];n2=(counts[2]-n1)//2
    n4=(counts[4]-n1-2*n2)//4
    n8=(counts[8]-n1-2*n2-4*n4)//8
    assert n1+2*n2+4*n4+8*n8==8
    return {'gcd_degree':counts,'factor_degrees':([1]*n1+[2]*n2+[4]*n4+[8]*n8)}


def cycles(matrix,ell):
    seen=set();counts=Counter()
    def action(v):return ((matrix[0][0]*v[0]+matrix[0][1]*v[1])%ell,
                          (matrix[1][0]*v[0]+matrix[1][1]*v[1])%ell)
    for x in range(ell):
        for y in range(ell):
            v=(x,y)
            if v==(0,0) or v in seen:continue
            w=v;size=0
            while w not in seen:
                seen.add(w);size+=1;w=action(w)
            assert w==v
            counts[size]+=1
    assert sum(d*n for d,n in counts.items())==ell*ell-1
    return sorted([d for d,n in counts.items() for _ in range(n)])


def main():
    source=profile(29);binary=profile(2)
    source_matrix=[[1,0],[0,2]]  # charpoly X^2-1 = X^2-t29 X+29 mod3, t29=0
    binary_matrix=[[0,1],[1,1]] # charpoly X^2-X-1 = X^2-t2 X+2 mod3, t2=-2
    source_cycles=cycles(source_matrix,3)
    binary_cycles=cycles(binary_matrix,3)
    assert source_cycles==source['factor_degrees']==[1,1,2,2,2]
    assert binary_cycles==binary['factor_degrees']==[8]
    print(json.dumps({'ell':3,'field_degree':8,'source_prime':29,'target_prime':2,
        'source_trace':0,'target_trace':-2,
        'source_factor_degrees':source['factor_degrees'],
        'target_factor_degrees':binary['factor_degrees'],
        'source_frobenius_cycles':source_cycles,
        'target_frobenius_cycles':binary_cycles,
        'source_gcd_degrees':source['gcd_degree'],
        'target_gcd_degrees':binary['gcd_degree'],
        'both_squarefree':True,'global_lift_algorithm':False,
        'scalable_ecdpl':False},sort_keys=True,separators=(',',':')))


if __name__=='__main__': main()
