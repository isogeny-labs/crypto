"""F3 pilot (coordinator, 2026-11-17).  Python 3 stdlib only.

Validates gate reachability and instrument sanity for the F3 protocol on
PILOT curves only (curve seeds >= 10001, disjoint from the decision seeds
1..600).  It never computes the decision metric on a decision instance.

Arms (genus 2, full base): P pencil sieve + descent + LA, W Cantor walk + split
test + LA, R negation-map rho with distinguished points.  The shared LA is
peel -> SGE (w_max, fill_max) -> dense Gauss (<= DENSE_MAX columns) or
Wiedemann.  Copied helpers: l01copy/{field,rng,curve}.py (see COPIED.sha256).
"""
import sys, os, json, math, time, heapq
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from l01copy.rng import mix64, SM64, M64, GOLD
from l01copy.field import Counter
from l01copy.curve import Curve, IDENT, key_of, is_identity

LANGE_ADD, LANGE_DBL = 35, 37   # 1I+25M, 1I+27M (Lange 2005, affine, literature)


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def random_f(p, g, cs):
    st = ((cs * GOLD) & M64) ^ p ^ (g << 48)
    f = []
    for _ in range(2 * g + 1):
        st = (st + GOLD) & M64
        f.append(mix64(st) % p)
    f.append(1)
    return f


def poly_gcd_deg(a, b, p):
    a = list(a); b = list(b)
    def trim(x):
        while x and x[-1] == 0:
            x.pop()
        return x
    trim(a); trim(b)
    while b:
        inv = pow(b[-1], p - 2, p)
        while len(a) >= len(b) and a:
            c = a[-1] * inv % p
            sh = len(a) - len(b)
            for i in range(len(b)):
                a[sh + i] = (a[sh + i] - c * b[i]) % p
            trim(a)
        a, b = b, a
    return len(a) - 1


def squarefree(f, p):
    df = [(i * f[i]) % p for i in range(1, len(f))]
    return poly_gcd_deg(f, df, p) == 0


