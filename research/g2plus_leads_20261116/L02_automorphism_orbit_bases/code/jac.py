"""Cantor arithmetic on the Jacobian of y^2 = f(x) (SPEC section 4)."""
import ff
from poly import *

IDENT = ([1], [])
OPS = [0]   # counted group additions (only while ff.CTR.enabled)

def is_ident(D):
    return len(D[0]) == 1

def neg(D):
    return (D[0], pneg(D[1]))

def add(D1, D2, C):
    u1, v1 = D1; u2, v2 = D2
    if len(u1) == 1:
        return D2
    if len(u2) == 1:
        return D1
    if ff.CTR.enabled:
        OPS[0] += 1
    f = C.f; g = C.g
    d1, e1, e2 = xgcd(u1, u2)
    d, c1, c2 = xgcd(d1, padd(v1, v2))
    s1 = pmul(c1, e1); s2 = pmul(c1, e2); s3 = c2
    u = pdivmod(pmul(u1, u2), pmul(d, d))[0]
    num = padd(padd(pmul(pmul(s1, u1), v2), pmul(pmul(s2, u2), v1)),
               pmul(s3, padd(pmul(v1, v2), f)))
    v = pdivmod(pdivmod(num, d)[0], u)[1]
    while len(u) - 1 > g:
        u = pdivmod(psub(f, pmul(v, v)), u)[0]
        v = pdivmod(pneg(v), u)[1]
    if u[-1] != 1:
        u = pmonic(u)
    return (u, v)

def dbl(D, C):
    return add(D, D, C)

def smul(k, D, C):
    if k < 0:
        return smul(-k, neg(D), C)
    if k == 0 or is_ident(D):
        return IDENT
    R = D
    for bit in bin(k)[3:]:
        R = add(R, R, C)
        if bit == '1':
            R = add(R, D, C)
    return R

def smul_rl(k, D, C):
    if k < 0:
        return smul_rl(-k, neg(D), C)
    R = IDENT; Q = D
    while k:
        if k & 1:
            R = add(R, Q, C)
        k >>= 1
        if k:
            Q = add(Q, Q, C)
    return R

def key(D, g, p):
    u, v = D
    d = len(u) - 1
    uu = list(u[:d]) + [0] * (g - d)
    vv = list(v) + [0] * (g - len(v))
    return [d] + uu + vv

def K(D, g, p):
    r = 0; m = 1
    for c in key(D, g, p):
        r += c * m
        m *= p
    return r

def point_divisor(x, y):
    return ([(-x) % ff.p, 1], [y % ff.p] if y % ff.p else [])

def random_point(C, rnd):
    p = C.p
    while True:
        x = rnd.randrange(p)
        fx = peval(C.f, x)
        if fx != 0 and ff.is_square(fx):
            break
    y = ff.sqrt_mod(fx)
    if rnd.randrange(2):
        y = p - y
    return x, y

def random_divisor(C, rnd):
    D = IDENT
    for _ in range(C.g):
        x, y = random_point(C, rnd)
        D = add(D, point_divisor(x, y), C)
    return D
