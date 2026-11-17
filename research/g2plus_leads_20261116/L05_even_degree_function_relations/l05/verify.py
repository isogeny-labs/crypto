from .field import GateFailure
from .poly import psub, pmul, trim, ev, div_class, IDENT


def v1(h, a, pts, ip, im, g, p, even):
    N = psub(h, pmul(a, a, p), p)
    if len(N) - 1 != len(pts):
        raise GateFailure("V1 degree: deg N=%d npts=%d" % (len(N) - 1, len(pts)))
    prod = [1]
    for (x, y) in pts:
        prod = pmul(prod, [(-x) % p, 1], p)
    lc = N[-1]
    if N != [c * lc % p for c in prod]:
        raise GateFailure("V1 N != lc prod(x-xk)")
    for (x, y) in pts:
        if ev(a, x, p) != y:
            raise GateFailure("V1 a(xk) != yk")
    if len({x for x, _ in pts}) != len(pts):
        raise GateFailure("V1 abscissae not distinct")
    if even:
        if len(pts) + ip + im != 0:
            raise GateFailure("V1 degree sum (even)")
    else:
        if len(pts) != 2 * g + 1:
            raise GateFailure("V1 degree sum (odd)")


def S_series(f, g, p):
    inv2 = (p + 1) // 2
    M = 2 * g + 3
    U = [f[2 * g + 2 - i] if i <= 2 * g + 2 else 0 for i in range(M)]
    S = [0] * M
    S[0] = 1
    for k in range(1, M):
        S[k] = (U[k] - sum(S[i] * S[k - i] for i in range(1, k))) * inv2 % p
    return S


def vinf_orders(f, a, g, p, S=None):
    if S is None:
        S = S_series(f, g, p)
    M = 2 * g + 3
    A = [(a[g + 1 - i] if 0 <= g + 1 - i < len(a) else 0) if i <= g + 1 else 0 for i in range(M)]
    out = []
    for sigma in (1, -1):
        k0 = None
        for i in range(M):
            if (sigma * S[i] - A[i]) % p != 0:
                k0 = i
                break
        if k0 is None:
            raise GateFailure("V_inf precision")
        out.append(k0 - (g + 1))
    return tuple(out)


def vinf(f, a, ip, im, g, p, S=None):
    o = vinf_orders(f, a, g, p, S)
    if o != (ip, im):
        raise GateFailure("V_inf orders %r != recorded %r" % (o, (ip, im)))


def v2_class(odd_h, omap, pts, ip, im, g, p, even):
    """Return the Cantor class of the divisor (identity => principal)."""
    if even:
        D = [(omap.map_point(P), 1) for P in pts]
        if ip:
            D.append((omap.map_point("inf+"), ip))
        if im:
            D.append((omap.map_point("inf-"), im))
    else:
        D = [(P, 1) for P in pts]
    return div_class(D, odd_h, g, p)


def v2(odd_h, omap, pts, ip, im, g, p, even):
    if v2_class(odd_h, omap, pts, ip, im, g, p, even) != IDENT:
        raise GateFailure("V2 not identity")


def mutations(odd_h, omap, anchors, residual, ip, im, g, p):
    """Return (m1_accepted, m2_accepted) for an even-arm relation."""
    pts = list(anchors) + list(residual)
    m1 = v2_class(odd_h, omap, pts, ip + 1, im - 1, g, p, True) == IDENT
    x, y = residual[-1]
    pts2 = list(anchors) + list(residual[:-1]) + [(x, (p - y) % p)]
    m2 = v2_class(odd_h, omap, pts2, ip, im, g, p, True) == IDENT
    return m1, m2
