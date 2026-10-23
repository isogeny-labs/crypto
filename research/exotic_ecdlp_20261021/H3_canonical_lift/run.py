
# H3: canonical-lift p-adic digits of Frobenius-fixed points -- additive defect, MI with log, closure.
# Plain-Python Sage program (from sage.all import * is prepended by the harness).
#
# LIFT METHOD (exact canonical lift, CM method):
#   Satoh's Phi_p(J, J^sigma)=0 iteration is degenerate over a PRIME field (Phi_p(X,X) = (X^p-X)^2 mod p,
#   every F_p point is a double point), so the canonical lift of j(E) is obtained instead as the unique
#   p-adic root of the Hilbert class polynomial H_d (d = disc End(E)) congruent to j(E) mod p, by Newton/Hensel
#   (H_d is separable mod p because p > |d|/4, Gross-Zagier).  The lifted curve is y^2 = x^3 + a x + B~ with
#   the SAME a as E and B~ the unique root with B~ = b mod p of j(a,B~) = J~.  Frobenius-fixed lift of a point P
#   is its unique lift of order | #E(F_p) (prime to p), P~ = [m]P~_0, m = 1 mod n, m = 0 mod p^3.
import os, sys, json, time, math, random, traceback
import builtins as _b
import numpy as np
min = _b.min; max = _b.max; sum = _b.sum; abs = _b.abs; round = _b.round; pow = _b.pow
sorted = _b.sorted; range = _b.range; len = _b.len; int = _b.int; float = _b.float

try:
    from cysignals.alarm import alarm, cancel_alarm, AlarmInterrupt
    HAVE_ALARM = True
except Exception:
    HAVE_ALARM = False
    class AlarmInterrupt(BaseException):
        pass
    def alarm(s): pass
    def cancel_alarm(): pass

OUT = os.environ.get('ISOLAB_OUTPUT_DIR', '.')
os.makedirs(OUT, exist_ok=True)
SEED = 20261022
set_random_seed(SEED)
T0 = time.time()
GLOBAL_BUDGET = 3000.0
C_KILL = 1.0          # kill if min_delta >= 1 - C_KILL/sqrt(p)
BITS = [12, 14, 16, 20]
CELLS = [(12, 0), (12, 1), (12, 2), (12, 3), (12, 4), (14, 0), (16, 0), (20, 0)]
def ckey(bits, rep):
    return str(bits) if rep == 0 else '%d_rep%d' % (bits, rep)
M_PAIRS = 50000
N_DIRS = 4000
S_MAX = 20000
N_PERM = 200


def log(*a):
    print('[%7.1fs]' % (time.time() - T0), *a, flush=True)


def jclean(o):
    if isinstance(o, dict):
        return {str(k): jclean(v) for k, v in o.items()}
    if isinstance(o, (list, tuple)):
        return [jclean(v) for v in o]
    if isinstance(o, (np.integer,)):
        return int(o)
    if isinstance(o, (np.floating,)):
        return float(o)
    if isinstance(o, (np.bool_,)):
        return bool(o)
    if isinstance(o, (bool, int, float, str)) or o is None:
        return o
    try:
        return int(o)
    except Exception:
        return str(o)


RESULT = {
    "experiment": "H3_canonical_lift",
    "seed": SEED,
    "lift_method": "canonical_lift_via_hilbert_class_polynomial_hensel (exact CM canonical lift; NOT the Teichmuller fallback)",
    "lift_method_note": ("Satoh Phi_p(J,J)=0 is degenerate over F_p (Phi_p(X,X)=(X^p-X)^2 mod p), so j~ is the p-adic root of "
                         "H_d (d=disc End(E)) lifting j(E), Newton to p^4; lifted curve y^2=x^3+a x+B~ (same a), B~ from j~; "
                         "Frobenius-fixed lift of P = unique lift of order dividing #E(F_p) (prime to p)."),
    "params": {"bits": BITS, "M_pairs": M_PAIRS, "n_dirs": N_DIRS, "S_max": S_MAX, "n_perm": N_PERM, "c_kill": C_KILL},
    "cells": {},
    "validation": {},
    "censored": [],
    "verdict": "inconclusive",
    "min_delta": None,
}


def write_result():
    try:
        with open(os.path.join(OUT, 'result.json'), 'w') as f:
            json.dump(jclean(RESULT), f, indent=1)
    except Exception as e:
        print('write_result failed', e, flush=True)


