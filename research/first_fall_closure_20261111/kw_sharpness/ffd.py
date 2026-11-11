"""Weil descent of S_4(X1,X2,X3,c) and the two first-fall computations.
Boolean-ring polynomials are Python ints: bit `mask` set <=> monomial prod_{i in mask} y_i present
(mask indexes the N = 3 n' Boolean variables; block k owns bits k*n' .. (k+1)*n'-1)."""
import numpy as np, random, time
from kwgf import GF
import semaev

M_DEG = 6  # m^2 - m for m = 3

_S4 = None
def s4_terms():
    global _S4
    if _S4 is None: _S4 = sorted(semaev.S4().keys())
    return _S4

def popcount_arr(N):
    a = np.arange(1 << N, dtype=np.uint32)
    c = np.zeros(1 << N, dtype=np.uint8)
    for i in range(N): c += ((a >> i) & 1).astype(np.uint8)
    return c

def arr_to_int(bits):
    return int.from_bytes(np.packbits(bits.astype(np.uint8), bitorder='little').tobytes(), 'little')

class Masks:
    def __init__(self, N):
        self.N = N
        pc = popcount_arr(N)
        a = np.arange(1 << N, dtype=np.uint32)
        self.deg = [arr_to_int(pc == d) for d in range(N + 1)]
        self.set = [arr_to_int(((a >> i) & 1) == 1) for i in range(N)]
        self.unset = [arr_to_int(((a >> i) & 1) == 0) for i in range(N)]
        self.pc = pc

_masks = {}
def masks(N):
    if N not in _masks: _masks[N] = Masks(N)
    return _masks[N]

def mulB(g, i, M):   # multiply by y_i in the Boolean ring (y_i^2 = y_i)
    return ((g & M.unset[i]) << (1 << i)) ^ (g & M.set[i])
def mulG(g, i, M):   # multiply by y_i in the graded ring Gr (y_i^2 = 0)
    return (g & M.unset[i]) << (1 << i)

def lead(g, M):
    for d in range(M.N, -1, -1):
        t = g & M.deg[d]
        if t: return d, t.bit_length() - 1
    return -1, -1
def lead_from(g, d0, M):
    for d in range(d0, -1, -1):
        t = g & M.deg[d]
        if t: return d, t.bit_length() - 1
    return -1, -1
def degree(g, M): return lead(g, M)[0] if g else -1

def rank_int(rows):
    piv = {}
    r = 0
    for v in rows:
        while v:
            b = v.bit_length() - 1
            if b in piv: v ^= piv[b]
            else: piv[b] = v; r += 1; break
    return r

# ---------------------------------------------------------------- descent
def block_polys(F, nus):
    """P_a(y) for a = 0..4:  x^a with x = sum nu_l y_l, as dict-array over masks of the block (values in GF)."""
    npr = len(nus); sz = 1 << npr
    res = {}
    # x^(2^j) = sum nu_l^(2^j) y_l  (Frobenius is additive, y_l^2=y_l)
    lin = []
    for j in range(3):
        lam = []
        for nu in nus:
            v = nu
            for _ in range(j): v = F.sq(v)
            lam.append(v)
        lin.append(lam)
    def mul_lin(p, lam):
        out = np.zeros(sz, dtype=np.int64)
        for l in range(npr):
            if lam[l] == 0: continue
            for mask in range(sz):
                if p[mask]:
                    out[mask | (1 << l)] ^= F.mul(int(p[mask]), lam[l])
        return out
    one = np.zeros(sz, dtype=np.int64); one[0] = 1
    bits = {0: [], 1: [0], 2: [1], 3: [0, 1], 4: [2]}
    for a in range(5):
        p = one
        for j in bits[a]: p = mul_lin(p, lin[j])
        res[a] = p
    return res

