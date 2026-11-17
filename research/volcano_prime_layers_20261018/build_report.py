#!/usr/bin/env python3
"""Layout only (no computation): figures, RESULT.md and PDF from tables.json / TABLES.md and the n=83 logs."""
import json, os, re, sys
H = os.path.dirname(os.path.abspath(__file__)); N83 = f"{H}/../volcano_koblitz_n83_20261018"
T = json.load(open(f"{H}/tables.json"))["volcanoes"]
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt, typst
plt.rcParams.update({"svg.fonttype": "none", "font.size": 9})
fig, ax = plt.subplots(1, len(T), figsize=(9.2, 3.4), sharey=False)
for a, v in zip(ax, T):
    for arm, col in [("rho-neg", "#1f6fb2"), ("rho-plain", "#c4561a")]:
        L = [r for r in v["levels"] if r["arm"] == arm]
        a.plot([r["level"] for r in L], [r["mean_gae"] for r in L], "o-", color=col, label=arm)
    a.set_title(f'{v["id"]}: depth {v["depth"]}'); a.set_xlabel("level (0 = crater, 8 = floor)"); a.set_ylim(bottom=0)
ax[0].set_ylabel("mean counted GAE"); ax[0].legend(frameon=False)
fig.tight_layout()
for d in (H, N83): pass
fig.savefig(f"{H}/fig_levels.svg")
tables = open(f"{H}/TABLES.md").read()
ratios = [l for l in tables.splitlines() if re.match(r"\| rho-.*(crater|middle)", l) or l.startswith("## ")]
txt = f"""# Prime-field volcano layers, ecbench: RESULT

Two depth-8 ℓ=2 volcanoes over F_p (m0: p=40327618377097, m5: p=57628051254530177), 2 curves per interior level, 6 targets each, solvers rho.negation and rho.plain, counted unit ecbench.gae. Both sessions audited: 864 records, 864 verified, 8 exact replays reproduced, 0 problems. Host busy, isolation L0 (counts only; no wall time). Neighbour/vertex curves are not in the curve registry.

## Measured
- m0: every ratio interval contains 1 (rho-neg floor/crater 0.889 [0.661, 1.255]; rho-plain 1.103 [0.822, 1.526]); level slope intervals contain 0.
- m5: rho-neg intervals all contain 1 (floor/crater 0.698 [0.498, 1.037]; slope -3.36% [-7.08, 2.09]). rho-plain middle/crater 0.688 [0.562, 0.891] and floor/crater 0.671 [0.524, 0.883] exclude 1; floor/middle 0.975 [0.793, 1.168] and the slope [-3.71, 5.53] do not.

## Reading
Of the 16 tested intervals (4 ratios plus a slope, for each of 2 solvers, on 2 volcanoes), 14 contain the null value; the 2 that exclude it are both rho.plain on m5 and both involve the crater, which is one curve with 6 targets. This is 2 volcanoes; the crater is a single curve , so a crater-specific effect cannot be separated from a curve-specific one. Nothing here supports or refutes any transfer attack, since no degree-ℓ isogeny map was computed. The non-excluding intervals are wide (about ±30%) and do not show equality to within a small margin.

## Tables
""" + "\n".join(ratios) + f"""

Full per-level means: TABLES.md / tables.json. Figure: fig_levels.svg.

# Koblitz n=83, ℓ=6473 (volcano_koblitz_n83_20261018)
Curve y²+xy=x³+1 over F_(2^83): t=-6151469093347, D_K=-7, f=6473·53676929, #E=2²·2417851639230796216685689. Native koblitz-step built Φ_6473(X,j₀) from 9864 CRT primes (606365-bit coefficients, 8590 s total on 3 threads), reduced mod 2, factored into 78 Frobenius orbits of size 83, and found 6474 distinct level-1 j-invariants in F_(2^83) (expected 6474). 48 of 48 sampled neighbours had exactly one twist of order N; 0 of 8 random j passed the control. ALL CHECKS PASS. j-list and log hashes: SHA256SUMS. Not done: ecbench measurements at n=83 (the group is far outside rho reach) and no isogeny maps; the level-1 neighbours share N, so this verifies the volcano structure at ℓ=6473 only.
"""
open(f"{H}/RESULT.md", "w").write(txt)
open(f"{N83}/RESULT.md", "w").write(txt.split("# Koblitz n=83")[1].replace(", ℓ=6473 (volcano_koblitz_n83_20261018)", "# Koblitz n=83, ℓ=6473\n", 1) if False else "# Koblitz n=83, ℓ=6473\n" + txt.split("ℓ=6473 (volcano_koblitz_n83_20261018)\n")[1])
def esc(s): return re.sub(r"([#$\[\]*_@<>`\\])", r"\\\1", s)
typ = '#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n'
for line in txt.splitlines():
    if line.startswith("# "): typ += f"= {esc(line[2:])}\n"
    elif line.startswith("## "): typ += f"== {esc(line[3:])}\n"
    elif line.startswith("|"):
        if "---" not in line: typ += esc(line) + " \\\n"
    elif line: typ += esc(line) + "\n\n"
    if line.startswith("## Reading"): pass
typ += '\n#figure(image("fig_levels.svg", width: 100%), caption: [Mean counted GAE per level.])\n'
open(f"{H}/RESULT.typ", "w").write(typ)
typst.compile(f"{H}/RESULT.typ", output=f"{H}/RESULT.pdf")
