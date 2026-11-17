#!/usr/bin/env python3
"""Independent F_p^2 point-count oracle using a full square lookup table.

Does NOT use Frobenius-screen's quadratic character, norm formula, or
generic extension-field exponentiation. Evaluates powers by the binomial
theorem, hashes every y^2, and counts roots of the curve equation.
"""
from collections import Counter
from math import comb

EXPECTED={
    101:((102,10606),(98,9958)),
    103:((64,10222),(104,10610)),
    107:((108,11878),(108,11450)),
    109:((106,12310),(110,12318)),
    127:((88,15838),(128,16130)),
}

def legendre(p,z):
    if z%p==0:return 0
    return 1 if pow(z%p,(p-1)//2,p)==1 else -1

def nonresidue(p):
    return next(d for d in range(2,p) if legendre(p,d)==-1)

def exp_pair(a,b,n,p,d):
    real=imag=0
    for k in range(n+1):
        coeff=comb(n,k)*pow(a,n-k,p)*pow(b,k,p)*pow(d,k//2,p)
        if k%2:imag+=coeff
        else:real+=coeff
    return real%p,imag%p

def independent_count(p,n,c=0):
    d=nonresidue(p)
    squares=Counter()
    for a in range(p):
        for b in range(p):
            squares[((a*a+d*b*b)%p,(2*a*b)%p)]+=1
    assert sum(squares.values())==p*p
    total=2 if n%2==0 else 1
    for a in range(p):
        for b in range(p):
            u,v=exp_pair(a,b,n,p,d)
            total+=squares[((u+c*a+1)%p,(v+c*b)%p)]
    return total

def main():
    checked=0
    for p,((n1,n2),(m1,m2)) in EXPECTED.items():
        for degree,expected in ((6,n2),(5,m2)):
            got=independent_count(p,degree)
            assert got==expected,(p,degree,got,expected)
            print(f"PASS independent F_{p}^2 square-table curve x^{degree}+1 N2={got}")
            checked+=1
    assert independent_count(101,5,c=3)==10273
    print(f"PASS independent 11 Fp2 counts ({checked} controls plus shared-r=3 negative case)")

if __name__=="__main__":main()
