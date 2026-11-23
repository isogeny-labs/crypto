"""Build Velu 2-isogeny towers over F_p and unit instances. Plain Python integers (p ~ 2^20, p = 3 mod 4)."""
import numpy as np
from tower_engine import s3_coeffs


def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True


def find_prime(lo=1 << 20):
    n = lo
    while not (is_prime(n) and n % 4 == 3): n += 1
    return n


def sq_table(p):
    s = np.zeros(p, dtype=np.int8)
    y = np.arange(p, dtype=np.int64)
    s[(y * y) % p] = 1
    return s


def count_points(p, a, b, sq):
    x = np.arange(p, dtype=np.int64)
    f = (x * x % p * x + a * x + b) % p
    return int(np.where(f == 0, 1, np.where(sq[f] == 1, 2, 0)).sum()) + 1


def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, p - 2, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, p - 2, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R


def rand_point(rng, a, b, p):
    while True:
        x = rng.randrange(p); f = (x ** 3 + a * x + b) % p
        if f == 0: continue
        y = pow(f, (p + 1) // 4, p)
        if y * y % p == f: return (x, y if rng.random() < .5 else p - y)


def order_pow2(P, a, p, cap=64):
    k = 0
    while P is not None:
        P = add(P, P, a, p); k += 1
        if k > cap: raise ValueError
    return k


def build_tower(p, t, rng, sq, max_curves=20000):
    tried = 0
    while True:
        tried += 1
        if tried > max_curves: raise RuntimeError("no E' found")
        a0, b0 = rng.randrange(p), rng.randrange(p)
        if (4 * a0 ** 3 + 27 * b0 ** 2) % p == 0: continue
        N = count_points(p, a0, b0, sq)
        if N % (1 << t): continue
        v = (N & -N).bit_length() - 1
        odd = N >> v
        Q = None
        for _ in range(40):
            P = mul(odd, rand_point(rng, a0, b0, p), a0, p)
            s = order_pow2(P, a0, p) if P is not None else 0
            if s >= t:
                Q = mul(1 << (s - t), P, a0, p); break
        if Q is None: continue
        Qs, T = set(), None
        for k in range(1 << t):
            Qs.add(T); T = add(T, Q, a0, p)
        for _ in range(200):
            R = rand_point(rng, a0, b0, p)
            if add(R, R, a0, p) not in Qs: break
        else: continue
        assert T is None  # order of Q divides 2^t, and below it is exactly 2^t
        assert mul(1 << (t - 1), Q, a0, p) is not None
        return dict(a0=a0, b0=b0, N=N, Q=Q, R=R, aux_curves_tried=tried)


def velu_data(p, t, E):
    a, b = E['a0'], E['b0']; Q, R = E['Q'], E['R']
    xi, tau = [], []
    for j in range(t):
        K = mul(1 << (t - 1 - j), Q, a, p)
        assert K is not None and K[1] == 0, 'kernel point not 2-torsion'
        x0 = K[0]; tj = (3 * x0 * x0 + a) % p
        xi.append(x0); tau.append(tj)
        def phi(P):
            x, y = P; d = pow(x - x0, p - 2, p)
            return ((x + tj * d) % p, y * (1 - tj * d * d) % p)
        Q = phi(Q) if j < t - 1 else None
        R = phi(R)
        a, b = (a - 5 * tj) % p, (b - 7 * x0 * tj) % p
        assert (R[1] ** 2 - (R[0] ** 3 + a * R[0] + b)) % p == 0
        if Q is not None: assert (Q[1] ** 2 - (Q[0] ** 3 + a * Q[0] + b)) % p == 0
    return xi, tau, R[0]


def base_V(p, t, E):
    a = E['a0']; Q, R = E['Q'], E['R']
    V, T = [], R
    for k in range(1 << t):
        V.append(T[0]); T = add(T, Q, a, p)
    return V


def check_tower(p, t, E, xi, tau, c, V):
    assert len(set(V)) == 1 << t, 'V not distinct'
    for v in V:
        x = v
        for j in range(t):
            assert x != xi[j], 'x_j == xi_j'
            x = (x + tau[j] * pow(x - xi[j], p - 2, p)) % p
        assert x == c, 'chain does not reach c'
    return True


def chain_points(p, t, xi, tau, V):
    out = []
    for v in V:
        xs, x = [], v
        for j in range(t):
            xs.append(x); x = (x + tau[j] * pow(x - xi[j], p - 2, p)) % p
        out.append(xs)
    return out


def pow_arr(X, e, p):
    r = np.ones_like(X)
    for _ in range(e): r = r * X % p
    return r


def unit_instance(p, t, V, rng, sq, max_draws=1000):
    Va = np.array(V, dtype=np.int64)
    rej = 0
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b ** 2) % p == 0: continue
        tx = rng.randrange(p); f = (tx ** 3 + a * tx + b) % p
        if f == 0 or not sq[f]: continue
        val = np.zeros((len(V), len(V)), dtype=np.int64)
        for (e1, e2), cf in s3_coeffs(p, a, b, tx).items():
            val = (val + cf * (pow_arr(Va[:, None], e1, p) * pow_arr(Va[None, :], e2, p) % p)) % p
        if int((val == 0).sum()) == 0:
            return dict(a=a, b=b, target=tx, rejected_nonunit=rej)
        rej += 1
        if rej > max_draws: raise RuntimeError('no unit instance')
