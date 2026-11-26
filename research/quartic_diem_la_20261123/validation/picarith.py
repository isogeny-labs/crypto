#!/usr/bin/env python3
"""Independent Pic^0 arithmetic on a smooth plane quartic over F_p (validator code).

Written for the validation of research/quartic_diem_ladder_20261121; it shares no
code with the solver (src/cryptanalysis/plane_quartic_pic.rs, quartic_ladder.rs).
Only the *encoding* of a class in the study's records is taken from the
instrument's documentation, and is cross-checked numerically:

  * a class x is stored through the degree-3 class x + K - P0 (K = line section);
  * Typ(m): m is the reduced row-echelon basis, in the monomial order
    [x^2, xy, xz, y^2, yz, z^2], of the conics through the unique effective
    degree-3 divisor D in that class (D not collinear);
  * Exc(P): the class x + K - P0 equals K - P (P rational); zero = Exc(P0).

Method (all F_p linear algebra; no extension fields, no root finding):

  An effective divisor A on C is carried by a *generating space* S of forms of
  one degree whose common zero divisor on C is exactly A.  For a form c with
  div(c) >= A, the residual B = div(c) - A has
      {h in V_k : h div >= B} = {h : h * s in (c, F) for all s in S},
  because (c, F) is a saturated complete intersection on the smooth curve C.
  A space {h in V_k : div(h) >= B} has common zero divisor exactly B when
  4k - deg B >= 2g = 6 (globally generated).  Sums of divisors are carried by
  products of generating spaces.

  add(x, y): E7 = D_x + D_y + P0 (product space), c = one product, R = div(c)-E7,
  cubics V through E7, c3 in V, R5 = div(c3) - E7, a conic q through R5,
  D_z = div(q) - R5 ~ D_x + D_y + P0 - K, i.e. x + y.  The canonical form of D_z
  is then computed (net of conics, or the point P of K - P).
"""
import random

import numpy as np


class Field:
    def __init__(self, p):
        self.p = int(p)


# ── monomials ────────────────────────────────────────────────────────────

_MONO = {}


def mons(n):
    """Degree-n monomials (a, b, c) in the instrument's order: a desc, then b desc."""
    if n not in _MONO:
        v = []
        for a in range(n, -1, -1):
            for b in range(n - a, -1, -1):
                v.append((a, b, n - a - b))
        _MONO[n] = (v, {m: i for i, m in enumerate(v)})
    return _MONO[n]


def dim(n):
    return (n + 1) * (n + 2) // 2 if n >= 0 else 0


_MULIDX = {}


def mulidx(d1, d2):
    """idx[j, i] = index in V_{d1+d2} of mons(d1)[j] * mons(d2)[i]."""
    key = (d1, d2)
    if key not in _MULIDX:
        m1, _ = mons(d1)
        m2, _ = mons(d2)
        _, ix = mons(d1 + d2)
        idx = np.empty((len(m1), len(m2)), dtype=np.int64)
        for j, (a, b, c) in enumerate(m1):
            for i, (e, f, g) in enumerate(m2):
                idx[j, i] = ix[(a + e, b + f, c + g)]
        _MULIDX[key] = idx
    return _MULIDX[key]


# ── linear algebra mod p ─────────────────────────────────────────────────

def rref(A, p):
    """Reduced row echelon form mod p; returns (rows, pivot columns)."""
    A = np.array(A, dtype=np.int64) % p
    if A.ndim != 2 or A.size == 0:
        return np.zeros((0, A.shape[1] if A.ndim == 2 else 0), dtype=np.int64), []
    m, n = A.shape
    piv = []
    r = 0
    for c in range(n):
        if r == m:
            break
        nz = np.nonzero(A[r:, c])[0]
        if len(nz) == 0:
            continue
        k = r + int(nz[0])
        if k != r:
            A[[r, k]] = A[[k, r]]
        inv = pow(int(A[r, c]), p - 2, p)
        A[r] = (A[r] * inv) % p
        col = A[:, c].copy()
        col[r] = 0
        nzr = np.nonzero(col)[0]
        if len(nzr):
            A[nzr] = (A[nzr] - np.outer(col[nzr], A[r]) % p) % p
        piv.append(c)
        r += 1
    return A[:r], piv


