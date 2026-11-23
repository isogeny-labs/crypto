"""Toy builder for Weil-descended point-decomposition systems over F_{2^n}
in three presentations, exported to msolve (char 2, field equations appended).

Presentations (m summands, typed affine-coset factor bases x_i in u_i + V_i):
  'xonly'  : chained Semaev S3 in x-coordinates only (programme standard; m=2,3)
             S3(X1,X2,X3) = (X1X2+X1X3+X2X3)^2 + X1X2X3 + b   (y^2+xy=x^3+a2 x^2+b)
  'rr'     : Riemann-Roch star: F in L((m+1)O) with top coefficient 1, F(-R)=0,
             F(P_i)=0, plus curve equations for y_i  (Boolean quadratic for m<=4)
  'nofield': same as xonly but WITHOUT field equations (closure-point census)

Field elements are Python ints (bit i = coefficient of t^i), modulus from PARI ffinit.
Boolean polynomials over F_2: dict {frozenset(varidx): 1}.
'Descended' polynomials: dict {frozenset(varidx): field_elem} and expanded bitwise.
"""
import random, subprocess, re, os, sys, time, json

MODS = {9: (9,8,6,5,4,1,0), 11: (11,10,8,4,3,2,0), 13: (13,12,10,7,4,3,0),
        15: (15,13,8,5,2,1,0), 17: (17,16,14,13,12,11,10,9,7,5,3,1,0),
        19: (19,18,16,11,8,6,4,1,0), 21: (21,12,11,9,7,6,5,3,0),
        23: (23,22,20,16,8,7,6,4,0)}

class GF2n:
    def __init__(self, n):
        self.n = n
        self.mod = 0
        for e in MODS[n]:
            self.mod |= 1 << e
        self.hi = 1 << n
    def mul(self, a, b):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a & self.hi:
                a ^= self.mod
        return r
    def sq(self, a):
        return self.mul(a, a)
    def pow(self, a, e):
        r = 1
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r
    def inv(self, a):
        assert a
        return self.pow(a, (1 << self.n) - 2)
    def trace(self, a):
        t = a
        s = a
        for _ in range(self.n - 1):
            t = self.sq(t)
            s ^= t
        return s  # 0 or 1
    def halftrace(self, c):
        # n odd: z with z^2+z=c when Tr(c)=0
        assert self.n % 2 == 1
        z = c
        t = c
        for _ in range((self.n - 1) // 2):
            t = self.sq(self.sq(t))
            z ^= t
        return z
    def rand(self, rng):
        return rng.randrange(1, self.hi)

class Curve:
    """y^2 + xy = x^3 + a2 x^2 + b over F_{2^n}; points as (x,y) or None for O."""
    def __init__(self, F, a2, b):
        self.F, self.a2, self.b = F, a2, b
    def on(self, P):
        if P is None:
            return True
        F = self.F; x, y = P
        lhs = F.sq(y) ^ F.mul(x, y)
        rhs = F.mul(F.sq(x), x) ^ (F.sq(x) if self.a2 else 0) ^ self.b
        return lhs == rhs
    def neg(self, P):
        if P is None:
            return None
        return (P[0], P[0] ^ P[1])
    def add(self, P, Q):
        F = self.F
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 ^ y2 == x1:  # Q = -P
                return None
            # doubling
            if x1 == 0:
                return None
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sq(lam) ^ lam ^ (1 if self.a2 else 0)
            y3 = F.sq(x1) ^ F.mul(lam ^ 1, x3)
            return (x3, y3)
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ (1 if self.a2 else 0)
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)
    def lift_x(self, x):
        """return y with (x,y) on curve, or None"""
        F = self.F
        if x == 0:
            # y^2 = b
            return F.pow(self.b, 1 << (F.n - 1))
        rhs = F.mul(F.sq(x), x) ^ (F.sq(x) if self.a2 else 0) ^ self.b
        c = F.mul(rhs, F.inv(F.sq(x)))
        if F.trace(c):
            return None
        z = F.halftrace(c)
        return F.mul(x, z)
    def rand_point(self, rng):
        F = self.F
        while True:
            x = F.rand(rng)
            y = self.lift_x(x)
            if y is not None:
                return (x, y)

# ---------- descended polynomial arithmetic ----------
# a 'dpoly' is dict: frozenset(bool var indices) -> field element (int, nonzero)

def dp_add(a, b):
    r = dict(a)
    for k, v in b.items():
        w = r.get(k, 0) ^ v
        if w:
            r[k] = w
        else:
            r.pop(k, None)
    return r

def dp_mul(F, a, b):
    r = {}
    for ka, va in a.items():
        for kb, vb in b.items():
            k = ka | kb  # b^2 = b
            v = F.mul(va, vb)
            w = r.get(k, 0) ^ v
            if w:
                r[k] = w
            else:
                r.pop(k, None)
    return r

