"""Elliptic curves y^2 = x^3 + a x + b over F_{p^2}: group law, point counting,
isomorphism-class enumeration, Frobenius/conductor data, class numbers,
division polynomials, rational l-isogenies (Velu) and the l-volcano walk.

Python prototype under a task-level exception to the no-Python rule
(PROTOCOL.md section 1).
"""
from __future__ import annotations

import random
import time
from math import gcd

import numpy as np

from cgp_field import (
    Fq, factorint, factor_squarefree, padd, pdeg, pdivmod, pmod, pmonic, pmul,
    proots, pscale, psub, ptrim,
)


# ------------------------------------------------------------------ the curve
class Curve:
    """y^2 = x^3 + a x + b over F_q; a, b are F_q integers."""

    def __init__(self, F: Fq, a: int, b: int):
        self.F, self.a, self.b = F, a, b
        disc = F.add(F.mul(F.const(4), F.mul(a, F.sq(a))), F.mul(F.const(27), F.sq(b)))
        if disc == 0:
            raise ValueError("singular curve")

    def f(self, x: int) -> int:
        F = self.F
        return F.add(F.add(F.mul(x, F.sq(x)), F.mul(self.a, x)), self.b)

    def on_curve(self, P) -> bool:
        if P is None:
            return True
        F = self.F
        x, y = P
        return F.sq(y) == self.f(x)

    def neg(self, P):
        return None if P is None else (P[0], self.F.neg(P[1]))

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if F.add(y1, y2) == 0:
                return None  # Q = -P (also the 2-torsion doubling case y = 0)
            lam = F.div(F.add(F.mul(F.const(3), F.sq(x1)), self.a), F.mul(F.const(2), y1))
        else:
            lam = F.div(F.sub(y2, y1), F.sub(x2, x1))
        x3 = F.sub(F.sub(F.sq(lam), x1), x2)
        y3 = F.sub(F.mul(lam, F.sub(x1, x3)), y1)
        return (x3, y3)

    def mul(self, k: int, P):
        if k < 0:
            return self.mul(-k, self.neg(P))
        R = None
        Q = P
        while k:
            if k & 1:
                R = self.add(R, Q)
            k >>= 1
            if k:
                Q = self.add(Q, Q)
        return R

    def points(self) -> list:
        """All affine points (exhaustive over x in F_q), plus None for infinity."""
        F = self.F
        pts = [None]
        for x in range(F.q):
            v = self.f(x)
            if v == 0:
                pts.append((x, 0))
            else:
                y = F.sqrt(v)
                if y is not None:
                    pts.append((x, y))
                    pts.append((x, F.neg(y)))
        return pts

    def j(self) -> int:
        return j_invariant(self.F, self.a, self.b)

    def twist(self, d: int) -> "Curve":
        F = self.F
        return Curve(F, F.mul(self.a, F.sq(d)), F.mul(self.b, F.mul(d, F.sq(d))))


def j_invariant(F: Fq, a: int, b: int) -> int:
    a3 = F.mul(a, F.sq(a))
    num = F.mul(F.const(1728 * 4), a3)
    den = F.add(F.mul(F.const(4), a3), F.mul(F.const(27), F.sq(b)))
    return F.div(num, den)


# -------------------------------------------------------------- point counting
class Counter:
    """Vectorised point counting over one F_q (character sum)."""

    def __init__(self, F: Fq):
        self.F = F
        q, p = F.q, F.p
        self.EXP = np.array(F.EXP[: q - 1], dtype=np.int64)
        self.LOG = np.array(F.LOG, dtype=np.int64)
        self.X = np.arange(q, dtype=np.int64)
        self.nz = self.X != 0
        lx = self.LOG[self.X]
        self.l1 = np.where(self.nz, lx, 0)
        self.cube = np.where(self.nz, self.EXP[(3 * self.l1) % (q - 1)], 0)

    def count(self, a: int, b: int) -> int:
        """N = #E(F_q) including infinity."""
        F = self.F
        q, p = F.q, F.p
        if a == 0:
            ax = np.zeros(q, dtype=np.int64)
        else:
            ax = np.where(self.nz, self.EXP[(self.LOG[a] + self.l1) % (q - 1)], 0)
        ca, cb = self.cube % p, self.cube // p
        aa, ab = ax % p, ax // p
        ba, bb = b % p, b // p
        sa = (ca + aa + ba) % p
        sb = (cb + ab + bb) % p
        v = sa + p * sb
        lv = self.LOG[v]
        chi = np.where(v == 0, 0, np.where(lv % 2 == 0, 1, -1))
        return int(q + 1 + chi.sum())


