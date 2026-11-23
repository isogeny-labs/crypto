#!/usr/bin/env python3
"""Generic TOWER closure engine (numpy research prototype; exact dense linear algebra over F_p).

Presentation: two summands, per summand variables x_0..x_{t-1}.  Normal forms = squarefree monomials in the 2t
variables (bit s*t+j = variable x_{s,j}); weight = popcount.  A tower is given by its rewrite rule for x_j^2:
  kummer: x_j^2 = x_{j+1}, x_{t-1}^2 = 1                      (mu_{2^t}; same column set as N8)
  velu  : (x_{j+1}-x_j)(x_j-xi_j) = tau_j, x_t = c  i.e.
          x_j^2 = x_j x_{j+1} - xi_j x_{j+1} + xi_j x_j - tau_j   (degree <= 2, involves x_{j+1})
V_D = least subspace of span(M_<=D) containing NF(S_3(x_{1,0},x_{2,0},target)) and closed under multiplication by
every variable x_{s,j} for elements of weight < D (products reduced by the rewrite rules).
Matches ml-cryptanalysis/scripts/ic_bounds_growth_lib.py (same closure/processing/verify logic); the difference is that
the shifts are sparse reduced products (not permutations) and the linear algebra is blocked float64-BLAS exact mod p.
"""
import itertools, time, sys
import numpy as np


# ---------------------------------------------------------------- rewrite rules
class Rules:
    def __init__(self, kind, t, p, xi=None, tau=None, c=None):
        self.kind, self.t, self.p = kind, t, p
        self.xi, self.tau, self.c = xi, tau, c
        self.memo = {}

    def mult1(self, m, j):
        """x_j * (squarefree monomial m of ONE summand, t-bit mask) -> dict {mask: coef mod p}."""
        key = (m, j)
        r = self.memo.get(key)
        if r is not None:
            return r
        t, p = self.t, self.p
        bit = 1 << j
        if not (m & bit):
            r = {m | bit: 1}
        else:
            mp = m & ~bit
            if self.kind == 'kummer':
                r = {mp: 1} if j == t - 1 else self.mult1(mp, j + 1)
            else:
                xi, tauj, c = self.xi[j], self.tau[j], self.c
                r = {}

                def add(d, s):
                    for mm, cf in d.items():
                        r[mm] = (r.get(mm, 0) + s * cf) % p
                if j == t - 1:
                    add({m: 1}, c)               # x_j x_t m' = c m
                    add({mp: 1}, -xi * c)        # - xi x_t m'
                else:
                    add(self.mult1(m, j + 1), 1)     # x_j x_{j+1} m' = x_{j+1} m
                    add(self.mult1(mp, j + 1), -xi)  # - xi x_{j+1} m'
                add({m: 1}, xi)                  # + xi x_j m'
                add({mp: 1}, -tauj)            # - tau m'
                r = {mm: cf for mm, cf in r.items() if cf}
        self.memo[key] = r
        return r

    def power_nf(self, e):
        """NF of x_0^e (e = 0, 1, 2) in one summand."""
        if e == 0: return {0: 1}
        if e == 1: return {1: 1}
        return self.mult1(1, 0)


def s3_coeffs(p, a, b, t):
    c = {}
    def add(e, v): c[e] = (c.get(e, 0) + v) % p
    add((2, 0), t * t); add((1, 1), -2 * t * t); add((0, 2), t * t)
    add((2, 1), -2 * t); add((1, 2), -2 * t); add((1, 0), -2 * t * a); add((0, 1), -2 * t * a)
    add((0, 0), -4 * t * b); add((2, 2), 1); add((1, 1), -2 * a); add((0, 0), a * a)
    add((1, 0), -4 * b); add((0, 1), -4 * b)
    return {e: v for e, v in c.items() if v}