def dp_const(c):
    return {frozenset(): c} if c else {}

def dp_scale(F, a, c):
    if not c:
        return {}
    return {k: F.mul(v, c) for k, v in a.items()}

def dp_linear(u, vars_coeffs):
    """u + sum_j b_j v_j ; vars_coeffs = list of (varidx, v_j)"""
    r = dp_const(u)
    for j, v in vars_coeffs:
        if v:
            r = dp_add(r, {frozenset([j]): v})
    return r

def dp_expand_bits(F, a):
    """Weil descent: list of n Boolean polynomials (sets of monomials)."""
    out = [set() for _ in range(F.n)]
    for k, v in a.items():
        i = 0
        while v:
            if v & 1:
                out[i].add(k)
            v >>= 1
            i += 1
    return [s for s in out]

def bool_poly_str(monos, names):
    if not monos:
        return "0"
    terms = []
    for mono in sorted(monos, key=lambda s: (len(s), sorted(s))):
        if not mono:
            terms.append("1")
        else:
            terms.append("*".join(names[i] for i in sorted(mono)))
    return "+".join(terms)

def write_msolve(path, names, polys, field_eqs=True):
    lines = [",".join(names), "2"]
    body = [bool_poly_str(p, names) for p in polys if p]
    if field_eqs:
        body += [f"{v}^2+{v}" for v in names]
    lines.append(",\n".join(body))
    with open(path, "w") as f:
        f.write("\n".join(lines) + "\n")

def run_msolve(path, out, timeout=600, extra=("-g", "2")):
    cmd = ["msolve", "-v", "2", *extra, "-f", path, "-o", out]
    t0 = time.time()
    try:
        pr = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
        txt = pr.stdout + pr.stderr
        status = "ok"
    except subprocess.TimeoutExpired as e:
        def _dec(v):
            if v is None:
                return ""
            return v.decode(errors="ignore") if isinstance(v, (bytes, bytearray)) else str(v)
        txt = _dec(e.stdout) + _dec(e.stderr)
        status = "timeout"
    dt = time.time() - t0
    degs = []
    for line in txt.splitlines():
        m = re.match(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)", line)
        if m:
            degs.append(int(m.group(1)))
    gb = None
    if status == "ok" and os.path.exists(out):
        gb = open(out).read()
    return {"status": status, "time": round(dt, 2), "maxdeg": max(degs) if degs else None,
            "degs": degs, "gb_is_one": (gb is not None and gb.strip().startswith("[1]")),
            "gb_len": (len([l for l in gb.splitlines() if l.strip() and not l.startswith('#')]) if gb else None)}

# ---------- instance generation ----------

def make_bases(F, m, l, rng):
    """m typed affine cosets u_i + span(w_i1..w_il), all m(l+1) vectors F_2-independent.
    Use monomial basis elements t^k for simplicity (independent)."""
    need = m * (l + 1)
    assert need <= F.n, "need m(l+1) <= n"
    idx = list(range(F.n))
    rng.shuffle(idx)
    bases = []
    for i in range(m):
        u = 1 << idx.pop()
        ws = [1 << idx.pop() for _ in range(l)]
        bases.append((u, ws))
    return bases

def coset_points(E, u, ws):
    """all points with x in u+span(ws) that lift (both y's)"""
    F = E.F
    pts = []
    l = len(ws)
    for mask in range(1 << l):
        x = u
        for j in range(l):
            if mask >> j & 1:
                x ^= ws[j]
        y = E.lift_x(x)
        if y is not None:
            pts.append((x, y)); pts.append((x, x ^ y))
    return pts

def all_decompositions(E, bases, R):
    """brute force: tuples (P_1..P_m) with P_i in coset i and sum R"""
    m = len(bases)
    lists = [coset_points(E, u, ws) for (u, ws) in bases]
    sols = []
    def rec(i, acc, S):
        if i == m:
            if S == R:
                sols.append(tuple(acc))
            return
        for P in lists[i]:
            rec(i + 1, acc + [P], E.add(S, P))
    rec(0, [], None)
    return sols

def build_xonly(E, bases, R, field_eqs=True):
    """chained S3 in x-coordinates; m=2 or 3. Returns (names, polys)."""
    F = E.F; m = len(bases)
    names = []; var = 0
    xs = []
    for i, (u, ws) in enumerate(bases):
        vc = []
        for j, w in enumerate(ws):
            names.append(f"b{i}_{j}"); vc.append((var, w)); var += 1
        xs.append(dp_linear(u, vc))
    xR = dp_const(R[0])
    def S3(X1, X2, X3):
        s = dp_add(dp_add(dp_mul(F, X1, X2), dp_mul(F, X1, X3)), dp_mul(F, X2, X3))
        return dp_add(dp_add(dp_mul(F, s, s), dp_mul(F, dp_mul(F, X1, X2), X3)), dp_const(E.b))
    polys = []
    if m == 2:
        polys += dp_expand_bits(F, S3(xs[0], xs[1], xR))
    elif m == 3:
        vc = []
        for k in range(F.n):
            names.append(f"y{k}"); vc.append((var, 1 << k)); var += 1
        Y = dp_linear(0, vc)
        polys += dp_expand_bits(F, S3(xs[0], xs[1], Y))
        polys += dp_expand_bits(F, S3(Y, xs[2], xR))
    else:
        raise NotImplementedError
    return names, polys

