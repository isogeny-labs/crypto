#!/usr/bin/env python3
"""Search for a Curve25519-faithful miniature family: primes p = 2^k - c (c smallest) with
  p = 5 (mod 8), v2(p-1) = 2, 3 | p-1, (p-1)/12 = q*C with q, C prime, q < 2^(k/8), v2(p+1) = 1,
then the smallest Montgomery A >= 3 with y^2 = x^3 + A x^2 + x having #E = 8*l (l prime), twist order 4*l' (l' prime),
and A^2 - 4 a non-square (one rational 2-torsion point, cyclic Z/8). Brute-force point counts (k <= 24).
Compare with the lab's family rule (p = 2^k - c smallest prime, any p-1 shape). stdlib only."""
import sys
def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True
def shape_ok(p, k):
    if p % 8 != 5 or (p-1) % 3 != 0 or (p+1) % 4 == 0: return None
    m = (p-1)//12
    if m % 2 == 0 or m % 3 == 0: return None
    # m = q*C with q prime < 2^(k/8), C prime
    q = 5
    lim = 2**(k/8)
    while q < lim:
        if m % q == 0:
            C = m//q
            if is_prime(q) and is_prime(C) and C > q: return (q, C)
        q += 2
    return None
_sq = {}
def squares(p):
    if p not in _sq:
        t = bytearray(p)
        for y in range(1, (p+1)//2):
            t[y*y % p] = 1
        _sq.clear(); _sq[p] = t
    return _sq[p]
def count(p, A):
    # #E for y^2 = x^3 + A x^2 + x with a residue table; returns (#E, #twist)
    sq = squares(p)
    n = 1
    for x in range(1, p):
        v = (x*x*x + A*x*x + x) % p
        if v == 0: n += 1
        elif sq[v]: n += 2
    return n + 1, 2*p + 2 - (n + 1)
for k in range(16, 25, 2):
    cmax = 4000 if k >= 20 else 60000
    c = 1; found = None; tried = 0
    while c < cmax:
        p = 2**k - c
        if is_prime(p):
            tried += 1
            s = shape_ok(p, k)
            if s: found = (p, c, s); break
        c += 2
    if not found:
        print(f"k={k}: no faithful prime with c < {cmax} ({tried} primes tried)"); sys.stdout.flush(); continue
    p, c, (q, C) = found
    print(f"k={k}: p = 2^{k} - {c} = {p}, p-1 = 4*3*{q}*{C}, p+1 v2=1 ({tried} primes tried)"); sys.stdout.flush()
    A = 3; hit = None
    while A < 5000:
        if pow((A*A - 4) % p, (p-1)//2, p) == 1: A += 1; continue   # need A^2-4 non-square (cyclic 2-part)
        nE, nT = count(p, A)
        if nE % 8 == 0 and is_prime(nE//8) and nT % 4 == 0 and is_prime(nT//4):
            hit = (A, nE//8, nT//4); break
        A += 1
    print(f"   curve: A = {hit[0]}, #E = 8*{hit[1]}, twist = 4*{hit[2]}" if hit else "   no curve with A < 5000"); sys.stdout.flush()