def generator_nf(rules, p, a, b, target):
    """NF of S_3(x_{1,0}, x_{2,0}, target) as {global mask: coef}."""
    t = rules.t
    g = {}
    for (e1, e2), cf in s3_coeffs(p, a, b, target).items():
        for m1, c1 in rules.power_nf(e1).items():
            for m2, c2 in rules.power_nf(e2).items():
                k = m1 | (m2 << t)
                g[k] = (g.get(k, 0) + cf * c1 * c2) % p
    return {k: v for k, v in g.items() if v}


# ---------------------------------------------------------------- layout and operators
class Layout:
    def __init__(self, t, D):
        self.t, self.D, self.nv = t, D, 2 * t
        ms = []
        for w in range(D, -1, -1):
            for comb in itertools.combinations(range(self.nv), w):
                ms.append(sum(1 << i for i in comb))
        self.masks = ms
        self.idx = {m: i for i, m in enumerate(ms)}
        self.n = len(ms)
        self.wt = np.array([bin(m).count('1') for m in ms])
        self.nD = int((self.wt == D).sum())   # columns 0..nD-1 have weight D; the rest are 'low' (< D)


def build_ops(rules, L):
    ops = {}
    t, p = L.t, rules.p
    full = (1 << t) - 1
    for v in range(L.nv):
        s, j = divmod(v, t)
        sh = s * t
        trip = []
        for c in range(L.nD, L.n):
            mask = L.masks[c]
            sub = (mask >> sh) & full
            other = mask ^ (sub << sh)
            for sm, cf in rules.mult1(sub, j).items():
                trip.append((c, L.idx[other | (sm << sh)], cf))   # KeyError => weight would exceed D
        groups, cnt = [], {}
        for (c, d, cf) in trip:
            g = cnt.get(d, 0); cnt[d] = g + 1
            if g == len(groups): groups.append(([], [], []))
            groups[g][0].append(c); groups[g][1].append(d); groups[g][2].append(cf)
        ops[v] = [(np.array(a), np.array(b), np.array(cc, dtype=np.int64)) for a, b, cc in groups]
    return ops


def apply_op(op, X, p):
    Y = np.zeros_like(X)
    for src, dst, cf in op:
        Y[:, dst] += X[:, src] * cf[None, :]
    return Y % p


# ---------------------------------------------------------------- exact linear algebra mod p
def modmm(A, B, p, chunk=4096):
    m, r = A.shape
    out = np.zeros((m, B.shape[1]), dtype=np.int64)
    for s in range(0, r, chunk):
        P = A[:, s:s + chunk].astype(np.float64) @ B[s:s + chunk].astype(np.float64)   # < 4096 * p^2 < 2^53 for p < 2^21
        out += np.mod(P, p).astype(np.int64)
    return out % p


def rref_small(M, p):
    M = np.array(M, dtype=np.int64) % p
    r, piv = 0, []
    rows, cols = M.shape
    for c in range(cols):
        if r == rows: break
        nz = np.nonzero(M[r:, c])[0]
        if nz.size == 0: continue
        s = r + nz[0]
        if s != r: M[[r, s]] = M[[s, r]]
        M[r] = (M[r] * pow(int(M[r, c]), p - 2, p)) % p
        col = M[:, c].copy(); col[r] = 0
        nzr = np.nonzero(col)[0]
        if nzr.size: M[nzr] = (M[nzr] - np.outer(col[nzr], M[r])) % p
        piv.append(c); r += 1
    return M[:r], piv