def nullspace(A, p):
    """Basis (rows) of {x : A x = 0 mod p}."""
    n = A.shape[1]
    R, piv = rref(A, p)
    free = [c for c in range(n) if c not in set(piv)]
    out = np.zeros((len(free), n), dtype=np.int64)
    for k, f in enumerate(free):
        out[k, f] = 1
        for i, pc in enumerate(piv):
            out[k, pc] = (-R[i, f]) % p
    return out


def rank(A, p):
    return len(rref(A, p)[1])


# ── forms ────────────────────────────────────────────────────────────────

def fmul(f, df, g, dg, p):
    idx = mulidx(df, dg)
    out = np.zeros(dim(df + dg), dtype=np.int64)
    prod = (np.outer(f, g) % p).ravel()
    np.add.at(out, idx.ravel(), prod)
    return out % p


def mulmat(s, ds, k):
    """Matrix of h -> s*h, V_k -> V_{k+ds} (columns indexed by mons(k))."""
    idx = mulidx(ds, k)
    M = np.zeros((dim(ds + k), dim(k)), dtype=np.int64)
    cols = np.broadcast_to(np.arange(dim(k)), idx.shape)
    M[idx, cols] = np.asarray(s, dtype=np.int64)[:, None]
    return M


def evalform(f, d, pt, p):
    m, _ = mons(d)
    x, y, z = pt
    s = 0
    for coef, (a, b, c) in zip(f, m):
        if coef:
            s = (s + int(coef) * pow(x, a, p) * pow(y, b, p) * pow(z, c, p)) % p
    return s