# ---------------------------------------------------------------- Jacobian arithmetic mod N = p^3
class Exceptional(Exception):
    pass


def make_group(p, N, A):
    def dbl(R):
        X1, Y1, Z1 = R
        XX = X1 * X1 % N
        YY = Y1 * Y1 % N
        YYYY = YY * YY % N
        ZZ = Z1 * Z1 % N
        S = 4 * X1 * YY % N
        M = (3 * XX + A * ZZ % N * ZZ) % N
        T = (M * M - 2 * S) % N
        Y3 = (M * (S - T) - 8 * YYYY) % N
        Z3 = 2 * Y1 * Z1 % N
        if T % p == 0 and Y3 % p == 0:
            raise Exceptional('dbl: non-primitive result')
        return (T, Y3, Z3)

    def madd(R, Q):
        X1, Y1, Z1 = R
        x2, y2 = Q
        if Z1 % p == 0:
            raise Exceptional('madd: R is O mod p')
        Z1Z1 = Z1 * Z1 % N
        U2 = x2 * Z1Z1 % N
        S2 = y2 * Z1 % N * Z1Z1 % N
        H = (U2 - X1) % N
        if H % p == 0:
            raise Exceptional('madd: H nonunit')
        HH = H * H % N
        I = 4 * HH % N
        J = H * I % N
        r = 2 * (S2 - Y1) % N
        V = X1 * I % N
        X3 = (r * r - J - 2 * V) % N
        Y3 = (r * (V - X3) - 2 * Y1 * J) % N
        Z3 = 2 * Z1 * H % N
        return (X3, Y3, Z3)

    def to_aff(R):
        X, Y, Z = R
        if Z % p == 0:
            raise Exceptional('to_aff: Z nonunit')
        zi = pow(Z, -1, N)
        zi2 = zi * zi % N
        return (X * zi2 % N, Y * zi2 % N * zi % N)

    def smul(k, P):
        bits = bin(k)[2:]
        R = (P[0], P[1], 1)
        for c in bits[1:]:
            R = dbl(R)
            if c == '1':
                R = madd(R, P)
        return R

    return dbl, madd, to_aff, smul


