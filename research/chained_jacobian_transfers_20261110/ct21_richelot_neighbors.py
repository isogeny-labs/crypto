#!/usr/bin/env python3
"""CT21: independent F_101 rational Richelot-neighbor screening.

Run: python3 -S ct21_richelot_neighbors.py
Self-check only; [pcc:unverified]. No production Cantor / isogeny imports.
The codomain formulas follow the standard determinant/bracket construction.
"""
from itertools import combinations

P = 101
# (name, monic sextic coefficients a4,a2,a0, irreducible cubic factor,
#  expected nonzero determinant pair, frozen target polynomial with positive x)
CASES = (
    ('original', (6, 13, 56), (67, 63, 25), (38, 63),
     (64, 83, 42, 84, 13, 7, 77)),
    ('CT20 B+A', (92, 44, 33), (88, 39, 17), (28, 73),
     (7, 15, 20, 20, 95, 24, 82)),
    ('CT20 B+B', (26, 42, 68), (29, 70, 35), (33, 68),
     (1, 63, 81, 46, 18, 69, 84)),
)


class Cubic:
    modulus = (67, 63, 25)

    def __init__(self, value=0):
        if isinstance(value, Cubic):
            self.a = value.a
        elif isinstance(value, int):
            self.a = (value % P, 0, 0)
        else:
            assert len(value) == 3
            self.a = tuple(int(x) % P for x in value)

    def __eq__(self, other):
        return self.a == Cubic(other).a

    def __add__(self, other):
        return Cubic([x + y for x, y in zip(self.a, Cubic(other).a)])

    __radd__ = __add__

    def __neg__(self):
        return Cubic([-x for x in self.a])

    def __sub__(self, other):
        return self + -Cubic(other)

    def __rsub__(self, other):
        return Cubic(other) - self

    def __mul__(self, other):
        other = Cubic(other)
        acc = [0] * 5
        for i in range(3):
            for j in range(3):
                acc[i+j] += self.a[i] * other.a[j]
        for i in (4, 3):
            for j, c in enumerate(self.modulus):
                acc[i-3+j] -= c * acc[i]
        return Cubic(acc[:3])

    __rmul__ = __mul__

    def __pow__(self, n):
        assert n >= 0
        out, base = Cubic(1), self
        while n:
            if n & 1:
                out = out * base
            base = base * base
            n >>= 1
        return out

    def __truediv__(self, other):
        other = Cubic(other)
        assert other != 0
        return self * (other ** (P**3 - 2))

    def fp(self):
        assert self.a[1:] == (0, 0), ('coefficient did not descend', self.a)
        return self.a[0]


def pol_add(a, b):
    z = [Cubic(0)] * max(len(a), len(b))
    for i, c in enumerate(a):
        z[i] += c
    for i, c in enumerate(b):
        z[i] += c
    return z


def pol_mul(a, b):
    z = [Cubic(0)] * (len(a) + len(b) - 1)
    for i, c in enumerate(a):
        for j, d in enumerate(b):
            z[i+j] += c * d
    return z


def pol_deriv(a):
    return [i*a[i] for i in range(1, len(a))]


def bracket(a, b):
    return pol_add(pol_mul(pol_deriv(a), b),
                   [-v for v in pol_mul(a, pol_deriv(b))])


def determinant(q):
    a, b, c = q
    return (a[0]*(b[1]*c[2]-b[2]*c[1])
            -a[1]*(b[0]*c[2]-b[2]*c[0])
            +a[2]*(b[0]*c[1]-b[1]*c[0]))


def all_matchings(items):
    if not items:
        yield ()
        return
    a, *rest = items
    for b in rest:
        for tail in all_matchings([v for v in rest if v != b]):
            yield ((a, b),) + tail


def rtrim(f):
    f = f[:]
    while f and f[-1] % P == 0:
        f.pop()
    return f


def polrem(a, b):
    a, b = rtrim(a), rtrim(b)
    assert b
    inv = pow(b[-1], -1, P)
    while len(a) >= len(b):
        d, v = len(a)-len(b), a[-1]*inv % P
        for i, c in enumerate(b):
            a[d+i] = (a[d+i]-v*c) % P
        a = rtrim(a)
    return a


def polgcd(a, b):
    while b:
        a, b = b, polrem(a, b)
    if not a:
        return []
    inv = pow(a[-1], -1, P)
    return [v*inv % P for v in a]


