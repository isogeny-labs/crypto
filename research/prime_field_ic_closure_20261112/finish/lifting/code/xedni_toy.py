#!/usr/bin/env python3
"""Toy prime-field xedni, 7-coefficient Weierstrass path, r = 2..6.  [pcc:unverified]

Per trial:
  1. E/F_p : y^2 = x^3 + A x + B with #E = q prime;  S generator, T = k S.
  2. P_i = s_i S - t_i T  (arm-dependent choice of s,t; honest arm: uniform).
  3. Lift: balanced integer representatives (x_i, y_i) (collinear lift in the
     positive-control arm).
  4. Fit a0 y^2 + a1 x y + a3 y = a00 x^3 + a2 x^2 + a4 x + a6 through the r
     integer points with u = (a0,a1,a3,a00,a2,a4,a6) == c*(1,0,0,1,0,A,B) mod p:
     integer kernel by LLL, congruence coset, LLL, then a {-1,0,1}
     neighbourhood minimising the generalised discriminant (JKSST step 9 analogue).
  5. Convert to an integral Weierstrass model E_W, then to y^2 = x^3 - 27c4 x - 54c6.
  6. Relation sieve: relation lattice of the r lifted points modulo q (exact,
     from s,t,k) and modulo auxiliary primes l with #E_s(F_l) prime; LLL.
     Either a short candidate (verified exactly over Z, or mod 120 extra primes
     if large) or a certificate "no relation with ||n||_inf <= X_cert".
  7. A verified relation n gives k = sum n_i s_i / sum n_i t_i mod q if useful.
"""
import sys, os, json, math, random, time, argparse, itertools
from fractions import Fraction
import numpy as np
from flint import fmpz_mat, fmpz

# ---------------------------------------------------------------- primes
_MR_BASES = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)


def is_prime(n):
    if n < 2:
        return False
    for sp in _MR_BASES:
        if n % sp == 0:
            return n == sp
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in _MR_BASES:
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


# ---------------------------------------------------------------- EC mod m (short Weierstrass)
def ec_add(P, Q, a, m):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % m == 0:
            return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, m) % m
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, m) % m
    x3 = (lam * lam - x1 - x2) % m
    return (x3, (lam * (x1 - x3) - y1) % m)


def ec_neg(P, m):
    return None if P is None else (P[0], (-P[1]) % m)


def ec_mul(k, P, a, m):
    if k < 0:
        k, P = -k, ec_neg(P, m)
    R, Q = None, P
    while k:
        if k & 1:
            R = ec_add(R, Q, a, m)
        Q = ec_add(Q, Q, a, m)
        k >>= 1
    return R


_CHI = {}


def chi_table(m):
    t = _CHI.get(m)
    if t is None:
        t = np.full(m, -1, dtype=np.int64)
        t[(np.arange(m, dtype=np.int64) ** 2) % m] = 1
        t[0] = 0
        _CHI[m] = t
    return t


def count_points(a, b, m):
    x = np.arange(m, dtype=np.int64)
    f = ((x * x % m) * x + a * x + b) % m
    return m + 1 + int(chi_table(m)[f].sum())


