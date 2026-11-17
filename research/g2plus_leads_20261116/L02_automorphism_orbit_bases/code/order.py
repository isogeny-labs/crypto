"""Curves, group orders, admission, lambda (SPEC section 5)."""
import math
import ff, jac
from ff import uncounted
from poly import peval, xgcd, trim
from seeds import rng

class Curve:
    def __init__(self, d):
        self.family = d.get("family"); self.curve_id = d.get("curve_id")
        self.p = d["p"]; self.q = d["q"]; self.g = d["g"]; self.f = list(d["f"])
        self.a = d.get("a"); self.j = d.get("j")
        self.zeta = d.get("zeta", 1); self.lam = d.get("lam", 1)
        self.h = d.get("h"); self.r = d.get("r"); self.N = d.get("N")
        self.nA = d.get("nA"); self.delta0 = d.get("delta0", 0); self.nB = d.get("nB")
        self.bits = d.get("bits")
        self.setup()
    def setup(self):
        cm = self.family in ("F5", "F7")
        self.aq = self.q if cm else 1
        self.ZP = [pow(self.zeta, e, self.p) for e in range(self.aq)]
        self.LP = [pow(self.lam, k, self.r) for k in range(self.aq)] if self.r else [1] * self.aq
    def todict(self):
        return {"family": self.family, "curve_id": self.curve_id, "p": self.p, "q": self.q,
                "g": self.g, "f": self.f, "a": self.a, "j": self.j, "zeta": self.zeta,
                "lam": self.lam, "h": self.h, "r": self.r, "N": self.N, "nA": self.nA,
                "delta0": self.delta0, "nB": self.nB, "bits": self.bits}

def is_prime(n):
    if n < 2: return False
    if n % 2 == 0: return n == 2
    i = 3
    while i * i <= n:
        if n % i == 0: return False
        i += 2
    return True

def next_prime_cong(lo, m, res=1):
    n = lo
    while True:
        if n % m == res and is_prime(n):
            return n
        n += 1