class Quartic:
    def __init__(self, p, f, p0):
        self.p = int(p)
        self.F = np.array(f, dtype=np.int64) % self.p
        self.P0 = tuple(int(v) for v in p0)
        assert len(self.F) == 15
        self._ideal_cache = {}
        self.L_P0 = self.lines_through(self.P0)

    # basic geometry ------------------------------------------------------
    def on_curve(self, pt):
        return evalform(self.F, 4, pt, self.p) == 0

    def partials(self):
        p = self.p
        m4, _ = mons(4)
        _, ix3 = mons(3)
        out = []
        for v in range(3):
            g = np.zeros(10, dtype=np.int64)
            for coef, e in zip(self.F, m4):
                if coef and e[v] > 0:
                    e2 = list(e)
                    e2[v] -= 1
                    g[ix3[tuple(e2)]] = (g[ix3[tuple(e2)]] + int(coef) * e[v]) % p
            out.append(g)
        return out

    def is_smooth(self):
        """No common zero of F_x, F_y, F_z over the algebraic closure iff the
        ideal they generate contains every form of degree 7 (Macaulay bound
        3+3+3-3+1 for three cubics in three variables)."""
        rows = []
        for g in self.partials():
            M = mulmat(g, 3, 4)
            rows.append(M.T)
        A = np.vstack(rows)
        return rank(A, self.p) == dim(7)

    def lines_through(self, pt):
        """Basis of the 2-dimensional space of linear forms vanishing at pt."""
        A = np.array([[pt[0], pt[1], pt[2]]], dtype=np.int64)  # mons(1) = x, y, z
        return nullspace(A, self.p)

    # ideal (c, F) in degree N: projection onto the quotient --------------
    def quotient_map(self, c, dc, N):
        """Q with Q v = 0 iff v in (c, F)_N; returned as (B, piv, nonpiv)."""
        p = self.p
        rows = []
        if N - dc >= 0:
            rows.append(mulmat(c, dc, N - dc).T)
        if N - 4 >= 0:
            rows.append(mulmat(self.F, 4, N - 4).T)
        if not rows:
            return np.zeros((0, dim(N)), dtype=np.int64), [], list(range(dim(N)))
        B, piv = rref(np.vstack(rows), p)
        ps = set(piv)
        nonpiv = [i for i in range(dim(N)) if i not in ps]
        return B, piv, nonpiv

    def residual_space(self, c, dc, S, dS, k):
        """{h in V_k : h*s in (c,F) for all s in S} (S: rows, forms of degree dS)."""
        p = self.p
        N = k + dS
        B, piv, nonpiv = self.quotient_map(c, dc, N)
        blocks = []
        for s in S:
            M = mulmat(s, dS, k)  # V_k -> V_N
            QM = (M[nonpiv, :] - (B[:, nonpiv].T @ M[piv, :]) % p) % p if len(piv) else M[nonpiv, :]
            blocks.append(QM)
        A = np.vstack(blocks) if blocks else np.zeros((0, dim(k)), dtype=np.int64)
        return nullspace(A, p)

    def modF_dim(self, S, d):
        """Dimension of span(S) modulo F*V_{d-4}."""
        if d < 4:
            return rank(S, self.p) if len(S) else 0
        FV = mulmat(self.F, 4, d - 4).T
        return rank(np.vstack([S, FV]), self.p) - rank(FV, self.p)

    def random_nonzero_modF(self, S, d, rng):
        p = self.p
        for _ in range(64):
            coeffs = [rng.randrange(p) for _ in range(len(S))]
            v = np.zeros(S.shape[1], dtype=np.int64)
            for a, s in zip(coeffs, S):
                v = (v + a * s) % p
            if d < 4:
                if v.any():
                    return v
            else:
                FV = mulmat(self.F, 4, d - 4).T
                if rank(np.vstack([FV, v[None, :]]), p) > rank(FV, p):
                    return v
        raise RuntimeError("no element nonzero mod F")

    # divisors ------------------------------------------------------------
    def product_space(self, spaces):
        """Products of generating spaces [(S, d), ...] -> (S', sum d)."""
        p = self.p
        cur = [np.array([1], dtype=np.int64)]
        dc = 0
        for S, d in spaces:
            nxt = []
            for a in cur:
                for s in S:
                    nxt.append(fmul(a, dc, s, d, p))
            cur = nxt
            dc += d
        return np.array(cur, dtype=np.int64), dc

    # classes -------------------------------------------------------------
    def class_from_record(self, s):
        """Parse 'Typ([[..],[..],[..]])' / 'Exc([x, y, z])' into a class."""
        s = s.strip()
        if s.startswith("Typ("):
            body = s[4:-1]
            rows = [[int(v) for v in r.strip(" []").split(",")] for r in body.strip()[1:-1].split("],")]
            net = np.array(rows, dtype=np.int64) % self.p
            assert net.shape == (3, 6), net.shape
            return ("Typ", net)
        if s.startswith("Exc("):
            pt = tuple(int(v) for v in s[4:-1].strip(" []").split(","))
            return ("Exc", pt)
        raise ValueError(s)

    def check_typ_record(self, net):
        """The recorded net is in RREF, spans 3 dims, its base scheme lies on C
        (F in the ideal of the net in degree 4) and has length 3 (not collinear)."""
        p = self.p
        R, piv = rref(net, p)
        is_rref = R.shape == net.shape and np.array_equal(R, net)
        ideal4 = np.vstack([mulmat(q, 2, 2).T for q in net])
        r4 = rank(ideal4, p)
        F_in = rank(np.vstack([ideal4, self.F[None, :]]), p) == r4
        # A 0-dimensional base scheme of length 3 has Hilbert function 3 in degree 4;
        # three conics through a common line (collinear D) leave codimension >= 5.
        return {"rref": bool(is_rref), "rank3": len(piv) == 3, "F_in_ideal": bool(F_in),
                "ideal4_codim": dim(4) - r4, "length3_base_scheme": dim(4) - r4 == 3}

    def gens(self, cls):
        """Generating space (S, d) of the degree-3 effective divisor D of cls."""
        kind, v = cls
        if kind == "Typ":
            return v, 2
        # Exc(P): D = div(L) - P for a line L through P
        P = v
        LP = self.lines_through(P)
        L = LP[0]
        S = self.residual_space(L, 1, LP, 1, 3)
        return S, 3

    def canonical(self, S_R5, dS, q, dq):
        """Canonical class of D = div(q) - R5, where S_R5 (degree dS) generates R5."""
        p = self.p
        lines = self.residual_space(q, dq, S_R5, dS, 1)
        if len(lines) == 0:
            net = self.residual_space(q, dq, S_R5, dS, 2)
            if len(net) != 3:
                raise RuntimeError(f"net of dimension {len(net)}")
            R, _ = rref(net, p)
            return ("Typ", R)
        assert len(lines) == 1, len(lines)
        L = lines[0]
        SD = self.residual_space(q, dq, S_R5, dS, 3)
        LP = self.residual_space(L, 1, SD, 3, 1)   # lines through P = div(L) - D
        assert len(LP) == 2, len(LP)
        a, b = LP
        P = [int(a[1] * b[2] - a[2] * b[1]) % p, int(a[2] * b[0] - a[0] * b[2]) % p,
             int(a[0] * b[1] - a[1] * b[0]) % p]
        i = next(k for k in range(3) if P[k])
        inv = pow(P[i], p - 2, p)
        P = tuple((c * inv) % p for c in P)
        assert self.on_curve(P)
        return ("Exc", P)

    def add(self, x, y, rng=None):
        rng = rng or random.Random(1)
        p = self.p
        Sx, dx = self.gens(x)
        Sy, dy = self.gens(y)
        S7, d7 = self.product_space([(Sx, dx), (Sy, dy), (self.L_P0, 1)])
        # c: a nonzero product (no factor vanishes on C: degrees < 4, F irreducible)
        c = S7[0]
        degR = 4 * d7 - 7
        k = max(1, -(-(degR + 6) // 4))
        SR = self.residual_space(c, d7, S7, d7, k)
        V3 = self.residual_space(c, d7, SR, k, 3)   # cubics through E7
        if len(V3) < 3:
            raise RuntimeError(f"cubics through E7: {len(V3)}")
        c3 = self.random_nonzero_modF(V3, 3, rng)
        SR5 = self.residual_space(c3, 3, S7, d7, 3)  # generates R5 = div(c3) - E7
        Qc = self.residual_space(c3, 3, S7, d7, 2)   # conics through R5 = div(c3) - E7
        if len(Qc) < 1:
            raise RuntimeError("no conic through R5")
        q = self.random_nonzero_modF(Qc, 2, rng)
        return self.canonical(SR5, 3, q, 2)

    def zero(self):
        return ("Exc", self.normalize(self.P0))

    def normalize(self, P):
        p = self.p
        i = next(k for k in range(3) if P[k] % p)
        inv = pow(int(P[i]), p - 2, p)
        return tuple((int(c) * inv) % p for c in P)

    def eq(self, x, y):
        if x[0] != y[0]:
            return False
        if x[0] == "Typ":
            return bool(np.array_equal(x[1] % self.p, y[1] % self.p))
        return self.normalize(x[1]) == self.normalize(y[1])

    def mul(self, x, n, rng=None):
        rng = rng or random.Random(n & 0xFFFFFFFF)
        if n == 0:
            return self.zero()
        acc = x
        for bit in bin(n)[3:]:
            acc = self.add(acc, acc, rng)
            if bit == "1":
                acc = self.add(acc, x, rng)
        return acc

    def class_of_points(self, pts):
        """Class of P1 + P2 + P3 - (K - P0) for rational points (degree-3 effective)."""
        spaces = [(self.lines_through(P), 1) for P in pts]
        S, d = self.product_space(spaces)
        # canonical form through one residual step: c in S, R = div(c) - D ...
        c = S[0]
        degR = 4 * d - 3
        k = max(1, -(-(degR + 6) // 4))
        SR = self.residual_space(c, d, S, d, k)
        return self.canonical(SR, k, c, d)

    # point search (for random classes) ------------------------------------
    def random_points(self, n, rng, limit=200000):
        p = self.p
        m4, _ = mons(4)
        out = []
        tries = 0
        zs = np.arange(p, dtype=np.int64)
        while len(out) < n and tries < limit:
            tries += 1
            y = rng.randrange(p)
            co = [0] * 5
            for coef, (a, b, c) in zip(self.F, m4):
                if coef:
                    co[c] = (co[c] + int(coef) * pow(y, b, p)) % p
            v = np.zeros(p, dtype=np.int64)
            for e in range(4, -1, -1):
                v = (v * zs + co[e]) % p
            roots = np.nonzero(v == 0)[0]
            if len(roots):
                z = int(roots[rng.randrange(len(roots))])
                pt = (1, y, z)
                assert self.on_curve(pt)
                out.append(pt)
        return out
