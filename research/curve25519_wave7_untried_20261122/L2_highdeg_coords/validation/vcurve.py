"""Validator's own curve code (independent of the L2 scripts).

Montgomery v^2 = u^3 + A u^2 + u over F_p (B = 1).  Group law is done on the
short Weierstrass model x = u + A/3, y = v, y^2 = x^3 + a4 x + a6.
Square roots use Atkin's formula for p = 5 (mod 8).
Generator: the point of order n = 8r with the SMALLEST u (and the smaller v);
this differs from the producer's random.Random(1) choice on purpose.
"""
import numpy as np

CURVES = {16: (65437, 1058, 65672), 18: (262133, 78, 263096), 20: (1048573, 926, 1049624)}


def isprime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
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


def npow(b, e, p):
    b = np.asarray(b, dtype=np.int64) % p
    r = np.ones_like(b)
    while e > 0:
        if e & 1:
            r = (r * b) % p
        b = (b * b) % p
        e >>= 1
    return r


def ninv(a, p):
    return npow(a, p - 2, p)


class VCurve:
    def __init__(self, k, build_logs=True):
        p, A, n = CURVES[k]
        self.k, self.p, self.A, self.n = k, p, A, n
        self.r = n // 8
        i3 = pow(3, p - 2, p)
        self.A3 = A * i3 % p
        self.a4 = (1 - A * A % p * i3) % p
        self.a6 = A * (2 * A * A - 9) % p * pow(27, p - 2, p) % p
        self.facts = {}
        self._points()
        if build_logs:
            self._logs()

    # Weierstrass arithmetic, points as (x, y) or None
    def wadd(self, P, Q):
        p = self.p
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            l = (3 * x1 * x1 + self.a4) * pow(2 * y1, p - 2, p) % p
        else:
            l = (y2 - y1) * pow((x2 - x1) % p, p - 2, p) % p
        x3 = (l * l - x1 - x2) % p
        return (x3, (l * (x1 - x3) - y1) % p)

    def wmul(self, m, P):
        R = None
        m %= self.n
        while m:
            if m & 1:
                R = self.wadd(R, P)
            P = self.wadd(P, P)
            m >>= 1
        return R

    def to_w(self, uv):
        return None if uv is None else ((uv[0] + self.A3) % self.p, uv[1])

    def to_m(self, xy):
        return None if xy is None else ((xy[0] - self.A3) % self.p, xy[1])

    def sqrt(self, a):
        p = self.p
        a %= p
        if a == 0:
            return 0
        if pow(a, (p - 1) // 2, p) != 1:
            return None
        b = pow(2 * a, (p - 5) // 8, p)
        i = 2 * a * b * b % p
        s = a * b * (i - 1) % p
        assert s * s % p == a
        return s

    def _points(self):
        p, A = self.p, self.A
        u = np.arange(p, dtype=np.int64)
        rhs = (u * u % p * u + A * (u * u % p) + u) % p
        leg = npow(rhs, (p - 1) // 2, p)
        nsq = int((leg == 1).sum()); nz = int((rhs == 0).sum())
        cnt = 1 + nz + 2 * nsq
        f = self.facts
        f['p_prime'] = isprime(p); f['p_mod_8'] = p % 8
        f['count'] = cnt; f['count_eq_n'] = cnt == self.n
        f['r_prime'] = isprime(self.r); f['n_eq_8r'] = self.n == 8 * self.r
        f['twist_order'] = 2 * p + 2 - self.n
        f['twist_4_prime'] = (2 * p + 2 - self.n) % 4 == 0 and isprime((2 * p + 2 - self.n) // 4)
        f['A2m4_nonsq'] = pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1
        f['Ap2_square'] = pow((A + 2) % p, (p - 1) // 2, p) == 1
        f['Am2_square'] = pow((A - 2) % p, (p - 1) // 2, p) == 1
        self.rhs = rhs

    def order_is_n(self, P):
        n = self.n
        return self.wmul(n, P) is None and self.wmul(n // 2, P) is not None and self.wmul(n // self.r, P) is not None

    def _logs(self):
        p, n = self.p, self.n
        for u in range(2, p):
            s = self.sqrt(int(self.rhs[u]))
            if s is None or s == 0:
                continue
            v = min(s, p - s)
            G = self.to_w((u, v))
            if self.order_is_n(G):
                break
        self.gen_uv = (u, v)
        G = self.to_w((u, v))
        U = np.full(n, -1, dtype=np.int64); V = np.full(n, -1, dtype=np.int64)
        P = None
        half = n // 2
        for l in range(1, half + 1):
            P = self.wadd(P, G)
            uu = (P[0] - self.A3) % p
            U[l] = uu; V[l] = P[1]
            if l < half:
                U[n - l] = uu; V[n - l] = (-P[1]) % p
        assert V[half] == 0 and U[half] == 0  # T0 = (0, 0)
        # every affine point appears exactly once
        key = U[1:] * p + V[1:]
        self.facts['all_points_distinct'] = int(np.unique(key).size) == n - 1
        # check the table against the curve equation
        uu = U[1:]; vv = V[1:]
        self.facts['table_on_curve'] = bool(np.all((vv * vv % p) == self.rhs[uu]))
        self.U, self.V = U, V
        self.key2log = None

    def log_of(self, uv):
        """discrete log (in this table's generator) of an affine point (u, v)."""
        if self.key2log is None:
            key = self.U[1:] * self.p + self.V[1:]
            o = np.argsort(key)
            self.key2log = (key[o], np.arange(1, self.n)[o])
        ks, ls = self.key2log
        q = uv[0] * self.p + uv[1]
        i = np.searchsorted(ks, q)
        assert ks[i] == q
        return int(ls[i])

    # ---- coordinate functions over the whole table (index = log, 0 = O)
    def fvals(self, name):
        p = self.p
        u = self.U.copy(); v = self.V.copy()
        fin = np.zeros(self.n, dtype=bool); fin[1:] = True
        u[0] = 0; v[0] = 0
        x = (u + self.A3) % p
        if name == 'x':
            num, den = x, np.ones_like(x)
        elif name == 'v':
            num, den = v, np.ones_like(v)
        elif name == 'q':          # Weierstrass x / y
            num, den = x, v
        elif name == 'uv':
            num, den = u * v % p, np.ones_like(u)
        elif name == 'xe':         # Edwards x = u / v (Montgomery X/Y)
            num, den = u, v
        elif name == 'ye':         # Edwards y = (u - 1)/(u + 1)
            num, den = (u - 1) % p, (u + 1) % p
        elif name == 'xeye':       # extended Edwards T/Z = x_E y_E = u (u-1) / (v (u+1))
            num, den = u * ((u - 1) % p) % p, v * ((u + 1) % p) % p
        elif name == 'jac6':       # Jacobian Y^2/X^3 = y^2 / x^3
            num, den = v * v % p, x * x % p * x % p
        else:
            raise KeyError(name)
        ok = fin & (den != 0)
        val = np.where(ok, num * ninv(np.where(ok, den, 1), p) % p, -1)
        return val, ok
