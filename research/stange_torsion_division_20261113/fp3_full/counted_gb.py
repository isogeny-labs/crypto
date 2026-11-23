#!/usr/bin/env python3
"""Counted zero-dimensional solver over F_p for 3 polynomials in (e1, e2, e3), replacing msolve.

Every F_p multiplication and inversion the algorithm performs is counted into a phase table.
Counting rule (sparse semantics): an elimination step "row -= c * pivot_row" costs
nnz(pivot_row) - 1 multiplications (the leading entry cancels); normalising a row costs 1
inversion + nnz - 1 multiplications. Dense numpy is only the execution vehicle: products with a
zero operand are not charged, as a sparse implementation would not perform them.

Pipeline (deterministic for a given rng state):
  1. F4 (normal selection strategy: all pairs of minimal lcm degree; Gebauer-Moeller criteria;
     symbolic preprocessing; column-ordered reduction by reducer rows, then echelon form of the
     remaining rows), grevlex with e1 > e2 > e3.
  2. Standard monomials of the leading-term ideal (quotient dimension D, 64 when generic).
  3. Multiplication matrix by e3 on the quotient: trivial columns (e3*b standard) are free; the
     other columns are normal forms computed with a memoised, counted monomial reduction.
  4. Wiedemann: s_i = u . M^i [1], i < 2D (sparse matvecs), Berlekamp-Massey -> minimal polynomial.
  5. F_p roots of the minimal polynomial: gcd(z^p - z, f) and Cantor-Zassenhaus (ic_fp2.proots).
  6. Per root z: evaluation functional by Horner, lambda = w^T g(M) with g = f / (T - z);
     e1, e2 read off as lambda[e1]/lambda[1], lambda[e2]/lambda[1].
"""
import numpy as np

# phase table: name -> [M, I]
PH = {}


def charge(ph, m=0, i=0):
    t = PH.setdefault(ph, [0, 0])
    t[0] += m; t[1] += i


# ------------------------------------------------------------------ monomials (grevlex)
MAXDEG = 16
MONS = [(a, b, d - a - b) for d in range(MAXDEG + 1) for a in range(d + 1) for b in range(d + 1 - a)]
MONS.sort(key=lambda m: (sum(m), -m[2], -m[1]))   # ascending grevlex, e1 > e2 > e3
MKEY = {m: i for i, m in enumerate(MONS)}          # larger index = larger monomial


def mmul(u, v):
    return (u[0] + v[0], u[1] + v[1], u[2] + v[2])


def mdiv(u, v):  # v | u ?
    return u[0] >= v[0] and u[1] >= v[1] and u[2] >= v[2]


def mquo(u, v):
    return (u[0] - v[0], u[1] - v[1], u[2] - v[2])


def mlcm(u, v):
    return (max(u[0], v[0]), max(u[1], v[1]), max(u[2], v[2]))


def coprime(u, v):
    return not (min(u[0], v[0]) or min(u[1], v[1]) or min(u[2], v[2]))


def lt(poly):
    return max(poly, key=MKEY.__getitem__)


# ------------------------------------------------------------------ F4
class Basis:
    def __init__(self):
        self.polys = []   # monic dicts
        self.lts = []
        self.active = []  # indices of non-redundant elements

    def reducer_for(self, m):
        for k in reversed(self.active):
            if mdiv(m, self.lts[k]):
                return k
        return None


def gm_update(B, pairs, h):
    """Gebauer-Moeller update with new element index h."""
    lth = B.lts[h]
    Cand = [(h, g) for g in B.active]
    D = []
    while Cand:
        hh, g1 = Cand.pop(0)
        l1 = mlcm(lth, B.lts[g1])
        if coprime(lth, B.lts[g1]) or not any(mdiv(l1, mlcm(lth, B.lts[g2])) for _, g2 in Cand + D):
            D.append((hh, g1))
    E = [(hh, g) for hh, g in D if not coprime(lth, B.lts[g])]
    newp = []
    for g1, g2 in pairs:
        l12 = mlcm(B.lts[g1], B.lts[g2])
        if not (mdiv(l12, lth) and mlcm(B.lts[g1], lth) != l12 and mlcm(lth, B.lts[g2]) != l12):
            newp.append((g1, g2))
    newp += E
    B.active = [g for g in B.active if not mdiv(B.lts[g], lth)] + [h]
    return newp


