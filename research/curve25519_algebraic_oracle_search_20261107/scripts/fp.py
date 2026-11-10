"""Counted F_p arithmetic and Montgomery-curve helpers for the k = 20 miniature.
E: y^2 = x^3 + A x^2 + x over F_p, p = 1,048,573, #E = 8r, r = 131,203.
Counted native ops: multiplications and squarings (CNT['M']); inversions run the
counted Fermat chain. Uncounted helpers (prefix u_) are for setup and analysis only."""
import random
p = 1048573; A = 926; r = 131203; N = 8 * r
CNT = {'M': 0}

def reset():
    CNT['M'] = 0

def mul(a, b):
    CNT['M'] += 1
    return a * b % p

def inv(a):  # counted square-and-multiply on p - 2
    e = p - 2; res = None; base = a % p
    for bit in bin(e)[2:]:
        if res is None:
            res = base; continue
        res = mul(res, res)
        if bit == '1':
            res = mul(res, base)
    return res

def batch_inv(xs):  # Montgomery trick: 3(n-1) mults + 1 inversion
    n = len(xs); pre = [0] * n; acc = 1
    for i, x in enumerate(xs):
        pre[i] = acc
        acc = x % p if i == 0 else mul(acc, x)
    ia = inv(acc); out = [0] * n
    for i in range(n - 1, -1, -1):
        out[i] = mul(ia, pre[i]) if i else ia
        if i:
            ia = mul(ia, xs[i])
    return out

# ---- uncounted helpers -------------------------------------------------------
def f(x):
    return (x * x * x + A * x * x + x) % p

def u_sqrt(a):  # p = 5 mod 8 (Atkin)
    a %= p
    if a == 0: return 0
    if pow(a, (p - 1) // 2, p) != 1: return None
    b = pow(2 * a, (p - 5) // 8, p); i = 2 * a * b * b % p
    s = a * b * (i - 1) % p
    assert s * s % p == a
    return s

O = None
def u_add(P, Q):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        lam = (3 * x1 * x1 + 2 * A * x1 + 1) * pow(2 * y1, p - 2, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, p - 2, p) % p
    x3 = (lam * lam - A - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)

def u_neg(P):
    return None if P is None else (P[0], (-P[1]) % p)

def u_mul(k, P):
    R = None; k %= N
    while k:
        if k & 1: R = u_add(R, P)
        P = u_add(P, P); k >>= 1
    return R

def u_lift(x):
    y = u_sqrt(f(x))
    return None if y is None else (x % p, y)

U = 8 * pow(8, -1, r) % N  # projector onto E[r]: U = 1 mod r, 0 mod 8
def u_proj(P):
    return u_mul(U, P)

def u_rand_Er(rng):
    while True:
        P = u_lift(rng.randrange(p))
        if P is not None:
            Q = u_mul(8, P)
            if Q is not None: return Q

def S3(x1, x2, x3):  # Montgomery summation polynomial
    return ((x1 - x2) ** 2 * x3 * x3 - 2 * ((x1 + x2) * (x1 * x2 + 1) + 2 * A * x1 * x2) * x3 + (x1 * x2 - 1) ** 2) % p

def self_check(seed=1):
    rng = random.Random(seed)
    for _ in range(5):
        P = u_lift(rng.randrange(p))
        while P is None: P = u_lift(rng.randrange(p))
        assert u_mul(N, P) is None
    G = u_rand_Er(rng); assert u_mul(r, G) is None
    # cyclic 8-part: some point has [r]P of order 8
    while True:
        P = u_lift(rng.randrange(p))
        if P is None: continue
        T = u_mul(r, P)
        if u_mul(4, T) is not None: break
    return True
