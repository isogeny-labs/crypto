from .poly import psub, pmul, trim


def sqrt_part(f, g, p, F_):
    inv2 = (p + 1) // 2
    s = [0] * (g + 2)
    s[g + 1] = 1
    for j in range(1, g + 2):
        acc = f[2 * g + 2 - j]
        for i in range(1, j):
            acc = F_.sub(acc, F_.mul(s[g + 1 - i], s[g + 1 - j + i]))
        s[g + 1 - j] = F_.mul(acc, inv2)
    assert len(trim(psub(f, pmul(s, s, p), p))) - 1 <= g
    return s


def top_part(s, e, g):
    t = [0] * (g + 2)
    for k in range(g + 1 - e, g + 2):
        t[k] = s[k]
    return t


def odd_transform(f, alpha, g, p):
    cur = list(f)
    c = []
    for i in range(2 * g + 3):
        # synthetic division by (x - alpha), highest first
        n = len(cur)
        q = [0] * (n - 1)
        carry = 0
        for k in range(n - 1, -1, -1):
            carry = (cur[k] + carry * alpha) % p if k == n - 1 else (cur[k] + carry * alpha) % p
            if k > 0:
                q[k - 1] = carry
            else:
                r = carry
        c.append(r)
        cur = q
    Fodd = [0] * (2 * g + 2)
    for i in range(1, 2 * g + 3):
        Fodd[2 * g + 2 - i] = c[i] * pow(c[1], (i - 2) % (p - 1), p) % p
    assert Fodd[-1] == 1 and len(Fodd) == 2 * g + 2
    cg = pow(c[1], g, p)
    return c, Fodd, cg


class OddMap:
    def __init__(self, f, alpha, g, p):
        self.p, self.g, self.alpha = p, g, alpha
        self.c, self.Fodd, self.cg = odd_transform(f, alpha, g, p)

    def map_point(self, P):
        p, g = self.p, self.g
        if P == "inf+":
            return (0, self.cg)
        if P == "inf-":
            return (0, (p - self.cg) % p)
        x, y = P
        if x == self.alpha:
            return None
        X = pow(x - self.alpha, p - 2, p)
        Xo, Yo = self.c[1] * X % p, self.cg * y * pow(X, g + 1, p) % p
        from .poly import ev
        assert Yo * Yo % p == ev(self.Fodd, Xo, p)
        return (Xo, Yo)