def shift(poly, u):
    return {mmul(m, u): c for m, c in poly.items()}


def reduce_matrix(todo, B, p, stats, pre=None):
    """todo: list of dicts; reducers: pre (lead monomial -> row, from the selected pairs) plus
    symbolic preprocessing from B. Returns the new monic polys (echelon rows with new leads)."""
    monos = set()
    reducers = dict(pre or {})
    for r in todo + list(reducers.values()):
        monos.update(r)
    queue = list(monos)
    done = set(reducers)
    while queue:
        m = queue.pop()
        if m in done:
            continue
        done.add(m)
        k = B.reducer_for(m)
        if k is None:
            continue
        row = shift(B.polys[k], mquo(m, B.lts[k]))
        reducers[m] = row
        for mm in row:
            if mm not in monos:
                monos.add(mm); queue.append(mm)
    cols = sorted(monos, key=MKEY.__getitem__, reverse=True)
    ci = {m: j for j, m in enumerate(cols)}
    nc = len(cols)
    A = np.zeros((len(todo), nc), dtype=np.int64)
    for i, r in enumerate(todo):
        for m, c in r.items():
            A[i, ci[m]] = c % p
    R = {}
    for m, row in reducers.items():
        v = np.zeros(nc, dtype=np.int64)
        for mm, c in row.items():
            v[ci[mm]] = c
        R[ci[m]] = (v, len(row))
    stats['mat'].append((len(todo) + len(reducers), nc))
    mul = 0; inv = 0
    # reduction by reducers, column order
    for j in range(nc):
        if j not in R:
            continue
        col = A[:, j]
        nz = np.nonzero(col)[0]
        if len(nz) == 0:
            continue
        v, nn = R[j]
        A[nz] = (A[nz] - np.outer(col[nz], v)) % p
        mul += len(nz) * (nn - 1)
    # echelon form of the remaining rows (columns without reducer)
    used = np.zeros(len(todo), dtype=bool)
    new = []
    for j in range(nc):
        if j in R:
            continue
        cand = np.nonzero((A[:, j] != 0) & ~used)[0]
        if len(cand) == 0:
            continue
        i0 = cand[0]
        used[i0] = True
        row = A[i0]
        nn = int(np.count_nonzero(row))
        lc = int(row[j])
        if lc != 1:
            il = pow(lc, p - 2, p); inv += 1
            row = row * il % p; mul += nn - 1
            A[i0] = row
        rest = cand[1:]
        if len(rest):
            A[rest] = (A[rest] - np.outer(A[rest, j], row)) % p
            mul += len(rest) * (nn - 1)
        new.append({cols[k]: int(row[k]) for k in np.nonzero(row)[0]})
    stats['zero'] += int((~used).sum())
    charge('f4', mul, inv)
    return new


def f4(polys, p, stats, maxrounds=40):
    B = Basis()
    pairs = []
    init = reduce_matrix([dict(P) for P in polys], B, p, stats)
    for h in init:
        B.polys.append(h); B.lts.append(lt(h))
        pairs = gm_update(B, pairs, len(B.polys) - 1)
    rounds = 0
    while pairs:
        rounds += 1
        if rounds > maxrounds:
            return None
        dmin = min(sum(mlcm(B.lts[i], B.lts[j])) for i, j in pairs)
        sel = [pr for pr in pairs if sum(mlcm(B.lts[pr[0]], B.lts[pr[1]])) == dmin]
        pairs = [pr for pr in pairs if sum(mlcm(B.lts[pr[0]], B.lts[pr[1]])) != dmin]
        if dmin > MAXDEG - 2:
            return None
        todo = []
        firstrow = {}
        for i, j in sel:
            l = mlcm(B.lts[i], B.lts[j])
            ri = shift(B.polys[i], mquo(l, B.lts[i])); rj = shift(B.polys[j], mquo(l, B.lts[j]))
            if l not in firstrow:
                firstrow[l] = ri
                todo.append(rj)
            else:
                todo.append(ri); todo.append(rj)
        # the first row per lcm is the pivot row of that column (standard F4 symbolic preprocessing)
        new = reduce_matrix(todo, B, p, stats, pre=firstrow)
        for h in new:
            B.polys.append(h); B.lts.append(lt(h))
            pairs = gm_update(B, pairs, len(B.polys) - 1)
    stats['rounds'] = rounds
    return B


