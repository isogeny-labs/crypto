#!/usr/bin/env python3
"""L11: memory-bounded Diem double-large-prime relation collection on plane quartics.
Implements SPEC.md. Python 3 standard library only, single thread."""
import argparse
import glob
import hashlib
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(HERE, "raw")

M64 = 2 ** 64 - 1


def mix64(z):
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


class SM:
    def __init__(self, seed):
        self.s = seed & M64
        self.draws = 0

    def next(self):
        self.s = (self.s + 0x9E3779B97F4A7C15) & M64
        self.draws += 1
        return mix64(self.s)


def shuffle(lst, sm):
    for i in range(len(lst) - 1, 0, -1):
        j = sm.next() % (i + 1)
        lst[i], lst[j] = lst[j], lst[i]


def mons(d):
    return [(a, b, d - a - b) for a in range(d, -1, -1) for b in range(d - a, -1, -1)]


M4 = mons(4)
M3 = mons(3)
M2 = mons(2)
M3I = {m: i for i, m in enumerate(M3)}
M2I = {m: i for i, m in enumerate(M2)}
M4I = {m: i for i, m in enumerate(M4)}

PRIMES = {8: 263, 10: 1031, 12: 4099, 14: 16411}


def l11_sha():
    with open(os.path.join(HERE, "l11.py"), "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def common():
    return {"python_version": sys.version.split()[0], "l11_sha256": l11_sha()}


def s_curve(k, i):
    return 0x4C31310000000000 ^ (k << 32) ^ i


# ---------------------------------------------------------------- polynomials
def ptrim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def padd(a, b, p):
    n = max(len(a), len(b))
    r = [((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)]
    return ptrim(r)


def psub(a, b, p):
    n = max(len(a), len(b))
    r = [((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)]
    return ptrim(r)


def pmul(a, b, p):
    if not a or not b:
        return []
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                r[i + j] += x * y
    return ptrim([c % p for c in r])


def pmonic(a, p):
    a = ptrim(list(a))
    if not a:
        return a
    iv = pow(a[-1], p - 2, p)
    return [c * iv % p for c in a]


def pdivmod(a, b, p):
    a = ptrim(list(a))
    b = ptrim(list(b))
    if not b:
        raise ZeroDivisionError("poly div by zero")
    db = len(b) - 1
    iv = pow(b[-1], p - 2, p)
    if len(a) - 1 < db:
        return [], a
    q = [0] * (len(a) - db)
    for i in range(len(a) - 1, db - 1, -1):
        c = a[i] * iv % p
        q[i - db] = c
        if c:
            for j in range(db + 1):
                a[i - db + j] = (a[i - db + j] - c * b[j]) % p
    return ptrim(q), ptrim(a[:db])


def pgcd(a, b, p):
    a = ptrim(list(a))
    b = ptrim(list(b))
    while b:
        a, b = b, pdivmod(a, b, p)[1]
    return pmonic(a, p)


def pmulmod(a, b, m, p):
    return pdivmod(pmul(a, b, p), m, p)[1]


def ppowmod(base, e, m, p):
    result = [1]
    b = pdivmod(base, m, p)[1]
    while e:
        if e & 1:
            result = pmulmod(result, b, m, p)
        b = pmulmod(b, b, m, p)
        e >>= 1
    return result


def deg(a):
    return len(a) - 1  # zero poly has deg -1


def roots(g, x0, p):
    g = ptrim(list(g))
    if not g:
        raise ValueError("roots of zero polynomial at x0=%s" % x0)
    if deg(g) == 0:
        return []
    g = pmonic(g, p)
    xp = ppowmod([0, 1], p, g, p)
    h = pgcd(g, psub(xp, [0, 1], p), p)
    sm = SM(mix64((p << 32) ^ x0 ^ 0xE7))

    def split(h):
        d = deg(h)
        if d <= 0:
            return []
        if d == 1:
            return [(-h[0]) % p]
        while True:
            dd = sm.next() % p
            w = pgcd(h, psub(ppowmod([dd, 1], (p - 1) // 2, h, p), [1], p), p)
            if 0 < deg(w) < deg(h):
                return split(w) + split(pdivmod(h, w, p)[0])

    return sorted(split(h))


def normalize(P, p):
    for c in P:
        if c % p:
            iv = pow(c, p - 2, p)
            return tuple(x * iv % p for x in P)
    raise ValueError("zero point")


# ---------------------------------------------------------------- curve
class Curve:
    def __init__(self, p, f):
        self.p = p
        self.f = list(f)
        self.e = (p + 1) // 4
        self.GT = [[0] * 10 for _ in range(3)]
        for i, mon in enumerate(M4):
            for k in range(3):
                if mon[k] > 0:
                    sub = list(mon)
                    sub[k] -= 1
                    m = M3I[tuple(sub)]
                    self.GT[k][m] = (self.GT[k][m] + f[i] * mon[k]) % p
        self.HT = [[0] * 6 for _ in range(6)]
        for i, mon in enumerate(M4):
            for s, q in enumerate(M2):
                if all(mon[t] >= q[t] for t in range(3)):
                    rest = tuple(mon[t] - q[t] for t in range(3))
                    coef = f[i] * math.comb(mon[0], q[0]) * math.comb(mon[1], q[1]) * math.comb(mon[2], q[2])
                    m = M2I[rest]
                    self.HT[s][m] = (self.HT[s][m] + coef) % p
        self._points = None

    def F_eval(self, P):
        p = self.p
        x, y, z = P
        t = 0
        for i, (a, b, c) in enumerate(M4):
            if self.f[i]:
                t += self.f[i] * pow(x, a, p) * pow(y, b, p) * pow(z, c, p)
        return t % p

    # ---- smoothness (uncounted)
    def _G_poly_y(self, G, x0):
        """G cubic coefficient vector over M3; polynomial in y of G(x0,y,1)."""
        p = self.p
        co = [0] * 4
        for m, mon in enumerate(M3):
            if G[m]:
                co[mon[1]] = (co[mon[1]] + G[m] * pow(x0, mon[0], p)) % p
        return co

    def is_smooth(self):
        p = self.p
        f = self.f
        GT = self.GT
        # F(x,1,0) must not be the zero polynomial (a line in the curve is singular anyway)
        if not any(f[i] for i, mon in enumerate(M4) if mon[2] == 0):
            return False
        ok = False
        for t in range(3):
            sm = SM(mix64((p << 8) ^ t ^ 0x53))
            ls = []
            for _ in range(3):
                ls.append((sm.next() % p, sm.next() % p))
            Gs = []
            for (lq, mq) in ls:
                Gs.append([(GT[0][m] + lq * GT[1][m] + mq * GT[2][m]) % p for m in range(10)])
            R = []
            for (A, B) in ((Gs[0], Gs[1]), (Gs[0], Gs[2])):
                vals = []
                for x0 in range(10):
                    a = self._G_poly_y(A, x0)[::-1]  # [a3,a2,a1,a0]
                    bb = self._G_poly_y(B, x0)[::-1]
                    mat = []
                    for sh in range(3):
                        mat.append([0] * sh + a + [0] * (2 - sh))
                    for sh in range(3):
                        mat.append([0] * sh + bb + [0] * (2 - sh))
                    vals.append(det_mod(mat, p))
                R.append(lagrange(vals, p))
            if R[0] and R[1] and deg(pgcd(R[0], R[1], p)) == 0:
                ok = True
                break
        if not ok:
            return False
        # line at infinity
        i100 = M3I[(3, 0, 0)]
        if GT[0][i100] == 0 and GT[1][i100] == 0 and GT[2][i100] == 0:
            return False
        us = []
        for k in range(3):
            u = [0] * 4
            for m, mon in enumerate(M3):
                if mon[2] == 0:
                    u[mon[0]] = (u[mon[0]] + GT[k][m]) % p
            us.append(ptrim(u))
        nz = [u for u in us if u]
        if not nz:
            return False
        g = nz[0]
        for u in nz[1:]:
            g = pgcd(g, u, p)
        g = pgcd(g, g, p)
        if deg(g) >= 1:
            return False
        return True

    # ---- enumeration (uncounted)
    def points(self):
        if self._points is not None:
            return self._points
        p = self.p
        f = self.f
        pts = set()
        byx = [[(a, b, f[i]) for i, (a, b, c) in enumerate(M4) if f[i] and b == j] for j in range(5)]
        pw = None
        for x0 in range(p):
            co = [0] * 5
            xs = [1, x0 % p, x0 * x0 % p, pow(x0, 3, p), pow(x0, 4, p)]
            for j in range(5):
                s = 0
                for (a, b, fv) in byx[j]:
                    s += fv * xs[a]
                co[j] = s % p
            for y in roots(co, x0, p):
                pts.add(normalize((x0, y, 1), p))
        # infinity: F(x,1,0)
        co = [0] * 5
        for i, (a, b, c) in enumerate(M4):
            if c == 0:
                co[a] = (co[a] + f[i]) % p
        for x in roots(co, p, p):
            pts.add(normalize((x, 1, 0), p))
        if f[M4I[(4, 0, 0)]] == 0:
            pts.add((1, 0, 0))
        res = sorted(pts)
        n1 = len(res)
        assert abs(n1 - p - 1) <= 6 * math.sqrt(p), "Hasse-Weil violated"
        self._points = res
        return res


def det_mod(mat, p):
    n = len(mat)
    a = [r[:] for r in mat]
    det = 1
    for c in range(n):
        piv = None
        for r in range(c, n):
            if a[r][c] % p:
                piv = r
                break
        if piv is None:
            return 0
        if piv != c:
            a[c], a[piv] = a[piv], a[c]
            det = -det
        det = det * a[c][c] % p
        iv = pow(a[c][c], p - 2, p)
        for r in range(c + 1, n):
            if a[r][c] % p:
                fct = a[r][c] * iv % p
                a[r] = [(a[r][j] - fct * a[c][j]) % p for j in range(n)]
    return det % p


def lagrange(vals, p):
    n = len(vals)
    res = []
    for i in range(n):
        num = [1]
        den = 1
        for j in range(n):
            if j != i:
                num = pmul(num, [(-j) % p, 1], p)
                den = den * (i - j) % p
        c = vals[i] * pow(den, p - 2, p) % p
        res = padd(res, [x * c % p for x in num], p)
    return res


# ---------------------------------------------------------------- counted arithmetic
class Fp:
    def __init__(self, p):
        self.p = p
        self.m = 0
        self.i = 0

    def mul(self, a, b):
        self.m += 1
        return a * b % self.p

    def inv(self, a):
        self.i += 1
        return pow(a, self.p - 2, self.p)

    def powc(self, a, e):
        r = a
        for bit in bin(e)[3:]:
            r = self.mul(r, r)
            if bit == "1":
                r = self.mul(r, a)
        return r


class Lines:
    """Taylor cache and line routine for one curve and one counter."""

    def __init__(self, cv, fp):
        self.cv = cv
        self.fp = fp
        self.cache = {}
        self.p = cv.p

    def taylor(self, P):
        t = self.cache.get(P)
        if t is not None:
            return t
        mul = self.fp.mul
        p = self.p
        cv = self.cv
        v2 = []
        for mon in M2:
            cl = [0] * mon[0] + [1] * mon[1] + [2] * mon[2]
            v2.append(mul(P[cl[0]], P[cl[1]]))
        v3 = []
        for mon in M3:
            c = next(k for k in range(3) if mon[k] > 0)
            sub = list(mon)
            sub[c] -= 1
            v3.append(mul(v2[M2I[tuple(sub)]], P[c]))
        g = []
        for k in range(3):
            s = 0
            for m in range(10):
                s += mul(cv.GT[k][m], v3[m])
            g.append(s % p)
        hq = []
        for s_ in range(6):
            s = 0
            for m in range(6):
                s += mul(cv.HT[s_][m], v2[m])
            hq.append(s % p)
        t = (g, hq, v2)
        self.cache[P] = t
        return t

    def line(self, Pi, Pj):
        fp = self.fp
        mul = fp.mul
        p = self.p
        gi, hi, _ = self.taylor(Pi)
        gj, _, v2j = self.taylor(Pj)
        c1 = 0
        for k in range(3):
            c1 += mul(gi[k], Pj[k])
        c3 = 0
        for k in range(3):
            c3 += mul(gj[k], Pi[k])
        c1 %= p
        c3 %= p
        if c1 == 0 or c3 == 0:
            return ("TANGENT", None, None)
        c2 = 0
        for s in range(6):
            c2 += mul(hi[s], v2j[s])
        c2 %= p
        disc = (mul(c2, c2) - mul(mul(c1, c3), 4)) % p
        if disc == 0:
            return ("TANGENT", None, None)
        d = fp.powc(disc, self.cv.e)
        if mul(d, d) != disc:
            return ("NOSPLIT", None, None)
        mu = 2 * c1 % p
        l1 = (d - c2) % p
        l2 = (-c2 - d) % p
        mq = [mul(mu, Pj[k]) for k in range(3)]
        out = []
        for l in (l1, l2):
            rr = [(mul(l, Pi[k]) + mq[k]) % p for k in range(3)]
            i0 = None
            for k in range(3):
                if rr[k] != 0:
                    i0 = k
                    break
            if i0 is None:
                return ("BAD", None, None)
            iv = fp.inv(rr[i0])
            R = [0, 0, 0]
            R[i0] = 1
            for k in range(3):
                if k != i0:
                    R[k] = mul(rr[k], iv)
            out.append(tuple(R))
        return ("SPLIT", out[0], out[1])


# ---------------------------------------------------------------- sizes, pairs
def isqrt_ceil_ge(target):
    """smallest b with b*b >= target"""
    b = math.isqrt(target)
    if b * b < target:
        b += 1
    return b


def icbrt_ceil_ge(target):
    b = int(round(target ** (1.0 / 3)))
    while b ** 3 < target:
        b += 1
    while b > 1 and (b - 1) ** 3 >= target:
        b -= 1
    return b


def b_max_rule(p):
    return max(isqrt_ceil_ge(72 * p), icbrt_ceil_ge(8 * p * p))


def pair_of(t):
    i = (1 + math.isqrt(1 + 8 * t)) // 2
    while i * (i - 1) // 2 > t:
        i -= 1
    while (i + 1) * i // 2 <= t:
        i += 1
    return i, t - i * (i - 1) // 2


# ---------------------------------------------------------------- graph
class Graph:
    def __init__(self):
        self.parent = [0]
        self.verts = [1]
        self.edges = [0]
        self.find_steps = 0

    def new_node(self):
        n = len(self.parent)
        self.parent.append(n)
        self.verts.append(1)
        self.edges.append(0)
        return n

    def find(self, x):
        par = self.parent
        while par[x] != x:
            par[x] = par[par[x]]
            x = par[x]
            self.find_steps += 1
        return x

    def add_edge(self, u, v):
        ru, rv = self.find(u), self.find(v)
        if ru == rv:
            self.edges[ru] += 1
            return
        if self.verts[ru] > self.verts[rv] or (self.verts[ru] == self.verts[rv] and ru < rv):
            w, l = ru, rv
        else:
            w, l = rv, ru
        self.parent[l] = w
        self.verts[w] = self.verts[ru] + self.verts[rv]
        self.edges[w] = self.edges[ru] + self.edges[rv] + 1

    def cyc(self):
        r0 = self.find(0)
        return self.edges[r0] - self.verts[r0] + 1


def g_admit(R1, R2, vid):
    return R1 in vid or R2 in vid


# ---------------------------------------------------------------- rank
RANKL = 2 ** 61 - 1


def rank_rows(rows):
    l = RANKL
    piv = {}
    for row in rows:
        row = {c: v % l for c, v in row.items() if v % l}
        while row:
            c = min(row)
            if c in piv:
                fct = row[c]
                pr = piv[c]
                for cc, vv in pr.items():
                    nv = (row.get(cc, 0) - fct * vv) % l
                    if nv:
                        row[cc] = nv
                    else:
                        row.pop(cc, None)
            else:
                iv = pow(row[c], l - 2, l)
                piv[c] = {cc: vv * iv % l for cc, vv in row.items()}
                break
    return len(piv)


# ---------------------------------------------------------------- arms
def _c_rule(c):
    return lambda p: isqrt_ceil_ge(c * c * p)


def _as_rule(D):
    return lambda p: isqrt_ceil_ge(9 * p * D)


ARM_ORDER = ["U", "U2", "U4", "UAA", "AF1", "AF2", "AF4", "AF8", "AS2", "AS4", "AS8", "G", "S"]
ARMS = {
    "U2": dict(b=_c_rule(2), alpha=None, mode="all"),
    "U": dict(b=_c_rule(3), alpha=None, mode="all"),
    "U4": dict(b=_c_rule(4), alpha=None, mode="all"),
    "UAA": dict(b=_c_rule(3), alpha=None, mode="all", salt=0xAA),
    "AF1": dict(b=_c_rule(3), alpha=1.0, mode="adm", af=True),
    "AF2": dict(b=_c_rule(3), alpha=0.5, mode="adm", af=True),
    "AF4": dict(b=_c_rule(3), alpha=0.25, mode="adm", af=True),
    "AF8": dict(b=_c_rule(3), alpha=0.125, mode="adm", af=True),
    "AS2": dict(b=_as_rule(2), alpha=0.5, mode="adm"),
    "AS4": dict(b=_as_rule(4), alpha=0.25, mode="adm"),
    "AS8": dict(b=_as_rule(8), alpha=0.125, mode="adm"),
    "G": dict(b=_c_rule(3), alpha=None, mode="G"),
    "S": dict(b=lambda p: icbrt_ceil_ge(8 * p * p), alpha=None, mode="S"),
}


def run_cell(cv, arm, k, i, sc, pts=None):
    cfg = ARMS[arm]
    p = cv.p
    pts = cv.points() if pts is None else pts
    N1 = len(pts)
    b = cfg["b"](p)
    salt = cfg.get("salt", 0)
    alpha = cfg["alpha"]
    mode = cfg["mode"]
    seed_base = mix64(sc ^ 0xBA5E000000000000 ^ (b << 8) ^ salt)
    seed_pair = mix64(sc ^ 0x9A12000000000000 ^ (b << 8) ^ salt)
    seed_g1 = mix64(seed_pair ^ 0x5A3)
    smb = SM(seed_base)
    order = list(range(N1))
    shuffle(order, smb)
    base = [pts[t] for t in order[:b]]
    P0 = base[0]
    base_set = set(base)
    colmap = {base[t]: t - 1 for t in range(1, b)}
    hcol = b - 1
    NP = b * (b - 1) // 2
    smp = SM(seed_pair)
    idx = list(range(NP))
    shuffle(idx, smp)

    fp = Fp(p)
    L = Lines(cv, fp)
    for P in base:
        L.taylor(P)
    mul_setup, inv_setup = fp.m, fp.i
    fp.m = 0
    fp.i = 0

    thr = None
    if alpha is not None and alpha < 1.0:
        thr = int(alpha * 2 ** 64)
    af_census = bool(cfg.get("af")) and alpha is not None and alpha < 1.0

    vid = {}
    G = Graph()
    dedup = set()
    stored = []
    Fr = 0
    E = 0
    target = b + 8 + b // 100
    cen = dict(tangent=0, nosplit=0, bad=0, split=0, full=0, lp1=0, lp2=0, rejected_admission=0, dup=0)
    seen_lp = {}
    dict_lookups = 0
    T = 0
    count = 0

    def adm(R):
        if alpha is None or alpha >= 1.0:
            return True
        x, y, z = R
        return mix64(((x << 42) | (y << 21) | z) ^ sc) < thr

    for t in idx:
        T += 1
        ii, jj = pair_of(t)
        Pi, Pj = base[ii], base[jj]
        kind, R1, R2 = L.line(Pi, Pj)
        if kind == "TANGENT":
            cen["tangent"] += 1
            continue
        if kind == "NOSPLIT":
            cen["nosplit"] += 1
            continue
        if kind == "BAD":
            cen["bad"] += 1
            continue
        cen["split"] += 1
        dict_lookups += 2
        lps = [R for R in (R1, R2) if R not in base_set]
        nlp = len(lps)
        if nlp == 0:
            cen["full"] += 1
        elif nlp == 1:
            cen["lp1"] += 1
        else:
            cen["lp2"] += 1
        if af_census:
            for R in lps:
                if R not in seen_lp:
                    seen_lp[R] = adm(R)
        # admission
        if mode == "all":
            ok = True
        elif mode == "adm":
            ok = all(adm(R) for R in lps)
        elif mode == "G":
            if nlp <= 1:
                ok = True
            else:
                dict_lookups += 2
                ok = g_admit(R1, R2, vid)
        else:  # S
            ok = nlp <= 1
        if not ok:
            cen["rejected_admission"] += 1
            continue
        if nlp < 2:
            key = tuple(sorted([Pi, Pj, R1, R2]))
            dict_lookups += 1
            if key in dedup:
                cen["dup"] += 1
                continue
            dedup.add(key)
        if nlp == 0:
            Fr += 1
            stored.append((Pi, Pj, R1, R2, 0, 0, 0))
        else:
            def node(R):
                nonlocal dict_lookups
                dict_lookups += 1
                n = vid.get(R)
                if n is None:
                    n = G.new_node()
                    vid[R] = n
                return n
            u = node(lps[0])
            v = node(lps[1]) if nlp == 2 else 0
            E += 1
            G.add_edge(u, v)
            stored.append((Pi, Pj, R1, R2, nlp, u, v))
        count = Fr + G.cyc()
        if count >= target:
            break
    exhausted = count < target
    V = len(vid) + 1
    D = len(dedup)
    lp_records = V + E
    mem_total = V + E + Fr + D + b

    # rows
    adj = {}
    for e, st in enumerate(stored):
        if st[4] >= 1:
            adj.setdefault(st[5], []).append((st[6], e))
            adj.setdefault(st[6], []).append((st[5], e))

    def A_of(st):
        A = {hcol: 1}
        for Pt in st[:4]:
            c = colmap.get(Pt)
            if c is not None:
                A[c] = A.get(c, 0) - 1
        return A

    rows = []
    expr = {0: {}}
    used = set()
    queue = [0]
    qi = 0
    while qi < len(queue):
        u = queue[qi]
        qi += 1
        for (w, e) in adj.get(u, ()):
            if e in used:
                continue
            used.add(e)
            A = A_of(stored[e])
            if w not in expr:
                ex = dict(A)
                for c, v in expr[u].items():
                    ex[c] = ex.get(c, 0) - v
                expr[w] = {c: v for c, v in ex.items() if v}
                queue.append(w)
            else:
                row = dict(expr[u])
                for c, v in expr[w].items():
                    row[c] = row.get(c, 0) + v
                for c, v in A.items():
                    row[c] = row.get(c, 0) - v
                rows.append({c: v for c, v in row.items() if v})
    cycle_rows = len(rows)
    for st in stored:
        if st[4] == 0:
            row = {}
            for Pt in st[:4]:
                c = colmap.get(Pt)
                if c is not None:
                    row[c] = row.get(c, 0) + 1
            row[hcol] = row.get(hcol, 0) - 1
            rows.append(row)
    zero_rows = sum(1 for r in rows if not r)
    nnz = sum(len(r) for r in rows)
    rank = rank_rows(rows) if k <= 10 else None

    # G1
    smg = SM(seed_g1)
    if k <= 10:
        sample = list(range(len(stored)))
    elif stored:
        sample = [smg.next() % len(stored) for _ in range(200)]
    else:
        sample = []
    g1_failures = 0
    for e in sample:
        Pi, Pj, R1, R2 = stored[e][:4]
        if len({Pi, Pj, R1, R2}) != 4:
            g1_failures += 1
            continue
        bad = False
        for Pt in (Pi, Pj, R1, R2):
            if cv.F_eval(Pt) != 0:
                bad = True
        for R in (R1, R2):
            if det_mod([list(Pi), list(Pj), list(R)], p) != 0:
                bad = True
        if bad:
            g1_failures += 1

    rec = {
        "lead": "L11", "k": k, "p": p, "curve": i, "s_curve": sc, "arm": arm, "alpha": alpha,
        "b": b, "salt": salt, "target": target, "T": T, "supply": NP, "exhausted": exhausted,
        "mul_setup": mul_setup, "inv_setup": inv_setup, "mul_lines": fp.m, "inv_lines": fp.i,
        "FPM_setup": mul_setup + 10 * inv_setup, "FPM_lines": fp.m + 10 * fp.i,
        "FPM_total": mul_setup + 10 * inv_setup + fp.m + 10 * fp.i,
        "count": count, "Fr": Fr, "V": V, "E": E, "D": D, "lp_records": lp_records,
        "mem_total": mem_total, "census": cen,
        "adm_seen": len(seen_lp) if af_census else None,
        "adm_admissible": sum(1 for v in seen_lp.values() if v) if af_census else None,
        "rows": len(rows), "cycle_rows": cycle_rows, "zero_rows": zero_rows, "nnz": nnz,
        "b_nnz": b * nnz, "rank": rank, "g1_checked": len(sample), "g1_failures": g1_failures,
        "unpriced": {"dict_lookups": dict_lookups, "find_steps": G.find_steps,
                     "rng_draws": smb.draws + smp.draws + smg.draws},
    }
    rec.update(common())
    return rec


# ---------------------------------------------------------------- curve generation
def gen_curve(K, i, with_points=True):
    p = PRIMES[K]
    bmax = b_max_rule(p)
    sc = s_curve(K, i)
    sm = SM(sc)
    j = 0
    while True:
        f = [sm.next() % p for _ in range(15)]
        cv = Curve(p, f)
        if cv.is_smooth():
            n1 = len(cv.points())
            if n1 >= bmax + 10:
                return cv, j + 1, sc
        j += 1


def parse_range(s, default_n=12):
    if s is None:
        return list(range(default_n))
    a, b = s.split("-")
    return list(range(int(a), int(b) + 1))


def load_curves_file(K):
    path = os.path.join(RAW, "curves_k%d.jsonl" % K)
    out = {}
    with open(path) as fh:
        for line in fh:
            if line.strip():
                r = json.loads(line)
                out[r["i"]] = r
    return out


def cmd_curves(args):
    K = args.k
    os.makedirs(RAW, exist_ok=True)
    path = os.path.join(RAW, "curves_k%d.jsonl" % K)
    existing = {}
    if os.path.exists(path):
        existing = load_curves_file(K)
    for i in parse_range(args.curves):
        cv, att, sc = gen_curve(K, i)
        pts = cv.points()
        pf = "points_k%d_%d.json" % (K, i)
        data = json.dumps({"p": cv.p, "f": cv.f, "points": [list(x) for x in pts]}, separators=(",", ":"))
        with open(os.path.join(RAW, pf), "w") as fh:
            fh.write(data)
        rec = {"k": K, "p": cv.p, "i": i, "s_curve": sc, "attempts": att, "f": cv.f, "N1": len(pts),
               "points_file": pf, "points_sha256": hashlib.sha256(data.encode()).hexdigest()}
        rec.update(common())
        existing[i] = rec
        print("k=%d curve %d attempts=%d N1=%d" % (K, i, att, len(pts)), flush=True)
    with open(path, "w") as fh:
        for i in sorted(existing):
            fh.write(json.dumps(existing[i]) + "\n")


def cmd_run(args):
    K = args.k
    arms = ARM_ORDER if args.arms == "all" else [a for a in ARM_ORDER if a in args.arms.split(",")]
    recs = load_curves_file(K)
    with open(args.out, "a") as out:
        for i in parse_range(args.curves):
            r = recs[i]
            with open(os.path.join(RAW, r["points_file"]), "rb") as fh:
                raw = fh.read()
            assert hashlib.sha256(raw).hexdigest() == r["points_sha256"], "points sha mismatch"
            d = json.loads(raw)
            cv = Curve(d["p"], d["f"])
            pts = [tuple(x) for x in d["points"]]
            for arm in arms:
                rec = run_cell(cv, arm, K, i, r["s_curve"], pts)
                out.write(json.dumps(rec) + "\n")
                out.flush()
                print("k=%d curve=%d %s T=%d FPM=%d mem=%d exh=%s" % (
                    K, i, arm, rec["T"], rec["FPM_total"], rec["mem_total"], rec["exhausted"]), flush=True)


# ---------------------------------------------------------------- gate-pic
def int_echelon(rows, n):
    M = [list(r) for r in rows]
    r = 0
    piv = []
    for c in range(n):
        while True:
            nz = [i for i in range(r, len(M)) if M[i][c] != 0]
            if not nz:
                break
            i0 = min(nz, key=lambda i: abs(M[i][c]))
            M[r], M[i0] = M[i0], M[r]
            for i in range(r + 1, len(M)):
                if M[i][c]:
                    q = M[i][c] // M[r][c]
                    if q:
                        M[i] = [a - q * bb for a, bb in zip(M[i], M[r])]
            if all(M[i][c] == 0 for i in range(r + 1, len(M))):
                break
        if r < len(M) and any(M[i][c] != 0 for i in range(r, len(M))):
            piv.append(abs(M[r][c]))
            r += 1
        else:
            piv.append(0)
    rank = sum(1 for x in piv if x)
    index = None
    if rank == n:
        index = 1
        for x in piv:
            index *= x
    return rank, index


def fermat(p):
    f = [0] * 15
    for mon in ((4, 0, 0), (0, 4, 0), (0, 0, 4)):
        f[M4I[mon]] = 1
    return Curve(p, f)


def n2_fermat(P):
    cnt = {}
    for a in range(P):
        for b in range(P):
            # w = a + b i, i^2 = -1 ; w^2 = (a^2-b^2) + 2ab i
            x2 = ((a * a - b * b) % P, 2 * a * b % P)
            x4 = ((x2[0] * x2[0] - x2[1] * x2[1]) % P, 2 * x2[0] * x2[1] % P)
            cnt[x4] = cnt.get(x4, 0) + 1
    aff = 0
    for a in range(P):
        for b in range(P):
            x2 = ((a * a - b * b) % P, 2 * a * b % P)
            x4 = ((x2[0] * x2[0] - x2[1] * x2[1]) % P, 2 * x2[0] * x2[1] % P)
            tgt = ((-(x4[0] + 1)) % P, (-x4[1]) % P)
            aff += cnt.get(tgt, 0)
    inf = cnt.get((P - 1, 0), 0)
    return aff + inf


def cmd_gate_pic(args):
    P = args.p
    cv = fermat(P)
    assert cv.is_smooth(), "Fermat not smooth"
    pts = cv.points()
    N1 = len(pts)
    assert N1 == P + 1, "N1 != P+1"
    N2 = n2_fermat(P)
    N2e = P * P + 1 + 6 * P
    jac = (P + 1) ** 3
    fp = Fp(P)
    L = Lines(cv, fp)
    for Pt in pts:
        L.taylor(Pt)
    n = N1
    col = {pts[t]: t - 1 for t in range(1, n)}
    seen = set()
    rows = []
    g1_fail = 0
    g1_checked = 0
    for i in range(1, n):
        for j in range(i):
            kind, R1, R2 = L.line(pts[i], pts[j])
            if kind != "SPLIT":
                continue
            key = tuple(sorted([pts[i], pts[j], R1, R2]))
            if key in seen:
                continue
            seen.add(key)
            g1_checked += 1
            if len(set(key)) != 4 or any(cv.F_eval(x) != 0 for x in key) or any(
                    det_mod([list(pts[i]), list(pts[j]), list(R)], P) != 0 for R in (R1, R2)):
                g1_fail += 1
            row = [0] * n
            for Pt in key:
                c = col.get(Pt)
                if c is not None:
                    row[c] += 1
            row[n - 1] = -1
            rows.append(row)
    rank, index = int_echelon(rows, n)
    res = {"p": P, "N1": N1, "N2": N2, "N2_expected": N2e, "jac": jac, "n": n,
           "relations": len(rows), "rank": rank, "index": index,
           "g1_checked": g1_checked, "g1_failures": g1_fail,
           "pass": bool(N2 == N2e and index == jac and g1_fail == 0)}
    res.update(common())
    print(json.dumps(res))


# ---------------------------------------------------------------- analysis
def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def ols_slope(xs, ys):
    n = len(xs)
    mx = sum(xs) / n
    my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    if sxx == 0:
        return float("nan")
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx


def interval(vals):
    s = sorted(vals)
    return s[50], s[1949]


NB = 2000
BOUNDED = ["AF2", "AF4", "AF8", "AS2", "AS4", "AS8", "G"]


def cmd_analyze(args):
    files = sorted(glob.glob(args.inp))
    runs = []
    for fn in files:
        with open(fn) as fh:
            for line in fh:
                if line.strip():
                    runs.append(json.loads(line))
    seen = set()
    for r in runs:
        key = (r["k"], r["arm"], r["curve"])
        if key in seen:
            sys.exit("duplicate run %s" % (key,))
        seen.add(key)
    R = {}
    for r in runs:
        R.setdefault((r["k"], r["arm"]), {})[r["curve"]] = r
    ks = sorted({r["k"] for r in runs})
    arms_present = [a for a in ARM_ORDER if any(a == r["arm"] for r in runs)]
    gates = {}

    # G0
    try:
        with open(os.path.join(RAW, "unit_tests.txt")) as fh:
            lines = [x for x in fh.read().splitlines() if x.strip()]
        gates["G0"] = {"pass": bool(lines and lines[-1].strip().endswith("OK")), "last_line": lines[-1] if lines else None}
    except OSError as e:
        gates["G0"] = {"pass": False, "error": str(e)}
    # G1
    fails = sum(r["g1_failures"] for r in runs)
    bads = sum(r["census"]["bad"] for r in runs)
    gates["G1"] = {"pass": fails == 0 and bads == 0, "g1_failures": fails, "bad": bads,
                   "g1_checked": sum(r["g1_checked"] for r in runs)}
    # G2
    g2 = {}
    for P in (103, 127):
        try:
            with open(os.path.join(RAW, "gate_pic_p%d.json" % P)) as fh:
                d = json.load(fh)
            g2[str(P)] = {"pass": bool(d.get("pass")), "index": d.get("index"), "jac": d.get("jac"),
                          "relations": d.get("relations")}
        except OSError as e:
            g2[str(P)] = {"pass": False, "error": str(e)}
    gates["G2"] = {"pass": all(v["pass"] for v in g2.values()), "detail": g2}
    # G3
    g3bad = [(r["k"], r["arm"], r["curve"]) for r in runs
             if r["cycle_rows"] != r["count"] - r["Fr"] or r["zero_rows"] > 0.01 * r["rows"]]
    gates["G3"] = {"pass": not g3bad, "violations": g3bad[:20],
                   "total_zero_rows": sum(r["zero_rows"] for r in runs)}
    # G4
    g4 = {}
    for a in arms_present:
        rr = [r for r in runs if r["arm"] == a and r["k"] in (8, 10) and r["rank"] is not None]
        if rr:
            fr = sum(1 for r in rr if r["rank"] >= r["b"] - 1) / len(rr)
            g4[a] = {"n": len(rr), "fraction_rank_ge_b_minus_1": fr, "pass": fr >= 0.95}
    gates["G4"] = {"pass": all(v["pass"] for v in g4.values()), "per_arm": g4}
    # G5
    g5bad = []
    for k in ks:
        u = R.get((k, "U"), {})
        a1 = R.get((k, "AF1"), {})
        for c in sorted(set(u) | set(a1)):
            if c not in u or c not in a1 or any(u[c][m] != a1[c][m] for m in ("T", "FPM_total", "mem_total")):
                g5bad.append((k, c))
    gates["G5"] = {"pass": not g5bad, "violations": g5bad}
    # G6
    g6 = {}
    for a in ("AF2", "AF4", "AF8"):
        for k in ks:
            cells = R.get((k, a), {})
            if not cells:
                continue
            s = sum(r["adm_seen"] for r in cells.values())
            ad = sum(r["adm_admissible"] for r in cells.values())
            al = ARMS[a]["alpha"]
            tol = 4 * math.sqrt(al * (1 - al) / s) if s else float("nan")
            frac = ad / s if s else float("nan")
            g6["%s_k%d" % (a, k)] = {"seen": s, "admissible": ad, "fraction": frac, "alpha": al, "tol": tol,
                                       "pass": bool(s and abs(frac - al) <= tol)}
    gates["G6"] = {"pass": all(v["pass"] for v in g6.values()), "detail": g6}

    # cells
    sm = SM(0x5EED)
    cells = {}
    tsv_lines = []
    hdr = "k\tarm\tb\tn\tn_exhausted\tgm_T\tgm_FPM\tgm_mem\tgm_lp\tr_T\tr_T_lo\tr_T_hi\tr_F\tr_F_lo\tr_F_hi\tr_M\tr_M_lo\tr_M_hi\tgm_nnz\tgm_b_nnz"
    tsv_lines.append(hdr)
    for k in ks:
        for a in ARM_ORDER:
            cur = R.get((k, a))
            if not cur:
                continue
            curves = sorted(cur)
            n = len(curves)
            st = {"k": k, "arm": a, "b": cur[curves[0]]["b"], "n": n,
                  "n_exhausted": sum(1 for c in curves if cur[c]["exhausted"])}
            st["gm_T"] = gm([cur[c]["T"] for c in curves])
            st["gm_FPM"] = gm([cur[c]["FPM_total"] for c in curves])
            st["gm_mem"] = gm([cur[c]["mem_total"] for c in curves])
            st["gm_lp"] = gm([cur[c]["lp_records"] for c in curves])
            st["gm_nnz"] = gm([max(cur[c]["nnz"], 1) for c in curves])
            st["gm_b_nnz"] = gm([max(cur[c]["b_nnz"], 1) for c in curves])
            U = R.get((k, "U"), {})
            pc = [c for c in curves if c in U]
            if pc and len(pc) > 0:
                rat = {m: [cur[c][mm] / U[c][mm] for c in pc]
                       for m, mm in (("T", "T"), ("F", "FPM_total"), ("M", "mem_total"))}
                npc = len(pc)
                for m in "TFM":
                    st["r_" + m] = gm(rat[m])
                boots = {m: [] for m in "TFM"}
                for _ in range(NB):
                    ids = [sm.next() % npc for _ in range(npc)]
                    for m in "TFM":
                        boots[m].append(gm([rat[m][j] for j in ids]))
                for m in "TFM":
                    lo, hi = interval(boots[m])
                    st["r_%s_lo" % m] = lo
                    st["r_%s_hi" % m] = hi
            cells[(k, a)] = st
            f = lambda v: "%.6g" % v if v is not None else ""
            rt = f(st.get("r_T"))
            if st["n_exhausted"] > 0 and rt:
                rt = ">=" + rt
            tsv_lines.append("\t".join([
                str(k), a, str(st["b"]), str(n), str(st["n_exhausted"]), f(st["gm_T"]), f(st["gm_FPM"]),
                f(st["gm_mem"]), f(st["gm_lp"]), rt, f(st.get("r_T_lo")), f(st.get("r_T_hi")),
                f(st.get("r_F")), f(st.get("r_F_lo")), f(st.get("r_F_hi")), f(st.get("r_M")),
                f(st.get("r_M_lo")), f(st.get("r_M_hi")), f(st["gm_nnz"]), f(st["gm_b_nnz"])]))

    # exponents
    METR = (("T", "T"), ("FPM", "FPM_total"), ("mem", "mem_total"))
    lnp = {k: math.log(PRIMES[k]) for k in ks}
    expo = {}
    curve_ids = {k: sorted({c for a in arms_present for c in R.get((k, a), {})}) for k in ks}
    smb = SM(0x5EED ^ 1)
    boot_slopes = {(a, m): [] for a in arms_present for m, _ in METR}
    for _ in range(NB):
        draw = {k: [smb.next() % 12 for _ in range(12)] for k in ks}
        for a in arms_present:
            for m, mm in METR:
                xs, ys = [], []
                for k in ks:
                    cur = R.get((k, a), {})
                    ids = curve_ids[k]
                    for j in draw[k]:
                        c = ids[j % len(ids)]
                        if c in cur:
                            xs.append(lnp[k])
                            ys.append(math.log(cur[c][mm]))
                boot_slopes[(a, m)].append(ols_slope(xs, ys))
    for a in arms_present:
        expo[a] = {}
        for m, mm in METR:
            xs, ys = [], []
            for k in ks:
                for c, r in R.get((k, a), {}).items():
                    xs.append(lnp[k])
                    ys.append(math.log(r[mm]))
            pt = ols_slope(xs, ys)
            lo, hi = interval(boot_slopes[(a, m)])
            d = [x - y for x, y in zip(boot_slopes[(a, m)], boot_slopes[("U", m)])] if "U" in arms_present else None
            ptu = expo.get("U", {}).get(m, {}).get("point") if a != "U" else pt
            e = {"point": pt, "lo": lo, "hi": hi}
            if d is not None:
                dlo, dhi = interval(d)
                e["diff_to_U_point"] = None  # filled below
                e["diff_lo"] = dlo
                e["diff_hi"] = dhi
            expo[a][m] = e
    for a in arms_present:
        for m, _ in METR:
            if "U" in expo:
                expo[a][m]["diff_to_U_point"] = expo[a][m]["point"] - expo["U"][m]["point"]

    # elasticity at k=14
    elas = None
    el_arms = ["U", "AS2", "AS4", "AS8"]
    if 14 in ks and all((14, a) in cells for a in el_arms):
        xs = [math.log(cells[(14, a)]["gm_mem"]) for a in el_arms]
        ys = [math.log(cells[(14, a)]["gm_T"]) for a in el_arms]
        pt = ols_slope(xs, ys)
        sme = SM(0x5EED ^ 2)
        ids14 = sorted(curve_ids[14])
        n14 = len(ids14)
        bs = []
        for _ in range(NB):
            ids = [ids14[sme.next() % n14] for _ in range(n14)]
            xx = [math.log(gm([R[(14, a)][c]["mem_total"] for c in ids])) for a in el_arms]
            yy = [math.log(gm([R[(14, a)][c]["T"] for c in ids])) for a in el_arms]
            bs.append(ols_slope(xx, yy))
        lo, hi = interval(bs)
        elas = {"arms": el_arms, "point": pt, "lo": lo, "hi": hi,
                "inside_[-1.3,-0.7]": bool(lo >= -1.3 and hi <= -0.7),
                "relation": "inside" if (lo >= -1.3 and hi <= -0.7) else (
                    "outside" if (hi < -1.3 or lo > -0.7) else "overlaps boundary")}

    # verdicts
    core_gates = all(gates[g]["pass"] for g in ("G0", "G1", "G2", "G3", "G5"))
    excluded = set()
    for a, v in g4.items():
        if not v["pass"]:
            excluded.add(a)
    if not gates["G6"]["pass"]:
        excluded.update(a for a in BOUNDED if a != "G")
    verdict = {"excluded_arms": sorted(excluded)}
    if not core_gates:
        verdict["h_brief"] = "no verdict: gate failure (G0/G1/G2/G3/G5)"
    else:
        aa = {}
        okaa = True
        for k in (10, 12, 14):
            c = cells.get((k, "UAA"))
            v = c.get("r_T") if c else None
            aa[k] = v
            if v is None or not (0.85 <= v <= 1.18):
                okaa = False
        pos = {}
        okpos = True
        for k in (12, 14):
            c = cells.get((k, "AS8"))
            v = c.get("r_T_lo") if c else None
            pos[k] = v
            if v is None or not v >= 3.0:
                okpos = False
        verdict["instrument"] = {"AA_r_T": aa, "AA_ok": okaa, "positive_AS8_r_T_lo": pos, "positive_ok": okpos}
        if not (okaa and okpos):
            verdict["h_brief"] = "instrument invalid"
        else:
            supp = []
            for a in BOUNDED:
                if a in excluded:
                    continue
                good = True
                det = {}
                for k in (12, 14):
                    c = cells.get((k, a))
                    if not c or "r_M_hi" not in c:
                        good = False
                        continue
                    det[k] = {"r_M_hi": c["r_M_hi"], "r_T_hi": c["r_T_hi"]}
                    if not (c["r_M_hi"] <= 0.5 and c["r_T_hi"] < 2.0):
                        good = False
                ex = expo[a]["mem"]
                det["mem_exp_diff"] = ex["diff_to_U_point"]
                det["mem_exp_diff_hi"] = ex["diff_hi"]
                if not (ex["diff_to_U_point"] <= -0.10 and ex["diff_hi"] < 0):
                    good = False
                det["meets_all"] = good
                supp.append((a, good, det))
            verdict["supported_detail"] = {a: d for a, g, d in supp}
            kill_ok = True
            kd = {}
            for k in (12, 14):
                low = [a for a in BOUNDED if a not in excluded and (k, a) in cells and cells[(k, a)].get("r_M", 9) <= 0.5]
                allhigh = True
                for a in low:
                    c = cells[(k, a)]
                    if not c["r_T_lo"] > 4.0:
                        allhigh = False
                    if c["n_exhausted"] > 0 and not c["r_T"] > 4.0:
                        allhigh = False
                kd[k] = {"cells_with_r_M_le_0.5": low, "all_r_T_lo_gt_4": allhigh}
                if not (low and allhigh):
                    kill_ok = False
            verdict["killed_detail"] = kd
            if any(g for a, g, d in supp):
                verdict["h_brief"] = "SUPPORTED (" + ",".join(a for a, g, d in supp if g) + ")"
            elif kill_ok:
                verdict["h_brief"] = "KILLED"
            else:
                verdict["h_brief"] = "INCONCLUSIVE"
    verdict["h_c4"] = elas
    if "G" in expo and "U" in expo:
        gm_ = expo["G"]["mem"]["point"] - expo["U"]["mem"]["point"]
        gt_ = expo["G"]["T"]["point"] - expo["U"]["T"]["point"]
        verdict["h_c5"] = {"G_mem_exp_minus_U": gm_, "G_T_exp_minus_U": gt_,
                           "supported": bool(gm_ <= -0.10 and gt_ <= 0.15)}
    out = {"files": files, "n_runs": len(runs), "gates": gates,
           "cells": {"%d,%s" % kk: v for kk, v in cells.items()},
           "exponents": expo, "elasticity_k14": elas, "verdict": verdict}
    out.update(common())
    with open(args.out, "w") as fh:
        json.dump(out, fh, indent=1, default=str)
    with open(args.tsv, "w") as fh:
        fh.write("\n".join(tsv_lines) + "\n")
    print(json.dumps({"gates": {g: v["pass"] for g, v in gates.items()}, "verdict": verdict.get("h_brief")}))


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("gate-pic")
    a.add_argument("--p", type=int, required=True, choices=[103, 127])
    a = sub.add_parser("curves")
    a.add_argument("--k", type=int, required=True)
    a.add_argument("--curves", default=None)
    a = sub.add_parser("run")
    a.add_argument("--k", type=int, required=True)
    a.add_argument("--arms", required=True)
    a.add_argument("--curves", default=None)
    a.add_argument("--out", required=True)
    a = sub.add_parser("analyze")
    a.add_argument("--in", dest="inp", required=True)
    a.add_argument("--out", required=True)
    a.add_argument("--tsv", required=True)
    args = ap.parse_args()
    {"gate-pic": cmd_gate_pic, "curves": cmd_curves, "run": cmd_run, "analyze": cmd_analyze}[args.cmd](args)


if __name__ == "__main__":
    main()