def merge(B, piv, Y, p, blk=256):
    n = Y.shape[1]
    res = (Y - modmm(Y[:, piv], B, p)) % p if len(piv) else Y % p
    res = res[res.any(axis=1)]
    if res.shape[0] == 0: return B, piv, 0
    newC = np.zeros((0, n), dtype=np.int64); newpc = []
    while res.shape[0]:
        C, pc = rref_small(res[:blk], p)
        rest = res[blk:]
        if pc:
            if newpc: newC = (newC - modmm(newC[:, pc], C, p)) % p
            newC = np.vstack([newC, C]); newpc = newpc + pc
            if rest.shape[0]:
                rest = (rest - modmm(rest[:, pc], C, p)) % p
                rest = rest[rest.any(axis=1)]
        res = rest
    # memory-lean update (same arithmetic): back-substitute in row chunks, in place; place rows by pivot order in one copy
    for s0 in range(0, B.shape[0], 1024):
        blk = B[s0:s0 + 1024]
        blk -= modmm(blk[:, newpc], newC, p)
        blk %= p
    allp = list(piv) + newpc
    order = np.argsort(allp)
    rank_of = np.empty(len(allp), dtype=np.int64); rank_of[order] = np.arange(len(allp))
    out = np.empty((len(allp), n), dtype=np.int64)
    out[rank_of[:B.shape[0]]] = B
    out[rank_of[B.shape[0]:]] = newC
    del B
    return out, [allp[i] for i in order], len(newpc)


def closure(L, g, ops, p, chunk=400, grp=4, early=False, log=None):
    """returns dict(one_in_V, rank, profile, B, piv). early=True stops as soon as 1 enters V."""
    n, D = L.n, L.D
    if any(L.wt[L.idx[m]] > D for m in g):
        return dict(one_in_V=None, rank=0, profile=None, note='generator weight > D', B=None, piv=[])
    gv = np.zeros(n, dtype=np.int64)
    for m, c in g.items(): gv[L.idx[m]] = c
    B, piv = rref_small(gv[None, :], p)
    processed = set()
    t0 = time.time(); one = n - 1
    while True:
        if early and one in piv: break
        todo = [c for c in piv if L.wt[c] < D and c not in processed][:chunk]
        if not todo: break
        pos = {c: r for r, c in enumerate(piv)}
        new = B[[pos[c] for c in todo]]
        processed.update(todo)
        for g0 in range(0, L.nv, grp):
            Y = np.vstack([apply_op(ops[v], new, p) for v in range(g0, min(g0 + grp, L.nv))])
            B, piv, _ = merge(B, piv, Y, p)
            if early and one in piv: break
        if log: log(f'  D={D} rank={len(piv)} low={sum(1 for c in piv if L.wt[c] < D)} processed={len(processed)} t={time.time()-t0:.0f}s')
    prof = [sum(1 for c in piv if L.wt[c] <= j) for j in range(D + 1)]
    return dict(one_in_V=(one in piv), rank=len(piv), profile=prof, B=B, piv=piv, secs=time.time() - t0)


def verify_closed(L, B, piv, ops, p, rows=600):
    """number of (low row, variable) products that do not reduce to 0 against (B, piv); 0 = certified closed."""
    D = L.D
    low = B[[r for r, c in enumerate(piv) if L.wt[c] < D]]
    bad = 0
    for s in range(0, low.shape[0], rows):
        part = low[s:s + rows]
        for v in range(L.nv):
            Y = apply_op(ops[v], part, p)
            R = (Y - modmm(Y[:, piv], B, p)) % p
            bad += int(R.any(axis=1).sum())
    return bad


def run_cell(rules, p, a, b, target, D, early=False, verify=True, log=None, chunk=400):
    L = Layout(rules.t, D)
    g = generator_nf(rules, p, a, b, target)
    gw = max(bin(m).count('1') for m in g)
    if gw > D:
        return dict(t=rules.t, D=D, columns=L.n, gen_weight=gw, one_in_V=False, dim=0, profile=None, secs=0.0,
                    note='generator weight > D: V_D = 0 (vacuous)')
    ops = build_ops(rules, L)
    r = closure(L, g, ops, p, chunk=chunk, early=early, log=log)
    out = dict(t=rules.t, D=D, columns=L.n, gen_weight=gw, one_in_V=r['one_in_V'],
               dim=r['rank'], profile=r['profile'], secs=round(r.get('secs', 0), 1))
    if r['B'] is not None and verify and not r['one_in_V']:
        out['verify_bad'] = verify_closed(L, r['B'], r['piv'], ops, p)
    return out
