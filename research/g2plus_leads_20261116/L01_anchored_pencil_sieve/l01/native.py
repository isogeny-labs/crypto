def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def horner_nat(c, x, n, p):
    m = len(c)
    acc = c[m - 1]
    for i in range(m - 2, -1, -1):
        acc = (acc * x + c[i]) % p
        n.mul += 1
        n.add += 1
    return acc


def disc_character(p, u, chi, n):
    d = len(u) - 1
    a = list(u)
    b = [u[i] * i % p for i in range(1, d + 1)]
    n.mul += d
    trim(b)
    sign = chi[p - 1] if (d * (d - 1) // 2) % 2 == 1 else 1
    while True:
        if not b:
            return 0
        m = len(a) - 1
        nn = len(b) - 1
        if nn == 0:
            return sign * chi[b[0]] if m % 2 == 1 else sign
        lb = b[nn]
        delta = m - nn + 1
        r = list(a)
        for step in range(delta):
            top = m - step
            c = r[top]
            for i in range(top):
                r[i] = r[i] * lb % p
            for j in range(nn):
                r[top - nn + j] = (r[top - nn + j] - c * b[j]) % p
            n.mul += top + nn
            n.add += nn
            del r[top:]
        trim(r)
        if not r:
            return 0
        dr = len(r) - 1
        if (m + dr + delta * nn) % 2 == 1:
            sign *= chi[lb]
        if (m * nn) % 2 == 1:
            sign *= chi[p - 1]
        a, b = b, r


class ModU:
    def __init__(self, p, u, n):
        self.p, self.u, self.n = p, u, n
        self.d = len(u) - 1
        self.neg = [(p - x) % p for x in u]

    def mul(self, a, b):
        p, d, n, neg = self.p, self.d, self.n, self.neg
        t = [0] * (2 * d)
        for i in range(d):
            ai = a[i]
            for j in range(d):
                t[i + j] += ai * b[j]
        t = [x % p for x in t]
        n.mul += d * d
        n.add += d * d
        for k in range(2 * d - 2, d - 1, -1):
            c = t[k]
            for j in range(d):
                t[k - d + j] = (t[k - d + j] + neg[j] * c) % p
            n.mul += d
            n.add += d
        return t[:d]

    def mul_xpa(self, a, aa):
        p, d, n, neg = self.p, self.d, self.n, self.neg
        top = a[d - 1]
        out = [0] + a[:d - 1]
        for j in range(d):
            out[j] = (out[j] + neg[j] * top + aa * a[j]) % p
        w = d if aa == 0 else 2 * d
        n.mul += w
        n.add += w
        return out

    def base(self, aa):
        if self.d == 1:
            return [(aa + self.p - self.u[0]) % self.p]
        r = [0] * self.d
        r[0] = aa
        r[1] = 1
        return r

    def pow_xpa(self, aa, e):
        res = self.base(aa)
        for i in range(e.bit_length() - 2, -1, -1):
            res = self.mul(res, res)
            if (e >> i) & 1:
                res = self.mul_xpa(res, aa)
        return res


def splits_completely(p, u, n):
    m = ModU(p, u, n)
    return m.pow_xpa(0, p) == m.base(0)


def cinv(p, a, n):
    n.inv += 1
    return pow(a, p - 2, p)


def vrem(p, a, m, n):
    r = trim(list(a))
    dm = len(m) - 1
    lead = m[dm]
    li = 1 if lead == 1 else cinv(p, lead, n)
    while len(r) > dm:
        top = len(r) - 1
        c = r[top]
        if li != 1:
            c = c * li % p
            n.mul += 1
        for j in range(dm):
            r[top - dm + j] = (r[top - dm + j] - c * m[j]) % p
        n.mul += dm
        n.add += dm
        r.pop()
        trim(r)
    return r


def vgcd_monic(p, a, b, n):
    a, b = trim(list(a)), trim(list(b))
    while b:
        r = vrem(p, a, b, n)
        a, b = b, r
    if a and a[-1] != 1:
        il = cinv(p, a[-1], n)
        a = [x * il % p for x in a]
        n.mul += len(a)
    return a


def vdiv_monic(p, a, m, n):
    dm = len(m) - 1
    r = list(a)
    q = [0] * (len(a) - dm)
    for top in range(len(a) - 1, dm - 1, -1):
        c = r[top]
        q[top - dm] = c
        for j in range(dm):
            r[top - dm + j] = (r[top - dm + j] - c * m[j]) % p
        n.mul += dm
        n.add += dm
    return q


def roots_of(p, u, rng, n):
    d = len(u) - 1
    if d == 0:
        return []
    if d == 1:
        return [(p - u[0]) % p]
    for _ in range(64):
        aa = rng.below(p)
        h = ModU(p, u, n).pow_xpa(aa, (p - 1) // 2)
        hm1 = list(h)
        hm1[0] = (hm1[0] - 1) % p
        trim(hm1)
        if not hm1:
            continue
        gg = vgcd_monic(p, u, hm1, n)
        dg = len(gg) - 1
        if dg == 0 or dg == d:
            continue
        q = vdiv_monic(p, u, gg, n)
        r1 = roots_of(p, gg, rng, n)
        if r1 is None:
            return None
        r2 = roots_of(p, q, rng, n)
        if r2 is None:
            return None
        return r1 + r2
    return None
