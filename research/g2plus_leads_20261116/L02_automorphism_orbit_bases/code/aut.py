"""Automorphism maps (SPEC section 6)."""
import ff
from ff import fmul
import jac

def rho_apply(D, C, k=1):
    u, v = D
    d = len(u) - 1
    q = C.q
    ZP = C.ZP
    p = ff.p
    k %= C.aq if C.aq > 1 else 1
    if k == 0 or C.aq == 1:
        return D
    u2 = list(u)
    for i in range(d):
        e = ((d - i) * k) % q
        if e:
            u2[i] = fmul(ZP[e], u[i])
    v2 = list(v)
    for i in range(d):
        e = (-i * k) % q
        if e:
            if i < len(v):
                v2[i] = fmul(ZP[e], v[i])
            else:
                ff.cnt_mul(1)   # position v_i (zero) still charged per SPEC
    while v2 and v2[-1] == 0:
        v2.pop()
    return (u2, v2)

def canon_neg(D):
    p = ff.p
    u, v = D
    if not v:
        return D, 1
    c = v[-1]
    if c > (p - 1) // 2:
        return jac.neg(D), -1
    return D, 1

def canon_aut(D, C):
    """returns (D2, s, k)"""
    if C.aq == 1:
        D2, s = canon_neg(D)
        return D2, s, 0
    p = ff.p; q = C.q; ZP = C.ZP
    u, v = D
    d = len(u) - 1
    j0 = None
    for j in range(1, d + 1):
        if u[d - j] != 0:
            j0 = j; break
    if j0 is not None:
        base = u[d - j0]
        best = base; kbest = 0
        for k in range(1, q):
            c = fmul(ZP[(j0 * k) % q], base)
            if c < best:
                best = c; kbest = k
        D1 = rho_apply(D, C, kbest)
        D2, s = canon_neg(D1)
        return D2, s, kbest
    i0 = None
    for i in range(1, d):
        if i < len(v) and v[i] != 0:
            i0 = i; break
    if i0 is not None:
        vi = v[i0]
        best = vi; kbest = 0; sbest = 1
        if (-vi) % p < best:
            best = (-vi) % p; sbest = -1
        for k in range(1, q):
            c = fmul(ZP[(-i0 * k) % q], vi)
            if c < best:
                best = c; kbest = k; sbest = 1
            cn = (-c) % p
            if cn < best:
                best = cn; kbest = k; sbest = -1
        D2 = rho_apply(D, C, kbest)
        if sbest == -1:
            D2 = jac.neg(D2)
        return D2, sbest, kbest
    D2, s = canon_neg(D)
    return D2, s, 0

def orbit_rep_point(x, C):
    """x = zeta^k xs; returns (xs, k)"""
    if C.aq == 1:
        return x, 0
    q = C.q; zeta = C.zeta
    imgs = [x]
    for k in range(1, q):
        imgs.append(fmul(imgs[-1], zeta))
    xs = min(imgs)
    kp = imgs.index(xs)
    return xs, (-kp) % q
