"""VALIDATOR EC arithmetic (shares nothing with the harness): short Weierstrass y^2 = x^3 + a x + b over F_p,
Jacobian coordinates, left-to-right double-and-add; deterministic Miller-Rabin for n < 3.3e24."""
import json
def is_prime(n):
    if n < 2: return False
    sm = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41)
    for q in sm:
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in sm:
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True
class Curve:
    def __init__(self, p, a, b): self.p, self.a, self.b = p, a % p, b % p
    def on(self, P):
        if P is None: return True
        x, y = P; p = self.p
        return (y * y - (x * x * x + self.a * x + self.b)) % p == 0
    def _dbl(self, X, Y, Z):
        p = self.p
        if Z == 0 or Y == 0: return (1, 1, 0)
        YY = Y * Y % p; S = 4 * X * YY % p; ZZ = Z * Z % p
        M = (3 * X * X + self.a * ZZ * ZZ) % p
        X3 = (M * M - 2 * S) % p; Y3 = (M * (S - X3) - 8 * YY * YY) % p; Z3 = 2 * Y * Z % p
        return (X3, Y3, Z3)
    def _addmixed(self, X1, Y1, Z1, x2, y2):
        p = self.p
        if Z1 == 0: return (x2, y2, 1)
        Z1Z1 = Z1 * Z1 % p; U2 = x2 * Z1Z1 % p; S2 = y2 * Z1 * Z1Z1 % p
        H = (U2 - X1) % p; r = (S2 - Y1) % p
        if H == 0:
            return self._dbl(X1, Y1, Z1) if r == 0 else (1, 1, 0)
        HH = H * H % p; HHH = H * HH % p; V = X1 * HH % p
        X3 = (r * r - HHH - 2 * V) % p; Y3 = (r * (V - X3) - Y1 * HHH) % p; Z3 = Z1 * H % p
        return (X3, Y3, Z3)
    def mul(self, k, P):
        if P is None or k == 0: return None
        x, y = P; R = (1, 1, 0)
        for bit in bin(k)[2:]:
            R = self._dbl(*R)
            if bit == "1": R = self._addmixed(*R, x, y)
        X, Y, Z = R
        if Z == 0: return None
        zi = pow(Z, -1, self.p); zi2 = zi * zi % self.p
        return (X * zi2 % self.p, Y * zi2 * zi % self.p)
def load(path):
    d = json.load(open(path)); p = int(d["field"]["modulus"])
    C = Curve(p, int(d["a"]), int(d["b"]))
    G = (int(d["generator"]["x"]), int(d["generator"]["y"]))
    return d, C, G, int(d["subgroup_order"])
