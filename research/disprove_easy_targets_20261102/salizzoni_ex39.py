#!/usr/bin/env python3
"""Target B of PROTOCOL.md: Salizzoni Example 3.9 at k = 2, F = {x^2 + y, y^2 + x, xy}.

Exact linear algebra on the Macaulay spaces V_{F,d} = span{m * f_i : deg(m * f_i) <= d}
(Caminata-Gorla's V_{F,d}: the smallest space containing the f_i of degree <= d and closed
under multiplication by variables while the degree stays <= d). Checks, over F_2, F_3,
F_5 and Q:

1. d_reg (Bardet-Faugere-Salvy) = 2: (F^top)_2 = R_2 and (F^top)_1 != R_1.
2. x, y not in V_{F,2}; x, y in V_{F,3}  (so d_x = d_y = 3).
3. (F) = (x, y): x and y lie in V_{F,3}, and every f_i lies in (x, y); and
   V_{F,d} = (F) ∩ R_{<=d} (= polynomials of degree <= d with zero constant term) for
   d = 3..6 (dimension check), so no fall above 3: last fall degree = 3.
   By Caminata-Gorla Thm 2.9 with the reduced Groebner basis {x, y}: last fall = max(d_x, d_y) = 3.
Conclusion: last fall degree 3 > 2 = d_reg, refuting 'last fall degree <= d_reg' when d_reg
is read in the BFS sense. Standard library only.
"""
from fractions import Fraction
import itertools
import sys

# Polynomials in x, y: dict {(a, b): coeff}.
F = [
    {(2, 0): 1, (0, 1): 1},  # x^2 + y
    {(0, 2): 1, (1, 0): 1},  # y^2 + x
    {(1, 1): 1},             # xy
]


def deg(p):
    return max(a + b for (a, b), c in p.items() if c != 0)


def monos_upto(d):
    return [(a, t - a) for t in range(d + 1) for a in range(t + 1)]


class Field:
    def __init__(self, p):
        self.p = p  # None means Q

    def norm(self, c):
        return Fraction(c) if self.p is None else c % self.p

    def inv(self, c):
        return 1 / Fraction(c) if self.p is None else pow(c, self.p - 2, self.p)

    def name(self):
        return "Q" if self.p is None else f"F_{self.p}"


def to_vec(p, cols, K):
    idx = {m: i for i, m in enumerate(cols)}
    v = [K.norm(0)] * len(cols)
    for m, c in p.items():
        v[idx[m]] = K.norm(v[idx[m]] + c)
    return v


def rank(rows, K):
    rows = [r[:] for r in rows]
    r, ncols = 0, len(rows[0]) if rows else 0
    for c in range(ncols):
        piv = next((i for i in range(r, len(rows)) if K.norm(rows[i][c]) != 0), None)
        if piv is None:
            continue
        rows[r], rows[piv] = rows[piv], rows[r]
        inv = K.inv(rows[r][c])
        rows[r] = [K.norm(x * inv) for x in rows[r]]
        for i in range(len(rows)):
            if i != r and K.norm(rows[i][c]) != 0:
                f = rows[i][c]
                rows[i] = [K.norm(a - f * b) for a, b in zip(rows[i], rows[r])]
        r += 1
    return r


def macaulay_rows(d, K):
    cols = monos_upto(d)
    rows = []
    for f in F:
        df = deg(f)
        if df > d:
            continue
        for (a, b) in monos_upto(d - df):
            prod = {(fa + a, fb + b): c for (fa, fb), c in f.items()}
            rows.append(to_vec(prod, cols, K))
    return rows, cols


def in_span(rows, vec, K):
    return rank(rows + [vec], K) == rank(rows, K)


def top(p):
    d = deg(p)
    return {m: c for m, c in p.items() if m[0] + m[1] == d}


def check(K):
    out, ok = [], True
    # 1. d_reg in the BFS sense: (F^top)_d = R_d.
    tops = [top(f) for f in F]
    deg2 = [(2, 0), (1, 1), (0, 2)]
    span2 = [to_vec(t, deg2, K) for t in tops if deg(t) == 2]
    dreg2 = rank(span2, K) == 3
    dreg1_fails = True  # no generator has degree <= 1, so (F^top)_1 = 0 != R_1
    out.append(f"d_reg: (F^top)_2 = R_2 {dreg2}; (F^top)_1 = 0 != R_1 {dreg1_fails} => d_reg = 2")
    ok &= dreg2 and dreg1_fails
    # 2. x, y in V_{F,d}?
    res = {}
    for d in (2, 3):
        rows, cols = macaulay_rows(d, K)
        for name, g in (("x", {(1, 0): 1}), ("y", {(0, 1): 1})):
            res[(name, d)] = in_span(rows, to_vec(g, cols, K), K)
    out.append("membership: " + ", ".join(f"{n} in V_(F,{d}): {res[(n, d)]}" for n in "xy" for d in (2, 3)))
    ok &= (not res[("x", 2)]) and (not res[("y", 2)]) and res[("x", 3)] and res[("y", 3)]
    # 3. V_{F,d} = (F) ∩ R_{<=d} = {deg <= d, zero constant term} for d = 3..6.
    dims = []
    for d in range(3, 7):
        rows, cols = macaulay_rows(d, K)
        r = rank(rows, K)
        want = len(cols) - 1
        dims.append((d, r, want))
        ok &= r == want
        # and the constant 1 is never in V_{F,d}
        ok &= not in_span(rows, to_vec({(0, 0): 1}, cols, K), K)
    out.append("V_(F,d) = (x,y) ∩ R_<=d: " + ", ".join(f"d={d} dim {r}/{w}" for d, r, w in dims))
    return ok, out


def main():
    all_ok = True
    for p in (2, 3, 5, None):
        K = Field(p)
        ok, lines = check(K)
        all_ok &= ok
        print(f"[{K.name()}] " + " | ".join(lines) + f" -> {'PASS' if ok else 'FAIL'}")
    print("RESULT: d_reg = 2, d_x = d_y = 3, last fall degree = 3 in every field tried -> "
          + ("'last fall <= d_reg (BFS)' REFUTED" if all_ok else "NOT ESTABLISHED"))
    return 0 if all_ok else 1


if __name__ == "__main__":
    sys.exit(main())
