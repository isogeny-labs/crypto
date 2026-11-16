#!/usr/bin/env python3
"""Figure from analysis.json (records only): CHAIN/REF CPU ratio by size, with the A/A floor."""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

a = json.load(open("analysis.json"))
QS = [262111, 524287, 1048507, 2097091, 4194271]
ks = [q.bit_length() for q in QS]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(11, 4.4))
for key, col, mk, lab in (("rel_all", "#c0392b", "o", "relations.cpu, D+DL1+DL (primary)"),
                          ("tot_DL", "#2471a3", "s", "total CPU, DL arm"),
                          ("tot_DL1", "#7f8c8d", "^", "total CPU, DL1 arm"),
                          ("tot_D", "#8e44ad", "v", "total CPU, D arm")):
    ax.plot(ks, [a["per_q_geomean"][str(q)][key] for q in QS], color=col, marker=mk, label=lab, lw=1.4)
for i, q in enumerate(QS):
    ys = [a["cells"][f"{q}/{s}"]["rel_all"]["mean"] for s in (1, 2, 3)]
    ax.scatter([ks[i]] * 3, ys, color="#c0392b", s=10, alpha=0.5)
f = a["aa_floor"]
ax.axhspan(1 - f, 1 + f, color="#bbb", alpha=0.5, label=f"A/A floor ±{f:.3f}")
ax.axhline(1, color="#333", lw=0.8)
ax.set_xlabel("k (q ≈ 2^k)")
ax.set_ylabel("CPU, compiled chain / reference (same instance)")
ax.set_title("L0 CPU ratio at identical counted work (geo-mean over seeds)", fontsize=9)
ax.set_ylim(0.4, 1.1)
ax.grid(alpha=0.3)
ax.legend(fontsize=7, loc="lower right")
cells = [a["cells"][f"{q}/{s}"]["rel_all"] for q in QS for s in (1, 2, 3)]
x = list(range(len(cells)))
bx.plot(x, [c["pass1"] for c in cells], "o", color="#c0392b", ms=4, label="pass 1")
bx.plot(x, [c["pass2"] for c in cells], "s", color="#2471a3", ms=4, label="pass 2")
bx.axhline(1, color="#333", lw=0.8)
bx.set_xticks([3 * i + 1 for i in range(5)])
bx.set_xticklabels([f"k={k}" for k in ks])
bx.set_ylabel("relations.cpu CHAIN / REF")
bx.set_title("Each (q, seed) cell, both passes", fontsize=9)
bx.grid(alpha=0.3)
bx.legend(fontsize=7)
fig.tight_layout()
fig.savefig("sqrt_compiled.svg")
fig.savefig("sqrt_compiled.png", dpi=200)
print("ok")
