"""Approximate Neron-Tate heights on y^2 = x^3 + A x + B over Q by the doubling limit.  [pcc:unverified]

Normalisation: hhat(P) = lim h(x(2^n P)) / (2 * 4^n), h(a/b) = log max(|a|,|b|) (Silverman's
normalisation, hhat ~ h(x)/2).  Truncation error after n doublings is bounded by
Bsil / 4^n with Bsil = h(j)/8 + h(Delta)/12 + 1.07 (Silverman 1990, Math. Comp. 55,
Theorem 1.1, constants recalled, not re-read; b2 = 0 model).
"""
import math
import numpy as np
from flint import fmpz


def log_abs(n):
    n = abs(int(n))
    b = n.bit_length()
    if b <= 900:
        return math.log(n)
    sh = b - 64
    return math.log(n >> sh) + sh * math.log(2.0)


def hfrac(num, den):
    if num == 0:
        return log_abs(den)
    return max(log_abs(num), log_abs(den))


def x_double(n, d, A, B):
    n2 = n * n
    d2 = d * d
    top = n2 * n2 - 2 * A * n2 * d2 - 8 * B * n * d2 * d + A * A * d2 * d2
    bot = 4 * d * (n2 * n + A * n * d2 + B * d2 * d)
    g = top.gcd(bot)
    top = top // g
    bot = bot // g
    if bot < 0:
        top, bot = -top, -bot
    return top, bot


def hhat_x(num, den, A, B, n_dbl):
    n, d = fmpz(num), fmpz(den)
    g = n.gcd(d)
    n, d = n // g, d // g
    A, B = fmpz(A), fmpz(B)
    for _ in range(n_dbl):
        n, d = x_double(n, d, A, B)
    return hfrac(n, d) / (2.0 * 4 ** n_dbl)


def sum_x(P, Q):
    (x1, y1), (x2, y2) = P, Q
    dx, dy = x2 - x1, y2 - y1
    num = dy * dy - (x1 + x2) * dx * dx
    den = dx * dx
    return num, den


def min_form(G, tol=0.0, max_nodes=2000000):
    """min over nonzero integer vectors of n^T G n (Fincke-Pohst, floats).
    If the smallest eigenvalue is <= tol (numerically dependent points, within the truncation
    error), return (min eigenvalue clipped at 0, 'singular') without enumerating."""
    r = G.shape[0]
    ev = np.linalg.eigvalsh(G)
    if ev[0] <= 0.0:   # lift-height round: only a non-PD form is 'singular' (tol shortcut removed)
        return max(0.0, float(ev[0])), 'singular'
    nodes = [0]
    try:
        Lc = np.linalg.cholesky(G)
    except np.linalg.LinAlgError:
        return 0.0, None
    # q_ii and mu from Cholesky: G = L L^T ; use R = L^T upper
    R = Lc.T
    qd = np.array([R[i, i] ** 2 for i in range(r)])
    mu = np.array([[R[i, j] / R[i, i] if j > i else 0.0 for j in range(r)] for i in range(r)])
    best = [min(np.diag(G)) * (1 + 1e-12), None]

    def rec(i, partial, x):
        if i < 0:
            if any(x):
                val = partial
                if val < best[0]:
                    best[0] = val
                    best[1] = list(x)
            return
        c = -sum(mu[i, j] * x[j] for j in range(i + 1, r))
        rad = math.sqrt(max(0.0, (best[0] - partial) / qd[i]))
        lo, hi = math.ceil(c - rad), math.floor(c + rad)
        nodes[0] += hi - lo + 1
        if nodes[0] > max_nodes:
            raise RuntimeError('enumeration cap')
        for v in range(lo, hi + 1):
            x[i] = v
            np_ = partial + qd[i] * (v - c) ** 2
            if np_ <= best[0]:
                rec(i - 1, np_, x)
        x[i] = 0

    rec(r - 1, 0.0, [0] * r)
    return best[0], best[1]