def qp_torsion_lift(p, a, Bt, gx, gy, mexp):
    """[mexp]P0 computed in Q_p (field arithmetic, precision tracked) for the one generator lift."""
    K = Qp(p, prec=150, type='capped-rel')
    aK = K(a)
    rhs = K(gx) ** 3 + aK * K(gx) + K(Bt)
    y0 = rhs.sqrt()
    if (y0 - K(gy)).valuation() <= 0:
        y0 = -y0
    if (y0 - K(gy)).valuation() <= 0:
        raise Exception('sqrt branch mismatch')
    P0 = (K(gx), y0)

    def qadd(P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        dx = x2 - x1
        if dx.is_zero():
            if (y1 + y2).is_zero():
                return None
            lam = (3 * x1 * x1 + aK) / (2 * y1)
        else:
            lam = (y2 - y1) / dx
        x3 = lam * lam - x1 - x2
        y3 = lam * (x1 - x3) - y1
        return (x3, y3)
    R = None
    for c in bin(mexp)[2:]:
        R = qadd(R, R)
        if c == '1':
            R = qadd(R, P0)
    if R is None:
        raise Exception('result is O')
    out = []
    for z in R:
        if z.valuation() < 0:
            raise Exception('result not integral')
        out.append(int(ZZ(z.residue(3))))
    return (out[0], out[1])


def horner2(coeffs, J, M):
    f = 0
    fp = 0
    for c in reversed(coeffs):
        fp = (fp * J + f) % M
        f = (f * J + c) % M
    return f, fp


def newton_sqrt(R, y0, N):
    y = y0 % N
    inv2 = (N + 1) // 2
    for _ in range(8):
        y = (y + R * pow(y, -1, N)) % N * inv2 % N
    return y


# ---------------------------------------------------------------- scan / stats helpers
def delta_scan(x1, x2, ks, ls, ss, p, dirs):
    d1 = x1[ss] - x1[ks] - x1[ls]
    d2 = x2[ss] - x2[ks] - x2[ls]
    M = len(ks)
    out = np.empty(len(dirs), dtype=np.float64)
    for i, (al, be) in enumerate(dirs):
        v = (al * d1 + be * d2) % p
        out[i] = np.count_nonzero(v) / M
    return out


def mi_counts(xb, kb, B, S):
    keys = xb * B + kb
    uniq, cnt = np.unique(keys, return_counts=True)
    px = np.bincount(xb, minlength=B).astype(np.float64)
    py = np.bincount(kb, minlength=B).astype(np.float64)
    xi = uniq // B
    ki = uniq % B
    return float(np.sum(cnt / S * np.log(cnt * S / (px[xi] * py[ki]))))


def mi_with_null(xb, kb, B, S, rng, nperm):
    mi = mi_counts(xb, kb, B, S)
    nulls = np.empty(nperm)
    kk = kb.copy()
    for i in range(nperm):
        rng.shuffle(kk)
        nulls[i] = mi_counts(xb, kk, B, S)
    return mi, float(nulls.mean()), float(nulls.std(ddof=1))


# ---------------------------------------------------------------- one cell
def run_cell(bits, rep=0):
    cell = {"bits": bits, "rep": rep}
    tc = time.time()
    p = int(next_prime(2 ** bits))
    cell["p"] = p
    N = p ** 3
    rngp = random.Random(SEED + bits + 1000 * rep)
    rng = np.random.default_rng(SEED + bits + 1000 * rep)
    cell["lift_precision"] = 3
    pari.allocatemem(2 * 10 ** 9) if False else None
    try:
        pari.allocatemem(2 * 10 ** 9)
    except Exception:
        pass

    # ---- 1. pick ordinary E and identify End(E)
    hmax_sched = [3000, 1200, 500, 200]
    sel = None
    for hmax in hmax_sched:
        for attempt in range(80):
            if time.time() - T0 > GLOBAL_BUDGET - 300:
                raise RuntimeError('global budget exhausted during curve selection')
            a = rngp.randrange(1, p)
            b = rngp.randrange(1, p)
            if (4 * a ** 3 + 27 * b * b) % p == 0:
                continue
            E = EllipticCurve(GF(p), [a, b])
            jE = int(E.j_invariant())
            if jE in (0, 1728 % p):
                continue
            n = int(E.cardinality())
            t = p + 1 - n
            if t % p == 0 or n % p == 0:
                continue
            D = t * t - 4 * p
            cd = pari(D).coredisc(1)
            D0 = int(cd[0]); c = int(cd[1])
            # candidate orders d = D0*g^2, g | c, increasing |d|
            divs = sorted([int(g) for g in divisors(c)])
            ok_sel = True
            for g in divs:
                d = D0 * g * g
                h = int(pari(d).qfbclassno())
                if h > hmax:
                    ok_sel = False
                    break
                # compute polclass and test
                try:
                    alarm(900 if bits >= 20 else 300)
                    try:
                        Hp = pari.polclass(d)
                        Hl = [int(cc) for cc in Hp.Vecrev()]
                    finally:
                        cancel_alarm()
                except (AlarmInterrupt, KeyboardInterrupt):
                    log('polclass timeout for d=%d h=%d' % (d, h))
                    ok_sel = False
                    break
                Hp_mod = [cc % p for cc in Hl]
                fval, fpval = horner2(Hp_mod, jE, p)
                if fval == 0:
                    if fpval == 0:
                        ok_sel = False
                        break
                    sel = dict(E=E, a=a, b=b, jE=jE, n=n, t=t, D=D, D0=D0, c=c, d=d, g=g, h=h, Hl=Hl, hmax=hmax)
                    break
            if sel is not None:
                break
        if sel is not None:
            break
    if sel is None:
        raise RuntimeError('could not select a curve with computable class polynomial')
    E = sel['E']; a = sel['a']; b = sel['b']; jE = sel['jE']; n = sel['n']; t = sel['t']
    d = sel['d']; h = sel['h']; Hl = sel['Hl']
    cell["curve"] = {"a": a, "b": b, "j": jE, "n": n, "trace": t, "D": sel['D'], "D0": sel['D0'], "conductor_c": sel['c'],
                     "End_disc_d": d, "End_cond_g": sel['g'], "class_number_h": h, "hmax_used": sel['hmax']}
    cell["ordinary"] = bool(t % p != 0)
    log('bits=%d p=%d n=%d t=%d d=%d h=%d (polclass done)' % (bits, p, n, t, d, h))

    # ---- 2. canonical lift of j: Hensel on H_d
    M4 = p ** 4
    cM = [cc % M4 for cc in Hl]
    J = jE
    f, fp = horner2(cM, J, M4)
    if fp % p == 0:
        raise RuntimeError('H_d has multiple root at j mod p')
    for _ in range(4):
        f, fp = horner2(cM, J, M4)
        J = (J - f * pow(fp, -1, M4)) % M4
    fchk, _ = horner2(cM, J, M4)
    Jt = J % N
    hensel_ok = (fchk == 0) and (Jt % p == jE)
    cell["hensel_residual_zero_mod_p4"] = bool(fchk == 0)
    # extra validations on the class polynomial
    val = {}
    val["H_d_of_Jtilde_zero_mod_p3"] = bool(horner2([cc % N for cc in Hl], Jt, N)[0] == 0)
    val["Jtilde_reduces_to_jE"] = bool(Jt % p == jE)
    try:
        alarm(200)
        try:
            R = PolynomialRing(GF(p), 'x')
            Hpoly = R(Hp_mod)
            nroots = len(Hpoly.roots(multiplicities=False))
        finally:
            cancel_alarm()
        val["H_d_splits_completely_mod_p"] = bool(nroots == h)
        val["H_d_roots_found"] = int(nroots)
    except (AlarmInterrupt, KeyboardInterrupt, Exception) as ex:
        val["H_d_splits_completely_mod_p"] = None
        val["H_d_roots_error"] = str(ex)[:100]
    val["trace_consistent_cardinality"] = bool(n == p + 1 - t and (t * t - 4 * p) == sel['D'])
    val["n_coprime_to_p"] = bool(n % p != 0)

    # ---- 3. lifted curve y^2 = x^3 + a x + B~
    Bsq = 4 * pow(a, 3, N) % N * ((1728 - Jt) % N) % N * pow(27 * Jt % N, -1, N) % N
    Bt = newton_sqrt(Bsq, b, N)
    val["B_tilde_squares_ok"] = bool((Bt * Bt - Bsq) % N == 0 and (Bt - b) % p == 0)
    # recompute j from (a, Bt) mod N
    num = 1728 * 4 * pow(a, 3, N) % N
    den = (4 * pow(a, 3, N) + 27 * Bt * Bt) % N
    j_back = num * pow(den, -1, N) % N
    val["j_of_lifted_curve_equals_Jtilde_mod_p3"] = bool(j_back == Jt)
    val["lifted_curve_reduces_to_E"] = bool((Bt - b) % p == 0)

    # ---- 4. group and torsion lift of generator
    dbl, madd, to_aff, smul = make_group(p, N, a)
    u = pow(n, -1, N)
    mexp = (1 - n * u) % (n * N)

    def on_curve(P):
        x, y = P
        return (y * y - (x * x % N * x + a * x + Bt)) % N == 0

    cand = []
    try:
        gens = E.gens()
        cand.append(gens[0])
    except Exception:
        pass
    for _ in range(40):
        cand.append(E.random_point())
    Gt = None; Gpt = None; ordG = None
    tried = 0
    why = []
    for Gpt_c in cand:
        if Gpt_c.is_zero():
            why.append('zero'); continue
        o = int(Gpt_c.order())
        if o < 1000 or o < n // 32:
            why.append('ord%d' % o); continue
        gx, gy = int(Gpt_c[0]), int(Gpt_c[1])
        if gy == 0:
            why.append('y0'); continue
        tried += 1
        Rhs = (gx ** 3 + a * gx + Bt) % N
        Y0 = newton_sqrt(Rhs, gy, N)
        P0 = (gx, Y0)
        if not on_curve(P0):
            why.append('P0 off curve'); continue
        try:
            Gt = qp_torsion_lift(p, a, Bt, gx, gy, mexp)
        except Exception as ex_:
            why.append('qp:' + str(ex_)[:60]); continue
        if not on_curve(Gt):
            why.append('Gt off curve'); Gt = None; continue
        Gpt = Gpt_c; ordG = o
        break
    if Gt is None:
        raise RuntimeError('no generator lift found: n=%d %s' % (n, why[:8]))
    val["lift_candidates_tried"] = tried
    val["G_tilde_on_curve"] = bool(on_curve(Gt))
    val["G_tilde_reduces_to_G"] = bool(Gt[0] % p == int(Gpt[0]) and Gt[1] % p == int(Gpt[1]))
    cell["ordG"] = ordG
    cell["group_order_n"] = n
    log('G~ found, ord=%d, building table' % ordG)

    # ---- 5. table T[k] = k*G~, k=1..ord-1 (x, y mod p^3)
    xs = [0] * ordG
    ys = [0] * ordG
    xs[1], ys[1] = Gt
    R = dbl((Gt[0], Gt[1], 1))
    xs[2], ys[2] = to_aff(R)
    Rj = R
    for k in range(3, ordG):
        Rj = madd(Rj, Gt)
        X, Y, Z = Rj
        zi = pow(Z, -1, N)
        zi2 = zi * zi % N
        xs[k] = X * zi2 % N
        ys[k] = Y * zi2 % N * zi % N
    log('table built, size %d' % ordG)
    # verify ord*G~ = O: T[ord-1] = -G~
    val["order_check_ord_G_tilde_is_zero"] = bool(xs[ordG - 1] == xs[1] and (ys[ordG - 1] + ys[1]) % N == 0)
    # homomorphism check with wraparound: T[a]+T[b] == T[(a+b)%ord]
    hom_ok = 0; hom_tot = 0; hom_wrap_tot = 0; hom_wrap_ok = 0
    for _ in range(400):
        ka = rngp.randrange(1, ordG); kb_ = rngp.randrange(1, ordG)
        s = (ka + kb_) % ordG
        if s == 0 or (ka - kb_) % ordG == 0:
            continue
        try:
            Rs = madd((xs[ka], ys[ka], 1), (xs[kb_], ys[kb_]))
            xa, ya = to_aff(Rs)
        except Exceptional:
            continue
        good = (xa == xs[s] and ya == ys[s])
        hom_tot += 1; hom_ok += int(good)
        if ka + kb_ > ordG:
            hom_wrap_tot += 1; hom_wrap_ok += int(good)
    val["hom_check_pairs"] = hom_tot
    val["hom_check_ok"] = hom_ok
    val["hom_check_wrap_pairs"] = hom_wrap_tot
    val["hom_check_wrap_ok"] = hom_wrap_ok
    # reduction check against Sage F_p arithmetic and curve membership
    red_ok = 0; red_tot = 0
    for _ in range(200):
        k = rngp.randrange(1, ordG)
        Pk = k * Gpt
        red_tot += 1
        if int(Pk[0]) == xs[k] % p and int(Pk[1]) == ys[k] % p and on_curve((xs[k], ys[k])):
            red_ok += 1
    val["reduction_matches_Fp_arith_and_on_curve"] = bool(red_ok == red_tot)
    # Frobenius-fixed (n-torsion lift) verified iff ord*G~=O and homomorphism with wraparound hold
    fixed_ok = bool(val["order_check_ord_G_tilde_is_zero"] and hom_ok == hom_tot and hom_tot > 0
                    and hom_wrap_ok == hom_wrap_tot and hom_wrap_tot > 0)
    cell["frobenius_fixed_verified"] = fixed_ok
    # a non-torsion lift sanity: the raw lift P0 of G is NOT ord-torsion (control that test is nontrivial)
    try:
        Rhs = (int(Gpt[0]) ** 3 + a * int(Gpt[0]) + Bt) % N
        Y0 = newton_sqrt(Rhs, int(Gpt[1]), N)
        # compute [ord-1]P0 then add P0: should NOT be zero mod p^3 generally (P0 not n-torsion)
        Tm = to_aff(smul(ordG - 1, (int(Gpt[0]), Y0)))
        val["control_raw_lift_not_torsion"] = bool(not (Tm[0] == int(Gpt[0]) and (Tm[1] + Y0) % N == 0))
    except Exceptional:
        val["control_raw_lift_not_torsion"] = None
    cell["validation"] = val

    # ---- 6. digit arrays
    xa = np.array(xs, dtype=object)
    x_int = np.array(xs, dtype=np.int64)
    x0 = x_int % p
    x1 = (x_int // p) % p
    x2 = x_int // (p * p)
    x0[0] = x1[0] = x2[0] = 0
    del xa
    pr = float(p)
    sq = int(round(math.sqrt(p)))
    B = sq

    # ---- 7. additive-defect scan
    ks = rng.integers(1, ordG, M_PAIRS)
    ls = rng.integers(1, ordG, M_PAIRS)
    ss = (ks + ls) % ordG
    msk = ss != 0
    ks = ks[msk]; ls = ls[msk]; ss = ss[msk]
    dirs = [(0, 1), (1, 0), (1, 1), (1, p - 1)]
    nb = min(p, N_DIRS - len(dirs))
    betas = rng.choice(p, size=nb, replace=False)
    seen = set(dirs)
    for be in betas:
        dd = (1, int(be))
        if dd not in seen:
            dirs.append(dd); seen.add(dd)
    dirs = dirs[:N_DIRS]
    dl = delta_scan(x1, x2, ks, ls, ss, p, dirs)
    imin = int(np.argmin(dl))
    cell["n_pairs"] = int(len(ks))
    cell["n_dirs"] = len(dirs)
    cell["min_delta"] = float(dl[imin])
    cell["argmin_L"] = list(dirs[imin])
    cell["delta_median"] = float(np.median(dl))
    cell["delta_max"] = float(dl.max())
    cell["min_delta_coincidences"] = int(round((1 - dl[imin]) * len(ks)))
    cell["delta_null_expected"] = 1 - 1.0 / p
    cell["delta_kill_threshold"] = 1 - C_KILL / math.sqrt(p)
    cell["delta_axes"] = {"(1,0)=x1": float(dl[1]), "(0,1)=x2": float(dl[0])}
    cell["delta_n_below_threshold"] = int(np.sum(dl < cell["delta_kill_threshold"]))
    log('delta scan: min=%.6f argmin=%s thr=%.6f' % (dl[imin], dirs[imin], cell["delta_kill_threshold"]))
    # x0 alone for reference (not in the functional family)
    d0 = x0[ss] - x0[ks] - x0[ls]
    cell["delta_x0_alone"] = float(np.count_nonzero(d0 % p) / len(ks))

    # planted control for the scan: x1' = k mod p, x2'=0, only non-wrapping pairs -> Delta must be 0
    cx1 = (np.arange(ordG, dtype=np.int64)) % p
    cx2 = np.zeros(ordG, dtype=np.int64)
    kc = rng.integers(1, ordG // 2, 20000); lc = rng.integers(1, ordG // 2, 20000); sc = kc + lc
    cdl = delta_scan(cx1, cx2, kc, lc, sc, p, dirs[:20])
    val["planted_additive_control_min_delta"] = float(cdl.min())
    val["planted_additive_control_detected"] = bool(cdl.min() == 0.0)

    # ---- 8. mutual information
    S = min(ordG - 1, S_MAX)
    ksamp = rng.choice(np.arange(1, ordG), size=S, replace=False)
    kb = (ksamp.astype(np.int64) * B // ordG).astype(np.int64)
    xb1 = (x1[ksamp] * B // p).astype(np.int64)
    xb2 = (x2[ksamp] * B // p).astype(np.int64)
    mi, mnull, msd = mi_with_null(xb1, kb, B, S, rng, N_PERM)
    cell["n_points_sampled"] = int(S)
    cell["mi_bins"] = B
    cell["mi_estimate"] = mi
    cell["mi_null_mean"] = mnull
    cell["mi_null_sd"] = msd
    cell["mi_z"] = (mi - mnull) / msd if msd > 0 else None
    mi2, mnull2, msd2 = mi_with_null(xb2, kb, B, S, rng, 50)
    cell["mi_x2"] = {"mi": mi2, "null_mean": mnull2, "null_sd": msd2, "z": (mi2 - mnull2) / msd2 if msd2 > 0 else None}
    # planted MI control: x-bin = k-bin (+ noise) -> MI >> null
    xb_ctrl = np.clip(kb + rng.integers(0, 2, S), 0, B - 1)
    mic, micn, micsd = mi_with_null(xb_ctrl, kb, B, S, rng, 20)
    val["planted_mi_control"] = {"mi": mic, "null_mean": micn, "null_sd": micsd, "detected": bool(mic > micn + 10 * max(micsd, 1e-12))}
    log('MI: %.5f null %.5f +- %.5f' % (mi, mnull, msd))

    # ---- 9. closure of argmin-L sublevel set
    def closure(al, be):
        f = (al * x1 + be * x2) % p
        f[0] = p  # exclude index 0
        thr = int(p / math.sqrt(p))
        A = np.nonzero(f < thr)[0]
        m = len(A)
        if m < 3:
            return {"set_size": int(m), "note": "set too small"}
        kk, ll = np.meshgrid(A, A, indexing='ij')
        iu = np.triu_indices(m, k=1)
        kk = kk[iu]; ll = ll[iu]
        if len(kk) > 2000000:
            sel_ = rng.choice(len(kk), size=2000000, replace=False)
            kk = kk[sel_]; ll = ll[sel_]
        s_ = (kk + ll) % ordG
        mk = s_ != 0
        s_ = s_[mk]
        hit = f[s_] < thr
        obs = float(hit.mean())
        base = m / (ordG - 1.0)
        npairs = len(s_)
        sd = math.sqrt(base * (1 - base) / npairs)
        # permutation null: relabel which m points are 'in' the set (same size), recompute closure statistic
        nper = 500
        pn = np.empty(nper)
        for ii in range(nper):
            A2 = rng.choice(ordG - 1, size=m, replace=False) + 1
            mask2 = np.zeros(ordG, dtype=bool)
            mask2[A2] = True
            k2 = A2[iu[0]]; l2 = A2[iu[1]]
            s2 = (k2 + l2) % ordG
            m2 = s2 != 0
            pn[ii] = mask2[s2[m2]].mean()
        pmean = float(pn.mean()); psd = float(pn.std(ddof=1))
        pz = (obs - pmean) / psd if psd > 0 else None
        pval = float((np.sum(pn >= obs) + 1) / (nper + 1))
        return {"perm_null_mean": pmean, "perm_null_sd": psd, "perm_z": pz, "perm_p_upper": pval, "n_perm": nper,
                "set_size": int(m), "n_pairs": int(npairs), "observed_closure": obs, "base_rate": base,
                "base_rate_theory": 1.0 / math.sqrt(p), "closure_ratio": obs / base if base > 0 else None,
                "z": (obs - base) / sd if sd > 0 else None, "threshold": thr}
    cl = closure(*dirs[imin])
    cell["closure"] = cl
    cell["closure_ratio"] = cl.get("closure_ratio")
    cell["base_rate"] = cl.get("base_rate")
    cell["closure_z"] = cl.get("z")
    cell["closure_perm_z"] = cl.get("perm_z")
    cell["closure_perm_null_mean"] = cl.get("perm_null_mean")
    cell["closure_perm_null_sd"] = cl.get("perm_null_sd")
    cell["closure_perm_p_upper"] = cl.get("perm_p_upper")
    cell["closure_extra"] = {"x1_(1,0)": closure(1, 0), "x2_(0,1)": closure(0, 1)}
    log('closure: %s' % json.dumps(jclean(cl)))

    # ---- 10. cell verdict
    kill_delta = cell["min_delta"] >= cell["delta_kill_threshold"]
    kill_mi = (cell["mi_z"] is None) or (abs(cell["mi_z"]) <= 3.0)
    kill_cl = (cell["closure_perm_z"] is None) or (abs(cell["closure_perm_z"]) <= 2.0)
    cell["kill_delta"] = bool(kill_delta); cell["kill_mi"] = bool(kill_mi); cell["kill_closure"] = bool(kill_cl)
    cell["cell_kill"] = bool(kill_delta and kill_mi and kill_cl)
    cell["win_strong"] = bool(cell["min_delta"] <= 0.5)
    cell["validation_all_ok"] = bool(
        val.get("H_d_of_Jtilde_zero_mod_p3") and val.get("Jtilde_reduces_to_jE") and val.get("j_of_lifted_curve_equals_Jtilde_mod_p3")
        and val.get("G_tilde_on_curve") and val.get("reduction_matches_Fp_arith_and_on_curve") and fixed_ok
        and val.get("planted_additive_control_detected") and val["planted_mi_control"]["detected"]
        and val.get("H_d_splits_completely_mod_p") is not False and cell["ordinary"])
    cell["seconds"] = time.time() - tc
    return cell


def main():
    done = []
    for (bits, rep) in CELLS:
        key = ckey(bits, rep)
        if time.time() - T0 > GLOBAL_BUDGET - 200:
            RESULT["censored"].append({"cell": key, "reason": "global time budget"})
            continue
        try:
            RESULT["cells"][key] = run_cell(bits, rep)
            done.append((bits, rep))
        except BaseException as e:
            tb = traceback.format_exc()
            log('cell %s failed: %s' % (key, e))
            print(tb, flush=True)
            RESULT["cells"][key] = {"bits": bits, "rep": rep, "error": str(e)[:300], "traceback": tb[-1500:], "censored": True}
            RESULT["censored"].append({"cell": key, "reason": "error: " + str(e)[:200]})
        write_result()
    fin = [ckey(b, r) for (b, r) in done if "min_delta" in RESULT["cells"][ckey(b, r)]]
    C = RESULT["cells"]
    if fin:
        big = max(C[k]["bits"] for k in fin)
        RESULT["min_delta"] = [C[k]["min_delta"] for k in fin if C[k]["bits"] == big][0]
        RESULT["min_delta_cell_bits"] = big
        RESULT["min_delta_by_cell"] = {k: C[k]["min_delta"] for k in fin}
        allval = all(C[k]["validation_all_ok"] for k in fin)
        RESULT["validation"] = {"all_cells_validation_ok": bool(allval),
                                "lift_verified_phi_fixed_to_p3": {k: C[k]["frobenius_fixed_verified"] for k in fin},
                                "cm_lift_H_d_zero_mod_p3": {k: C[k]["validation"].get("H_d_of_Jtilde_zero_mod_p3") for k in fin}}
        reps = [C[k] for k in fin if C[k]["bits"] == 12]
        zs = [c["closure_perm_z"] for c in reps if c.get("closure_perm_z") is not None]
        rep_summary = {"n_curves": len(zs), "perm_z_each": {k: C[k]["closure_perm_z"] for k in fin if C[k]["bits"] == 12},
                       "perm_p_each": {k: C[k]["closure_perm_p_upper"] for k in fin if C[k]["bits"] == 12},
                       "closure_ratio_each": {k: C[k]["closure_ratio"] for k in fin if C[k]["bits"] == 12},
                       "mean_z": float(np.mean(zs)) if zs else None, "sd_z": float(np.std(zs, ddof=1)) if len(zs) > 1 else None,
                       "n_with_z_gt_2": int(sum(1 for z in zs if z > 2.0))}
        RESULT["p2_12_replicates"] = rep_summary
        RESULT["closure_perm_z_by_cell"] = {k: C[k]["closure_perm_z"] for k in fin}
        rep_reproduces = rep_summary["n_with_z_gt_2"] >= 3
        big_ok = all(abs(C[k]["closure_perm_z"]) <= 2.0 for k in fin if C[k]["bits"] >= 16 and C[k].get("closure_perm_z") is not None)
        delta_ok = all(C[k]["kill_delta"] for k in fin)
        mi_ok = all(C[k]["kill_mi"] for k in fin)
        RESULT["strict_all_cells_kill"] = bool(all(C[k]["cell_kill"] for k in fin))
        RESULT["kill_basis"] = {"additive_defect_null_all_cells": bool(delta_ok), "mi_null_all_cells": bool(mi_ok),
                                "closure_at_base_rate_for_p_ge_2^16": bool(big_ok),
                                "p2_12_excess_reproduces_across_curves": bool(rep_reproduces),
                                "case": ("(b) reproduces across curves: finite-size effect that does not scale" if rep_reproduces else
                                         "(a) does not reproduce across the 5 curves: sampling fluke")}
        if allval and delta_ok and mi_ok and big_ok:
            RESULT["verdict"] = "kill"
            RESULT["verdict_reason"] = ("Scale criterion: additive-defect and MI are null at all p; closure is at base rate for all p >= 2^16; "
                                        "the p=2^12 excess " + ("reproduces => genuine finite-size effect that does not scale" if rep_reproduces else
                                        "does not reproduce across the 5 curves => sampling fluke") + ". No closure structure survives to cryptographic scale.")
        else:
            RESULT["verdict"] = "inconclusive"
            RESULT["verdict_reason"] = "validation/delta/MI/large-p closure criterion failed; see kill_basis"
    else:
        RESULT["verdict_reason"] = "no cell finished"
    RESULT["total_seconds"] = time.time() - T0
    write_result()


try:
    main()
except BaseException as e:
    RESULT["fatal"] = traceback.format_exc()[-2000:]
    write_result()
    raise
finally:
    write_result()
