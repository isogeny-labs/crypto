"""VALIDATOR elliptic-curve arithmetic for the mt_timed check (shares no code with the harness or the producer):
short Weierstrass y^2 = x^3 + a x + b over F_p in affine coordinates with Python's modular inverse, right-to-left
double-and-add; deterministic Miller-Rabin (bases up to 41, valid for n < 3.3e24)."""
import json


def is_prime(n):
    if n < 2:
        return False
    bases = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41)
    for q in bases:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in bases:
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


class Curve:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a % p, b % p

    def on(self, P):
        if P is None:
            return True
        x, y = P
        return (y * y - x * x * x - self.a * x - self.b) % self.p == 0

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        p = self.p
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            lam = (3 * x1 * x1 + self.a) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, k, P):
        R, B = None, P
        while k > 0:
            if k & 1:
                R = self.add(R, B)
            B = self.add(B, B)
            k >>= 1
        return R


def load(path):
    d = json.load(open(path))
    num = lambda v: int(str(v).strip('"'))
    p = num(d["field"]["modulus"])
    C = Curve(p, num(d["a"]), num(d["b"]))
    G = (num(d["generator"]["x"]), num(d["generator"]["y"]))
    return d, C, G, num(d["subgroup_order"])
