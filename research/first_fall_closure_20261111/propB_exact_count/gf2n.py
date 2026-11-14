"""Minimal GF(2^n) over F2[z]/(m), elements = Python ints (bit i = coeff of z^i)."""
IRRED = {  # explicit irreducible polynomials used (verified by irreducible() at import of the check script)
    4: 0b10011,            # z^4+z+1
    5: 0b100101,           # z^5+z^2+1
    6: 0b1000011,          # z^6+z+1
    7: 0b10000011,         # z^7+z+1
    8: 0b100011011,        # z^8+z^4+z^3+z+1
    9: 0b1000010001,       # z^9+z^4+1
    10: 0b10000001001,     # z^10+z^3+1
    11: 0b100000000101,    # z^11+z^2+1
    12: 0b1000001010011,   # z^12+z^6+z^4+z+1
    16: 0b10000000000101011,  # z^16+z^5+z^3+z+1 (used for the E1 cross-check)
}
def poly_str(m):
    return " + ".join(("1" if i == 0 else "z" if i == 1 else "z^%d" % i) for i in range(m.bit_length()-1, -1, -1) if m >> i & 1)

class GF:
    def __init__(self, n):
        self.n = n; self.m = IRRED[n]; self.size = 1 << n
        assert irreducible(self.m, n), "modulus not irreducible"
    def mul(self, a, b):
        r = 0; m = self.m; n = self.n
        while b:
            if b & 1: r ^= a
            b >>= 1; a <<= 1
            if a >> n & 1: a ^= m
        return r
    def sq(self, a): return self.mul(a, a)
    def pow(self, a, e):
        r = 1
        while e:
            if e & 1: r = self.mul(r, a)
            a = self.sq(a); e >>= 1
        return r
    def inv(self, a):
        assert a; return self.pow(a, self.size - 2)
    def frob(self, a, k):
        for _ in range(k): a = self.sq(a)
        return a
    def tr(self, a):
        t = 0; x = a
        for _ in range(self.n): t ^= x; x = self.sq(x)
        assert t in (0, 1); return t

def pgcd(a, b):
    while b:
        while a and a.bit_length() >= b.bit_length(): a ^= b << (a.bit_length() - b.bit_length())
        a, b = b, a
    return a

def irreducible(m, n):
    """Rabin test: z^(2^n)=z mod m and gcd(z^(2^(n/p))-z, m)=1 for each prime p | n."""
    def mm(a, b):
        r = 0
        while b:
            if b & 1: r ^= a
            b >>= 1; a <<= 1
            if a >> n & 1: a ^= m
        return r
    def fr(k):
        y = 2
        for _ in range(k): y = mm(y, y)
        return y
    if fr(n) != 2: return False
    ps = [p for p in range(2, n + 1) if n % p == 0 and all(p % q for q in range(2, p))]
    return all(pgcd(m, fr(n // p) ^ 2) == 1 for p in ps)
