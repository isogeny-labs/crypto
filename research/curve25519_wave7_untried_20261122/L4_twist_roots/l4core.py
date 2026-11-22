#!/usr/bin/env python3
"""L4 core: exact F_p root counting of the Montgomery S3 over x-pairs, with the
curve class (E / twist / 2-torsion) of every root.  Python stdlib + numpy only.

Montgomery curve  y^2 = x^3 + A x^2 + x  (B = 1); f(x) = x^3 + A x^2 + x.
Class code of an x in F_p: 0 = Z (f(x) = 0: x on both curves, 2-torsion or x = 0),
1 = E (f(x) a nonzero square), 2 = T (f(x) a non-square: x only on the twist).

S3 variants, as a X^2 - 2 B2 X + c = 0 with B2 = (x1+x2)(x1 x2+1) + 2A x1 x2:
  'std'   a = (x1-x2)^2,     c = (x1 x2 - 1)^2   (the standard Montgomery
          differential-addition polynomial; roots x(P1+P2), x(P1-P2))
  'given' a = (x1 x2 - 1)^2, c = (x1-x2)^2        (as written in the L4 task text:
          the reversed polynomial; its roots are 1/x(P1+-P2) = x(P1+-P2+T0))
"""
import hashlib
import numpy as np

CURVES = {
    # k: (p, A, claimed #E)   claimed values are re-verified by curve_facts()
    16: (65437, 1058, 65672),
    20: (1048573, 926, 8 * 131203),
    24: (16777141, 7842, None),   # brief's faithful member; order verified at run time
    # k = 9 is a SMOKE-ONLY toy curve (full rational 2-torsion: A = 2 + 1/2 mod p), not a family member
    9: (1021, 513, None),
}


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def factor_small(n):
    out, q = [], 2
    while q * q <= n:
        while n % q == 0:
            out.append(q)
            n //= q
        q += 1
    if n > 1:
        out.append(n)
    return out


class Curve:
    def __init__(self, k):
        self.k = k
        self.p, self.A, self.claimed_order = CURVES[k]
        p, A = self.p, self.A
        x = np.arange(p, dtype=np.int64)
        sq = x * x % p
        leg = -np.ones(p, dtype=np.int8)
        leg[sq] = 1
        leg[0] = 0
        self.leg = leg                                   # Legendre symbol table
        sqrt = np.zeros(p, dtype=np.int64)
        sqrt[sq] = x
        self.sqrt = sqrt                                 # some square root of each QR
        self.inv = np.zeros(p, dtype=np.int64)
        self.inv[1:] = np.array([pow(int(a), -1, p) for a in range(1, p)], dtype=np.int64)
        fx = x * ((x * x + A * x + 1) % p) % p
        self.fx = fx
        lf = leg[fx]
        cls = np.zeros(p, dtype=np.int8)                 # 0 Z, 1 E, 2 T
        cls[lf == 1] = 1
        cls[lf == -1] = 2
        self.cls = cls
        self.Zx = np.nonzero(cls == 0)[0]
        self.Ex = np.nonzero(cls == 1)[0]
        self.Tx = np.nonzero(cls == 2)[0]
        # smallest non-residue (twist parameter)
        self.nonres = int(np.nonzero(leg == -1)[0][0])

    def facts(self):
        p, A = self.p, self.A
        nE = p + 1 + int(self.leg[self.fx].astype(np.int64).sum())
        # twist: B y^2 = f with B non-residue has p + 1 - (nE - p - 1) points
        nT = 2 * (p + 1) - nE
        disc_sq = int(self.leg[(A * A - 4) % p])
        return {
            "k": self.k, "p": p, "p_prime": is_prime(p), "A": A,
            "p_mod_8": p % 8,
            "#E": nE, "#E_claimed": self.claimed_order, "#E_matches_claim": (None if self.claimed_order is None else nE == self.claimed_order),
            "#E_factored": factor_small(nE), "#twist": nT, "#twist_factored": factor_small(nT),
            "#E+#twist==2p+2": nE + nT == 2 * p + 2,
            "n_x_Z": int(len(self.Zx)), "n_x_E": int(len(self.Ex)), "n_x_T": int(len(self.Tx)),
            "Z_x_values": [int(v) for v in self.Zx],
            "A2m4_is_square": disc_sq == 1,   # full rational 2-torsion iff true
            "x_eq_1_class": int(self.cls[1]), "x_eq_minus1_class": int(self.cls[p - 1]),
            "nonresidue_used_for_twist": self.nonres,
        }


