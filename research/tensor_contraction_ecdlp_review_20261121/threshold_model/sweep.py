#!/usr/bin/env python3
"""Lever-combination sweep for the density-one #3-SAT main-term threshold.

Baseline B0 = Majdoddin column (U+C): tree base e0 = 0.2554, exact U (bar 0.1321),
Lemma C top, x0 optimised, rho = 1/sqrt2, paper overheads oz = oK = 3.
P1/P2 are the proved-by-reading overhead corrections (README). Rows marked
'conditional' rest on matrix-multiplication preprints; 'hypothetical' rows assume
pieces or balancing lemmas that do not exist. Values are main-term log2 m*.
"""
import math, sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else ".")
from threshold_model import bar, run_levels, lemmaC_threshold

RHO0 = 1 / math.sqrt(2)


def best(e0, rho, oz, oK, bar_, x0s=range(5, 15)):
    res = None
    for x0 in x0s:
        fronts = run_levels(x0, e0, rho, bar_, oz, oK)
        r = lemmaC_threshold(fronts, bar_)
        if r is None:
            continue
        if res is None or r[0] < res['log2m']:
            res = dict(log2m=r[0], x0=x0, levels=r[1], log2K=r[2].log2K, e_L=r[2].e,
                       zeta_top=r[3], eps=r[4], log2s_top=r[5], q=r[6])
    return res


B = bar(1.0, exact_U=True)      # 0.1321  (PSX 1.4423^m counting bound, U = 4m)
B125 = 0.5 / 4.0                # 0.125   (if Zhou-Yin-Zhou 1.4142^m for #3-SAT stands)

cases = [
    ("B0  Majdoddin (U+C) as proved: oz=3, oK=3", 0.2554, RHO0, 3, 3, B, "proved"),
    ("P1  oK 3->1 (case (ii) groups have exactly g/K nodes; drop c*lambda<=N/K)", 0.2554, RHO0, 3, 1, B, "proved (re-reading)"),
    ("P2  oz 3->2 (keep N/g as 1/g instead of zeta)", 0.2554, RHO0, 2, 3, B, "proved (re-reading)"),
    ("P1+P2", 0.2554, RHO0, 2, 1, B, "proved (re-reading)"),
    ("P1+P2 + e0 = 0.25 (omega(1,1/2,1) = 2)", 0.25, RHO0, 2, 1, B, "conditional"),
    ("P1+P2 + e0 = 0.2456 (K=4 one-level construction atop the tree)", 0.2456, RHO0, 2, 1, B, "conditional"),
    ("P1+P2 + oz 1 (sharper balancing)", 0.2554, RHO0, 1, 1, B, "hypothetical"),
    ("P1+P2 + oz 0 (no balancing slack)", 0.2554, RHO0, 0, 1, B, "hypothetical"),
    ("P1+P2 + rho 2/3 (floor of App. B piece shape)", 0.2554, 2 / 3, 2, 1, B, "hypothetical (new pieces)"),
    ("P1+P2 + rho 0.6", 0.2554, 0.6, 2, 1, B, "hypothetical (new pieces)"),
    ("P1+P2 + rho 1/2 (two-merge floor)", 0.2554, 0.5, 2, 1, B, "hypothetical (new pieces)"),
    ("B0 with bar 0.125 (Zhou-Yin-Zhou 1.4142^m competitor)", 0.2554, RHO0, 3, 3, B125, "bar sensitivity"),
    ("P1+P2 with bar 0.125", 0.2554, RHO0, 2, 1, B125, "bar sensitivity"),
]
rows = []
print(f"{'case':72s} {'status':26s} {'bar':>6s} {'log2m*':>7s} {'x0':>3s} {'lev':>3s} {'log2K':>7s} {'e_L':>7s} {'q':>5s}")
for name, e0, rho, oz, oK, b, status in cases:
    r = best(e0, rho, oz, oK, b)
    rows.append(dict(case=name, status=status, bar=b, **r))
    print(f"{name:72s} {status:26s} {b:6.4f} {r['log2m']:7.1f} {r['x0']:3d} {r['levels']:3d} {r['log2K']:7.1f} {r['e_L']:7.4f} {r['q']:5.2f}")

with open(__file__.rsplit("/", 1)[0] + "/sweep_results.json" if "/" in __file__ else "sweep_results.json", "w") as f:
    json.dump(rows, f, indent=1)
