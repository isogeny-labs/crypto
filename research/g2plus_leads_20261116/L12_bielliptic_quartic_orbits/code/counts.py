"""Uncounted point counts over F_p and F_{p^2} (SPEC.md section 4)."""
import math


def sqrt_table(p):
    T = [-1] * p
    T[0] = 0
    for v in range(1, (p - 1) // 2 + 1):
        T[v * v % p] = v
    return T


def nonresidue(p, T):
    d = 2
    while T[d % p] >= 0:
        d += 1
    return d


def count_Fp(curve):
    p = curve.p
    T = sqrt_table(p)
    inv2 = pow(2, p - 2, p)
    nE = nC = 0

    def nsq(w):
        if w == 0:
            return 1
        return 2 if T[w] > 0 else 0
    for x in range(p):
        hx = curve.h(x)
        fx = curve.f(x)
        D = (hx * hx + 4 * fx) % p
        if D == 0:
            nE += 1
            nC += nsq((-hx * inv2) % p)
        elif T[D] > 0:
            s = T[D]
            nE += 2
            nC += nsq((-hx + s) * inv2 % p) + nsq((-hx - s) * inv2 % p)
    return nE + 1, nC + 1


def count_Fp2(curve):
    p = curve.p
    T = sqrt_table(p)
    d = nonresidue(p, T)
    inv2 = pow(2, p - 2, p)
    a1, a3, a2, a4, a6 = curve.a1, curve.a3, curve.a2, curve.a4, curve.a6
    nE = nC = 0

    def nsq2(w0, w1):
        if w0 == 0 and w1 == 0:
            return 1
        N = (w0 * w0 - d * w1 * w1) % p
        return 2 if T[N] > 0 else 0

    for x0 in range(p):
        for x1 in range(p):
            # x^2, x^3
            s0 = (x0 * x0 + d * x1 * x1) % p
            s1 = (2 * x0 * x1) % p
            c0 = (s0 * x0 + d * s1 * x1) % p
            c1 = (s0 * x1 + s1 * x0) % p
            h0 = (a1 * x0 + a3) % p
            h1 = (a1 * x1) % p
            f0 = (c0 + a2 * s0 + a4 * x0 + a6) % p
            f1 = (c1 + a2 * s1 + a4 * x1) % p
            # D = h^2 + 4 f
            D0 = (h0 * h0 + d * h1 * h1 + 4 * f0) % p
            D1 = (2 * h0 * h1 + 4 * f1) % p
            if D0 == 0 and D1 == 0:
                nE += 1
                nC += nsq2((-h0 * inv2) % p, (-h1 * inv2) % p)
                continue
            N = (D0 * D0 - d * D1 * D1) % p
            if T[N] < 0:
                continue
            # sqrt2(D)
            if D1 == 0:
                if T[D0] >= 0:
                    b0, b1 = T[D0], 0
                else:
                    b0, b1 = 0, T[D0 * pow(d, p - 2, p) % p]
            else:
                n = T[N]
                z = (D0 + n) * inv2 % p
                if T[z] < 0:
                    z = (D0 - n) * inv2 % p
                b0 = T[z]
                b1 = D1 * pow(2 * b0, p - 2, p) % p
            assert ((b0 * b0 + d * b1 * b1) % p, (2 * b0 * b1) % p) == (D0, D1)
            nE += 2
            nC += nsq2((-h0 + b0) * inv2 % p, (-h1 + b1) * inv2 % p)
            nC += nsq2((-h0 - b0) * inv2 % p, (-h1 - b1) * inv2 % p)
    return nE + 1, nC + 1


def full_counts(curve, nE1=None):
    p = curve.p
    nE1c, nC1 = count_Fp(curve)
    nE2, nC2 = count_Fp2(curve)
    t1E = p + 1 - nE1c
    t1C = p + 1 - nC1
    t2E = p * p + 1 - nE2
    t2C = p * p + 1 - nC2
    s1 = t1C - t1E
    T2 = t2C - t2E
    parity = ((s1 * s1 - T2) % 2 == 0)
    s2 = (s1 * s1 - T2) // 2
    nP = 1 - s1 + s2 - p * s1 + p * p
    sq = math.sqrt(p)
    weil = ((sq - 1) ** 4 <= nP <= (sq + 1) ** 4)
    echeck = (nE2 == p * p + 1 - (t1E * t1E - 2 * p))
    return {"nE1": nE1c, "nC1": nC1, "nE2": nE2, "nC2": nC2, "t1E": t1E, "t1C": t1C,
            "t2E": t2E, "t2C": t2C, "s1": s1, "T2": T2, "s2": s2, "nP": nP,
            "parity": parity, "weil": weil, "echeck": echeck}