class Tables:
    def __init__(self, p, f):
        self.p = p
        chi = [-1] * p; chi[0] = 0
        sq = [None] * p; sq[0] = 0
        for y in range(1, (p - 1) // 2 + 1):
            a = y * y % p
            chi[a] = 1; sq[a] = y
        fx = [0] * p
        for x in range(p):
            acc = 1
            for c in reversed(f[:-1]):
                acc = (acc * x + c) % p
            fx[x] = acc
        self.chi, self.sq, self.fx = chi, sq, fx
        self.base = [x for x in range(p) if chi[fx[x]] == 1]
        self.col = {x: i for i, x in enumerate(self.base)}
        self.setup_mul = (p - 1) // 2 + 5 * p   # squarings + Horner (charged to P and W)


def n1(t):
    return 1 + sum(1 + t.chi[t.fx[x]] for x in range(t.p))


def n2_brute(p, f):
    sq = [False] * p
    for y in range(1, p):
        sq[y * y % p] = True
    nu = next(a for a in range(2, p) if not sq[a])
    n = 1
    for a in range(p):
        for b in range(0, (p - 1) // 2 + 1):
            ra = rb = 0
            for c in reversed(f):
                na = (ra * a + nu * (rb * b % p)) % p
                nb = (ra * b + rb * a) % p
                ra = (na + c) % p; rb = nb
            w = 1 if b == 0 else 2
            if ra == 0 and rb == 0:
                n += w
            else:
                norm = (ra * ra - nu * rb * rb) % p
                if sq[norm]:
                    n += 2 * w
    return n


def order_from_counts(p, N1, N2):
    c1 = N1 - p - 1
    s2 = N2 - p * p - 1
    assert (c1 * c1 + s2) % 2 == 0
    c2 = (c1 * c1 + s2) // 2
    return 1 + c1 + c2 + p * c1 + p * p


def order_bsgs(C, t, rng):
    """#J from N1 and BSGS over the window c2 in [-6p, 6p] intersected with Hasse-Weil."""
    p = t.p
    c1 = n1(t) - p - 1
    base = 1 + c1 + p * c1 + p * p
    sp = math.sqrt(p)
    lo = max(base - 6 * p, int((sp - 1) ** 4) - 1, 1)
    hi = min(base + 6 * p, int((sp + 1) ** 4) + 2)
    width = hi - lo
    m = math.isqrt(width) + 1
    for _ in range(8):
        d0 = C.random_divisor(rng)
        baby = {}
        cur = IDENT; small = False
        for j in range(m + 1):
            if j > 0 and is_identity(cur):
                small = True; break
            if cur in baby:
                small = True; break
            baby[cur] = j
            cur = C.cantor(cur, d0)
        if small:
            continue
        q = C.negate(C.mul_ltr(d0, lo))
        step = C.negate(C.mul_ltr(d0, m))
        cands = []
        for i in range(width // m + 2):
            if q in baby:
                n = lo + i * m + baby[q]
                if n <= hi and is_identity(C.mul_ltr(d0, n)):
                    cands.append(n)
            q = C.cantor(q, step)
        cands = sorted(set(cands))
        tests = [C.random_divisor(rng) for _ in range(16)]
        surv = [n for n in cands if all(is_identity(C.mul_ltr(d, n)) for d in tests)]
        return surv
    return []


def split_order(n, max_h=4):
    for h in range(1, max_h + 1):
        if n % h == 0:
            r = n // h
            if r > h and is_prime(r):
                return h, r
    return None


class Inst:
    pass


def make_instance(p, cs, ts, max_h=4, check_n2=False):
    f = random_f(p, 2, cs)
    if not squarefree(f, p):
        return None, "not squarefree"
    t = Tables(p, f)
    C = Curve(p, 2, f, t)
    rng = SM64(mix64(cs ^ 0xA5A55A5A ^ (p << 32)))
    cands = order_bsgs(C, t, rng)
    if len(cands) != 1:
        return None, "order ambiguous %r" % (cands,)
    order = cands[0]
    if check_n2:
        o2 = order_from_counts(p, n1(t), n2_brute(p, f))
        assert o2 == order, (o2, order)
    so = split_order(order, max_h)
    if so is None:
        return None, "#J=%d not h*r" % order
    h, r = so
    rng = SM64(mix64(ts ^ 0x123456789ABCDEF0 ^ (p << 20) ^ cs))
    G = IDENT
    for _ in range(64):
        d = C.mul_ltr(C.random_divisor(rng), h) if h > 1 else C.random_divisor(rng)
        if not is_identity(d) and is_identity(C.mul_ltr(d, r)):
            G = d; break
    assert not is_identity(G)
    tsec = 1 + rng.next() % (r - 1)
    T = C.mul_ltr(G, tsec)
    I = Inst()
    I.p, I.f, I.t, I.order, I.h, I.r, I.G, I.T, I.tsec, I.cs = p, f, t, order, h, r, G, T, tsec, cs
    return I, "ok"


def fresh_curve(I):
    return Curve(I.p, 2, I.f, I.t)


# --------------------------------------------------------------- LA (mod r)
class LACount:
    def __init__(self):
        self.mul = 0; self.add = 0; self.inv = 0
    def d(self):
        return {"mul_r": self.mul, "add_r": self.add, "inv_r": self.inv}


def peel(rows, nb):
    """rows: list of dict col->coef (base cols < nb; nb = T, nb+1 = S).  Return alive row ids."""
    alive = [True] * len(rows)
    colrows = {}
    for i, rw in enumerate(rows):
        for c in rw:
            if c < nb:
                colrows.setdefault(c, set()).add(i)
    stack = [c for c, s in colrows.items() if len(s) == 1]
    while stack:
        c = stack.pop()
        s = colrows.get(c)
        if not s or len(s) != 1:
            continue
        i = next(iter(s))
        alive[i] = False
        for c2 in rows[i]:
            if c2 < nb:
                colrows[c2].discard(i)
                if len(colrows[c2]) == 1:
                    stack.append(c2)
    ids = [i for i in range(len(rows)) if alive[i]]
    cols = set()
    for i in ids:
        for c in rows[i]:
            if c < nb:
                cols.add(c)
    return ids, cols


def stop_ok(rows, nb):
    ids, cols = peel(rows, nb)
    k = len(cols)
    return len(ids) >= k + 8 + k // 100, len(ids), k


def mulr(a, b, r, ct):
    if b == 1:
        return a
    if b == r - 1:
        return (-a) % r
    if a == 1:
        return b
    if a == r - 1:
        return (-b) % r
    ct.mul += 1
    return a * b % r


def sge(rows, nb, r, ct, w_max=8, fill_max=2.0):
    """rows: list of dict.  Markowitz-style merges of base columns of weight <= w_max."""
    rows = {i: dict(rw) for i, rw in enumerate(rows)}
    colrows = {}
    for i, rw in rows.items():
        for c in rw:
            if c < nb:
                colrows.setdefault(c, set()).add(i)
    nnz0 = sum(len(rw) for rw in rows.values())
    nnz = nnz0
    heap = [(len(s), c) for c, s in colrows.items()]
    heapq.heapify(heap)
    refused = set(); merged = peeled = 0
    while heap:
        w, c = heapq.heappop(heap)
        s = colrows.get(c)
        if not s:
            continue
        if len(s) != w:
            heapq.heappush(heap, (len(s), c)); continue
        if w > w_max:
            break
        if c in refused:
            continue
        if w == 1:
            i = next(iter(s))
            for c2 in rows[i]:
                if c2 < nb and c2 != c:
                    colrows[c2].discard(i)
                    heapq.heappush(heap, (len(colrows[c2]), c2))
            nnz -= len(rows[i]); del rows[i]; del colrows[c]; peeled += 1
            continue
        piv = min(s, key=lambda i: (len(rows[i]), i))
        rp = rows[piv]
        inc = (w - 1) * (len(rp) - 2) - len(rp)
        if nnz + inc > fill_max * nnz0:
            refused.add(c); continue
        cp = rp[c]
        for i in list(s):
            if i == piv:
                continue
            ri = rows[i]; ci = ri[c]
            old = len(ri)
            new = {}
            for k2, v in ri.items():
                new[k2] = mulr(v, cp, r, ct)
            for k2, v in rp.items():
                term = mulr(v, ci, r, ct)
                ct.add += 1
                new[k2] = (new.get(k2, 0) - term) % r
            new = {k2: v for k2, v in new.items() if v}
            for k2 in ri:
                if k2 < nb and k2 not in new:
                    colrows[k2].discard(i)
                    heapq.heappush(heap, (len(colrows[k2]), k2))
            for k2 in new:
                if k2 < nb:
                    colrows.setdefault(k2, set()).add(i)
            rows[i] = new
            nnz += len(new) - old
            for k2 in new:
                if k2 < nb:
                    heapq.heappush(heap, (len(colrows[k2]), k2))
        for k2 in rp:
            if k2 < nb:
                colrows[k2].discard(piv)
                if k2 != c:
                    heapq.heappush(heap, (len(colrows[k2]), k2))
        nnz -= len(rp); del rows[piv]; colrows.pop(c, None); merged += 1
    out = [rw for rw in rows.values() if rw]
    return out, {"merged": merged, "peeled": peeled, "refused": len(refused), "nnz_in": nnz0, "nnz_out": nnz}


def dense_kernel(rows, colorder, r, ct, rng):
    """Return (z dict) with z[S]=1, or None.  colorder ends with T, S."""
    idx = {c: j for j, c in enumerate(colorder)}
    k = len(colorder)
    M = []
    for rw in rows:
        v = [0] * k
        for c, x in rw.items():
            v[idx[c]] = x
        M.append(v)
    piv_of = {}
    rowi = 0
    for j in range(k):
        pr = None
        for i in range(rowi, len(M)):
            if M[i][j]:
                pr = i; break
        if pr is None:
            continue
        M[rowi], M[pr] = M[pr], M[rowi]
        ct.inv += 1
        iv = pow(M[rowi][j], r - 2, r)
        rowp = M[rowi]
        for jj in range(j, k):
            if rowp[jj]:
                rowp[jj] = mulr(rowp[jj], iv, r, ct)
        for i in range(len(M)):
            if i != rowi and M[i][j]:
                c = M[i][j]; Mi = M[i]
                for jj in range(j, k):
                    if rowp[jj]:
                        Mi[jj] = (Mi[jj] - mulr(rowp[jj], c, r, ct)) % r
                        ct.add += 1
        piv_of[j] = rowi
        rowi += 1
        if rowi == len(M):
            break
    S, T = k - 1, k - 2
    if S in piv_of or T not in piv_of:
        return None
    z = [0] * k
    for j in range(k):
        if j not in piv_of:
            z[j] = 1 if j == S else rng.next() % r
    for j, i in piv_of.items():
        s = 0
        for jj in range(j + 1, k):
            if M[i][jj] and z[jj]:
                s = (s + mulr(M[i][jj], z[jj], r, ct)) % r
                ct.add += 1
        z[j] = (-s) % r
    return z[T]


def matvec(rows, x, r, ct):
    out = []
    for rw in rows:
        s = 0
        for j, v in rw:
            s = (s + mulr(v, x[j], r, ct)) % r
            ct.add += 1
        out.append(s)
    return out


def berlekamp_massey(seq, r, ct):
    C = [1]; B = [1]; L = 0; m = 1; b = 1
    for n in range(len(seq)):
        d = seq[n]
        for i in range(1, L + 1):
            d = (d + mulr(C[i], seq[n - i], r, ct)) % r
            ct.add += 1
        if d == 0:
            m += 1; continue
        ct.inv += 1
        coef = d * pow(b, r - 2, r) % r; ct.mul += 1
        Tm = list(C)
        if len(C) < len(B) + m:
            C += [0] * (len(B) + m - len(C))
        for i in range(len(B)):
            C[i + m] = (C[i + m] - mulr(coef, B[i], r, ct)) % r
            ct.add += 1
        if 2 * L <= n:
            L = n + 1 - L; B = Tm; b = d; m = 1
        else:
            m += 1
    return C[:L + 1], L


def wiedemann_kernel(rows, colorder, r, ct, rng, tries=3):
    idx = {c: j for j, c in enumerate(colorder)}
    k = len(colorder)
    allrows = [[(idx[c], v) for c, v in rw.items()] for rw in rows]
    if len(allrows) < k:
        return None
    order = list(range(len(allrows)))
    for attempt in range(tries):
        if attempt > 0:
            for i in range(len(order) - 1, 0, -1):
                j = rng.next() % (i + 1); order[i], order[j] = order[j], order[i]
        Srows = [allrows[i] for i in order[:k]]
        v = [rng.next() % r for _ in range(k)]
        u = [rng.next() % r for _ in range(k)]
        seq = []
        w = v
        for _ in range(2 * k + 2):
            s = 0
            for a, b in zip(u, w):
                if b:
                    s = (s + mulr(a, b, r, ct)) % r; ct.add += 1
            seq.append(s)
            w = matvec(Srows, w, r, ct)
        C, L = berlekamp_massey(seq, r, ct)
        # connection poly C(x)=1+c1 x+..+cL x^L ; minimal poly f(x)=x^L C(1/x): coeffs f_i = C[L-i]
        fcoef = [C[L - i] if L - i < len(C) else 0 for i in range(L + 1)]
        e = 0
        while e <= L and fcoef[e] == 0:
            e += 1
        f1 = fcoef[e:]
        y = [mulr(f1[-1], x, r, ct) for x in v]
        for c in reversed(f1[:-1]):
            y = matvec(Srows, y, r, ct)
            for i in range(k):
                y[i] = (y[i] + mulr(c, v[i], r, ct)) % r; ct.add += 1
        if not any(y):
            continue
        z = None
        cur = y
        for _ in range(e + 2):
            nxt = matvec(Srows, cur, r, ct)
            if not any(nxt):
                z = cur; break
            cur = nxt
        if z is None:
            continue
        full = matvec(allrows, z, r, ct)
        if any(full) or z[k - 1] == 0:
            continue
        ct.inv += 1
        iv = pow(z[k - 1], r - 2, r)
        return mulr(z[k - 2], iv, r, ct)
    return None


def solve(rowsbase, nb, r, rng, use_sge=True, dense_max=1024, force_wied=False):
    """rowsbase: list of (entries dict col->coef, tau, rho).  Return (t or None, counts, stats)."""
    ct = LACount()
    rows = []
    for ent, tau, rho in rowsbase:
        d = dict(ent)
        if tau % r:
            d[nb] = tau % r
        if rho % r:
            d[nb + 1] = (-rho) % r
        rows.append(d)
    ids, cols = peel(rows, nb)
    rows = [rows[i] for i in ids]
    st = {"peel_rows": len(rows), "peel_cols": len(cols)}
    if use_sge:
        rows, s2 = sge(rows, nb, r, ct)
        st.update(s2)
    colset = set()
    for rw in rows:
        colset.update(c for c in rw if c < nb)
    colorder = sorted(colset) + [nb, nb + 1]
    st["rem_rows"], st["rem_cols"] = len(rows), len(colorder)
    if len(colorder) <= dense_max and not force_wied:
        st["solver"] = "dense"
        t = dense_kernel(rows, colorder, r, ct, rng)
    else:
        st["solver"] = "wiedemann"
        t = wiedemann_kernel(rows, colorder, r, ct, rng)
    return t, ct, st


# --------------------------------------------------------------- arms
def stream(tag, p, cs, ss):
    return SM64(mix64(mix64(tag ^ (p << 20)) ^ (cs << 8) ^ ss))


def gprice(ops):
    return LANGE_ADD * ops["add"] + LANGE_DBL * ops["dbl"]


def split_points(I, D, nat):
    """Split test + roots of a reduced divisor (table based).  Returns list of (x,y) or None."""
    p, t = I.p, I.t
    u, v = D
    deg = len(u) - 1
    v0 = v[0] if len(v) > 0 else 0
    v1 = v[1] if len(v) > 1 else 0
    if deg == 1:
        x = (-u[0]) % p
        return [(x, v0)]
    if deg != 2:
        return None
    disc = (u[1] * u[1] - 4 * u[0]) % p; nat.mul += 1
    if t.chi[disc] != 1:
        return None
    s = t.sq[disc]
    inv2 = (p + 1) // 2
    xs = [((-u[1] + s) * inv2) % p, ((-u[1] - s) * inv2) % p]; nat.mul += 2
    pts = []
    for x in xs:
        y = (v1 * x + v0) % p; nat.mul += 1
        pts.append((x, y))
    return pts


def row_of(I, pts):
    ent = {}
    p = I.p
    for x, y in pts:
        if y == 0:
            continue
        c = I.t.col[x]
        e = 1 if y == I.t.sq[I.t.fx[x]] else -1
        ent[c] = ent.get(c, 0) + e
    return {c: e % I.r for c, e in ent.items() if e % I.r}


def verify(I, C, t):
    """Charged: one scalar multiplication [t]G on the arm's counted curve."""
    return t is not None and C.mul_ltr(I.G, t) == I.T


def negctl(I, t):
    """Uncounted negative control: [t+1]G must differ from T."""
    if t is None:
        return None
    return fresh_curve(I).mul_ltr(I.G, (t + 1) % I.r) != I.T


def arm_W(I, ss, max_attempts=50, la_kw=None):
    la_kw = la_kw or {}
    p, r, h = I.p, I.r, I.h
    C = fresh_curve(I)
    rng = stream(0x57, p, I.cs, ss)
    nat = Counter()
    nb = len(I.t.base)
    restart = min(4096, max(16, math.isqrt(r) // 2))
    short = min(r, 4096)
    jumps = []
    for _ in range(16):
        a = 1 + rng.next() % (short - 1); b = 1 + rng.next() % (short - 1)
        jumps.append((C.cantor(C.mul_ltr(I.G, a), C.mul_ltr(I.T, b)), a, b))
    rows = []; seen = set()
    cur = None; ca = cb = 0; since = restart
    trials = cand = splits = 0
    delta = max(8, nb // 64)
    next_check = delta
    la_tot = LACount(); attempts = 0; t = None; last = None; good = False
    while attempts < max_attempts:
        while len(rows) < next_check:
            if since >= restart:
                ca = 1 + rng.next() % (r - 1); cb = 1 + rng.next() % (r - 1)
                cur = C.cantor(C.mul_ltr(I.G, ca), C.mul_ltr(I.T, cb)); since = 0
            else:
                j = (mix64(key_of(cur)) >> 24) % 16
                m, aj, bj = jumps[j]
                cur = C.cantor(cur, m); ca = (ca + aj) % r; cb = (cb + bj) % r; since += 1
            trials += 1
            if is_identity(cur):
                continue
            if len(cur[0]) - 1 == 2:
                cand += 1
            pts = split_points(I, cur, nat)
            if pts is None:
                continue
            splits += 1
            ent = row_of(I, pts)
            if not ent:
                continue
            key = (tuple(sorted(ent.items())), ca, cb)
            if key in seen:
                continue
            seen.add(key)
            rows.append((ent, (-h * cb) % r, (h * ca) % r))
        ok, cr, cc = stop_ok([dict(e) for e, _, _ in rows], nb)
        if not ok:
            next_check = len(rows) + delta; continue
        attempts += 1
        t, ct, st = solve(rows, nb, r, rng, **la_kw)
        la_tot.mul += ct.mul; la_tot.add += ct.add; la_tot.inv += ct.inv
        last = st
        good = verify(I, C, t)
        if good:
            break
        next_check = len(rows) + delta
    return dict(arm="W", ok=good and t == I.tsec, negctl=negctl(I, t), trials=trials, cand=cand, splits=splits,
                rels=len(rows), attempts=attempts, ops=dict(C.ops), cantor_mul=C.ctr.mul,
                cantor_inv=C.ctr.inv, nat_mul=nat.mul, nat_inv=nat.inv, setup_mul=I.t.setup_mul,
                la=la_tot.d(), la_stats=last, nb=nb)


def arm_P(I, ss, max_attempts=50, la_kw=None, collect_rate=False):
    la_kw = la_kw or {}
    p, r, h, t = I.p, I.r, I.h, I.t
    C = fresh_curve(I)
    rng = stream(0x50, p, I.cs, ss)
    nat = Counter()
    nb = len(t.base)
    perm = list(t.base)
    for i in range(len(perm) - 1, 0, -1):
        j = rng.next() % (i + 1); perm[i], perm[j] = perm[j], perm[i]
    pos = 0
    rows = []; seen = set(); tuples = 0; per_tuple = []
    v1fail = 0; maxbucket = 0
    inv2 = None

    def one_tuple():
        nonlocal pos, tuples, v1fail, maxbucket
        x1, x2 = perm[pos], perm[pos + 1]; pos += 2
        y1, y2 = t.sq[t.fx[x1]], t.sq[t.fx[x2]]
        nat.inv += 1
        slope = (y2 - y1) * pow((x2 - x1) % p, p - 2, p) % p; nat.mul += 1
        a0 = (y1 - slope * x1) % p; nat.mul += 1          # a0(0)
        Pi = x1 * x2 % p; nat.mul += 1                       # Pi(0)
        dPi = (1 - x1 - x2) % p                              # Pi(1)-Pi(0)
        av = [0] * p; pv = [0] * p
        for x in range(p):                                    # forward differences (additions)
            av[x] = a0; pv[x] = Pi
            a0 = (a0 + slope) % p; Pi = (Pi + dPi) % p; dPi = (dPi + 2) % p
        xs = [x for x in t.base if x != x1 and x != x2]
        # batch inversion: 3(n-1) mul + 1 inv
        pre = [0] * len(xs); acc = 1
        for i, x in enumerate(xs):
            acc = acc * pv[x] % p if i else pv[x]
            pre[i] = acc
        nat.mul += len(xs) - 1
        nat.inv += 1
        ti = pow(pre[-1], p - 2, p)
        invs = [0] * len(xs)
        for i in range(len(xs) - 1, 0, -1):
            invs[i] = ti * pre[i - 1] % p; ti = ti * pv[xs[i]] % p
        invs[0] = ti
        nat.mul += 2 * (len(xs) - 1)
        buckets = {}
        for i, x in enumerate(xs):
            yc = t.sq[t.fx[x]]
            for y in (yc, p - yc):
                lam = (y - av[x]) * invs[i] % p; nat.mul += 1
                buckets.setdefault(lam, []).append((x, y))
        got = 0
        for lam, pts in buckets.items():
            maxbucket = max(maxbucket, len(pts))
            if len(pts) != 3:
                continue
            allp = pts + [(x1, y1), (x2, y2)]
            # V1 (uncounted): every point on the line y = a0 + lam*Pi and on the curve, 5 distinct x
            for (x, y) in allp:
                if (y - (av[x] + lam * pv[x])) % p or (y * y - t.fx[x]) % p:
                    v1fail += 1
            if len(set(x for x, _ in allp)) != 5:
                v1fail += 1
            ent = row_of(I, allp)
            key = tuple(sorted(ent.items()))
            nkey = tuple(sorted((c, (-e) % r) for c, e in ent.items()))
            if key in seen or nkey in seen:
                continue
            seen.add(key)
            rows.append((ent, 0, 0)); got += 1
        tuples += 1
        per_tuple.append(got)

    def descent(D0, mult, add_el, tau_is_t):
        trials = 0
        k = mult; D = D0
        while True:
            trials += 1
            pts = split_points(I, D, nat) if not is_identity(D) else None
            if pts is not None:
                ent = row_of(I, pts)
                if ent and all(c in core_cols for c in ent):
                    if tau_is_t:
                        return (ent, (-h * k) % r, 0), trials
                    return (ent, 0, (h * k) % r), trials
            D = C.cantor(D, add_el); k = (k + 1) % r

    attempts = 0; tsol = None; la_tot = LACount(); last = None; good = False
    desc_rows = []; desc_trials = 0
    while attempts < max_attempts:
        while True:
            if pos + 2 > len(perm):
                for i in range(len(perm) - 1, 0, -1):
                    j = rng.next() % (i + 1); perm[i], perm[j] = perm[j], perm[i]
                pos = 0
            one_tuple()
            if collect_rate:
                return dict(per_tuple=per_tuple, p=p, maxbucket=maxbucket, v1fail=v1fail, nat_mul=nat.mul)
            ok, cr, cc = stop_ok([dict(e) for e, _, _ in rows], nb)
            if ok:
                break
        ids, core_cols = peel([dict(e) for e, _, _ in rows], nb)
        if not desc_rows:
            a = 1 + rng.next() % (r - 1)
            rg, tg = descent(C.mul_ltr(I.G, a), a, I.G, False)
            b = 1 + rng.next() % (r - 1)
            rt, tt = descent(C.mul_ltr(I.T, b), b, I.T, True)
            desc_rows = [rg, rt]; desc_trials = tg + tt
        attempts += 1
        tsol, ct, st = solve(rows + desc_rows, nb, r, rng, **la_kw)
        la_tot.mul += ct.mul; la_tot.add += ct.add; la_tot.inv += ct.inv
        last = st
        good = verify(I, C, tsol)
        if good:
            break
        # failure: one more tuple, then retry
        if pos + 2 > len(perm):
            pos = 0
        one_tuple()
    return dict(arm="P", ok=good and tsol == I.tsec, negctl=negctl(I, tsol), tuples=tuples, rels=len(rows),
                per_tuple=per_tuple, attempts=attempts, desc_trials=desc_trials, ops=dict(C.ops),
                cantor_mul=C.ctr.mul, cantor_inv=C.ctr.inv, nat_mul=nat.mul, nat_inv=nat.inv,
                setup_mul=I.t.setup_mul, la=la_tot.d(), la_stats=last, v1fail=v1fail,
                maxbucket=maxbucket, nb=nb)


def canon(C, D, a, b, r):
    u, v = D
    nv = tuple((-x) % C.p for x in v)
    if tuple(reversed(nv)) < tuple(reversed(v)):
        return (u, nv), (-a) % r, (-b) % r
    return D, a, b


def theta_bits_of(r):
    return min(8, max(2, r.bit_length() // 4 - 2))


def arm_R(I, ss, nj=32, theta_bits=None, max_steps=None):
    p, r = I.p, I.r
    C = fresh_curve(I)
    rng = stream(0x52, p, I.cs, ss)
    theta_bits = theta_bits if theta_bits is not None else theta_bits_of(r)
    # jumps M_j = [alpha_j] G, alpha_j in [1, 2^12), from the table G*2^i (i < 12)
    tab = [I.G]
    for i in range(11):
        tab.append(C.cantor(tab[-1], tab[-1]))
    jumps = []
    for _ in range(nj):
        a = 1 + rng.next() % 4095
        D = IDENT
        for i in range(12):
            if (a >> i) & 1:
                D = C.cantor(D, tab[i])
        jumps.append((D, a, 0))
    setup_ops = dict(C.ops)
    dmask = (1 << theta_bits) - 1
    timeout = 20 << theta_bits
    table = {}
    steps = restarts = esc2 = fruitless_dp = 0
    max_steps = max_steps or 50 * math.isqrt(r) + 10000

    def start():
        a = 1 + rng.next() % (r - 1); b = 1 + rng.next() % (r - 1)
        D = C.cantor(C.mul_ltr(I.G, a), C.mul_ltr(I.T, b))
        return canon(C, D, a, b, r)

    W, a, b = start(); prev = None; since = 0; tsol = None
    while steps < max_steps:
        if is_identity(W):
            if b:
                tsol = (-a) * pow(b, r - 2, r) % r; break
            W, a, b = start(); prev = None; since = 0; restarts += 1; continue
        k = key_of(W)
        if (mix64(k ^ 0xD15) & dmask) == 0:
            if W in table:
                a2, b2 = table[W]
                if (b - b2) % r:
                    tsol = (a2 - a) * pow((b - b2) % r, r - 2, r) % r
                    break
                fruitless_dp += 1
                W, a, b = start(); prev = None; since = 0; restarts += 1; continue
            table[W] = (a, b)
            since = 0
        if since > timeout:
            W, a, b = start(); prev = None; since = 0; restarts += 1; continue
        j = (mix64(k) >> 24) % nj
        m, aj, bj = jumps[j]
        N, na, nb_ = canon(C, C.cantor(W, m), (a + aj) % r, (b + bj) % r, r)
        steps += 1; since += 1
        if prev is not None and N == prev[0]:
            esc2 += 1
            Wm = min((W, a, b), prev, key=lambda z: key_of(z[0]))
            N, na, nb_ = canon(C, C.cantor(Wm[0], Wm[0]), 2 * Wm[1] % r, 2 * Wm[2] % r, r)
        prev = (W, a, b)
        W, a, b = N, na, nb_
    good = verify(I, C, tsol)
    return dict(arm="R", ok=good and tsol == I.tsec, negctl=negctl(I, tsol), steps=steps, restarts=restarts,
                esc2=esc2, fruitless_dp=fruitless_dp, dps=len(table), ops=dict(C.ops),
                setup_ops=setup_ops, cantor_mul=C.ctr.mul, cantor_inv=C.ctr.inv,
                rho_pred=math.sqrt(math.pi * r / 4))


def total(res, kappa):
    g = gprice(res["ops"])
    nat = res.get("nat_mul", 0) + 10 * res.get("nat_inv", 0) + res.get("setup_mul", 0)
    la = res.get("la", {"mul_r": 0, "inv_r": 0})
    return g + nat + kappa * (la["mul_r"] + 10 * la["inv_r"])
