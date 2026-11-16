#!/usr/bin/env python3
"""Is pi scalar on E[11] for the k = 20 miniature? Decides whether 11 divides the conductor of End(E).

pi acts on E[11] with characteristic polynomial (X - 3)^2 mod 11. It is the scalar 3 iff every
11-torsion abscissa has Frobenius orbit length 5, i.e. iff gcd(psi_11, X^(p^5) - X) has degree 60;
in the Jordan case the gcd is the single eigenline factor, degree 5. Writes data/conductor11.json.
"""
import json
from pathlib import Path

p, A = 1048573, 926
OUT = Path(__file__).resolve().parent.parent / "data" / "conductor11.json"


def trim(f):
    while f and f[-1] == 0:
        f.pop()
    return f


def padd(f, g):
    n = max(len(f), len(g))
    return trim([((f[i] if i < len(f) else 0) + (g[i] if i < len(g) else 0)) % p for i in range(n)])


def pneg(f):
    return [(-c) % p for c in f]


def pmul(f, g):
    if not f or not g:
        return []
    h = [0] * (len(f) + len(g) - 1)
    for i, a in enumerate(f):
        if a:
            for j, b in enumerate(g):
                h[i + j] = (h[i + j] + a * b) % p
    return trim(h)


def pmod(f, g):
    f = f[:]
    ig = pow(g[-1], p - 2, p)
    while len(f) >= len(g):
        c = f[-1] * ig % p
        s = len(f) - len(g)
        for i, b in enumerate(g):
            f[s + i] = (f[s + i] - c * b) % p
        trim(f)
    return f


def pgcd(f, g):
    while g:
        f, g = g, pmod(f, g)
    return [c * pow(f[-1], p - 2, p) % p for c in f]


def ppowmod(f, e, m):
    r, b = [1], pmod(f, m)
    while e:
        if e & 1:
            r = pmod(pmul(r, b), m)
        b = pmod(pmul(b, b), m)
        e >>= 1
    return r


i3 = pow(3, p - 2, p)
a = (1 - A * A * i3) % p
b = (2 * A ** 3 - 9 * A) * pow(27, p - 2, p) % p
F = [4 * b % p, 4 * a % p, 0, 4]
F2 = pmul(F, F)
f = {0: [], 1: [1], 2: [1]}
f[3] = trim([(-a * a) % p, 12 * b % p, 6 * a % p, 0, 3])
f[4] = trim([2 * c % p for c in [(-8 * b * b - a ** 3) % p, (-4 * a * b) % p, (-5 * a * a) % p, 20 * b % p, 5 * a % p, 0, 1]])


def cube(g):
    return pmul(pmul(g, g), g)


def odd(m):  # f_{2m+1}
    if m % 2 == 0:
        return padd(pmul(F2, pmul(f[m + 2], cube(f[m]))), pneg(pmul(f[m - 1], cube(f[m + 1]))))
    return padd(pmul(f[m + 2], cube(f[m])), pneg(pmul(F2, pmul(f[m - 1], cube(f[m + 1])))))


def even(m):  # f_{2m}
    return pmul(f[m], padd(pmul(f[m + 2], pmul(f[m - 1], f[m - 1])), pneg(pmul(f[m - 2], pmul(f[m + 1], f[m + 1])))))


f[5] = odd(2)
f[6] = even(3)
f[7] = odd(3)
f[11] = odd(5)
psi = f[11]
deg = len(psi) - 1
# sanity: psi_3 roots are 3-torsion abscissae; check psi_11 degree (121 - 1)/2 = 60
xp = [0, 1]
for _ in range(5):
    xp = ppowmod(xp, p, psi)
g = pgcd(psi, padd(xp, [0, p - 1]))
res = {"deg_psi11": deg, "deg_gcd(psi11, X^(p^5)-X)": len(g) - 1,
       "pi_scalar_on_E[11]": len(g) - 1 == deg,
       "reading": "60 means 11 does not divide the conductor of End(E) (End contains (pi+525)/11); 5 means it does"}
OUT.write_text(json.dumps(res, indent=1))
print(res)
