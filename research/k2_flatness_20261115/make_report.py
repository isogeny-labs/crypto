"""Build REPORT.typ from results.json and compile REPORT.pdf with the typst module."""
import json
from pathlib import Path

import sys
import typst

sys.path.insert(0, str(Path(__file__).resolve().parent))

HERE = Path(__file__).resolve().parent
res = json.loads((HERE / "results.json").read_text())
v = res["verdict"]
c = res["controls"]


def cell_row(x):
    ds = " ".join((">" + str(d - 1) if cen else str(d)) if d is not None else "skip"
                  for d, cen in zip(x["dsol"], x["censored"]))
    secs = x["median_secs_D4"]
    return [x["cell"], str(x["m"]), str(x["n"]), str(x["k"]), str(x["N"]), f"{x['targets']} ({x['sat_targets']})",
            ds, str(x["d_reg"] or "-"), str(x["cols_D4"] or "-"), f"{secs:.1f}" if secs is not None else "-"]


rows = [cell_row(x) for x in res["cells"]]
hdr = ["series", "m", "n", "k", "N", "targets (sat)", "D_sol", "d_reg", "cols D=4", "s at D=4"]
esc = lambda s: s.replace(">", "\\>").replace("#", "\\#")
table = "#table(columns: 10, align: right, stroke: 0.4pt,\n" + ",\n".join(
    "  " + ", ".join(f"[*{h}*]" for h in hdr) if i == 0 else "  " + ", ".join(f"[{esc(t)}]" for t in r)
    for i, r in enumerate([hdr] + rows)) + "\n)"

body = f"""#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= K2 flatness test: GF(2) degree-bounded closure of binary decomposition systems (m = 3, 4)
_2026-11-15, research/k2_flatness_20261115. All numbers [pcc:unverified]. Frozen protocol: PROTOCOL.md (committed before the grid)._

*Question.* Does the degree-bounded GF(2) closure of the Weil-descended m = 3 / m = 4 point-decomposition
system solve it at a degree that stays flat as n grows (k ~ n/m), or does the degree grow?

*Premise correction.* The direct $S_4$ descent has Boolean degree 6 ($S_4$ is quartic in each abscissa),
so a degree-4 closure of the direct m = 3 system is undefined. Measured on the chain presentation
($S_3 (x_1, x_2, U) = 0$, $S_3 (U, x_3, x_R) = 0$; m = 4 with two auxiliary abscissae), every equation of degree at most 3,
N = mk + (m-2)n Boolean variables.

*Instrument.* Native standalone Rust (k2close.rs): closure $W_D$ under multiplication by variables, semi-echelon GF(2)
elimination, solutions by enumeration; solved at D iff rank $W_D$ equals the dimension of the degree-at-most-D part of the vanishing ideal.

*Controls.* Positive (m = 2 direct $S_3$): {"PASS" if c["positive_pass"] else "FAIL"}. Negative (random systems of the chain's shape): {"PASS" if c["negative_pass"] else "FAIL"}.

{table}

D_sol: first degree at which the closure solves the system (">4" = not solved at the largest degree computed);
d_reg: Boolean semi-regular degree of the same shape.

#figure(image("figures/k2_dsol.svg", width: 85%))

*Verdict (registered rule).* Series A (m = 3): *{v["A_m3"]}*; series C (m = 4): *{v["C_m4"]}*; overall: *{v["overall"]}* on the measured range.

*What it does not show.* Nothing beyond the measured n (k at most 6 for m = 3, 3 for m = 4); not the lead's square cells
(k = 10, 11; k = 8); not a cost below rho: the per-relation yield 2^(mk-n)/m! is unchanged and the solve matrix
M(N, at most 4) grows as N^4. Closure degree is a proxy for solver cost (WEAKNESSES E-GB-064).
"""
(HERE / "REPORT.typ").write_text(body)
typst.compile(str(HERE / "REPORT.typ"), output=str(HERE / "REPORT.pdf"), root=str(HERE))
print("wrote REPORT.pdf")