def sqrt_mod(n, p):
    n %= p
    if n == 0:
        return 0
    if pow(n, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(n, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    M, c, t, R = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (M - i - 1), p)
        M, c, t, R = i, b * b % p, t * b * b % p, R * b % p
    return R


def random_point(a, b, p, rng):
    while True:
        x = rng.randrange(p)
        y = sqrt_mod(x * x * x + a * x + b, p)
        if y is not None and y != 0:
            return (x, y if rng.random() < 0.5 else (-y) % p)


def bsgs_table(G, N, a, m):
    w = math.isqrt(N) + 1
    tab = {}
    R = None
    for j in range(w):
        if R not in tab:
            tab[R] = j
        R = ec_add(R, G, a, m)
    step = ec_neg(ec_mul(w, G, a, m), m)
    return tab, step, w


def bsgs(tabinfo, P, N, a, m):
    tab, step, w = tabinfo
    Q = P
    for i in range(w + 1):
        j = tab.get(Q)
        if j is not None:
            return (i * w + j) % N
        Q = ec_add(Q, step, a, m)
    raise RuntimeError("bsgs failed")


# ---------------------------------------------------------------- target curves
def make_target(bits, rng):
    while True:
        p = rng.randrange(1 << (bits - 1), 1 << bits) | 1
        if not is_prime(p):
            continue
        for _ in range(400):
            A = rng.randrange(1, p)
            B = rng.randrange(1, p)
            if (4 * A ** 3 + 27 * B ** 2) % p == 0:
                continue
            N = count_points(A, B, p)
            if N > 16 and N != p and is_prime(N):
                S = random_point(A, B, p, rng)
                return dict(p=p, A=A, B=B, q=N, S=S)


# ---------------------------------------------------------------- lattice helpers
def int_kernel(Bm, ncols):
    """Saturated integer kernel {u in Z^ncols : Bm u = 0} via LLL on [I | K Bm^T]."""
    r = len(Bm)
    maxe = max(1, max(abs(v) for row in Bm for v in row))
    K = 1 << ((r + 1) * (maxe.bit_length() + 4) + 40)
    rows = [[1 if i == j else 0 for j in range(ncols)] + [K * Bm[k][i] for k in range(r)]
            for i in range(ncols)]
    red = fmpz_mat(rows).lll().tolist()
    ker = [[int(v) for v in row[:ncols]] for row in red if all(int(v) == 0 for v in row[ncols:])]
    assert len(ker) == ncols - r, ("kernel rank", len(ker), ncols - r)
    for u in ker:
        for row in Bm:
            assert sum(x * y for x, y in zip(row, u)) == 0
    return ker


def solve_mod(cols, target, p):
    """Find w with sum_j w_j cols[j] == target (mod p); cols: list of vectors."""
    m, n = len(cols), len(target)
    # augmented n x (m+1)
    M = [[cols[j][i] % p for j in range(m)] + [target[i] % p] for i in range(n)]
    piv_cols, row = [], 0
    for c in range(m):
        pr = next((i for i in range(row, n) if M[i][c] % p), None)
        if pr is None:
            continue
        M[row], M[pr] = M[pr], M[row]
        inv = pow(M[row][c], -1, p)
        M[row] = [v * inv % p for v in M[row]]
        for i in range(n):
            if i != row and M[i][c]:
                f = M[i][c]
                M[i] = [(vi - f * vr) % p for vi, vr in zip(M[i], M[row])]
        piv_cols.append(c)
        row += 1
    for i in range(row, n):
        if M[i][m] % p:
            return None
    w = [0] * m
    for i, c in enumerate(piv_cols):
        w[c] = M[i][m]
    return w


def gs_min_norm2(basis):
    """Exact min_i ||b_i*||^2 (Fraction) of a basis given as integer rows."""
    bs = []
    out = []
    for b in basis:
        v = [Fraction(x) for x in b]
        for bstar, n2 in bs:
            mu = sum(x * y for x, y in zip(v, bstar)) / n2
            v = [x - mu * y for x, y in zip(v, bstar)]
        n2 = sum(x * x for x in v)
        bs.append((v, n2))
        out.append(n2)
    return min(out)


# ---------------------------------------------------------------- curve fitting
def gen_disc(u):
    a0, a1, a3, a00, a2, a4, a6 = u
    A_ = 4 * a0 * a00
    C2 = 4 * a0 * a2 + a1 * a1
    C1 = 4 * a0 * a4 + 2 * a1 * a3
    C0 = 4 * a0 * a6 + a3 * a3
    return 18 * A_ * C2 * C1 * C0 - 4 * C2 ** 3 * C0 + C2 ** 2 * C1 ** 2 - 4 * A_ * C1 ** 3 - 27 * A_ ** 2 * C0 ** 2


def weier_invariants(a1, a2, a3, a4, a6):
    b2 = a1 * a1 + 4 * a2
    b4 = 2 * a4 + a1 * a3
    b6 = a3 * a3 + 4 * a6
    b8 = a1 * a1 * a6 + 4 * a2 * a6 - a1 * a3 * a4 + a2 * a3 * a3 - a4 * a4
    c4 = b2 * b2 - 24 * b4
    c6 = -b2 ** 3 + 36 * b2 * b4 - 216 * b6
    D = -b2 * b2 * b8 - 8 * b4 ** 3 - 27 * b6 * b6 + 9 * b2 * b4 * b6
    return b2, b4, b6, b8, c4, c6, D


def fit_curve(pts, p, A, B, neigh=1):
    rows = [[y * y, x * y, y, -x ** 3, -x * x, -x, -1] for (x, y) in pts]
    r = len(rows)
    ker = int_kernel(rows, 7)
    uE = [1, 0, 0, 1, 0, A % p, B % p]
    w = solve_mod(ker, uE, p)
    if w is None:
        return None, "no-coset"
    m = len(ker)
    t = next(j for j in range(m) if w[j] % p)
    inv = pow(w[t], -1, p)
    wt = [(wj * inv) % p for wj in w]
    Wb = []
    for j in range(m):
        if j == t:
            Wb.append(wt)
        else:
            e = [0] * m
            e[j] = p
            Wb.append(e)
    Lb = [[sum(xi * ker[j][c] for j, xi in enumerate(xv)) for c in range(7)] for xv in Wb]
    red = [[int(v) for v in row] for row in fmpz_mat(Lb).lll().tolist()]
    base = red[:min(len(red), 5)]
    best = None
    for coeffs in itertools.product(range(-neigh, neigh + 1), repeat=len(base)):
        if all(c == 0 for c in coeffs):
            continue
        u = [sum(c * b[i] for c, b in zip(coeffs, base)) for i in range(7)]
        if u[0] % p == 0:          # c == 0 mod p : reduces to the zero form
            continue
        g = gen_disc(u)
        if g == 0:
            continue
        key = (abs(g).bit_length(), sum(v * v for v in u))
        if best is None or key < best[0]:
            best = (key, u)
    if best is None:
        return None, "no-nonsingular"
    u = best[1]
    # sanity: incidence and congruence
    for row in rows:
        assert sum(a * b for a, b in zip(row, u)) == 0
    c = u[0] % p
    assert all((ui - c * ei) % p == 0 for ui, ei in zip(u, uE))
    return u, "ok"


def to_short(u, pts):
    a0, a1, a3, a00, a2, a4, a6 = u
    W = (a1, a2 * a0, a3 * a0 * a00, a4 * a0 * a0 * a00, a6 * a0 ** 3 * a00 ** 2)
    b2, b4, b6, b8, c4, c6, D = weier_invariants(*W)
    As, Bs = -27 * c4, -54 * c6
    spts = []
    for (x, y) in pts:
        X, Y = a0 * a00 * x, a0 * a0 * a00 * y
        # check on E_W
        assert Y * Y + W[0] * X * Y + W[2] * Y == X ** 3 + W[1] * X * X + W[3] * X + W[4]
        xs, ys = 36 * X + 3 * b2, 108 * (2 * Y + W[0] * X + W[2])
        assert ys * ys == xs ** 3 + As * xs + Bs
        spts.append((xs, ys))
    return dict(W=W, c4=c4, c6=c6, D=D, As=As, Bs=Bs, spts=spts)


# ---------------------------------------------------------------- exact arithmetic over Z (Jacobian)
def jac_dbl(P, a):
    X, Y, Z = P
    if Z == 0 or Y == 0:
        return (1, 1, 0)
    Y2 = Y * Y
    S = 4 * X * Y2
    Z2 = Z * Z
    M = 3 * X * X + a * Z2 * Z2
    X3 = M * M - 2 * S
    Y3 = M * (S - X3) - 8 * Y2 * Y2
    Z3 = 2 * Y * Z
    return (X3, Y3, Z3)


def jac_add(P, Q, a):
    if P[2] == 0:
        return Q
    if Q[2] == 0:
        return P
    X1, Y1, Z1 = P
    X2, Y2, Z2 = Q
    Z1s, Z2s = Z1 * Z1, Z2 * Z2
    U1, U2 = X1 * Z2s, X2 * Z1s
    S1, S2 = Y1 * Z2s * Z2, Y2 * Z1s * Z1
    if U1 == U2:
        if S1 != S2:
            return (1, 1, 0)
        return jac_dbl(P, a)
    H, R = U2 - U1, S2 - S1
    H2 = H * H
    H3 = H2 * H
    X3 = R * R - H3 - 2 * U1 * H2
    Y3 = R * (U1 * H2 - X3) - S1 * H3
    Z3 = H * Z1 * Z2
    return (X3, Y3, Z3)


def jac_mul(k, P, a):
    if k < 0:
        k, P = -k, (P[0], -P[1], P[2])
    R, Q = (1, 1, 0), P
    while k:
        if k & 1:
            R = jac_add(R, Q, a)
        Q = jac_dbl(Q, a)
        k >>= 1
    return R


def exact_relation_holds(n, spts, As):
    acc = (1, 1, 0)
    for ni, (x, y) in zip(n, spts):
        if ni:
            acc = jac_add(acc, jac_mul(ni, (x, y, 1), As), As)
    return acc[2] == 0


# ---------------------------------------------------------------- relation sieve
def relation_sieve(sh, c_scalars, q, rng_aux, target_bits=40, cand_bits=20, max_primes=400):
    """Return dict with candidate relation (or None) and certificate."""
    spts, As, Bs, D = sh["spts"], sh["As"], sh["Bs"], sh["D"]
    r = len(spts)
    inv1 = pow(c_scalars[0], -1, q)
    mods = [q]
    resid = [[1] + [ci * inv1 % q for ci in c_scalars[1:]]]
    used = {q}
    logM = math.log2(q)
    l = 1031 + 2 * rng_aux.randrange(0, 1500)
    tried = 0
    aux_primes = []
    while logM < r * target_bits and tried < max_primes:
        l += 2
        if not is_prime(l):
            continue
        tried += 1
        if (6 * D) % l == 0:
            continue
        a, b = As % l, Bs % l
        N = count_points(a, b, l)
        if not is_prime(N) or N in used:
            continue
        P1 = (spts[0][0] % l, spts[0][1] % l)
        tab = bsgs_table(P1, N, a, l)
        al = [1]
        for (x, y) in spts[1:]:
            al.append(bsgs(tab, (x % l, y % l), N, a, l))
        mods.append(N)
        resid.append(al)
        used.add(N)
        aux_primes.append(l)
        logM += math.log2(N)
    M = 1
    for mm in mods:
        M *= mm
    beta = [0] * r
    for i in range(r):
        acc = 0
        for mm, al in zip(mods, resid):
            Mi = M // mm
            acc += al[i] * Mi * pow(Mi, -1, mm)
        beta[i] = acc % M
    assert beta[0] == 1
    basis = [[M] + [0] * (r - 1)]
    for i in range(1, r):
        row = [(-beta[i]) % M] + [0] * (r - 1)
        row[i] = 1
        basis.append(row)
    red = [[int(v) for v in row] for row in fmpz_mat(basis).lll().tolist()]
    gs2 = gs_min_norm2(red)
    # certificate: no nonzero n in the lattice has ||n||_2^2 < gs2
    xcert = math.isqrt(int(gs2 // r))  # sqrt(r)*X < sqrt(gs2)  =>  X < sqrt(gs2/r)
    while r * xcert * xcert >= gs2:
        xcert -= 1
    b1 = min(red, key=lambda v: sum(x * x for x in v))
    b1inf = max(abs(x) for x in b1)
    out = dict(log2M=logM, n_aux=len(aux_primes), xcert_log2=math.log2(xcert) if xcert > 0 else 0.0,
               b1inf_log2=math.log2(b1inf), cand=None, verified=None)
    if b1inf <= (1 << cand_bits):
        n = b1
        if b1inf <= 64:
            ok = exact_relation_holds(n, spts, As)
            out["verified"] = "exact" if ok else "exact-FAIL"
        else:
            ok = True
            cnt = 0
            l2 = 50021
            while cnt < 120:
                l2 += 2
                if not is_prime(l2) or (6 * D) % l2 == 0:
                    continue
                a = As % l2
                acc = None
                for ni, (x, y) in zip(n, spts):
                    acc = ec_add(acc, ec_mul(ni, (x % l2, y % l2), a, l2), a, l2)
                if acc is not None:
                    ok = False
                    break
                cnt += 1
            out["verified"] = "mod120" if ok else "mod-FAIL"
        out["cand"] = n if ok else None
        out["cand_raw"] = n
    return out


# ---------------------------------------------------------------- one trial
BOX = {}


def box_vectors(r, C):
    key = (r, C)
    if key not in BOX:
        v = np.array([t for t in itertools.product(range(-C, C + 1), repeat=r) if any(t)], dtype=np.int64)
        BOX[key] = v
    return BOX[key]


def balanced(v, p):
    v %= p
    return v - p if v > p // 2 else v


def collinear_lift(P1l, P2l, P3, p):
    """Integer point on the line through integer points P1l, P2l reducing to P3 mod p."""
    (x1, y1), (x2, y2) = P1l, P2l
    dx, dy = x2 - x1, y2 - y1
    g = math.gcd(dx, dy)
    dx, dy = dx // g, dy // g
    if dx % p:
        t = (P3[0] - x1) * pow(dx, -1, p) % p
    else:
        t = (P3[1] - y1) * pow(dy, -1, p) % p
    t = balanced(t, p)
    x3, y3 = x1 + t * dx, y1 + t * dy
    assert (x3 - P3[0]) % p == 0 and (y3 - P3[1]) % p == 0
    return (x3, y3)


def run_trial(tg, r, arm, seed, heights=False):
    rng = random.Random(seed)
    p, A, B, q, S = tg["p"], tg["A"], tg["B"], tg["q"], tg["S"]
    k = rng.randrange(1, q)
    T = ec_mul(k, S, A, p)
    s = [rng.randrange(q) for _ in range(r)]
    t = [rng.randrange(q) for _ in range(r)]
    planted = None
    if arm in ("PC", "POS"):          # P1+P2+P3 = O
        s[2], t[2] = (-s[0] - s[1]) % q, (-t[0] - t[1]) % q
        planted = [1, 1, 1] + [0] * (r - 3)
    elif arm == "PS":                  # P3 = P1 + P2
        s[2], t[2] = (s[0] + s[1]) % q, (t[0] + t[1]) % q
        planted = [1, 1, -1] + [0] * (r - 3)
    elif arm == "PD":                  # P2 = 2 P1
        s[1], t[1] = (2 * s[0]) % q, (2 * t[0]) % q
        planted = [2, -1] + [0] * (r - 2)
    cs = [(si - k * ti) % q for si, ti in zip(s, t)]
    rec = dict(p=p, q=q, r=r, arm=arm, seed=seed)
    if any(c == 0 for c in cs):
        rec["status"] = "point-at-infinity"
        return rec
    Ppts = [ec_add(ec_mul(si, S, A, p), ec_neg(ec_mul(ti, T, A, p), p), A, p) for si, ti in zip(s, t)]
    for P, c in zip(Ppts, cs):
        assert P == ec_mul(c, S, A, p)
    # mod-q small-relation events (union-bound events), boxes C=1,2
    cvec = np.array(cs, dtype=np.int64)
    for C in (1, 2):
        V = box_vectors(r, C)
        rec[f"e{C}"] = bool(np.any((V @ cvec) % q == 0))
    # trivial collisions P_i = +-P_j
    rec["collision"] = any(Ppts[i][0] == Ppts[j][0] for i in range(r) for j in range(i + 1, r))
    # lift
    pts = [(balanced(P[0], p), balanced(P[1], p)) for P in Ppts]
    if arm == "POS":
        if pts[0] == pts[1]:           # P1 = P2: no line through two distinct points
            rec["status"] = "degenerate-control"
            return rec
        pts[2] = collinear_lift(pts[0], pts[1], Ppts[2], p)
    if len(set(pts)) < r:
        # identical lifts: the curve fit is underdetermined; record as trivial relation
        rec["status"] = "identical-lifts"
        i, j = next((i, j) for i in range(r) for j in range(i + 1, r) if pts[i] == pts[j])
        n = [0] * r
        n[i], n[j] = 1, -1
        rec["rel"] = n
        rec["rel_inf"] = 1
        rec["rel_trivial"] = True
        den = sum(ni * ti for ni, ti in zip(n, t)) % q
        rec["useful"] = den != 0
        rec["success"] = bool(den != 0 and (sum(ni * si for ni, si in zip(n, s)) * pow(den, -1, q)) % q == k)
        return rec
    u, why = fit_curve(pts, p, A, B)
    if u is None:
        rec["status"] = why
        return rec
    sh = to_short(u, pts)
    rec["status"] = "ok"
    rec["u_maxbits"] = max(abs(v).bit_length() for v in u)
    rec["log2_absDW"] = math.log2(abs(sh["D"])) if sh["D"] else None
    rec["log2_gen_disc"] = math.log2(abs(gen_disc(u)))
    rec["max_xs_bits"] = max(abs(x).bit_length() for x, _ in sh["spts"])
    # sanity: reduction mod p of the short model is isomorphic to E and points map to P_i:
    # check scalar dlogs mod p on a cheap invariant: compare relation lattice mod p via the planted / e1 events later.
    rng_aux = random.Random(seed ^ 0x5EED)
    sv = relation_sieve(sh, cs, q, rng_aux)
    rec.update({k_: v for k_, v in sv.items() if k_ not in ("cand_raw",)})
    n = sv["cand"]
    rec["rel"] = n
    if n is not None:
        rec["rel_inf"] = max(abs(x) for x in n)
        assert sum(ni * ci for ni, ci in zip(n, cs)) % q == 0, "char-0 relation must hold mod q"
        den = sum(ni * ti for ni, ti in zip(n, t)) % q
        rec["useful"] = den != 0
        rec["success"] = bool(den != 0 and (sum(ni * si for ni, si in zip(n, s)) * pow(den, -1, q)) % q == k)
        supp = [x for x in n if x]
        rec["rel_trivial"] = bool(len(supp) <= 2 and all(abs(x) == 1 for x in supp))
    else:
        rec["useful"] = False
        rec["success"] = False
    if planted is not None:
        # realized iff the planted vector (or a multiple) lies in the char-0 relation lattice
        rec["planted_realized"] = bool(exact_relation_holds(planted, sh["spts"], sh["As"]))
        if arm in ("PC", "POS"):
            (x1, y1), (x2, y2), (x3, y3) = pts[0], pts[1], pts[2]
            rec["int_collinear"] = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1) == 0
    if heights:
        from heights import height_data
        rec["heights"] = height_data(sh, n_dbl=heights if isinstance(heights, int) and heights > 1 else 4)
    return rec


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--bits", type=int, required=True)
    ap.add_argument("--r", type=int, required=True)
    ap.add_argument("--arm", default="X")
    ap.add_argument("--trials", type=int, default=100)
    ap.add_argument("--curves", type=int, default=4)
    ap.add_argument("--seed", type=int, default=20261112)
    ap.add_argument("--heights", type=int, default=0, help="number of doublings for heights (0 = off)")
    ap.add_argument("--heights-first", type=int, default=0, help="compute heights only for the first N trials")
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    rng = random.Random(a.seed * 1000 + a.bits)
    targets = [make_target(a.bits, rng) for _ in range(a.curves)]
    t0 = time.time()
    with open(a.out, "w") as fh:
        fh.write(json.dumps(dict(meta=True, bits=a.bits, r=a.r, arm=a.arm, trials=a.trials,
                                 targets=[{k: (list(v) if isinstance(v, tuple) else v) for k, v in tg.items()} for tg in targets],
                                 seed=a.seed)) + "\n")
        for i in range(a.trials):
            tg = targets[i % len(targets)]
            seed = (a.seed * 1_000_003 + a.bits * 10007 + a.r * 101 + hash(a.arm) % 997 * 0 + i) & ((1 << 62) - 1)
            seed = seed ^ (sum(ord(ch) for ch in a.arm) << 40)
            hflag = a.heights if (a.heights and i < a.heights_first) else False
            rec = run_trial(tg, a.r, a.arm, seed, heights=hflag)
            rec["i"] = i
            fh.write(json.dumps(rec) + "\n")
    print(f"done bits={a.bits} r={a.r} arm={a.arm} trials={a.trials} in {time.time()-t0:.1f}s", file=sys.stderr)


if __name__ == "__main__":
    main()
