"""Prime-field curve y^2 = x^3 + ax + b over F_p and its BTN relation encoding.

Prime-field analogue of the binary prototype (research/btn_ecdl_20261029/).
The paper (Akshima et al., v0.2) targets prime-field curves; this builds the
only curve-specific ingredients it names -- point validity and point addition
as Boolean tensor networks -- over F_p, and reuses btn.Net / btn.contract.

Correctness-first: modular multiply is schoolbook (O(d^2) edges, carry chains),
not the paper's O(d) Fourier multiplier.  Every count is checked against brute
force at small primes.
"""
import numpy as np
from btn import Net


class Fp:
    def __init__(self, p):
        self.p = p
        self.d = p.bit_length()  # bits to hold an element in [0, p)

    def inv(self, a):
        return pow(a, self.p - 2, self.p)


class CurveP:
    """Affine short Weierstrass y^2 = x^3 + a x + b over F_p, O is None."""

    def __init__(self, F, a, b):
        self.F, self.a, self.b = F, a % F.p, b % F.p

    def on(self, P):
        if P is None:
            return True
        x, y = P
        p = self.F.p
        return (y * y - (x * x * x + self.a * x + self.b)) % p == 0

    def add(self, P, Q):
        if P is None: return Q
        if Q is None: return P
        p = self.F.p
        x1, y1 = P; x2, y2 = Q
        if x1 == x2 and (y1 + y2) % p == 0:
            return None
        if P == Q:
            lam = (3 * x1 * x1 + self.a) * self.F.inv(2 * y1 % p) % p
        else:
            lam = (y2 - y1) * self.F.inv((x2 - x1) % p) % p
        x3 = (lam * lam - x1 - x2) % p
        y3 = (lam * (x1 - x3) - y1) % p
        return (x3, y3)

    def mulk(self, k, P):
        R = None
        k %= 1 << 64
        while k:
            if k & 1: R = self.add(R, P)
            P = self.add(P, P); k >>= 1
        return R

    def points(self):
        p = self.F.p
        pts = []
        for x in range(p):
            rhs = (x * x * x + self.a * x + self.b) % p
            for y in range(p):
                if y * y % p == rhs:
                    pts.append((x, y))
        return pts


# ---- integer / modular gadgets on the Net (little-endian bit words) ----

def _int_add(net, A, B, width):
    """Z = A + B as `width` bits (A, B up to width bits); ripple carry."""
    Z = net.word(width)
    carry = None
    for i in range(width):
        ins = [Z[i]]
        a = A[i] if i < len(A) else None
        b = B[i] if i < len(B) else None
        src = [t for t in (a, b, carry) if t is not None]
        nxt = net.word(1)[0] if i < width - 1 else None
        sc = [Z[i]] + src + ([nxt] if nxt is not None else [])

        def fn(*v, k=len(src), has_nxt=nxt is not None):
            z = v[0]; bits = v[1:1 + k]; s = sum(bits)
            lo = s & 1; hi = s >> 1
            if z != lo: return False
            if has_nxt: return v[1 + k] == hi
            return hi == 0
        net.add(sc, fn)
        carry = nxt
    return Z


def _mul_schoolbook(net, A, B, width):
    """Z = A * B (A: da bits, B: db bits) as `width` bits.  Sum of shifted
    AND-partial-products via a ripple accumulator; parsimonious."""
    da, db = len(A), len(B)
    acc = None
    for j in range(db):
        # partial = (B[j] AND A) << j, held in `width` bits
        pp = [None] * j
        for i in range(da):
            g = net.word(1)[0]
            net.add([g, B[j], A[i]], lambda o, b, a: o == (b & a))
            pp.append(g)
        pp = (pp + [None] * width)[:width]
        ppw = net.word(width)
        for i in range(width):
            if pp[i] is None:
                net.add([ppw[i]], lambda o: o == 0)
            else:
                net.add([ppw[i], pp[i]], lambda o, s: o == s)
        acc = ppw if acc is None else _int_add(net, acc, ppw, width)
    if acc is None:
        acc = net.word(width)
        for i in range(width): net.add([acc[i]], lambda o: o == 0)
    return acc


def _fix_const(net, word, value):
    for i, w in enumerate(word):
        net.add([w], lambda o, i=i: o == (value >> i & 1))


def _lt_const(net, A, c):
    """Force integer(A) < c (c a public constant)."""
    d = len(A)
    # OR-chain style: compare from MSB. state = A already > prefix? Build via
    # standard: A < c iff exists i: A[i]=0,c[i]=1 and A[>i]=c[>i].
    # Encode with a 'still-equal' chain from MSB.
    eq = net.word(1)[0]
    net.add([eq], lambda o: o == 1)  # equal-so-far above the top bit
    done_lt = net.word(1)[0]
    net.add([done_lt], lambda o: o == 0)
    for i in range(d - 1, -1, -1):
        ci = c >> i & 1
        neq = net.word(1)[0]; ndl = net.word(1)[0]
        # if eq and A[i]<ci -> lt; if eq and A[i]>ci -> not allowed unless done_lt
        net.add([neq, eq, A[i]], lambda no, e, ai, ci=ci: no == (1 if (e and ai == ci) else 0))
        net.add([ndl, done_lt, eq, A[i]],
                lambda nd, dl, e, ai, ci=ci: nd == (1 if (dl or (e and ai == 0 and ci == 1)) else 0))
        # forbid eq and A[i]=1,ci=0 (would make A>c in this prefix)
        net.add([eq, A[i]], lambda e, ai, ci=ci: not (e and ai == 1 and ci == 0))
        eq, done_lt = neq, ndl
    net.add([done_lt, eq], lambda dl, e: dl == 1 and e == 0)  # strictly less


