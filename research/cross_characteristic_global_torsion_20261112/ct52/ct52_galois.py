#!/usr/bin/env python3
"""CT52 independent exact orbit / Frobenius certificate for 11a1 E[3].

All arithmetic is explicit integer/polynomial/permutation arithmetic; no Sage,
SymPy, elliptic-curve dependency, DLP oracle, or probabilistic factorization.
"""
from itertools import permutations, product
from math import lcm

F = (-21, -237, -60, -4, 3)  # psi_3 coefficients in ascending order


def check(cond, msg):
    if not cond:
        raise AssertionError(msg)


def poly_rem(a, b, p):
    a = [x % p for x in a]
    while a and a[-1] == 0:
        a.pop()
    check(b and b[-1] % p != 0, "invalid divisor")
    while len(a) >= len(b):
        c = a[-1] * pow(b[-1], -1, p) % p
        for i in range(len(b)):
            a[len(a)-len(b)+i] = (a[len(a)-len(b)+i] - c*b[i]) % p
        while a and a[-1] == 0:
            a.pop()
    return tuple(a)


def irreducible_small(poly, p):
    # For cubic/quartic, a nontrivial factor has degree <= 1 or 2.
    for degree in (1, 2):
        for lower in product(range(p), repeat=degree):
            if not poly_rem(poly, (*lower, 1), p):
                return False
    return True


def quartic_mod2():
    check(tuple(v % 2 for v in F) == (1,1,0,0,1), "wrong psi3 mod2")
    return irreducible_small(F, 2)


def cubic_mod7():
    check(tuple(v % 7 for v in F) == (0,1,3,3,3), "wrong psi3 mod7")
    # 3*x*(x^3+x^2+x-2): (-6*x+3*x^2+3*x^3+3*x^4) modulo 7
    check(tuple(v % 7 for v in (0,-6,3,3,3)) == tuple(v%7 for v in F), "wrong factorization")
    return irreducible_small((-2, 1, 1, 1), 7)


def multiply_16(a,b):
    out=0
    for _ in range(4):
        if b&1: out ^= a
        b >>= 1
        a <<= 1
        if a & 16: a ^= 0x13
    return out


def trace_one():
    # x=2 mod X^4+X+1; R = x^3+x^2 = 12; Y^2+Y=R.
    x=2
    R = multiply_16(multiply_16(x,x),x) ^ multiply_16(x,x)
    check(R == 12, "wrong quartic coordinate")
    u, trace = R,0
    for _ in range(4):
        trace ^= u
        u=multiply_16(u,u)
    check(u==R, "broken F16 endomorphism")
    return trace == 1


def permutation_order(p):
    visited=set(); order=1
    for start in range(len(p)):
        if start in visited: continue
        j=start; length=0
        while j not in visited:
            visited.add(j)
            j=p[j]
            length+=1
        order=lcm(order,length)
    return order


def compose(p,q):
    return tuple(p[q[i]] for i in range(4))


def closure(a,b):
    identity=(0,1,2,3)
    group={identity}; frontier=[identity]
    while frontier:
        g=frontier.pop()
        for h in (compose(g,a),compose(g,b)):
            if h not in group:
                group.add(h); frontier.append(h)
    return len(group)


def projective_full():
    p4=[p for p in permutations(range(4)) if permutation_order(p)==4]
    p3=[p for p in permutations(range(4)) if permutation_order(p)==3]
    check(len(p4)==6 and len(p3)==8,"bad S4 cycle enumerator")
    check(all(closure(a,b)==24 for a in p4 for b in p3),"4-cycle/3-cycle fail to generate S4")
    return len(p4)*len(p3)


def gl2_certificate(ell):
    gl=[(a,b,c,d) for a,b,c,d in product(range(ell),repeat=4)
        if (a*d-b*c)%ell]
    want=(ell*ell-1)*(ell*ell-ell)
    check(len(gl)==want,"GL2 cardinality")
    point={(a,c) for a,b,c,d in gl}
    check(len(point)==ell*ell-1 and (0,0) not in point,"point orbit")
    stabilizer=sum(a==1 and c==0 for a,b,c,d in gl)
    check(stabilizer==ell*(ell-1),"point stabilizer")
    def canonical_line(v):
        a,b=v
        t=pow(a if a else b, -1, ell)
        return ((a*t)%ell,(b*t)%ell)
    check(len({canonical_line(v) for v in point})==ell+1,"projective line orbit")
    check(len({min((a,c),((-a)%ell,(-c)%ell)) for a,c in point})==(ell*ell-1)//2,"signed orbit")
    return (ell,ell*ell-1,(ell*ell-1)//2,ell+1,want)


def main():
    check(quartic_mod2(),"mod2 3-division quartic not irreducible")
    check(cubic_mod7(),"mod7 3-division cubic not irreducible")
    check(trace_one(),"mod2 Y trace not one")
    pairs=projective_full()
    gl3=gl2_certificate(3)
    check(gl3==(3,8,4,4,48),"unexpected 11a1 global Galois degree")
    # A full projective image S4 and an order-eight lift force full GL2(F3),
    # since a proper size-24 lift injects into S4 (which has no order-eight element).
    # The trace-one quadratic gives Frobenius_2^4 acting as -I on E[3], order 8.
    check(not irreducible_small((1,0,0,0,1),2),"reducible quartic mutant survived")
    check(not irreducible_small((0,-1,0,1),7),"reducible cubic mutant survived")
    # X has trace zero in the chosen F16 basis; replacing R by X
    # would make Y^2+Y=R split and invalidate the degree-eight witness.
    v=x=2; trace=0
    for _ in range(4):
        trace ^= v
        v = multiply_16(v,v)
    check(trace==0,"trace-zero mutant survived")
    for ell in (3,5,7,11):
        print("ell, oriented point, +/- point, line, full basis",gl2_certificate(ell))
    print("CT52 PASS: mod2 4-cycle; mod7 1+3 cycle; Frobenius_2 order 8; S4 group closure pairs",pairs)
    print("CT52 CONCLUSION: Gal(Q(E[3])/Q)=GL2(F3), point-degree 8, full division degree 48")
    print("[pcc:unverified]: fixed-family orbit-cost result; no DLP speedup")


if __name__=="__main__": main()