def roots(C, x1, x2, A=None, variant="std", want_roots=False):
    """Exact F_p roots of S3(x1, x2, X) for arrays x1, x2 (int64, same shape).

    Returns dict of arrays: nroots (distinct finite F_p roots, 0..2), nE, nT, nZ (roots by
    class), degen (leading coeff 0: one root is X = infinity or the polynomial is identically
    zero), allzero (identically zero polynomial), pc (unordered pair class lo*3+hi, class codes
    0 Z, 1 E, 2 T) and, if want_roots, X1, X2 (-1 where absent).  A overrides C.A (controls).
    """
    p = C.p
    A = C.A if A is None else A
    m = x1 * x2 % p
    B2 = ((x1 + x2) * (m + 1) + 2 * A * m) % p
    u = (x1 - x2) % p
    u2 = u * u % p
    w = (m - 1) % p
    w2 = w * w % p
    if variant == "std":
        a, c = u2, w2
    elif variant == "given":
        a, c = w2, u2
    else:
        raise ValueError(variant)
    d = (B2 * B2 - a * c) % p
    l = C.leg[d].astype(np.int64)
    nz = a != 0
    n = np.zeros(x1.shape, dtype=np.int8)
    X1 = -np.ones(x1.shape, dtype=np.int64)
    X2 = -np.ones(x1.shape, dtype=np.int64)
    ia = C.inv[a]
    sq = C.sqrt[d]
    m1 = nz & (l >= 0)
    m2 = nz & (l == 1)
    X1[m1] = ((B2 + sq) * ia % p)[m1]
    X2[m2] = (((B2 - sq) % p) * ia % p)[m2]
    # a == 0: linear  -2 B2 X + c = 0
    lin = (~nz) & (B2 != 0)
    if lin.any():
        X1[lin] = (c * C.inv[(2 * B2) % p] % p)[lin]
    allzero = (~nz) & (B2 == 0) & (c == 0)
    ok1 = X1 >= 0
    ok2 = X2 >= 0
    n = ok1.astype(np.int8) + ok2.astype(np.int8)
    c1 = np.where(ok1, C.cls[np.where(ok1, X1, 0)], -1)
    c2 = np.where(ok2, C.cls[np.where(ok2, X2, 0)], -1)
    nE = (c1 == 1).astype(np.int8) + (c2 == 1)
    nT = (c1 == 2).astype(np.int8) + (c2 == 2)
    nZ = (c1 == 0).astype(np.int8) + (c2 == 0)
    k1 = C.cls[x1].astype(np.int64)
    k2 = C.cls[x2].astype(np.int64)
    pc = np.minimum(k1, k2) * 3 + np.maximum(k1, k2)
    out = dict(nroots=n, nE=nE, nT=nT, nZ=nZ, degen=(~nz), allzero=allzero, pc=pc, disc=d)
    if want_roots:
        out["X1"], out["X2"] = X1, X2
    return out


KEYSIZE = 2 * 9 * 27


def hist_key(r):
    """Histogram key: degen*243 + pc*27 + nE*9 + nT*3 + nZ  (allzero folded into degen)."""
    return (r["degen"].astype(np.int64) * 243 + r["pc"] * 27 + r["nE"].astype(np.int64) * 9
            + r["nT"].astype(np.int64) * 3 + r["nZ"].astype(np.int64))


def derived_seed(*parts):
    """Seed rule: first 8 bytes of sha256 of the '|'-joined parts."""
    h = hashlib.sha256("|".join(str(q) for q in parts).encode()).digest()
    return int.from_bytes(h[:8], "big")


PC_NAMES = {0: "ZZ", 1: "ZE", 2: "ZT", 4: "EE", 5: "ET", 8: "TT"}


def decode_hist(h):
    """h: array of length KEYSIZE -> list of dict rows (nonzero cells)."""
    rows = []
    for key in np.nonzero(h)[0]:
        key = int(key)
        degen, rest = divmod(key, 243)
        pc, rest = divmod(rest, 27)
        nE, rest = divmod(rest, 9)
        nT, nZ = divmod(rest, 3)
        rows.append(dict(degenerate_a0=bool(degen), pair_class=PC_NAMES[pc], roots=nE + nT + nZ,
                         roots_on_E=nE, roots_on_twist=nT, roots_on_Z=nZ, count=int(h[key])))
    return rows