def build_rr(E, bases, R):
    """Riemann-Roch star: F in L((m+1)O), top coefficient 1, F(-R)=0, F(P_i)=0, curve eqs."""
    F = E.F; m = len(bases)
    # basis of L((m+1)O) by pole order: 1,x,y,x^2,xy,x^3,x^2y,...
    mons = [(0, 0), (1, 0), (0, 1), (2, 0), (1, 1), (3, 0), (2, 1), (4, 0)]  # (a,b): x^a y^b
    basis = mons[: m + 1]
    top = basis[-1]
    names = []; var = 0
    # coefficient unknowns for non-top, non-constant monomials (constant fixed by F(-R)=0)
    coefs = {}
    for mon in basis[1:-1]:
        vc = []
        for k in range(F.n):
            names.append(f"c{mon[0]}{mon[1]}_{k}"); vc.append((var, 1 << k)); var += 1
        coefs[mon] = dp_linear(0, vc)
    coefs[top] = dp_const(1)
    def mon_eval_const(mon, P):
        x, y = P
        return F.mul(F.pow(x, mon[0]), F.pow(y, mon[1]))
    # c0 from F(-R) = 0 : c0 = sum_{mon != const} c_mon * mon(-R)
    negR = E.neg(R)
    c0 = {}
    for mon in basis[1:]:
        c0 = dp_add(c0, dp_scale(F, coefs[mon], mon_eval_const(mon, negR)))
    coefs[(0, 0)] = c0
    polys = []
    for i, (u, ws) in enumerate(bases):
        vc = []
        for j, w in enumerate(ws):
            names.append(f"b{i}_{j}"); vc.append((var, w)); var += 1
        X = dp_linear(u, vc)
        vc = []
        for k in range(F.n):
            names.append(f"y{i}_{k}"); vc.append((var, 1 << k)); var += 1
        Y = dp_linear(0, vc)
        X2 = dp_mul(F, X, X)
        # curve: y^2 + xy + x^3 + a2 x^2 + b = 0
        cur = dp_add(dp_add(dp_mul(F, Y, Y), dp_mul(F, X, Y)), dp_mul(F, X2, X))
        if E.a2:
            cur = dp_add(cur, X2)
        cur = dp_add(cur, dp_const(E.b))
        polys += dp_expand_bits(F, cur)
        # F(P_i) = sum c_mon x^a y^b
        powsx = {0: dp_const(1), 1: X, 2: X2, 3: dp_mul(F, X2, X), 4: dp_mul(F, X2, X2)}
        Fp = {}
        for mon in basis:
            term = dp_mul(F, coefs[mon], powsx[mon[0]])
            if mon[1]:
                term = dp_mul(F, term, Y)
            Fp = dp_add(Fp, term)
        polys += dp_expand_bits(F, Fp)
    return names, polys

def make_instance(n, m, l, seed, want="rand", a2=None):
    rng = random.Random(seed)
    F = GF2n(n)
    a2 = rng.randrange(2) if a2 is None else a2
    E = Curve(F, a2, F.rand(rng))
    bases = make_bases(F, m, l, rng)
    if want == "sat":
        pts = [rng.choice(coset_points(E, u, ws)) for (u, ws) in bases]
        R = None
        for P in pts:
            R = E.add(R, P)
        if R is None:
            return make_instance(n, m, l, seed + 1000, want, a2)
    else:
        R = E.rand_point(rng)
    # avoid x_R in any coset
    for (u, ws) in bases:
        if any(R[0] == P[0] for P in coset_points(E, u, ws)):
            return make_instance(n, m, l, seed + 1000, want, a2)
    return E, bases, R

if __name__ == "__main__":
    n, m, l = 11, 3, 2
    E, bases, R = make_instance(n, m, l, 1, "sat")
    sols = all_decompositions(E, bases, R)
    print("decomps", len(sols))
    names, polys = build_xonly(E, bases, R)
    print("xonly vars", len(names), "eqs", len(polys), "maxdeg", max(len(mn) for p in polys for mn in p))
    names2, polys2 = build_rr(E, bases, R)
    print("rr vars", len(names2), "eqs", len(polys2), "maxdeg", max(len(mn) for p in polys2 for mn in p))