# ------------------------------------------------------------------ quotient, multiplication matrix
def standard_monomials(B):
    lts = [B.lts[k] for k in B.active]
    std = [m for m in MONS if not any(mdiv(m, l) for l in lts)]
    if any(sum(m) >= MAXDEG - 1 for m in std):
        return None   # positive dimensional (or degree beyond the table)
    return std


class NF:
    """memoised counted normal forms of monomials as vectors over the standard basis"""

    def __init__(self, B, std, p):
        self.B = B; self.p = p
        self.si = {m: i for i, m in enumerate(std)}
        self.D = len(std)
        self.memo = {}
        self.mul = 0

    def vec(self, m):
        if m in self.si:
            v = np.zeros(self.D, dtype=np.int64); v[self.si[m]] = 1
            return v
        return self.nf(m)

    def nf(self, m):
        if m in self.memo:
            return self.memo[m]
        k = self.B.reducer_for(m)
        g = self.B.polys[k]; u = mquo(m, self.B.lts[k])
        acc = np.zeros(self.D, dtype=np.int64)
        for t, c in g.items():
            if t == self.B.lts[k]:
                continue
            tu = mmul(t, u)
            if tu in self.si:
                acc[self.si[tu]] -= c
            else:
                w = self.nf(tu)
                acc -= c * w
                self.mul += int(np.count_nonzero(w))
        acc %= self.p
        self.memo[m] = acc
        return acc


def mult_matrix(B, std, p):
    """returns (triv_src, triv_dst, nontriv_idx, Ncols (D x k)) for multiplication by e3"""
    nf = NF(B, std, p)
    si = nf.si
    ts, td, nt, cols = [], [], [], []
    for j, b in enumerate(std):
        m = mmul(b, (0, 0, 1))
        if m in si:
            ts.append(j); td.append(si[m])
        else:
            nt.append(j); cols.append(nf.nf(m))
    charge('multmatrix', nf.mul, 0)
    Nc = np.array(cols, dtype=np.int64).T if cols else np.zeros((len(std), 0), dtype=np.int64)
    return np.array(ts), np.array(td), np.array(nt), Nc


def matvec(Mx, v, p):
    """w = M v (column j of M = NF(e3 b_j)); counted"""
    ts, td, nt, Nc = Mx
    w = np.zeros_like(v)
    if len(ts):
        w[td] = v[ts]
    if len(nt):
        vn = v[nt]
        nzc = np.nonzero(vn)[0]
        if len(nzc):
            w = (w + Nc[:, nzc] @ vn[nzc]) % p
            return w, int(np.count_nonzero(Nc[:, nzc]))
    return w % p, 0


def vecmat(Mx, u, p):
    """u^T M; counted"""
    ts, td, nt, Nc = Mx
    out = np.zeros_like(u)
    if len(ts):
        out[ts] = u[td]
    m = 0
    if len(nt):
        out[nt] = (u @ Nc) % p
        nzu = u != 0
        m = int(np.count_nonzero(Nc[nzu, :]))
    return out, m