def descend(F, nus, c, a6):
    """returns list of n Python ints s_0..s_{n-1} (bit k of the GF coefficient), N = 3 n'"""
    npr = len(nus); N = 3 * npr
    P = block_polys(F, nus)
    total = np.zeros(1 << N, dtype=np.int64)
    # collect kappa[(a,b,e)] in GF
    kappa = {}
    for mon in s4_terms():
        a6e, e1, e2, e3, e4 = mon
        coef = F.mul(F.pow(a6, a6e) if a6e else 1, F.pow(c, e4) if e4 else 1)
        key = (e1, e2, e3)
        kappa[key] = kappa.get(key, 0) ^ coef
    for (e1, e2, e3), kv in kappa.items():
        if kv == 0: continue
        t = F.vmul(np.full(1 << npr, kv, dtype=np.int64), P[e1])
        # outer product with block order (block3, block2, block1) -> index m3*2^(2n') + m2*2^n' + m1
        outer = lambda X, Y: F.vmul(np.repeat(X, len(Y)), np.tile(Y, len(X)))
        full = outer(P[e3], outer(P[e2], t))
        total ^= full
    n = F.n
    s = []
    for k in range(n):
        s.append(arr_to_int(((total >> k) & 1).astype(np.uint8)))
    return s

def moore_e(F, nus):
    """e_{ab} = nu_a nu_b^2 + nu_a^2 nu_b = nu_a nu_b (nu_a+nu_b), a<b"""
    e = {}
    for a in range(len(nus)):
        for b in range(a + 1, len(nus)):
            e[(a, b)] = F.mul(nus[a], F.sq(nus[b])) ^ F.mul(F.sq(nus[a]), nus[b])
    return e

def top_formula(F, nus, c):
    """degree-6 homogeneous part from the closed formula c * prod_k sum_{a<b} e_ab y_ka y_kb (as GF-valued array)"""
    npr = len(nus); N = 3 * npr
    e = moore_e(F, nus)
    blk = np.zeros(1 << npr, dtype=np.int64)
    for (a, b), v in e.items(): blk[(1 << a) | (1 << b)] = v
    outer = lambda X, Y: F.vmul(np.repeat(X, len(Y)), np.tile(Y, len(X)))
    full = outer(blk, outer(blk, blk))
    full = F.vmul(full, np.full(len(full), c, dtype=np.int64))
    return [arr_to_int(((full >> k) & 1).astype(np.uint8)) for k in range(F.n)]

# ---------------------------------------------------------------- top-form (HPS / KW / CG) convention
def topform_analysis(s, N, extra_j=1):
    M = masks(N)
    # F_2-basis of V = span(s)
    piv = {}
    basis = []
    for v in s:
        w = v
        while w:
            b = w.bit_length() - 1
            if b in piv: w ^= piv[b]
            else: piv[b] = w; basis.append(w); break
    r = len(basis)
    maxdeg = max(degree(v, M) for v in basis)
    top = [v & M.deg[M_DEG] for v in basis]
    rk_top = rank_int(top)
    out = dict(dimV=r, maxdeg=maxdeg, dimVbar=rk_top)
    if rk_top < r:
        out.update(Dff_top=M_DEG, reason="top forms dependent (dim Vbar < dim V)")
        return out
    # coefficient degree j-d = 1..: map (Gr_{t})^r -> Gr_{d+t}; U is zero in coefficient degrees < d
    from itertools import combinations
    for t in range(1, 1 + extra_j):
        rows = []
        for i_tup in combinations(range(N), t):
            for v in top:
                w = v
                for i in i_tup: w = mulG(w, i, M)
                rows.append(w)
        # NB: monomial multipliers of degree t; y^2=0 so only squarefree monomials
        rk = rank_int(rows)
        null = len(rows) - rk
        out["rank_deg%d" % (M_DEG + t)] = rk; out["rows_deg%d" % (M_DEG + t)] = len(rows); out["nullity_deg%d" % (M_DEG + t)] = null
        if null > 0:
            out.update(Dff_top=M_DEG + t, reason="non-trivial top-form syzygy with coefficients of degree %d" % t)
            return out
    out.update(Dff_top=None, reason="no syzygy found up to degree %d" % (M_DEG + extra_j))
    return out