def prim_root(p):
    fs = []; n = p - 1; d = 2
    while d * d <= n:
        if n % d == 0:
            fs.append(d)
            while n % d == 0: n //= d
        d += 1
    if n > 1: fs.append(n)
    gam = 2
    while any(pow(gam, (p - 1) // l, p) == 1 for l in fs):
        gam += 1
    return gam

def largest_prime_factor(n):
    best = 1; d = 2
    while d * d <= n:
        while n % d == 0:
            best = d; n //= d
        d += 1
    if n > 1: best = max(best, n)
    return best

def legendre_table(p):
    t = [-1] * p
    t[0] = 0
    for x in range(1, p):
        t[x * x % p] = 1
    return t

def fvals(f, p):
    out = []
    for x in range(p):
        r = 0
        for c in reversed(f):
            r = (r * x + c) % p
        out.append(r)
    return out

def count_N1(f, p, leg=None):
    leg = leg or legendre_table(p)
    return 1 + sum(1 + leg[v] for v in fvals(f, p))

def count_N2(f, p, leg=None):
    """#C(F_{p^2}) incl. infinity; uses conjugation symmetry x1 <-> -x1 (exact)."""
    leg = leg or legendre_table(p)
    dn = 2
    while leg[dn] != -1: dn += 1
    deg = len(f) - 1
    xs0 = list(range(p))
    total = 0
    for x1 in range((p - 1) // 2 + 1):
        w = 1 if x1 == 0 else 2
        dx1 = dn * x1 % p
        C0 = [f[deg]] * p; C1 = [0] * p
        for i in range(deg - 1, -1, -1):
            fi = f[i]
            C0, C1 = ([(c0 * x0 + dx1 * c1 + fi) % p for c0, c1, x0 in zip(C0, C1, xs0)],
                      [(c0 * x1 + c1 * x0) % p for c0, c1, x0 in zip(C0, C1, xs0)])
        s = 0
        for c0, c1 in zip(C0, C1):
            if c0 == 0 and c1 == 0:
                s += 1
            else:
                s += 1 + leg[(c0 * c0 - dn * c1 * c1) % p]
        total += w * s
    return 1 + total

def ceil_sqrt(m):
    s = math.isqrt(m)
    return s if s * s == m else s + 1

def bsgs_candidates(C, D, N0, lo, hi):
    p, g = C.p, C.g
    n = hi - lo + 1
    m = ceil_sqrt(n)
    table = {}
    R = jac.IDENT
    for j in range(m):
        table.setdefault(tuple(jac.key(R, g, p)), []).append(j)
        R = jac.add(R, D, C)
    Q = jac.smul(N0 + lo, D, C)
    S = jac.smul(m, D, C)
    cands = []
    for i in range(-(-n // m) + 1):
        kq = tuple(jac.key(jac.neg(Q), g, p))
        for j in table.get(kq, ()):
            c = N0 + lo + i * m + j
            if N0 + lo <= c <= N0 + hi:
                cands.append(c)
        Q = jac.add(Q, S, C)
    return cands

def jacobian_order(C, N1=None, N2=None):
    """Returns (N or None, info). Everything uncounted."""
    with uncounted():
        ff.set_p(C.p)
        p, g = C.p, C.g
        leg = legendre_table(p)
        N1 = count_N1(C.f, p, leg)
        s1 = p + 1 - N1
        a1 = -s1
        if g == 2:
            N0 = 1 + a1 + p * a1 + p * p
            lo = ceil_sqrt(4 * p * a1 * a1) - 2 * p - 1
            hi = (a1 * a1) // 4 + 2 * p + 1
        else:
            N2 = count_N2(C.f, p, leg)
            s2 = p * p + 1 - N2
            e2 = (s1 * s1 - s2) // 2
            a2 = e2
            N0 = 1 + a1 + a2 + p * a2 + p * p * a1 + p ** 3
            b = ceil_sqrt(400 * p ** 3)
            lo, hi = -b - 1, b + 1
        rnd = rng("order:" + C.curve_id)
        divs = [jac.random_divisor(C, rnd) for _ in range(32)]
        cands = bsgs_candidates(C, divs[0], N0, lo, hi)
        surv = []
        for c in cands:
            if all(jac.is_ident(jac.smul(c, D, C)) for D in divs):
                surv.append(c)
        surv = sorted(set(surv))
        if len(surv) == 1:
            return surv[0], {"N1": N1}
        return None, {"N1": N1, "survivors": len(surv)}

def squarefree(f, p):
    with uncounted():
        ff.set_p(p)
        df = trim([(i * c) % p for i, c in enumerate(f)][1:])
        d, _, _ = xgcd(f, df)
        return len(d) == 1

def find_lambda(C):
    """Unique lam in primitive q-th roots mod r with rho(G0) = [lam] G0; None on failure."""
    import aut
    q, r, h = C.q, C.r, C.h
    if (r - 1) % q != 0:
        return None
    with uncounted():
        ff.set_p(C.p)
        l0 = 1; w = 2
        while True:
            l0 = pow(w, (r - 1) // q, r)
            if l0 != 1: break
            w += 1
        cands = [pow(l0, j, r) for j in range(1, q)]
        rnd = rng("order:" + C.curve_id)
        G0 = jac.IDENT
        while jac.is_ident(G0):
            D = jac.random_divisor(C, rnd)
            G0 = jac.smul(h, D, C)
        img = aut.rho_apply(G0, C)
        ok = [l for l in cands if jac.smul(l, G0, C) == img]
    return ok[0] if len(ok) == 1 else None

def build_curve(family, p, q, f, ident, bits=None, admit=True):
    """ident: dict with curve_id and a (CM) or j (control). Returns (Curve|None, reason)."""
    g = (q - 1) // 2
    ff.set_p(p)
    if not squarefree(f, p):
        return None, "not_squarefree"
    cm = family in ("F5", "F7")
    d = {"family": family, "curve_id": ident["curve_id"], "p": p, "q": q, "g": g, "f": f,
         "a": ident.get("a"), "j": ident.get("j"), "bits": bits}
    if cm:
        gam = prim_root(p)
        d["zeta"] = pow(gam, (p - 1) // q, p)
    C = Curve(d)
    N, info = jacobian_order(C)
    if N is None:
        return None, "order_ambiguous"
    r = largest_prime_factor(N); h = N // r
    if admit and not (r > h and h <= 64):
        return None, "not_near_prime"
    C.N, C.h, C.r = N, h, r
    with uncounted():
        ff.set_p(p)
        vals = fvals(f, p)
        leg = legendre_table(p)
        C.nA = sum(1 for v in vals if leg[v] == 1)
        C.delta0 = 1 if (cm and leg[vals[0]] == 1) else 0
        C.nB = (C.nA - C.delta0) // q if cm else C.nA
    C.setup()
    if cm:
        lam = find_lambda(C)
        if lam is None:
            return None, "lambda_fail"
        C.lam = lam
        C.setup()
    return C, "ok"
