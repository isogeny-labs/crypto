#!/usr/bin/env python3
"""Independent GL(2,l) fixed-vector census and E/Q reduction checks."""
from fractions import Fraction
from math import isqrt

def require(ok, desc):
    if not ok:
        raise AssertionError(desc)

def census(l):
    fixes = {}
    total = 0
    chosen = 0
    for a in range(l):
        for b in range(l):
            for c in range(l):
                for d in range(l):
                    if (a*d-b*c)%l==0: continue
                    total += 1
                    if a == 1 and c == 0: chosen += 1
                    # Independent direct enumeration of every nonzero fixed vector.
                    k = sum((a*x+b*y-x)%l==0 and (c*x+d*y-y)%l==0
                            for x in range(l) for y in range(l) if x or y)
                    fixes[k] = fixes.get(k,0)+1
    wanted = (l*l-1)*(l*l-l)
    expected = {0:wanted-l*(l*l-2), l-1:l**3-2*l-1, l*l-1:1}
    require(total==wanted, f'GL2 order l={l}')
    require(chosen == l*(l-1), f'oriented stabilizer l={l}')
    require(fixes==expected, (l,fixes,expected))
    return total, fixes, chosen

def primes(limit):
    mark = bytearray(b'\x01')*(limit+1)
    mark[:2] = b'\0\0'
    for d in range(2,isqrt(limit)+1):
        if mark[d]:
            mark[d*d::d] = b'\x00'*len(mark[d*d::d])
    return [p for p in range(2,limit+1) if mark[p]]

def order_mod(p):
    # E: y^2+y=x^3-x^2-10x-20; 11a1.
    # z=2y+1, giving #E(Fp)=p+1+sum_x Legendre(1+4 RHS).
    acc=0
    for x in range(p):
        d=(1+4*(x*x*x-x*x-10*x-20))%p
        if d:
            chi = pow(d,(p-1)//2,p)
            require(chi in (1,p-1),f'non-Euler character at {p}')
            acc += 1 if chi==1 else -1
    n = p + 1 + acc
    require(acc*acc <= 4*p, f'Hasse bound failed at p={p}')
    return n

def main():
    for l in (2,3,5,7,11):
        total, by_fixed, oriented=census(l)
        hit=total-by_fixed[0]
        d=Fraction(hit,total)
        formula=Fraction(l*l-2,(l-1)**2*(l+1))
        require(d==formula,(l,d,formula))
        require(Fraction(oriented,total)==Fraction(1,l*l-1), "oriented density")
        # Negative control: an incorrect oriented/existential identification must fail.
        require(Fraction(oriented,total)!=d, "confused oriented and existential density")
        print('GL2',l,'total',total,'fixing_matrices',hit,'any_density',str(d),'oriented_density',str(Fraction(oriented,total)),'rank_counts',sorted(by_fixed.items()))
    n_p=0; hit3=0; hit5=0
    for p in primes(5000):
        if p in (2,3,5,11): continue
        n = order_mod(p)
        hit3 += (n%3==0)
        hit5 += (n%5==0)
        n_p += 1
    require(n_p > 600 and hit5==n_p, (n_p,hit5))
    print('E_11a1 prime sweep p<=5000 excluding 2,3,5,11: samples',n_p,
          '3-divisible',hit3,'3-density',str(Fraction(hit3,n_p)),
          '5-divisible',hit5,'rational-5-torsion control')
    print('PASS CT54 finite counts; Chebotarev theorem conditional, finite sweep is descriptive')
if __name__=='__main__':main()
