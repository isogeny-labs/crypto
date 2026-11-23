"""Positive control for the Coppersmith harness (Idea 16 for the lattice).

Univariate f(x) = x^2 + a x + b mod N with a known root x0 <= N^e.  Theory:
Coppersmith recovers x0 for e < 1/2 (degree 2).  If this control does not sit
at ~0.45-0.50, the harness is miscalibrated and no bivariate number from it
may be reported.

Also serves as the accounting reference: every lattice run reports the
documented n^3 L / 64 charge plus raw coefficient work.
"""
import json
import sys

import sympy as sp

from lib import lattice as L
from lib.accounting import Record, write_jsonl


def univariate_shifts(coeffs, m, N):
    """x^i f^j N^(m-j), 0 <= j <= m, 0 <= i < deg f."""
    deg = len(coeffs) - 1
    f2 = {(k, 0): c for k, c in enumerate(coeffs)}
    fj = [{(0, 0): 1}]
    for _ in range(m):
        fj.append(L.poly_mul(fj[-1], f2))
    shifts = []
    for j in range(m + 1):
        for i in range(deg):
            g = {(k + i, 0): c * N**(m - j) for (k, _), c in fj[j].items()}
            shifts.append(((0, 0), g))
    return shifts


def run(pbits, xexp, m, a=3):
    N = int(sp.nextprime(2**pbits))
    x0 = int(N**xexp)
    b = (-x0*x0 - a*x0) % N
    shifts = univariate_shifts([b, a, 1], m, N)
    M, monoms = L.build({}, shifts, x0, 1)
    ranks = len(M)
    bits = max(abs(c).bit_length() for row in M for c in row if c)
    rows = L.reduce_fplll(M)
    found = 0
    pm = N**m
    for row in rows:
        h = L.to_poly(row, monoms, x0, 1)
        if abs(L.eval_int(h, x0, 0)) < pm:
            found += 1
    rec = Record(
        experiment="control_univariate",
        config={"p_bits": pbits, "x_exp": xexp, "m": m},
        counts={"lattice_word_ops": L.lll_charge(ranks, bits)},
        metrics={"rank": ranks, "monoms": len(monoms), "max_bits": bits,
                 "vanishing": found, "threshold": 0.5},
    )
    return rec, found > 0


if __name__ == "__main__":
    out = []
    print(f"{'p':>6} {'e':>5} {'m':>2} {'rank':>4} {'bits':>5} {'found':>5}")
    for pbits in (40, 56):
        for e in (0.30, 0.35, 0.40, 0.45, 0.48, 0.52):
            for m in (2,):
                rec, ok = run(pbits, e, m)
                d = rec.to_dict()
                print(f"{pbits:>6} {e:>5} {m:>2} {d['metrics']['rank']:>4} "
                      f"{d['metrics']['max_bits']:>5} {d['metrics']['vanishing']:>5}")
                d["passed"] = ok
                out.append(d)
    write_jsonl(sys.argv[1] if len(sys.argv) > 1 else "control_univariate.jsonl", out)