def naive_count(F: Fq, a: int, b: int) -> int:
    """N by brute force over all (x, y) pairs with slow arithmetic: the unit-test oracle."""
    n = 1
    for x in range(F.q):
        fx = F.add(F.add(F.mul_slow(x, F.mul_slow(x, x)), F.mul_slow(a, x)), b)
        for y in range(F.q):
            if F.mul_slow(y, y) == fx:
                n += 1
    return n


# ----------------------------------------------------------- enumeration of classes
def enumerate_classes(F: Fq):
    """Every F_q-isomorphism class of y^2 = x^3 + a x + b as (j, w, a, b).

    Generic j: base model and its quadratic twist (w = 0, 1); j = 0: the six
    twists b = g^i; j = 1728: the four twists a = g^i.  Total 2q + 6.
    """
    p, q = F.p, F.q
    g = F.gen
    j1728 = F.const(1728)
    d = g  # a non-square
    out = []
    for j in range(q):
        if j == 0:
            for i in range(6):
                out.append((0, i, 0, F.pow(g, i)))
        elif j == j1728:
            for i in range(4):
                out.append((j, i, F.pow(g, i), 0))
        else:
            k = F.mul(j, F.sub(j1728, j))
            a = F.mul(F.const(3), k)
            b = F.mul(F.const(2), F.mul(k, F.sub(j1728, j)))  # 2 j (1728 - j)^2
            out.append((j, 0, a, b))
            out.append((j, 1, F.mul(a, F.sq(d)), F.mul(b, F.mul(d, F.sq(d)))))
    assert len(out) == 2 * q + 6
    for (j, w, a, b) in out:  # every model is nonsingular and has the j it is labelled with
        assert j_invariant(F, a, b) == j, (j, w, a, b)
    return out


def twist_partner_index(j: int, w: int, F: Fq) -> int:
    """Index w' of the quadratic twist of class (j, w)."""
    if j == 0:
        return (w + 3) % 6
    if j == F.const(1728):
        return (w + 2) % 4
    return 1 - w


