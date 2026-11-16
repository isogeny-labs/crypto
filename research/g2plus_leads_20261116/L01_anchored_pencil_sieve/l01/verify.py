from .curve import IDENT


def pmul(a, b, p):
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] = (r[i + j] + x * y) % p
    return r


def peval(c, x, p):
    v = 0
    for t in reversed(c):
        v = (v * x + t) % p
    return v


def v1_pencil(p, f, g, a0, Pi, lam, points):
    """f - a^2 == prod (x - x_k) over all 2g+1 points, and y_k == a(x_k)."""
    if len(points) != 2 * g + 1:
        return False
    a = [(a0[k] + lam * Pi[k]) % p for k in range(g)] + [lam * Pi[g] % p]
    a2 = pmul(a, a, p)
    N = [(f[i] - (a2[i] if i < len(a2) else 0)) % p for i in range(len(f))]
    prod = [1]
    for x, _ in points:
        prod = pmul(prod, [(-x) % p, 1], p)
    if N != prod:
        return False
    return all(peval(a, x, p) == y for x, y in points)


def v1_walk(p, f, g, D, points):
    """u == prod (x - x_k), v(x_k) == y_k, y_k^2 == f(x_k)."""
    u, v = D
    if len(points) != g or len(u) - 1 != g:
        return False
    prod = [1]
    for x, _ in points:
        prod = pmul(prod, [(-x) % p, 1], p)
    if list(u) != prod:
        return False
    return all(peval(list(v), x, p) == y and y * y % p == peval(f, x, p) for x, y in points)


def v2_sum(vc, points):
    D = IDENT
    for x, y in points:
        D = vc.cantor(D, vc.point(x, y))
    return D
