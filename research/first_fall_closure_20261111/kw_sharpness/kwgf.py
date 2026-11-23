"""GF(2^n) arithmetic (n <= 21) with log/exp tables; primitive polynomials verified at build time."""
import numpy as np

PRIM = {3:0xB,4:0x13,5:0x25,6:0x43,7:0x89,8:0x11D,9:0x211,10:0x409,11:0x805,12:0x1053,
        13:0x201B,14:0x4443,15:0x8003,16:0x1100B,17:0x20009,18:0x40081,19:0x80027,
        20:0x100009,21:0x200005}

class GF:
    def __init__(self, n):
        self.n = n; self.q = 1 << n; self.poly = PRIM[n]
        q1 = self.q - 1
        exp = np.zeros(2 * q1 + 2, dtype=np.int64)
        log = np.zeros(self.q, dtype=np.int64)
        x = 1
        for i in range(q1):
            exp[i] = x; log[x] = i
            x <<= 1
            if x >> n: x ^= self.poly
        assert x == 1, "polynomial not primitive for n=%d" % n
        assert len(set(exp[:q1].tolist())) == q1
        exp[q1:2*q1] = exp[:q1]
        self.exp = exp; self.log = log; self.q1 = q1
    def mul(self, a, b):
        if a == 0 or b == 0: return 0
        return int(self.exp[self.log[a] + self.log[b]])
    def inv(self, a):
        assert a; return int(self.exp[(self.q1 - self.log[a]) % self.q1])
    def pow(self, a, e):
        if a == 0: return 0 if e else 1
        return int(self.exp[(self.log[a] * e) % self.q1])
    def sq(self, a): return self.mul(a, a)
    def vmul(self, x, y):
        """vectorised product of integer arrays"""
        r = self.exp[(self.log[x] + self.log[y]) % self.q1]
        return np.where((x == 0) | (y == 0), 0, r)
    def trace(self, a):
        t = 0; x = a
        for _ in range(self.n):
            t ^= x; x = self.sq(x)
        assert t in (0, 1)
        return t