# ----------------------------------------------- Frobenius order and class numbers
def disc_decomposition(t: int, q: int) -> tuple[int, int, int]:
    """t^2 - 4q = f^2 * D_K with D_K the fundamental discriminant.  Returns (Delta, f, D_K)."""
    delta = t * t - 4 * q
    assert delta < 0
    m = -delta
    fac = factorint(m)
    k, s = 1, 1
    for pr, e in fac.items():
        k *= pr ** (e // 2)
        if e % 2:
            s *= pr
    d0 = -s
    if d0 % 4 == 1:
        return delta, k, d0
    assert k % 2 == 0, "discriminant is not 0,1 mod 4?"
    return delta, k // 2, 4 * d0


def class_number(D: int) -> int:
    """h(D) for D < 0, D = 0,1 mod 4: primitive reduced forms (a, b, c), b^2 - 4ac = D."""
    assert D < 0 and D % 4 in (0, 1)
    h = 0
    a = 1
    while 3 * a * a <= -D:
        for b in range(-a + 1, a + 1):
            if (b * b - D) % (4 * a):
                continue
            c = (b * b - D) // (4 * a)
            if c < a:
                continue
            if c == a and b < 0:
                continue
            if gcd(gcd(a, abs(b)), c) != 1:
                continue
            h += 1
        a += 1
    return h


def divisors(n: int) -> list[int]:
    return sorted(d for d in range(1, n + 1) if n % d == 0)


def kronecker_class_number(f_pi: int, d_k: int) -> dict[int, int]:
    """{f: h(f^2 D_K)} over f | f_pi (Schoof: the number of classes with trace t is the sum)."""
    return {f: class_number(f * f * d_k) for f in divisors(f_pi)}


# ----------------------------------------------------------- division polynomials
class DivPolys:
    """psi_n as (poly in x, e) meaning poly(x) * y^e (e in {0, 1}), y^2 = x^3 + a x + b."""

    def __init__(self, F: Fq, a: int, b: int):
        self.F, self.a, self.b = F, a, b
        self.fx = [b, a, 0, 1]
        self.cache: dict[int, tuple[list[int], int]] = {}
        c = F.const
        a2 = F.sq(a)
        # psi_3 = 3x^4 + 6a x^2 + 12 b x - a^2
        self.cache[0] = ([], 0)
        self.cache[1] = ([1], 0)
        self.cache[2] = ([2], 1)
        self.cache[3] = (ptrim([F.neg(a2), F.mul(c(12), b), F.mul(c(6), a), 0, c(3)]), 0)
        # psi_4 = 4y (x^6 + 5a x^4 + 20 b x^3 - 5 a^2 x^2 - 4 a b x - 8 b^2 - a^3)
        a3 = F.mul(a, a2)
        b2 = F.sq(b)
        inner = ptrim([
            F.neg(F.add(F.mul(c(8), b2), a3)),
            F.neg(F.mul(c(4), F.mul(a, b))),
            F.neg(F.mul(c(5), a2)),
            F.mul(c(20), b),
            F.mul(c(5), a),
            0,
            1,
        ])
        self.cache[4] = (pscale(F, inner, c(4)), 1)

    def _mul(self, A, B):
        F = self.F
        P = pmul(F, A[0], B[0])
        e = A[1] + B[1]
        if e == 2:
            P = pmul(F, P, self.fx)
            e = 0
        return (P, e)

    def _sub(self, A, B):
        assert A[1] == B[1]
        return (psub(self.F, A[0], B[0]), A[1])

    def psi(self, n: int):
        if n in self.cache:
            return self.cache[n]
        F = self.F
        m = n // 2
        g = self.psi
        if n % 2:  # n = 2m + 1
            t1 = self._mul(g(m + 2), self._mul(g(m), self._mul(g(m), g(m))))
            t2 = self._mul(g(m - 1), self._mul(g(m + 1), self._mul(g(m + 1), g(m + 1))))
            res = self._sub(t1, t2)
        else:  # n = 2m: psi_m (psi_{m+2} psi_{m-1}^2 - psi_{m-2} psi_{m+1}^2) / (2y)
            u1 = self._mul(g(m + 2), self._mul(g(m - 1), g(m - 1)))
            u2 = self._mul(g(m - 2), self._mul(g(m + 1), g(m + 1)))
            br = self._sub(u1, u2)
            prod = self._mul(g(m), br)
            P, e = prod
            inv2 = F.inv(F.const(2))
            if e == 1:  # P*y / (2y) = P/2
                res = (pscale(F, P, inv2), 0)
            else:  # P / (2y) = (P / (2 f)) * y
                quo, rem = pdivmod(F, P, self.fx)
                assert not rem, "psi_2m: f does not divide the numerator"
                res = (pscale(F, quo, inv2), 1)
        self.cache[n] = res
        return res

    def psi_x(self, ell: int) -> list[int]:
        """The x-polynomial of psi_ell for odd ell (the l-torsion x-coordinates)."""
        P, e = self.psi(ell)
        assert e == 0 and ell % 2 == 1
        return P


# ------------------------------------------------------------- rational kernels
def _delta_stable(F: Fq, a: int, b: int, h: list[int]) -> bool:
    """Is the root set of h closed under x -> x(2P)?  (h monic, squarefree)"""
    d = pdeg(h)
    A = pmul(F, psub(F, [0, 0, 1], [a]), psub(F, [0, 0, 1], [a]))  # (x^2 - a)^2
    A = psub(F, A, pscale(F, [0, 1], F.mul(F.const(8), b)))
    B = pscale(F, [b, a, 0, 1], F.const(4))
    powA = [[1]]
    powB = [[1]]
    for _ in range(d):
        powA.append(pmul(F, powA[-1], A))
        powB.append(pmul(F, powB[-1], B))
    acc: list[int] = []
    for i, c in enumerate(h):
        if c == 0:
            continue
        acc = padd(F, acc, pscale(F, pmul(F, powA[i], powB[d - i]), c))
    return not pmod(F, acc, h)


def rational_kernels(F: Fq, a: int, b: int, ell: int, rng=None, dp: DivPolys | None = None):
    """F_q-rational cyclic subgroups of order ell, as kernel polynomials (monic, degree
    (ell-1)/2, product over the x-coordinates of one point from each +-pair; for ell = 2
    the linear polynomial x - x0).  Also returns the degree multiset of the factors of
    psi_ell (the witness that makes the count re-checkable)."""
    rng = rng or random.Random(12345)
    if ell == 2:
        fx = [b, a, 0, 1]
        facs = factor_squarefree(F, fx, rng)  # x^3 + a x + b is squarefree (nonsingular)
        roots = [F.neg(fa[0]) for fa, dd in facs if dd == 1]
        return [[F.neg(r), 1] for r in roots], {"psi_factor_degrees": sorted(dd for _, dd in facs)}
    d = (ell - 1) // 2
    dp = dp or DivPolys(F, a, b)
    psi = pmonic(F, dp.psi_x(ell))
    assert pdeg(psi) == (ell * ell - 1) // 2
    facs = factor_squarefree(F, psi, rng)
    witness = {"psi_factor_degrees": sorted(dd for _, dd in facs)}
    cand = [fa for fa, dd in facs if dd <= d]
    out: list[list[int]] = []

    def dfs(start: int, deg: int, prod: list[int]):
        if deg == d:
            if _delta_stable(F, a, b, prod):
                out.append(pmonic(F, prod))
            return
        for i in range(start, len(cand)):
            dd = pdeg(cand[i])
            if deg + dd <= d:
                dfs(i + 1, deg + dd, pmul(F, prod, cand[i]) if prod != [1] else cand[i])

    dfs(0, 0, [1])
    return out, witness


def velu_image(F: Fq, a: int, b: int, h: list[int], ell: int) -> tuple[int, int]:
    """Image curve (a', b') of E : y^2 = x^3 + a x + b modulo the subgroup with kernel
    polynomial h (Velu).  Uses only power sums p1..p3 of the roots of h."""
    d = pdeg(h)
    c = F.const
    # elementary symmetric functions e_k = (-1)^k h_{d-k}
    def e(k):
        if k > d:
            return 0
        v = h[d - k]
        return v if k % 2 == 0 else F.neg(v)

    e1, e2, e3 = e(1), e(2), e(3)
    p1 = e1
    p2 = F.sub(F.mul(e1, p1), F.mul(c(2), e2))
    p3 = F.add(F.sub(F.mul(e1, p2), F.mul(e2, p1)), F.mul(c(3), e3))
    if ell == 2:
        x0 = F.neg(h[0])
        t = F.add(F.mul(c(3), F.sq(x0)), a)
        return F.sub(a, F.mul(c(5), t)), F.sub(b, F.mul(c(7), F.mul(x0, t)))
    t = F.add(F.mul(c(6), p2), F.mul(F.mul(c(2), a), c(d)))
    w = F.add(F.add(F.mul(c(10), p3), F.mul(F.mul(c(6), a), p1)), F.mul(F.mul(c(4), b), c(d)))
    return F.sub(a, F.mul(c(5), t)), F.sub(b, F.mul(c(7), w))


# ------------------------------------------------------------------- the volcano
def neighbours(F: Fq, a: int, b: int, ell: int, rng, expected_N: int, counter: Counter):
    """Rational l-isogenies from (a, b): list of dicts with kernel and image, and the
    kernel count.  Every image is checked to have #E' = expected_N."""
    dp = DivPolys(F, a, b) if ell != 2 else None
    ks, witness = rational_kernels(F, a, b, ell, rng, dp)
    out = []
    for h in ks:
        a2, b2 = velu_image(F, a, b, h, ell)
        N2 = counter.count(a2, b2)
        out.append({"kernel": h, "a": a2, "b": b2, "j": j_invariant(F, a2, b2), "N_image": N2,
                    "N_ok": N2 == expected_N})
    return out, witness


def volcano_height(F: Fq, a: int, b: int, ell: int, depth: int, expected_N: int,
                   counter: Counter, deadline: float | None = None, max_vertices: int = 400,
                   rng=None):
    """Height of (a, b) above the l-floor by BFS over j in the rational l-isogeny graph.

    Floor vertex iff exactly one rational l-isogeny (l | f_pi).  The BFS distance to the
    nearest floor vertex is the height h; v_l(f_E) = depth - h.  Returns a dict with the
    height, the explicit path and every edge computed.
    """
    rng = rng or random.Random(777)
    j0 = j_invariant(F, a, b)
    models = {j0: (a, b)}
    dist = {j0: 0}
    parent = {j0: None}
    queue = [j0]
    edges = []
    anomalies = []
    counts = {}
    qi = 0
    while qi < len(queue):
        if deadline is not None and time.perf_counter() > deadline:
            return {"status": "timeout", "edges": edges, "counts": counts}
        if len(queue) > max_vertices:
            return {"status": "vertex_cap", "edges": edges, "counts": counts}
        jv = queue[qi]
        qi += 1
        av, bv = models[jv]
        nb, witness = neighbours(F, av, bv, ell, rng, expected_N, counter)
        counts[jv] = {"kernels": len(nb), **witness}
        if len(nb) not in (1, ell + 1):
            anomalies.append({"j": jv, "kernels": len(nb)})
        for k in nb:
            edges.append({"ell": ell, "from_j": jv, "from_a": av, "from_b": bv,
                          "kernel_poly": k["kernel"], "to_j": k["j"], "to_a": k["a"],
                          "to_b": k["b"], "N_image": k["N_image"], "N_ok": k["N_ok"],
                          "from_dist": dist[jv], "from_kernel_count": len(nb),
                          "label": "unclassified"})
        if len(nb) == 1:  # floor reached at BFS distance dist[jv]
            path = []
            cur = jv
            while cur is not None:
                path.append(cur)
                cur = parent[cur]
            path.reverse()
            on_path = set(zip(path[:-1], path[1:]))
            for e in edges:
                if (e["from_j"], e["to_j"]) in on_path and e["ell"] == ell:
                    e["label"] = "descending"
            return {"status": "ok", "height": dist[jv], "floor_j": jv, "path_j": path,
                    "edges": edges, "counts": counts, "anomalies": anomalies,
                    "vertices_expanded": qi}
        for k in nb:
            if k["j"] not in dist:
                dist[k["j"]] = dist[jv] + 1
                parent[k["j"]] = jv
                models[k["j"]] = (k["a"], k["b"])
                queue.append(k["j"])
    return {"status": "no_floor", "edges": edges, "counts": counts, "anomalies": anomalies}


# --------------------------------------------------- group structure (Method B)
def torsion_count(C: Curve, pts: list, k: int) -> int:
    """#{P in E(F_q) : [k]P = O}."""
    return sum(1 for P in pts if C.mul(k, P) is None)


def ell_part_of_n1(C: Curve, pts: list, ell: int, vmax: int) -> int:
    """Exact v_ell(n1) where E(F_q) ~ Z/n1 x Z/n2: largest j <= vmax with E[ell^j] inside E(F_q)."""
    best = 0
    for j in range(1, vmax + 1):
        if torsion_count(C, pts, ell ** j) == ell ** (2 * j):
            best = j
        else:
            break
    return best


def vp(n: int, p: int) -> int:
    """p-adic valuation (n = 0 gives a large number)."""
    if n == 0:
        return 10 ** 6
    v = 0
    while n % p == 0:
        n //= p
        v += 1
    return v


def predicted_vn1(x: int, f_pi: int, v_fE: int, ell: int) -> int:
    """v_ell of gcd(x - 1, f_pi / f_E) for a candidate v_ell(f_E) = v_fE."""
    return min(vp(x - 1, ell), vp(f_pi, ell) - v_fE)


def frobenius_x(t: int, f_pi: int, d_k: int) -> int:
    """pi = x + f_pi*omega_K with omega_K = (D_K + sqrt(D_K))/2: x = (t - f_pi*D_K)/2."""
    num = t - f_pi * d_k
    assert num % 2 == 0
    return num // 2
