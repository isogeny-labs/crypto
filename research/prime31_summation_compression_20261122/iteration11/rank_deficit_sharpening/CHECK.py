#!/usr/bin/env python3
"""Exact integer/rational combinatorics only; no EC operations, PRNG, or timing."""
from fractions import Fraction as F
from math import comb

B, r, H = 31, 2147446831, 503440

def C(n, k):
    return comb(n, k) if 0 <= k <= n else 0

def old(k):
    return sum((16 - 2**(4-l))*C(k,l)*C(B-k,4-l) for l in range(1,5))

def w(l,t):
    if l+t == 0: return 0
    if t == 0 or l+t == 1: return 16
    if l == 0: return 8
    return 16 - 2**(4-l)

def bound(k,s):
    return sum(w(l,t)*C(k,l)*C(s-k,t)*C(B-s,4-l-t)
               for l in range(5) for t in range(5-l))

def new(k):
    return min(bound(k,s) for s in range(k,B+1))

def check_tails(fn, n95, n99):
    ps = [F(fn(k),r-1) for k in range(1,B+1)]
    x = min(ps)/2
    # z=1+x; log(z) <= x and log(z) >= x-x*x/2.
    # log MGF <= B*x + sum(-log(1-a_k)).
    upper = B*x
    for p in ps:
        a = (1-p)*x/p
        assert 0 <= a < 1
        upper += sum(a**j/j for j in range(1,21)) + a**21/(21*(1-a))
    u95 = upper-n95*(x-x*x/2)
    u99 = upper-n99*(x-x*x/2)
    assert u95 < -3 and u99 < F(-461,100)
    # Taylor partial sums certify exp(3)>20 and exp(4.61)>100.
    from math import factorial
    assert sum(F(3)**j/factorial(j) for j in range(21)) > 20
    assert sum(F(461,100)**j/factorial(j) for j in range(31)) > 100
    mean = sum(1/p for p in ps)
    print("mean queries", float(mean), "mean positive draws", float(mean*F(H,r-1)))
    print("log-tail upper bounds", float(u95), float(u99))

if __name__ == "__main__":
    assert 16*C(B,4)==H
    assert [new(k) for k in (1,2,3,4,5,8,16,31)] == [64960,123424,175840,222640,264240,288344,367080,503440]
    for k in range(1,B+1):
        assert new(k) >= old(k)
        assert new(k) <= 16*(C(B,4)-C(B-k,4))
        if k<=5 or k==B:
            assert new(k) == 16*(C(B,4)-C(B-k,4))
        print(k,old(k),new(k))
    check_tails(old,760000,973000)
    check_tails(new,453000,560000)
