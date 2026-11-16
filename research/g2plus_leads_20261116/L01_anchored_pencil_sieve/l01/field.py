class Counter:
    __slots__ = ("mul", "inv", "add", "fd_add", "bucket_ops", "bucket_scan", "lookups")

    def __init__(self):
        self.mul = 0
        self.inv = 0
        self.add = 0
        self.fd_add = 0
        self.bucket_ops = 0
        self.bucket_scan = 0
        self.lookups = 0

    def fpm(self, I=10):
        return self.mul + I * self.inv

    def snap(self):
        return tuple(getattr(self, k) for k in self.__slots__)

    def delta(self, s):
        o = Counter()
        for k, v in zip(self.__slots__, s):
            setattr(o, k, getattr(self, k) - v)
        return o


class F:
    def __init__(self, p, ctr):
        self.p = p
        self.ctr = ctr

    def mul(self, a, b):
        self.ctr.mul += 1
        return a * b % self.p

    def sqr(self, a):
        self.ctr.mul += 1
        return a * a % self.p

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

    def horner(self, c, x):
        """4.3: exactly len(c)-1 multiplications."""
        m = len(c)
        acc = c[m - 1]
        for i in range(m - 2, -1, -1):
            acc = self.add(self.mul(acc, x), c[i])
        return acc

    def batch_inv(self, vals):
        k = len(vals)
        pre = [0] * k
        pre[0] = vals[0]
        for i in range(1, k):
            pre[i] = self.mul(pre[i - 1], vals[i])
        t = self.inv(pre[k - 1])
        out = [0] * k
        for i in range(k - 1, 0, -1):
            out[i] = self.mul(t, pre[i - 1])
            t = self.mul(t, vals[i])
        out[0] = t
        return out