def polpowmod(base, exp, modulus):
    v = [1]
    while exp:
        if exp & 1:
            v = polrem(polmul_mod(v, base), modulus)
        base = polrem(polmul_mod(base, base), modulus)
        exp >>= 1
    return v


def polmul_mod(a, b):
    out = [0]*(len(a)+len(b)-1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i+j] = (out[i+j]+x*y) % P
    return out


def is_irreducible_sextic(f):
    """Rabin criterion: gcd(X^(q^2)-X,f)=gcd(X^(q^3)-X,f)=1,
    and X^(q^6)=X mod f. Prime divisors of six are 2,3.
    """
    assert len(f) == 7 and f[-1] != 0
    inv = pow(f[-1], -1, P)
    modulus = [(v*inv) % P for v in f]
    x = [0, 1]
    for k in (2, 3):
        y = polpowmod(x, P**k, modulus)
        z = y[:]
        z[1] = (z[1]-1) % P
        if polgcd(modulus, rtrim(z)) != [1]:
            return False
    return polpowmod(x, P**6, modulus) == x


def point_counts(f):
    sq1 = {y*y % P for y in range(P)}
    sq2 = {((a*a+2*b*b) % P, 2*a*b % P)
           for a in range(P) for b in range(P)}
    assert len(sq2) == (P*P+1)//2
    n1 = 2 if f[-1] in sq1 else 0
    for x in range(P):
        y = 0
        for c in reversed(f):
            y = (y*x+c) % P
        n1 += 1 if y == 0 else (2 if y in sq1 else 0)
    n2 = 2  # Every nonzero Fp leading coefficient is a square in Fp².
    for x in range(P):
        for z in range(P):
            a, b = 0, 0
            for c in reversed(f):
                a, b = ((a*x+2*b*z+c) % P, (a*z+b*x) % P)
            n2 += 1 if (a, b) == (0, 0) else (2 if (a, b) in sq2 else 0)
    s1, s2 = P+1-n1, P*P+1-n2
    a1, a2 = -s1, (s1*s1-s2)//2
    return n1, n2, a1, a2, 1+a1+a2+P*a1+P*P


def run():
    for name, triple, cubic_mod, determinant_expected, target_expected in CASES:
        Cubic.modulus = cubic_mod
        r = Cubic((0, 1, 0))
        roots = [r**(P**k) for k in range(3)]
        assert len({v.a for v in roots}) == 3
        branches = roots + [-v for v in roots]
        assert len({v.a for v in branches}) == 6
        source = [Cubic(1)]
        for z in branches:
            source = pol_mul(source, [-z, Cubic(1)])
        assert [v.fp() for v in source] == [triple[2],0,triple[1],0,triple[0],0,1]
        assert not is_irreducible_sextic([v.fp() for v in source])
        frob = {0:1,1:2,2:0,3:4,4:5,5:3}
        stable = [m for m in all_matchings(list(range(6)))
                  if frozenset(tuple(sorted((frob[i],frob[j]))) for i,j in m)
                  == frozenset(m)]
        assert len(stable) == 3
        deltas, codomains = [], []
        for matching in stable:
            q = [pol_mul([-branches[i],Cubic(1)],[-branches[j],Cubic(1)])
                 for i,j in matching]
            delta = determinant(q)
            deltas.append(delta.fp())
            if delta == 0:
                continue
            h = [bracket(q[(i+1)%3], q[(i+2)%3]) for i in range(3)]
            assert all(len(t) == 4 and t[3] == 0 for t in h)
            h = [t[:3] for t in h]
            dest = pol_mul(pol_mul(h[0],h[1]),h[2])
            f = [int(v.fp()) for v in (c/delta for c in dest)]
            codomains.append(tuple(f))
            assert is_irreducible_sextic(f)
            assert point_counts(f) == (120,10436,18,279,12317)
            # Deliberately wrong twist: it must NOT pass the same local zeta gate.
            assert point_counts([2*c % P for c in f]) != (120,10436,18,279,12317)
        assert tuple(deltas) == (0,) + determinant_expected
        assert len(codomains) == 2
        assert codomains[0] == target_expected
        assert all(codomains[1][i] == ((-1)**i*codomains[0][i]) % P
                   for i in range(7))
        print(name, 'pairings=3; det=', tuple(deltas),
              'target=',codomains[0], 'mirror=x->-x',
              'irreducible=both; #J=12317; next rational pairing=one')
    print('[pcc:unverified] developmental arithmetic checks, no Jacobian map')


if __name__ == '__main__':
    run()
