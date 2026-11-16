"""Characteristic-2 Schonhage (triadic) multiplication gadget as an XOR-layer circuit.

Ring B_M = F2[x]/(x^{2M}+x^M+1), M = 3^v; x is a principal 3M-th root of unity.
Multiply two n-bit carry-less polynomials (n <= N, N = 3^nu) with ONE split N = N1*N2
(N2 | N1, both powers of 3):  A(x) = sum_{i<N2} a_i(x) y^i, y = x^{N1}, a_i of N1 bits,
viewed as elements of B_{N1} (2*N1 bits wide).  psi = x^{N1/N2} in B_{N1} has order 3*N2.
Product c(y) = A(y)B(y) has y-degree < 2*N2-1, is determined by its values at the 2*N2
points psi^j, 3 !| j (roots of y^{2N2}+y^{N2}+1).  Inverse: length-3N2 DFT of the values
(zero at 3|j) gives r = c*(y^{2N2}+y^{N2}) mod y^{3N2}-1, and c is read off by a block
permutation: c_j = r_{j+N2} (j<N2), c_j = r_{j-N2} (N2<=j<2N2).  1/(3N2) = 1 in char 2.
Both DFTs are done as a four-step (Cooley-Tukey) factorisation L = P*Q.
Then carry-less recombination P(x) = sum_j c_j(x) x^{N1 j} and a linear reduction mod f.
"""
from __future__ import annotations


def ceil_pow3(n):
    N, nu = 1, 0
    while N < n:
        N *= 3
        nu += 1
    return N, nu


