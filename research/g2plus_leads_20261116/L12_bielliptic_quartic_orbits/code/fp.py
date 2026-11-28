"""Counted F_p arithmetic (SPEC.md section 2)."""


class Ops:
    def __init__(self):
        self.mul = 0
        self.inv = 0
        self.add = 0
        self.lookup = 0

    def fpm(self):
        return self.mul + 10 * self.inv

    def as_dict(self):
        return {"mul": self.mul, "inv": self.inv, "add": self.add,
                "lookup": self.lookup, "fpm": self.fpm()}


class Fp:
    def __init__(self, p, ops):
        self.p = p
        self.ops = ops
        self.SQRT = None

    def mul(self, a, b):
        self.ops.mul += 1
        return a * b % self.p

    def inv(self, a):
        self.ops.inv += 1
        assert a != 0
        return pow(a, self.p - 2, self.p)

    def add(self, a, b):
        self.ops.add += 1
        return (a + b) % self.p

    def sub(self, a, b):
        self.ops.add += 1
        return (a - b) % self.p

    def neg(self, a):
        self.ops.add += 1
        return (-a) % self.p

    def dbl(self, a):
        return self.add(a, a)

    def times4(self, a):
        return self.dbl(self.dbl(a))

    def times3(self, a):
        return self.add(self.dbl(a), a)

    def build_sqrt(self):
        p = self.p
        T = [-1] * p
        T[0] = 0
        for v in range(1, (p - 1) // 2 + 1):
            T[self.mul(v, v)] = v
        self.SQRT = T

    def sqrt(self, a):
        self.ops.lookup += 1
        return self.SQRT[a]


def pmul(F, A, B):
    C = [0] * (len(A) + len(B) - 1)
    for i in range(len(A)):
        for j in range(len(B)):
            C[i + j] = F.add(C[i + j], F.mul(A[i], B[j]))
    return C


def trim(A):
    A = list(A)
    while len(A) > 1 and A[-1] == 0:
        A.pop()
    return A


def divroot(F, S, x0):
    S = trim(S)
    d = len(S) - 1
    assert d >= 1
    q = [0] * d
    q[d - 1] = S[d]
    for k in range(d - 1, 0, -1):
        q[k - 1] = F.add(S[k], F.mul(x0, q[k]))
    rem = F.add(S[0], F.mul(x0, q[0]))
    if rem != 0:
        raise RuntimeError("divroot: nonzero remainder (instrument error)")
    return q, rem


# uncounted helpers
def u_inv(a, p):
    return pow(a, p - 2, p)


def u_pmul(A, B, p):
    C = [0] * (len(A) + len(B) - 1)
    for i, a in enumerate(A):
        for j, b in enumerate(B):
            C[i + j] = (C[i + j] + a * b) % p
    return C