def height_data(sh, n_dbl=4):
    A, B, D, c4 = sh["As"], sh["Bs"], sh["D"], sh["c4"]
    pts = sh["spts"]
    r = len(pts)
    g = math.gcd(abs(c4 ** 3), abs(D))
    hj = hfrac(c4 ** 3 // g, D // g)
    Ds = -16 * (4 * A ** 3 + 27 * B * B)
    hD = log_abs(Ds)
    bsil = hj / 8 + hD / 12 + 1.07
    eps = bsil / 4 ** n_dbl
    hh = [hhat_x(x, 1, A, B, n_dbl) for (x, y) in pts]
    G = np.zeros((r, r))
    for i in range(r):
        G[i, i] = hh[i]
    for i in range(r):
        for j in range(i + 1, r):
            if pts[i][0] == pts[j][0]:
                if pts[i][1] == pts[j][1]:      # P_j = P_i : hhat(2P) = 4 hhat(P)
                    hij = 4 * hh[i]
                else:                            # P_j = -P_i : P_i + P_j = O
                    hij = 0.0
            else:
                num, den = sum_x(pts[i], pts[j])
                hij = hhat_x(num, den, A, B, n_dbl)
            G[i, j] = G[j, i] = (hij - hh[i] - hh[j]) / 2
    try:
        lam, vec = min_form(G, tol=3.0 * r * eps)
    except RuntimeError:
        lam, vec = float(np.linalg.eigvalsh(G)[0]), 'enumeration-cap'   # lower bound for the minimum
    ev = np.linalg.eigvalsh(G)
    logDW = log_abs(D)
    return dict(n_dbl=n_dbl, eps=eps, bsil=bsil, h_j=hj, log_absDs=hD, log_absDW=logDW,
                hhat=hh, H=max(hh), hmin=min(hh), lambda_G=lam, lambda_vec=vec,
                rho_G=(max(hh) / lam) if lam > 0 else None,
                lang_hmin=min(hh) / logDW, lang_lambdaG=lam / logDW,
                H_over_logDW=max(hh) / logDW,
                min_eig=float(ev[0]), log_det=float(np.sum(np.log(np.abs(ev)))),
                G=[[round(float(G[i, j]), 6) for j in range(r)] for i in range(r)])


def height_data_q(A, B, pts, n_dbl=5):
    """lift-height round: same doubling limit, but on the short model y^2 = x^3 + A x + B obtained
    from the integral Washington model WITHOUT the extra scaling V that clears the points'
    denominators (points stay rational).  The V-scaling multiplies Delta by V^12 and slows the
    doubling limit by O(log V)/4^n; the canonical height does not depend on the model."""
    from fractions import Fraction
    A, B = int(A), int(B)
    Ds = -16 * (4 * A ** 3 + 27 * B * B)
    num_j = -1728 * (4 * A) ** 3          # j = -1728 (4A)^3 / Delta
    g = math.gcd(abs(num_j), abs(Ds))
    hj = hfrac(num_j // g, Ds // g)
    hD = log_abs(Ds)
    bsil = hj / 8 + hD / 12 + 1.07
    eps = bsil / 4 ** n_dbl
    P = [(Fraction(x), Fraction(y)) for (x, y) in pts]
    r = len(P)
    hh = [hhat_x(x.numerator, x.denominator, A, B, n_dbl) for (x, y) in P]
    G = np.zeros((r, r))
    for i in range(r):
        G[i, i] = hh[i]
    for i in range(r):
        for j in range(i + 1, r):
            if P[i][0] == P[j][0]:
                hij = 4 * hh[i] if P[i][1] == P[j][1] else 0.0
            else:
                n_, d_ = sum_x(P[i], P[j])
                xs = n_ / d_
                hij = hhat_x(xs.numerator, xs.denominator, A, B, n_dbl)
            G[i, j] = G[j, i] = (hij - hh[i] - hh[j]) / 2
    try:
        lam, vec = min_form(G)
    except RuntimeError:
        lam, vec = float(np.linalg.eigvalsh(G)[0]), 'enumeration-cap'
    ev = np.linalg.eigvalsh(G)
    return dict(n_dbl=n_dbl, eps=eps, bsil=bsil, h_j=hj, log_absDs=hD, log_absDW=hD,
                hhat=hh, H=max(hh), hmin=min(hh), lambda_G=lam, lambda_vec=vec,
                rho_G=(max(hh) / lam) if lam > 0 else None,
                lang_lambdaG=lam / hD, min_eig=float(ev[0]),
                G=[[round(float(G[i, j]), 6) for j in range(r)] for i in range(r)])


def hhat_x_levels(num, den, A, B, levels):
    """doubling-limit values h(x(2^n P)) / (2 4^n) for every n in levels (one doubling chain)."""
    n, d = fmpz(num), fmpz(den)
    g = n.gcd(d)
    n, d = n // g, d // g
    A, B = fmpz(A), fmpz(B)
    out = {}
    top = max(levels)
    for k in range(top + 1):
        if k in levels:
            out[k] = hfrac(n, d) / (2.0 * 4 ** k)
        if k < top:
            n, d = x_double(n, d, A, B)
    return out


def height_data_ml(A, B, pts, levels=(4, 5, 6)):
    """Gram matrices at several doubling depths on the unscaled short model, plus the Richardson
    extrapolation G_inf = G_n + (G_n - G_{n-1}) / 3 (error model C / 4^n).  lambda is always the
    Fincke-Pohst lattice minimum of the form in question."""
    from fractions import Fraction
    A, B = int(A), int(B)
    Ds = -16 * (4 * A ** 3 + 27 * B * B)
    num_j = -1728 * (4 * A) ** 3
    g = math.gcd(abs(num_j), abs(Ds))
    hj = hfrac(num_j // g, Ds // g)
    hD = log_abs(Ds)
    bsil = hj / 8 + hD / 12 + 1.07
    P = [(Fraction(x), Fraction(y)) for (x, y) in pts]
    r = len(P)
    lv = sorted(levels)
    hh = [hhat_x_levels(x.numerator, x.denominator, A, B, lv) for (x, y) in P]
    Gs = {k: np.zeros((r, r)) for k in lv}
    for k in lv:
        for i in range(r):
            Gs[k][i, i] = hh[i][k]
    for i in range(r):
        for j in range(i + 1, r):
            if P[i][0] == P[j][0]:
                hij = {k: (4 * hh[i][k] if P[i][1] == P[j][1] else 0.0) for k in lv}
            else:
                n_, d_ = sum_x(P[i], P[j])
                xs = n_ / d_
                hij = hhat_x_levels(xs.numerator, xs.denominator, A, B, lv)
            for k in lv:
                Gs[k][i, j] = Gs[k][j, i] = (hij[k] - hh[i][k] - hh[j][k]) / 2
    Gs["rich"] = Gs[lv[-1]] + (Gs[lv[-1]] - Gs[lv[-2]]) / 3.0
    Gs["rich_prev"] = Gs[lv[-2]] + (Gs[lv[-2]] - Gs[lv[-3]]) / 3.0 if len(lv) >= 3 else None
    out = dict(levels=lv, bsil=bsil, log_absDs=hD, h_j=hj)
    for key, G in Gs.items():
        if G is None:
            continue
        try:
            lam, vec = min_form(G)
        except RuntimeError:
            lam, vec = float(np.linalg.eigvalsh(G)[0]), 'enumeration-cap'
        H = float(max(np.diag(G)))
        out[str(key)] = dict(H=H, lambda_G=float(lam), lambda_vec=vec, rho_G=(H / lam) if lam > 0 else None,
                             eps=(bsil / 4 ** key) if isinstance(key, int) else None,
                             G=[[round(float(G[i, j]), 6) for j in range(r)] for i in range(r)])
    return out
