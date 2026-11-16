#!/usr/bin/env python3
"""Independent exact computation of d'_F and d_F(inside) over F_2 (standard library only).

Shares no code with the Rust instrument. Polynomials are sets of exponent tuples; linear
algebra over F_2 uses Python ints as bit rows. A fall at degree d (Nagao Definitions 5/6,
equality reading) exists iff the space K of admissible multiplier tuples whose sum has
degree < d is contained neither in Z (tuples with sum 0) nor in W (tuples whose every
product has degree <= d - 1): a vector space is never the union of two proper subspaces.
Membership of every basis vector of K in Z (resp. W) is tested directly.

* True degree, field equations inside: generators F + {x_i^2 + x_i}; a multiplier g_i
  ranges over polynomials of degree <= d - deg f_i (F_2[x] is a domain, so
  deg(g_i f_i) = deg g_i + deg f_i).
* Fake degree: generators reduced modulo the field equations; multipliers range over the
  multilinear quotient; every product is reduced; the field equations themselves reduce
  to 0 and are dropped.

Usage: independent_falls.py "<poly>; <poly>" n   e.g. "YZ+1; XZ+X+1" 3
       independent_falls.py --selftest
"""
import itertools
import re
import sys

VARS = "XYZWUVST"


def parse(s, n):
    poly = set()
    for term in s.replace(" ", "").split("+"):
        e = [0] * n
        if term != "1":
            for v, p in re.findall(r"([XYZWUVST])(?:\^(\d+))?", term):
                e[VARS.index(v)] += int(p) if p else 1
        poly ^= {tuple(e)}
    return frozenset(poly)


def deg(p):
    return max((sum(m) for m in p), default=-1)


def mono_mul(a, b):
    return tuple(x + y for x, y in zip(a, b))


def red_mono(m):
    return tuple(min(e, 1) for e in m)


def mul_mono(m, p, reduce=False):
    out = set()
    for t in p:
        r = mono_mul(m, t)
        if reduce:
            r = red_mono(r)
        out ^= {r}
    return out


def monos_upto(n, k):
    return [m for m in itertools.product(range(k + 1), repeat=n) if sum(m) <= k]


def kernel_basis(columns, nrows):
    """columns: list of int bit-columns (row index -> bit). Returns kernel basis as list of
    int masks over column indices (Gaussian elimination on the transpose)."""
    pivots = {}  # pivot row bit -> (column image, combination mask)
    basis = []
    for c, col in enumerate(columns):
        v, comb = col, 1 << c
        while v:
            low = v & -v
            if low in pivots:
                pv, pc = pivots[low]
                v ^= pv
                comb ^= pc
            else:
                pivots[low] = (v, comb)
                break
        if not v:
            basis.append(comb)
    return basis


def fall_exists(entries, cond_rows, d):
    """entries: list of (image poly as set of monos, product degree). cond_rows: monomials
    whose coefficient in the sum must vanish (degree >= d). Builds K, tests K ⊄ Z, K ⊄ W."""
    row_index = {m: i for i, m in enumerate(cond_rows)}
    cols = []
    for img, _ in entries:
        c = 0
        for m in img:
            if m in row_index:
                c ^= 1 << row_index[m]
        cols.append(c)
    not_in_z = not_in_w = False
    for comb in kernel_basis(cols, len(cond_rows)):
        s, uses_top = set(), False
        idx = comb
        while idx:
            low = idx & -idx
            i = low.bit_length() - 1
            s ^= entries[i][0]
            if entries[i][1] == d:
                uses_top = True
            idx ^= low
        not_in_z |= bool(s)
        not_in_w |= uses_top
        if not_in_z and not_in_w:
            return True
    return not_in_z and not_in_w


def true_first_fall_inside(F, n, dmax):
    gens = list(F) + [frozenset({tuple(2 if j == i else 0 for j in range(n)), tuple(1 if j == i else 0 for j in range(n))}) for i in range(n)]
    for d in range(1, dmax + 1):
        entries = []
        for f in gens:
            df = deg(f)
            if df > d:
                continue
            for m in monos_upto(n, d - df):
                entries.append((frozenset(mul_mono(m, f)), sum(m) + df))
        cond = [m for m in monos_upto(n, d) if sum(m) == d]
        if fall_exists(entries, cond, d):
            return d
    return None


def fake_first_fall(F, n):
    ml = [m for m in itertools.product((0, 1), repeat=n)]
    gens = [frozenset(red_mono(t) for t in f) for f in F]
    gens = [g for g in (frozenset(x for x in set().union(*[{red_mono(t)} for t in f]) if [red_mono(u) for u in f].count(x) % 2 == 1) for f in F) if g]
    for d in range(1, n + 1):
        entries = []
        for gi, f in enumerate(gens):
            for m in ml:
                img = frozenset(mul_mono(m, f, reduce=True))
                entries.append((img, deg(img), gi))
        # Conditions: every product of degree <= d (per generator, monomials of degree > d
        # vanish) and the sum of degree < d (monomials of degree >= d vanish).
        high = [m for m in ml if sum(m) > d]
        topd = [m for m in ml if sum(m) >= d]
        rows = [("g", gi, m) for gi in range(len(gens)) for m in high] + [("s", None, m) for m in topd]
        ri = {r: i for i, r in enumerate(rows)}
        cols = []
        for img, _, gi in entries:
            c = 0
            for m in img:
                if ("g", gi, m) in ri:
                    c ^= 1 << ri[("g", gi, m)]
                if ("s", None, m) in ri:
                    c ^= 1 << ri[("s", None, m)]
            cols.append(c)
        not_in_z = not_in_w = False
        for comb in kernel_basis(cols, len(rows)):
            s = set()
            per = [set() for _ in gens]
            idx = comb
            while idx:
                low = idx & -idx
                i = low.bit_length() - 1
                s ^= entries[i][0]
                per[entries[i][2]] ^= entries[i][0]
                idx ^= low
            not_in_z |= bool(s)
            not_in_w |= any(deg(p) >= d for p in per)
        if not_in_z and not_in_w:
            return d
    return None


def selftest():
    cases = [
        ("YZ+1; XZ+X+1", 3, 3, 1),          # A2 witness: inside 3, fake 1
        ("X^2Y; XY+X", 2, 3, 2),            # witness A: inside 3, fake 2
        ("XY+Z; XZ+Y", 3, 3, 2),            # witness B: inside 3, fake 2
        ("X; X+1", 1, 1, 1),                # positive control
    ]
    ok = True
    for s, n, want_in, want_fake in cases:
        F = [parse(p, n) for p in s.split(";")]
        got = (true_first_fall_inside(F, n, 6), fake_first_fall(F, n))
        good = got == (want_in, want_fake)
        ok &= good
        print(f"SELFTEST {{{s}}} n={n}: d_F(inside)={got[0]} d'_F={got[1]} expected ({want_in}, {want_fake}) -> {'PASS' if good else 'FAIL'}")
    return ok


if __name__ == "__main__":
    if sys.argv[1:] == ["--selftest"]:
        sys.exit(0 if selftest() else 1)
    sys_s, n = sys.argv[1], int(sys.argv[2])
    dmax = int(sys.argv[3]) if len(sys.argv) > 3 else 8
    F = [parse(p, n) for p in sys_s.split(";")]
    din, dfk = true_first_fall_inside(F, n, dmax), fake_first_fall(F, n)
    gap = (din - dfk) if (din is not None and dfk is not None) else None
    print(f"INDEPENDENT {{{sys_s}}} n={n}: d_F(inside)={din if din is not None else '>'+str(dmax)} d'_F={dfk} gap={gap}")
