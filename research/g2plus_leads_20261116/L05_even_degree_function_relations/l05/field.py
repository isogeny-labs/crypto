class GateFailure(Exception):
    def __init__(self, msg, relation=None, stage=None):
        super().__init__(msg)
        self.msg = msg
        self.relation = relation
        self.stage = stage


class Counter:
    def __init__(self):
        self.mul = 0
        self.inv = 0
        self.add = 0
        self.fd_add = 0
        self.bucket_ops = 0
        self.bucket_scan = 0

    FIELDS = ("mul", "inv", "add", "fd_add", "bucket_ops", "bucket_scan")

    def snap(self):
        return tuple(getattr(self, k) for k in self.FIELDS)

    def delta(self, snap):
        return {k: getattr(self, k) - s for k, s in zip(self.FIELDS, snap)}

    def fpm(self, I=10):
        return self.mul + I * self.inv


class F:
    def __init__(self, p, ctr):
        self.p = p
        self.ctr = ctr

    def mul(self, a, b):
        self.ctr.mul += 1
        return a * b % self.p

    def inv(self, a):
        assert a % self.p != 0
        self.ctr.inv += 1
        return pow(a, self.p - 2, self.p)

    def add(self, a, b):
        self.ctr.add += 1
        return (a + b) % self.p

    def sub(self, a, b):
        self.ctr.add += 1
        return (a - b) % self.p

    def neg(self, a):
        self.ctr.add += 1
        return (-a) % self.p


def horner_F(F_, c, x):
    m = len(c)
    acc = c[m - 1]
    for i in range(m - 2, -1, -1):
        acc = F_.add(F_.mul(acc, x), c[i])
    return acc


def batch_inv(F_, vals):
    k = len(vals)
    pre = [0] * k
    pre[0] = vals[0]
    for i in range(1, k):
        pre[i] = F_.mul(pre[i - 1], vals[i])
    t = F_.inv(pre[k - 1])
    out = [0] * k
    for i in range(k - 1, 0, -1):
        out[i] = F_.mul(t, pre[i - 1])
        t = F_.mul(t, vals[i])
    out[0] = t
    return out


def inv_table(p, F_):
    inv = [0] * p
    inv[1] = 1
    for i in range(2, p):
        inv[i] = F_.mul((p - p // i) % p, inv[p % i])
    return inv
