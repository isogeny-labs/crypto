#!/usr/bin/env python3
"""Exact noncryptographic occupancy oracle for planted Prime31 four-leaf rows.

Two independent calculations for each sampling mode; no elliptic arithmetic,
cryptographic test, PCC signature or source-frozen original-Rust execution.
"""
from fractions import Fraction
from math import comb, factorial

B = 31

def ie_with_replacement(rows):
    denominator = B ** (4 * rows)
    return sum(Fraction((-1)**j * comb(B, j) * (B-j)**(4*rows),
                        denominator) for j in range(B+1))

def stirling_with_replacement(rows):
    st = [0] * (B+1)
    st[0] = 1
    for t in range(1, 4*rows+1):
        new = [0] * (B+1)
        for k in range(1, min(t, B)+1):
            new[k] = k * st[k] + st[k-1]
        st = new
    return Fraction(factorial(B) * st[B], B**(4*rows))

def ie_distinct_per_row(rows):
    denominator = comb(B, 4)**rows
    return sum(Fraction((-1)**j * comb(B, j) * comb(B-j, 4)**rows,
                        denominator)
               for j in range(B-3))

def dp_distinct_per_row(rows):
    den = comb(B, 4)
    states = [Fraction(0)] * (B+1)
    states[0] = Fraction(1)
    for _ in range(rows):
        next_states = [Fraction(0)] * (B+1)
        for old, prob in enumerate(states):
            if prob == 0:
                continue
            for fresh in range(5):
                overlap = 4-fresh
                if overlap <= old and fresh <= B-old:
                    ways = comb(old, overlap)*comb(B-old, fresh)
                    next_states[old+fresh] += prob*Fraction(ways, den)
        states = next_states
    return states[B]

def fmt(value):
    return f"{float(value):.12f}"

for r in (1,20,25,31,40,42,43,44,45,50,60):
    a,b = ie_with_replacement(r), stirling_with_replacement(r)
    c,d = ie_distinct_per_row(r), dp_distinct_per_row(r)
    assert a == b, ("WR mismatch", r)
    assert c == d, ("NR mismatch", r)
    print(f"R={r:02d} WR={fmt(a)} NR={fmt(c)}")
assert ie_with_replacement(43) < Fraction(9,10) <= ie_with_replacement(44)
assert ie_distinct_per_row(41) < Fraction(9,10) <= ie_distinct_per_row(42)
print("PASS: two exact arithmetic methods agree for each model; thresholds 44 and 42")
