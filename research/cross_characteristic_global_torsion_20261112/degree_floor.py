#!/usr/bin/env python3
"""Exact Hasse-Weil necessary residue-degree audit, no floating point."""
from math import comb

def possible(n, g, q):
    if n < 1 or g < 1 or q < 2:
        raise ValueError("invalid n, g, or q")
    even = sum(comb(2*g, 2*j) * q**j for j in range(g+1))
    odd = sum(comb(2*g, 2*j+1) * q**j for j in range(g))
    return n <= even or (n-even)**2 <= q * odd**2

def first_degree(n, g, characteristic=2, limit=2048):
    for f in range(1, limit + 1):
        if possible(n,g,characteristic**f):
            return f
    raise ValueError("degree outside bounded range")

def main():
    tests=(5,17,257,65537,(1<<128)-159,(1<<256)-189)
    expected=((1,1,1,1),(4,1,1,1),(8,4,2,1),(16,8,5,4),
              (128,64,43,32),(256,128,86,64))
    for n,row in zip(tests,expected):
        observed=tuple(first_degree(n,g) for g in range(1,5))
        if observed!=row:
            raise AssertionError((n,observed,row))
        for g,f in enumerate(observed,1):
            if not possible(n,g,2**f) or (f>1 and possible(n,g,2**(f-1))):
                raise AssertionError("degree not minimal")
        print("N_bits",n.bit_length(),"g=1..4",observed)
    if not possible(5,1,2) or possible(6,1,2):
        raise AssertionError("F2 elliptic upper-bound control")
    print("PASS exact necessary residue-degree controls; not an existence test")
if __name__=="__main__":
    main()
