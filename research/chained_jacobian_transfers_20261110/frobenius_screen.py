#!/usr/bin/env python3
"""Independent toy genus-2 Frobenius factor sieve by exact point counts.

Brute-force #C(F_p), #C(F_p^2), using quadratic characters via norm.
This is a NECESSARY elliptic isogeny factor test; it is not a map.
"""
import json
import math
PRIMES=(101,103,107,109,127)

def quad_nonresidue(p):
    return next(z for z in range(2,p) if pow(z,(p-1)//2,p)==p-1)

def qm(p,d,x,y):
    a,b=x; c,e=y
    return ((a*c+d*b*e)%p,(a*e+b*c)%p)

def qp(p,d,z,n):
    out=(1,0)
    while n:
        if n&1:out=qm(p,d,out,z)
        z=qm(p,d,z,z);n>>=1
    return out

def chi(p,n):
    n%=p
    return 0 if n==0 else (1 if pow(n,(p-1)//2,p)==1 else -1)

def point_count_base(p,degree):
    return (2 if degree%2==0 else 1)+sum(1+chi(p,pow(x,degree,p)+1) for x in range(p))

def point_count_quadratic(p,d,degree):
    total=2 if degree%2==0 else 1
    for a in range(p):
        for b in range(p):
            z=qp(p,d,(a,b),degree)
            u=(z[0]+1)%p;v=z[1]
            if u==0 and v==0:total+=1
            else:total+=1+chi(p,(u*u-d*v*v)%p)
    return total

def generic_quintic_count(p,d,a=3):
    """y^2=x^5+a*x+1; only called on a smooth fixture."""
    assert (3125+256*pow(a,5,p))%p
    n1=1+sum(1+chi(p,pow(x,5,p)+a*x+1) for x in range(p))
    n2=1
    for x in range(p):
        for y in range(p):
            v=qp(p,d,(x,y),5)
            w=((v[0]+a*x+1)%p,(v[1]+a*y)%p)
            n2+=1 if w==(0,0) else 1+chi(p,(w[0]**2-d*w[1]**2)%p)
    return n1,n2

def elliptic_count(p):
    return 1+sum(1+chi(p,(x*x*x+1)%p) for x in range(p))

def coefficients(p,n1,n2):
    a1=p+1-n1
    num=a1*a1-(p*p+1-n2)
    assert num%2==0
    return a1,num//2

def screen(p,a1,a2,t):
    return a2==t*(a1-t)+2*p

def main():
    records=[]
    for p in PRIMES:
        d=quad_nonresidue(p)
        assert pow(d,(p-1)//2,p)==p-1
        # Independent Euler-power check of norm quadratic character on 49 points.
        for a in range(7):
            for b in range(7):
                z=(a,b)
                sq=qp(p,d,z,(p*p-1)//2)
                want=chi(p,(a*a-d*b*b)%p)
                assert sq==((0,0) if want==0 else ((1,0) if want==1 else (p-1,0)))
        ne=elliptic_count(p);t=p+1-ne
        for name,degree in (("x6_plus_1",6),("x5_plus_1_control",5)):
            n1=point_count_base(p,degree)
            n2=point_count_quadratic(p,d,degree)
            a1,a2=coefficients(p,n1,n2)
            passes=screen(p,a1,a2,t)
            if name=="x6_plus_1":
                assert passes and (a1,a2)==(2*t,t*t+2*p)
            row={"p":p,"curve":name,"nonresidue":d,"E_order":ne,"E_trace":t,
                 "C_Fp":n1,"C_Fp2":n2,"a1":a1,"a2":a2,
                 "elliptic_factor_necessary_test":passes}
            records.append(row)
            print("PASS counted",row)
    assert any(not row["elliptic_factor_necessary_test"] for row in records if row["curve"]!="x6_plus_1")
    p=101;d=quad_nonresidue(p);t=p+1-elliptic_count(p)
    n1,n2=generic_quintic_count(p,d)
    a1,a2=coefficients(p,n1,n2)
    jorder=1-a1+a2-p*a1+p*p
    assert (n1,n2,a1,a2,jorder)==(87,10273,15,148,8820)
    assert math.gcd(jorder,elliptic_count(p))==6
    assert not screen(p,a1,a2,t)
    print(f"NEGATIVE SHARED-SUBGROUP CONTROL: p={p}, #E=102, #J=8820, gcd=6, r=3 shared; Frobenius factor test fails")
    print("COUNTS "+json.dumps(records,sort_keys=True))

if __name__=="__main__":main()