def berlekamp_massey(s, p):
    """minimal polynomial (monic, low degree first) of the sequence s over F_p; counted"""
    mul = 0; inv = 0
    Cp = [1]; Bp = [1]; L = 0; m = 1; b = 1
    for n in range(len(s)):
        d = s[n]
        for i in range(1, L + 1):
            if i < len(Cp) and Cp[i]:
                d = (d + Cp[i] * s[n - i]) % p; mul += 1
        if d == 0:
            m += 1
            continue
        coef = d * pow(b, p - 2, p) % p; inv += 1; mul += 1
        T = Cp[:]
        need = len(Bp) + m
        if len(Cp) < need:
            Cp = Cp + [0] * (need - len(Cp))
        for i, x in enumerate(Bp):
            if x:
                Cp[i + m] = (Cp[i + m] - coef * x) % p; mul += 1
        if 2 * L <= n:
            L = n + 1 - L; Bp = T; b = d; m = 1
        else:
            m += 1
    Cp = (Cp + [0] * (L + 1))[:L + 1]
    # connection polynomial C(x) = 1 + c1 x + ... + cL x^L; minimal polynomial = reverse: x^L + c1 x^{L-1} + ... + cL
    f = list(reversed(Cp))   # low degree first: f[0] = cL, ..., f[L] = 1
    charge('wiedemann', mul, inv)
    return f, L


def solve(polys, p, rng, proots):
    """polys: list of dicts {(i,j,k): c}. Returns (solutions list of (e1,e2,e3), D, status, stats)."""
    stats = dict(mat=[], zero=0)
    B = f4(polys, p, stats)
    if B is None:
        return [], 0, 'f4_abort', stats
    std = standard_monomials(B)
    if std is None:
        return [], -1, 'posdim', stats
    D = len(std)
    if D == 0:
        return [], 0, 'nosol', stats
    si = {m: i for i, m in enumerate(std)}
    if (0, 0, 0) not in si or (1, 0, 0) not in si or (0, 1, 0) not in si:
        return [], D, 'nongeneric', stats
    Mx = mult_matrix(B, std, p)
    stats['nontriv'] = int(len(Mx[2]))
    i1 = si[(0, 0, 0)]
    f = None
    for attempt in range(2):
        u = np.array([rng.randrange(p) for _ in range(D)], dtype=np.int64)
        v = np.zeros(D, dtype=np.int64); v[i1] = 1
        s = []; mul = 0
        for _ in range(2 * D):
            s.append(int(u @ v % p)); mul += int(np.count_nonzero(v))
            v, mm = matvec(Mx, v, p); mul += mm
        charge('wiedemann', mul, 0)
        f, L = berlekamp_massey(s, p)
        if L == D:
            break
    if L != D:
        return [], D, 'not_cyclic', stats
    # F_p roots of the minimal polynomial (counted inside proots via ic_fp2.C -> moved to phase by caller)
    roots = proots(f[:])
    sols = []
    for z in roots:
        # g = f / (T - z), synthetic division
        g = [0] * D
        acc = f[D]
        mul = 0
        for k in range(D - 1, -1, -1):
            g[k] = acc
            acc = (f[k] + acc * z) % p; mul += 1
        # lambda = w^T g(M) by Horner; w = unit vector at '1' first (adding g_k w is free),
        # then a random w (D extra mults per step) if lambda[1] came out 0
        inv = 0
        for wtry in range(3):
            if wtry == 0:
                w = np.zeros(D, dtype=np.int64); w[i1] = 1; wm = 0
            else:
                w = np.array([rng.randrange(p) for _ in range(D)], dtype=np.int64); wm = D
            lam = g[D - 1] * w % p; mul += wm
            for k in range(D - 2, -1, -1):
                lam, mm = vecmat(Mx, lam, p); mul += mm
                lam = (lam + g[k] * w) % p; mul += wm
            if lam[i1] != 0:
                break
        if lam[i1] == 0:
            charge('eigen', mul, inv)
            continue
        il = pow(int(lam[i1]), p - 2, p); inv += 1
        e1 = int(lam[si[(1, 0, 0)]]) * il % p; e2 = int(lam[si[(0, 1, 0)]]) * il % p; mul += 2
        charge('eigen', mul, inv)
        sols.append((e1, e2, z))
    return sols, D, 'ok', stats
