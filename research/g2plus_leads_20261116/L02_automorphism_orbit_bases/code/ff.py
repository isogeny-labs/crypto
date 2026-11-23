"""Counted finite-field arithmetic (SPEC section 3). Current prime is module global p."""
import contextlib

class Ctr:
    def __init__(self):
        self.reset()
        self.enabled = True
    def reset(self):
        self.mul = 0; self.inv = 0; self.rmul = 0; self.rinv = 0
    def snapshot(self):
        return {"mul": self.mul, "inv": self.inv, "rmul": self.rmul, "rinv": self.rinv}

CTR = Ctr()
p = 2
_NR = {}

def set_p(q):
    global p
    p = q

@contextlib.contextmanager
def uncounted():
    old = CTR.enabled
    CTR.enabled = False
    try:
        yield
    finally:
        CTR.enabled = old

def cnt_mul(n):
    if CTR.enabled:
        CTR.mul += n

def fmul(a, b):
    if CTR.enabled:
        CTR.mul += 1
    return a * b % p

def finv(a):
    assert a % p != 0
    if CTR.enabled:
        CTR.inv += 1
    return pow(a, p - 2, p)

def fadd(a, b): return (a + b) % p
def fsub(a, b): return (a - b) % p
def fneg(a): return (-a) % p

def fpow(a, e):
    if e == 0:
        return 1
    res = a
    for bit in bin(e)[3:]:
        res = fmul(res, res)
        if bit == '1':
            res = fmul(res, a)
    return res

def is_square(a):
    a %= p
    if a == 0:
        return False
    return fpow(a, (p - 1) // 2) == 1

def nonresidue():
    if p not in _NR:
        z = 2
        while pow(z, (p - 1) // 2, p) != p - 1:
            z += 1
        _NR[p] = z
    return _NR[p]

def sqrt_mod(a):
    a %= p
    if a == 0:
        return 0
    if p % 4 == 3:
        return fpow(a, (p + 1) // 4)
    q = p - 1; s = 0
    while q % 2 == 0:
        q //= 2; s += 1
    z = nonresidue()
    c = fpow(z, q); x = fpow(a, (q + 1) // 2); t = fpow(a, q); m = s
    while t != 1:
        i = 0; tt = t
        while tt != 1:
            tt = fmul(tt, tt); i += 1
        b = c
        for _ in range(m - i - 1):
            b = fmul(b, b)
        x = fmul(x, b); c = fmul(b, b); t = fmul(t, c); m = i
    return x

def rmul(a, b, r):
    if CTR.enabled:
        CTR.rmul += 1
    return a * b % r

def rinv(a, r):
    if CTR.enabled:
        CTR.rinv += 1
    return pow(a, r - 2, r)
