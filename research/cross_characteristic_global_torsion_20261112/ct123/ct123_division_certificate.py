#!/usr/bin/env python3
"""CT123: exact integer division polynomial and finite-field Frobenius certificates.
No CAS or third-party packages. The formulas are the standard universal
Weierstrass division-polynomial recurrence (see Sage elliptic-curve manual).
"""
import hashlib
import json


def trim(a):
    a = list(a)
    while len(a) > 1 and a[-1] == 0:
        a.pop()
    return a


def add(a, b):
    return trim([(a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)
                 for i in range(max(len(a), len(b)))])


def neg(a):
    return [-v for v in a]


def sub(a, b):
    return add(a, neg(b))


def mul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, ai in enumerate(a):
        if ai:
            for j, bj in enumerate(b):
                out[i + j] += ai * bj
    return trim(out)


def power(a, n):
    z = [1]
    while n:
        if n & 1:
            z = mul(z, a)
        n >>= 1
        if n:
            a = mul(a, a)
    return z


def division11(aa):
    a1, a2, a3, a4, a6 = aa
    b2 = a1 * a1 + 4 * a2
    b4 = 2 * a4 + a1 * a3
    b6 = a3 * a3 + 4 * a6
    b8 = a1*a1*a6 + 4*a2*a6 - a1*a3*a4 + a2*a3*a3 - a4*a4
    C = [b6, 2*b4, b2, 4]
    u = [b8, 3*b6, 3*b4, b2, 3]  # psi3
    v = [b4*b8 - b6*b6, b2*b8-b4*b6, 10*b8, 10*b6, 5*b4, b2, 2]  # psi4 / psi2
    # Odd-index psi_n and even-index psi_n/psi2 respectively.
    a5 = sub(mul(v, power(C, 2)), power(u, 3))
    a6 = mul(u, sub(a5, power(v, 2)))
    a7 = sub(mul(a5, power(u, 3)), mul(power(v, 3), power(C, 2)))
    a11 = sub(mul(a7, power(a5, 3)), mul(mul(v, power(a6, 3)), power(C, 2)))
    return trim(a11), (b2,b4,b6,b8)


def modpoly(f, p):
    return trim([v % p for v in f])


def divrem(a, b, p):
    a = modpoly(a, p); b = modpoly(b, p)
    if b == [0]:
        raise ZeroDivisionError('zero polynomial')
    inv = pow(b[-1], -1, p)
    while a != [0] and len(a) >= len(b):
        q = (a[-1] * inv) % p
        off = len(a) - len(b)
        for j, bv in enumerate(b):
            a[off+j] = (a[off+j]-q*bv)%p
        a = trim(a)
    return a


def gcd(a, b, p):
    a, b = modpoly(a,p), modpoly(b,p)
    while b != [0]:
        a, b = b, divrem(a, b, p)
    if a == [0]:
        return a
    inv = pow(a[-1], -1, p)
    return modpoly([v*inv for v in a],p)


def remmul(a, b, f, p):
    return divrem(mul(a,b), f, p)


def rempow(a, e, f, p):
    acc = [1]
    while e:
        if e & 1:
            acc = remmul(acc,a,f,p)
        e >>= 1
        if e:
            a = remmul(a,a,f,p)
    return acc


def frob_gcd_degrees(f, p, ks):
    # h_k = X^(p^k) mod f. No factorization oracle.
    f = modpoly(f,p)
    assert f[-1] % p != 0
    assert gcd(f, modpoly([i*v for i,v in enumerate(f)][1:],p), p) == [1]
    h = [0,1]
    out = {}
    for k in range(1, max(ks)+1):
        h = rempow(h,p,f,p)
        if k in ks:
            g = gcd(f, sub(h,[0,1]),p)
            out[str(k)] = len(g)-1
    return out, h == [0,1]


def main():
    source = (0,-1,1,-10,-20)
    plus = (59,-1,60,-10,39)
    minus = (59,58,60,-10,39)
    fs = {n:division11(a)[0] for n,a in [('source',source),('plus',plus),('minus',minus)]}
    assert all(len(f)==61 and f[-1]==11 for f in fs.values())
    assert all(modpoly(f,59)==modpoly(fs['source'],59) for f in fs.values())
    roots = [k for k in range(59) if sum(v*pow(k,i,59) for i,v in enumerate(fs['source']))%59 == 0]
    assert roots == [19,20,22,43,46], roots
    assert [len(str(abs(x))) for x in fs['source']][-1] == 2
    out = {'ct':'CT123', 'source_roots_mod59':roots, 'models':{}}
    for name,prime in [('source',None),('plus',73),('minus',29)]:
        f = fs[name]
        p2, _ = frob_gcd_degrees(f, 2, [1,2,5,10,20])
        p59, _ = frob_gcd_degrees(f, 59, [1,5])
        assert p59 == {'1':5,'5':60},(name,p59)
        expected2 = {'1':0,'2':0,'5':10,'10':60,'20':60} if name!='source' else {'1':0,'2':0,'5':0,'10':0,'20':60}
        assert p2 == expected2, (name,p2)
        entry = {'degree':len(f)-1, 'max_coefficient_digits':max(len(str(abs(v))) for v in f),
                 'sha256_high_to_low':hashlib.sha256(','.join(str(v) for v in reversed(f)).encode()).hexdigest(),
                 'frob_gcd_degrees_2':p2, 'frob_gcd_degrees_59':p59}
        if prime:
            witness, last = frob_gcd_degrees(f, prime, [12,20,30,60])
            assert witness=={'12':0,'20':0,'30':0,'60':60} and last,(name,prime,witness)
            entry['irreducible_mod_prime'] = prime
            entry['rabin_gcd_degrees'] = witness
        out['models'][name] = entry
    print(json.dumps(out, sort_keys=True, separators=(',',':')))

if __name__ == '__main__':
    main()