# ---------------------------------------------------------------- operational convention (HKY V_{F,c} closure, in B)
def closure(s, N, c, M=None, rowcap=200000, deadline=None):
    """W_c = reduction of V_{F,c} for F = F_2-span of s (plus the field equations, built into B).
    Returns (dim W_c, dim (W_c cap B_{<=c-1}), number of pivot rows)."""
    M = M or masks(N)
    # echelon of F alone
    pivF = {}
    for v in s:
        w = v
        while w:
            d, l = lead(w, M)
            if l in pivF: w ^= pivF[l][0]
            else: pivF[l] = (w, d); break
    piv = {}
    queue = []
    def add(v):
        if not v: return
        d, l = lead(v, M)
        while True:
            if l in piv:
                v ^= piv[l][0]
                if not v: return
                d, l = lead_from(v, d, M)
            else:
                piv[l] = (v, d)
                queue.append(l)
                return
    for l, (w, d) in pivF.items():
        if d <= c: add(w)
    qi = 0
    while qi < len(queue):
        l = queue[qi]; qi += 1
        v, d = piv[l]
        if d <= c - 1:
            for i in range(N): add(mulB(v, i, M))
        if len(piv) > rowcap: raise RuntimeError("rowcap exceeded")
        if deadline and time.time() > deadline: raise TimeoutError
    low = sum(1 for (v, d) in piv.values() if d <= c - 1)
    return len(piv), low

def stage_one_low(s, N, c, M):
    """dim of (span{ m f : f in F cap B_{<=c}, deg m + deg f <= c, m a variable or 1 } cap B_{<=c-1}):
    a LOWER bound for dim(W_c cap B_{<=c-1}) because the stage-one span is contained in W_c."""
    pivF = {}
    for v in s:
        w = v
        while w:
            d, l = lead(w, M)
            if l in pivF: w ^= pivF[l][0]
            else: pivF[l] = (w, d); break
    piv = {}
    def add(v):
        if not v: return
        d, l = lead(v, M)
        while True:
            if l in piv:
                v ^= piv[l][0]
                if not v: return
                d, l = lead_from(v, d, M)
            else:
                piv[l] = (v, d); return
    for l, (w, d) in pivF.items():
        if d <= c:
            add(w)
            if d <= c - 1:
                for i in range(N): add(mulB(w, i, M))
    return sum(1 for (v, d) in piv.values() if d <= c - 1), len(piv)

def operational_first_fall(s, N, cmax=8, deadline=None, full_limit=0):
    """first c with dim(W_c cap B_{<=c-1}) > dim W_{c-1}.  At each c a cheap stage-one test is tried first
    (a detected fall is a real fall); only if it fails is the full closure W_c computed.
    Returns (c or None, log) where log has one dict per c."""
    M = masks(N)
    prev_dim = 0
    log = []
    for c in range(1, cmax + 1):
        low1, tot1 = stage_one_low(s, N, c, M)
        if low1 > prev_dim:
            log.append(dict(c=c, method="stage1", low_lower_bound=low1, prev_dimW=prev_dim))
            return c, log
        dimW, low = closure(s, N, c, M, deadline=deadline)
        log.append(dict(c=c, method="full", dimW=dimW, low=low, prev_dimW=prev_dim))
        if low > prev_dim: return c, log
        prev_dim = dimW
    return None, log

# ---------------------------------------------------------------- sampling of instances
def random_instance(n, rnd, npr=None, nus=None):
    F = GF(n)
    npr = npr or 3
    # a random point on a random curve: choose a2,a6, x with Tr(...)=0
    while True:
        a2 = rnd.randrange(2); a6 = rnd.randrange(1, F.q)
        for _ in range(100):
            x = rnd.randrange(1, F.q)
            rhs = F.mul(F.sq(x), x) ^ F.mul(a2, F.sq(x)) ^ a6
            # y^2 + xy = rhs  <=> (y/x)^2 + (y/x) = rhs / x^2 ; solvable iff Tr(rhs/x^2) = 0
            if F.trace(F.mul(rhs, F.inv(F.sq(x)))) == 0:
                return F, a2, a6, x
