#!/usr/bin/env python3
"""Coppersmith reach of the Semaev summation polynomials under the
Jochemsz–May basic strategy, computed exactly from the monomial box, and
the family-wide ceiling for any shift family of a single polynomial.

For S_{m+1}(x_1, …, x_m, x_R) with x_R known, the unknowns are the m
factor-base abscissae, each of degree d = 2^{m-1} in S_{m+1} (the Newton
polytope is the full box [0, d]^m up to a finite set of missing corners;
the BKK/box-saturation result in the ledger, THM_BKKMV2, says mixed volume
equals the Bézout box for m ≤ 5, so the box is the right lattice shape).

Basic strategy (Jochemsz–May 2006): with shift level t, lattice monomials
M_t = [0, d·t]^m; shift polynomials x^a · f^j · p^{t−j} for x^a in the
layer M_j \\ M_{j−1}.  With every |x_i| ≤ p^δ the Howgrave-Graham condition
det(L) < p^{t·dim} reads

    δ · s_x + s_p < t · dim,   s_x = Σ_{a ∈ M_t} |a|_1,
    s_p = Σ_{j=0}^{t} (t − j) · |M_j \\ M_{j−1}|,   dim = |M_t|,

so δ_t = (t·dim − s_p) / s_x, and δ_∞ = lim_{t→∞} δ_t.  This script computes
δ_t exactly in rationals and its limit in closed form, for m = 2 … 5.

Run: python3 reach.py
"""
from fractions import Fraction
from itertools import product


def box_count(m, upper):
    """Number of exponent vectors in [0, upper]^m."""
    return (upper + 1) ** m


def sum_l1(m, upper):
    """Σ |a|_1 over a ∈ [0, upper]^m = m · (upper(upper+1)/2) · (upper+1)^(m−1)."""
    return m * (upper * (upper + 1) // 2) * (upper + 1) ** (m - 1)


def delta_t(m, d, t):
    dim = box_count(m, d * t)
    s_x = sum_l1(m, d * t)
    # Shift x^a f^j p^{t-j} with x^a in the layer M_{t-j} \ M_{t-j-1}, so every
    # lattice monomial lies in M_t and each is a leading monomial once; the
    # p-exponent of the layer at distance k = t - j is k.
    s_p = 0
    prev = 0
    for k in range(0, t + 1):
        layer = box_count(m, d * k) - prev
        prev = box_count(m, d * k)
        s_p += k * layer
    return Fraction(t * dim - s_p, s_x), dim


def delta_inf(m, d):
    """Limit of delta_t.  With u = d·t: dim ~ u^m, s_x ~ m·u^{m+1}/2, and
    s_p = Σ_k k·|M_k \\ M_{k-1}| ~ d^m · m · t^{m+1}/(m+1), so
    t·dim − s_p ~ d^m t^{m+1}/(m+1) and
    δ_∞ = (d^m t^{m+1}/(m+1)) / (m d^{m+1} t^{m+1}/2) = 2 / (m (m+1) d),
    which is the program's 1/(m(m+1)·2^{m−2}) for d = 2^{m−1}."""
    return Fraction(2, m * (m + 1) * d)


if __name__ == "__main__":
    print("Jochemsz–May basic strategy on the box [0, d]^m of S_{m+1}, x_R known")
    print("m  d=2^(m-1)  needed(§2.4)   δ_t at t=1..6 (lattice dim)            δ_∞ (closed form)")
    needed = {2: Fraction(1, 2), 3: Fraction(1, 4), 4: Fraction(1, 5), 5: Fraction(1, 6)}
    for m in (2, 3, 4, 5):
        d = 2 ** (m - 1)
        row = []
        for t in range(1, 7):
            dl, dim = delta_t(m, d, t)
            row.append(f"{float(dl):.4f}({dim})")
        print(f"{m}  {d:>3}        {str(needed[m]):>5}        {' '.join(row)}   {delta_inf(m, d)} = {float(delta_inf(m, d)):.4f}")
    print()
    print("Family-wide ceiling for any shift family of one polynomial (research/lit2026_w10_s5_coppersmith_20261011): 1/(m·d) =",
          ", ".join(f"m={m}: {Fraction(1, m * 2 ** (m - 1))}" for m in (2, 3, 4, 5)))