def mul_mod(net, F, A, B):
    """Return Z (d bits) with Z = A*B mod p, 0<=Z<p.  Introduces q and the
    2d-bit products; all auxiliaries uniquely determined, so parsimonious."""
    p, d = F.p, F.d
    w = 2 * d + 2
    prod = _mul_schoolbook(net, A, B, w)           # A*B
    q = net.word(d + 1)                            # quotient, < ~2^(d+1)
    Z = net.word(d)
    _lt_const(net, Z, p)
    pconst = net.word(d + 1)
    _fix_const(net, pconst, p)
    qp = _mul_schoolbook(net, q, pconst, w)        # q*p
    tot = _int_add(net, qp, Z, w)                  # q*p + Z
    for i in range(w):                             # prod == q*p + Z
        net.add([prod[i], tot[i]], lambda a, b: a == b)
    return Z


def lin_mod(net, F, pos, neg, out, kmax):
    """Constrain integer(out) == sum(pos) - sum(neg) (mod p), 0<=out<p, where
    each word in pos/neg holds a value < p.  kmax bounds the number of terms so
    the quotient range is known.  All auxiliaries determined -> parsimonious."""
    p, d = F.p, F.d
    w = d + kmax.bit_length() + 2
    # S = sum(pos) + q_neg*... handle neg by moving to other side:
    # sum(pos) + qn*p... instead: sum(pos) + (sum over neg of (p - neg_as_value))
    # Simpler: require sum(pos) + Kp == out + sum(neg) + q*p  with K chosen so LHS>=RHS.
    # Build L = sum(pos) + len(neg)*p (constant shift), R = out + sum(neg), then L - R = q*p.
    acc = None
    for word in pos:
        wi = (list(word) + [None] * w)[:w]
        ww = net.word(w)
        for i in range(w):
            if wi[i] is None: net.add([ww[i]], lambda o: o == 0)
            else: net.add([ww[i], wi[i]], lambda o, s: o == s)
        acc = ww if acc is None else _int_add(net, acc, ww, w)
    shift = net.word(w); _fix_const(net, shift, (len(neg)) * p)
    L = _int_add(net, acc, shift, w) if acc is not None else shift
    # R = out + sum(neg)
    oi = (list(out) + [None] * w)[:w]
    Rw = net.word(w)
    for i in range(w):
        if oi[i] is None: net.add([Rw[i]], lambda o: o == 0)
        else: net.add([Rw[i], oi[i]], lambda o, s: o == s)
    for word in neg:
        wi = (list(word) + [None] * w)[:w]
        ww = net.word(w)
        for i in range(w):
            if wi[i] is None: net.add([ww[i]], lambda o: o == 0)
            else: net.add([ww[i], wi[i]], lambda o, s: o == s)
        Rw = _int_add(net, Rw, ww, w)
    _lt_const(net, out, p)
    q = net.word(kmax.bit_length() + 2)
    pconst = net.word(d + 1); _fix_const(net, pconst, p)
    qp = _mul_schoolbook(net, q, pconst, w)
    tot = _int_add(net, qp, Rw, w)
    for i in range(w):
        net.add([L[i], tot[i]], lambda a, b: a == b)


def on_curve_p(net, C, P):
    """y^2 = x^3 + a x + b  (mod p)."""
    F = C.F; x, y = P
    _lt_const(net, x, F.p); _lt_const(net, y, F.p)  # coordinates are field elements
    y2 = mul_mod(net, F, y, y)
    x2 = mul_mod(net, F, x, x)
    x3 = mul_mod(net, F, x2, x)
    aw = net.word(F.d); _fix_const(net, aw, C.a)
    axv = mul_mod(net, F, aw, x)
    bw = net.word(F.d); _fix_const(net, bw, C.b)
    # y2 == x3 + ax + b (mod p)
    lin_mod(net, F, [x3, axv, bw], [], y2, kmax=3)


def add_relation(net, C, P1, P2, R):
    """Constrain P1 + P2 = R, generic case x1 != x2, both affine."""
    F = C.F; d = F.d
    (x1, y1), (x2, y2) = P1, P2
    x3, y3 = R
    lam = net.word(d); _lt_const(net, lam, F.p)
    # lam*(x2-x1) = (y2-y1) mod p  ->  lam*dx = dy
    dx = net.word(d); lin_mod(net, F, [x2], [x1], dx, kmax=2)
    dy = net.word(d); lin_mod(net, F, [y2], [y1], dy, kmax=2)
    ldx = mul_mod(net, F, lam, dx)
    for i in range(d): net.add([ldx[i], dy[i]], lambda a, b: a == b)
    net.add(dx, lambda *b: any(b))  # x1 != x2
    # lam^2 = x3 + x1 + x2 mod p
    l2 = mul_mod(net, F, lam, lam)
    lin_mod(net, F, [x3, x1, x2], [], l2, kmax=3)
    # y3 + y1 = lam*(x1 - x3) mod p
    dxx = net.word(d); lin_mod(net, F, [x1], [x3], dxx, kmax=2)
    rhs = mul_mod(net, F, lam, dxx)
    lin_mod(net, F, [y3, y1], [], rhs, kmax=2)
    return lam
