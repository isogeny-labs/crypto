from .poly import cantor, smul, point, IDENT, ev


def find_modulus(p, k, chi):
    if k == 2:
        ns = next(c for c in range(2, p) if chi[c] == -1)
        return ns
    if k == 3:
        for c in range(1, p):
            if all((x ** 3 + x + c) % p != 0 for x in range(p)):
                return c
        raise AssertionError("no cubic modulus")
    raise ValueError


def N_k(h, p, k, chi, n_inf, par=None):
    """Number of points over F_{p^k}."""
    if k == 1:
        tot = n_inf
        for x in range(p):
            tot += 1 + chi[ev(h, x, p)]
        return tot
    if k == 2:
        ns = par
        tot = n_inf
        hh = list(reversed(h))
        for a in range(p):
            for b in range(p):
                x0, x1 = 0, 0
                # Horner in F_p[t]/(t^2 - ns)
                r0, r1 = hh[0], 0
                for c in hh[1:]:
                    n0 = (r0 * a + ns * r1 * b + c) % p
                    n1 = (r0 * b + r1 * a) % p
                    r0, r1 = n0, n1
                nm = (r0 * r0 - ns * r1 * r1) % p
                tot += 1 + chi[nm]
        return tot
    if k == 3:
        c0 = par  # t^3 = -t - c0
        tot = n_inf
        hh = list(reversed(h))
        for a in range(p):
            for b in range(p):
                for c in range(p):
                    r0, r1, r2 = hh[0], 0, 0
                    for co in hh[1:]:
                        # (r0+r1 t+r2 t^2)*(a+bt+ct^2)
                        m0 = r0 * a
                        m1 = r0 * b + r1 * a
                        m2 = r0 * c + r1 * b + r2 * a
                        m3 = r1 * c + r2 * b
                        m4 = r2 * c
                        # t^3=-t-c0 ; t^4=-t^2-c0 t
                        n0 = m0 - c0 * m3
                        n1 = m1 - m3 - c0 * m4
                        n2 = m2 - m4
                        r0, r1, r2 = (n0 + co) % p, n1 % p, n2 % p
                    # norm = det of multiplication matrix
                    # z*t = (-c0*r2, r0 - r2, r1) ; z*t^2 = (-c0*r1, -c0*r2 - r1 + ... )
                    # compute via generic mul
                    zt = _mulk3((r0, r1, r2), (0, 1, 0), c0, p)
                    zt2 = _mulk3((r0, r1, r2), (0, 0, 1), c0, p)
                    z = (r0, r1, r2)
                    nm = (z[0] * (zt[1] * zt2[2] - zt[2] * zt2[1])
                          - zt[0] * (z[1] * zt2[2] - z[2] * zt2[1])
                          + zt2[0] * (z[1] * zt[2] - z[2] * zt[1])) % p
                    tot += 1 + chi[nm]
        return tot
    raise ValueError


def _mulk3(u, v, c0, p):
    m0 = u[0] * v[0]
    m1 = u[0] * v[1] + u[1] * v[0]
    m2 = u[0] * v[2] + u[1] * v[1] + u[2] * v[0]
    m3 = u[1] * v[2] + u[2] * v[1]
    m4 = u[2] * v[2]
    return ((m0 - c0 * m3) % p, (m1 - m3 - c0 * m4) % p, (m2 - m4) % p)


def lpoly(h, p, g, chi, n_inf):
    Ns = {}
    for k in range(1, g + 1):
        par = None if k == 1 else find_modulus(p, k, chi)
        Ns[k] = N_k(h, p, k, chi, n_inf, par)
    L = [0] * (2 * g + 1)
    L[0] = 1
    for k in range(1, g + 1):
        tot = sum((Ns[i] - p ** i - 1) * L[k - i] for i in range(1, k + 1))
        assert tot % k == 0
        L[k] = tot // k
    for k in range(0, g):
        L[2 * g - k] = p ** (g - k) * L[k]
    return L, Ns


def order_of_delta(Dd, J, h, g, p):
    n = J
    fac = []
    q = 2
    m = J
    while q * q <= m:
        if m % q == 0:
            fac.append(q)
            while m % q == 0:
                m //= q
        q += 1
    if m > 1:
        fac.append(m)
    o = J
    for q in fac:
        while o % q == 0 and smul(Dd, o // q, h, g, p) == IDENT:
            o //= q
    return o


def random_divisor(rng, tab, h, g, p):
    D = IDENT
    for _ in range(g):
        while True:
            hv = rng.next()
            X = hv % p
            if tab.chi[tab.hx[X]] == 1:
                break
        Y = tab.sq[tab.hx[X]]
        if (hv >> 63) & 1:
            Y = p - Y
        D = cantor(D, point(X, Y, p), h, g, p)
    return D