def split(nu):
    n1 = 3 ** ((nu + 1) // 2)
    n2 = 3 ** (nu // 2)
    return n1, n2


def factor_L(L):
    k = 0
    t = L
    while t > 1:
        t //= 3
        k += 1
    P = 3 ** ((k + 1) // 2)
    return P, L // P


class Circ:
    """Gate list: ('x', out, [ins]) XOR of ins; ('a', out, u, v) AND.  Var ids from 1."""

    def __init__(self, nstart):
        self.nxt = nstart + 1
        self.gates = []
        self.stat = {}
        self.vertices = []   # (tag, set(in vars), n_out_bits)
        self._cin = None
        self._cout = 0

    def begin(self):
        self._cin, self._cout = set(), 0

    def end(self, tag):
        self.vertices.append((tag, set(self._cin), self._cout))
        self._cin = None

    def new(self):
        v = self.nxt
        self.nxt += 1
        return v

    def mat(self, expr, tag):
        """Materialise a bit expression (frozenset of vars) as a single var; empty -> None."""
        if not expr:
            return None
        self.stat[tag] = self.stat.get(tag, 0) + 1   # one wire per nonempty output bit
        if self._cin is not None:
            self._cin |= expr
            self._cout += 1
        if len(expr) == 1:
            return next(iter(expr))
        v = self.new()
        self.gates.append(('x', v, sorted(expr)))
        return v

    def mat_word(self, word, tag):
        return [frozenset([v]) if (v := self.mat(e, tag)) is not None else frozenset() for e in word]

    def AND(self, u, v, tag):
        o = self.new()
        self.gates.append(('a', o, u, v))
        self.stat[tag] = self.stat.get(tag, 0) + 1
        return o


def xmul(word, k, N1):
    """multiply word (2*N1 bit exprs) by x^k in B_{N1}."""
    W = 2 * N1
    k %= 3 * N1
    w = list(word)
    for _ in range(k):
        o = w[-1]
        w = [frozenset()] + w[:-1]
        w[0] = w[0] ^ o
        w[N1] = w[N1] ^ o
    return w


def xmul_fast(word, k, N1):
    # same as xmul but via x^{3N1}=1 and x^{N1} = omega steps are just repeated mulx; k < 3*N1 <= 81
    return xmul(word, k, N1)


def wadd(a, b):
    return [u ^ v for u, v in zip(a, b)]


def dft_small(words, root_shift, N1, tag=None):
    """direct DFT of len(words) points with root x^root_shift: X_k = sum_i w_i x^{root_shift*i*k}."""
    n = len(words)
    W = 2 * N1
    out = []
    for k in range(n):
        acc = [frozenset()] * W
        for i in range(n):
            acc = wadd(acc, xmul(words[i], root_shift * i * k, N1))
        out.append(acc)
    return out


def dft4(C, words, s, N1, P, Q, sign, want, tag):
    """four-step DFT length L=P*Q, psi = x^{sign*s} (sign=+1 forward, -1 inverse).  i = Q*i1+i2,
    k = k1 + P*k2.  Returns {k: word} for k in want, mid layer materialised."""
    L = P * Q
    e = (s * sign) % (3 * N1)
    # step 1: column DFTs over i1 (P points) for each i2, root omega_P = psi^Q, then twiddle psi^{i2*k1}
    mid = {}
    for i2 in range(Q):
        col = [words[Q * i1 + i2] for i1 in range(P)]
        Y = dft_small(col, (e * Q) % (3 * N1), N1)
        C.begin()
        for k1 in range(P):
            T = xmul(Y[k1], (e * i2 * k1) % (3 * N1), N1)
            mid[(k1, i2)] = C.mat_word(T, tag + '_mid')
        C.end(tag + '_col')
    out = {}
    for k1 in range(P):
        row = [mid[(k1, i2)] for i2 in range(Q)]
        X = dft_small(row, (e * P) % (3 * N1), N1)
        C.begin()
        for k2 in range(Q):
            k = k1 + P * k2
            if k in want:
                out[k] = C.mat_word(X[k2], tag + '_out')
        C.end(tag + '_row')
    return out


def mul_circuit(C, A, B, n, N, red_rows, tag='m'):
    """A,B: n-bit lists of exprs.  Returns n reduced output exprs (linear in product wires).
    red_rows[i] = set of product bit indices t (0..2n-2) XORed into output bit i."""
    nu = 0
    t = N
    while t > 1:
        t //= 3
        nu += 1
    N1, N2 = split(nu)
    assert N1 * N2 == N and n <= N
    W = 2 * N1
    L = 3 * N2
    P, Q = factor_L(L)
    s = N1 // N2
    pts = [j for j in range(L) if j % 3]

    def load(X):
        Xp = list(X) + [frozenset()] * (N - len(X))
        return [Xp[i * N1:(i + 1) * N1] + [frozenset()] * N1 for i in range(N2)] + \
               [[frozenset()] * W for _ in range(L - N2)]
    words = []
    for X, nm in ((A, 'fa'), (B, 'fb')):
        w = load(X)
        F = dft4(C, w, s, N1, P, Q, +1, set(pts), tag + nm)
        words.append({j: F[j] for j in pts})
    # pointwise products in B_{N1}
    prod = {}
    for j in pts:
        a, b = words[0][j], words[1][j]
        C.begin()
        coef = [frozenset()] * (2 * W - 1)
        for u in range(W):
            if not a[u]:
                continue
            for v in range(W):
                if not b[v]:
                    continue
                g = C.AND(next(iter(a[u])), next(iter(b[v])), tag + '_and')
                coef[u + v] = coef[u + v] ^ frozenset([g])
        # reduce mod x^{2N1}+x^{N1}+1 (linear)
        red = list(coef[:W]) + [frozenset()] * 0
        hi = list(coef[W:])
        # fold bits k>=W downward: x^k = x^{k-N1} + x^{k-2N1}
        full = list(coef)
        for k in range(len(full) - 1, W - 1, -1):
            o = full[k]
            if o:
                full[k - N1] = full[k - N1] ^ o
                full[k - 2 * N1] = full[k - 2 * N1] ^ o
        inset = set()
        for w_ in (a, b):
            for e_ in w_:
                inset |= e_
        C._cin = None
        prod[j] = C.mat_word(full[:W], tag + '_pw')
        C._cin, C._cout = inset, sum(1 for e_ in prod[j] if e_)
        C.end(tag + '_pt')
    # inverse DFT: values at pts, zero elsewhere
    zero = [frozenset()] * W
    vals = [prod[j] if j in prod else zero for j in range(L)]
    R = dft4(C, vals, s, N1, P, Q, -1, set(range(2 * N2)), tag + 'inv')
    r = R
    c = {}
    for j in range(2 * N2):
        c[j] = r[j + N2] if j < N2 else r[j - N2]
    # carry-less recombination
    Pbits = [frozenset() for _ in range(2 * n - 1)]
    for j in range(2 * N2):
        for u in range(W):
            t_ = N1 * j + u
            if t_ < 2 * n - 1 and c[j][u]:
                Pbits[t_] = Pbits[t_] ^ c[j][u]
    Pw = []
    for b0 in range(0, 2 * n - 1, N1):
        C.begin()
        for e_ in Pbits[b0:b0 + N1]:
            Pw.append(C.mat(e_, tag + '_P'))
        C.end(tag + '_recomb')
    out = []
    for i0 in range(0, n, N1):
        C.begin()
        for i in range(i0, min(n, i0 + N1)):
            C._cin |= {Pw[t_] for t_ in red_rows[i] if Pw[t_] is not None}
            C._cout += 1
        C.end(tag + '_red')
    C.stat[tag + '_redout'] = n
    for i in range(n):
        e_ = frozenset()
        for t_ in red_rows[i]:
            if Pw[t_] is not None:
                e_ = e_ ^ frozenset([Pw[t_]])
        out.append(e_)
    return out


def reduction_rows(n, mod):
    """row i = set of t in 0..2n-2 s.t. x^t mod f has bit i."""
    rows = [set() for _ in range(n)]
    cur = 1
    for t in range(2 * n - 1):
        for i in range(n):
            if (cur >> i) & 1:
                rows[i].add(t)
        cur <<= 1
        if cur >> n:
            cur ^= mod
    return rows


def evaluate(gates, assign):
    """assign: dict var->0/1; fills in gate outputs."""
    for g in gates:
        if g[0] == 'x':
            v = 0
            for i in g[2]:
                v ^= assign[i]
            assign[g[1]] = v
        else:
            assign[g[1]] = assign[g[2]] & assign[g[3]]
    return assign
