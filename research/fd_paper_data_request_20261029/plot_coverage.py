#!/usr/bin/env python3
"""Coverage of the m=2 binary descent data: recorded cells (Table 6.1) versus
cells requested by FD-BINARY-CLOSURE-EXT and FD-F4-STEP-DEGREE. Reads the
treatise's data file and the two protocol records; computes nothing else."""
import json, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "../../papers/ecdlp-index-calculus-treatise/figures/data-falldegree-instances.json")))
kw = D["kousidis_wiemers_2019_table1"]["m2"]
rows = {"P": "P (polynomial V)", "R": "R (random V)", "F4": "F4 D_reg (ours)", "KW": "KW 2019 D_reg"}
y = {k: i for i, k in enumerate(["KW", "F4", "R", "P"])}
fig, ax = plt.subplots(figsize=(11, 3.4))
def mark(n, row, style, label=None):
    ax.scatter([n], [y[row]], **style, label=label)
seen = set()
def lab(k):
    if k in seen: return None
    seen.add(k); return k
fam = {"S": "P", "U": "R", "T": "R"}
for r in D["m2_rows"]:
    row = fam.get(r["family"], r["family"])
    ge5 = any(k.startswith(">=") for k in r["dF_counts"])
    mark(r["n"], row, dict(marker="s", s=70, color="#E45756" if ge5 else "#4C78A8"), lab("recorded, d_F >= 5" if ge5 else "recorded, d_F exact (3 or 4)"))
for n in kw["n"]:
    mark(n, "KW", dict(marker="D", s=50, color="#9D9D9D"), lab("published (Kousidis-Wiemers)"))
req_R = [41, 43, 45, 47, 49, 51]; req_P = [37, 39, 47, 49, 51]; w5 = [33, 35, 37]
for n in req_R: mark(n, "R", dict(marker="s", s=70, facecolors="none", edgecolors="#E45756", linewidths=1.6), lab("requested: D=4 closure + certificate"))
for n in req_P: mark(n, "P", dict(marker="s", s=70, facecolors="none", edgecolors="#E45756", linewidths=1.6))
for n in w5: mark(n, "R", dict(marker="*", s=160, color="#F58518"), lab("requested: D=5 closure (is d_F = 6?)"))
for n in range(34, 49): mark(n, "F4", dict(marker="o", s=55, facecolors="none", edgecolors="#54A24B", linewidths=1.6), lab("requested: F4 step degree on our instances"))
ax.set_yticks(list(y.values())); ax.set_yticklabels([rows[k] for k in y])
ax.set_xlabel("n (field degree of F_{2^n}),  m = 2,  k = ceil(n/2)"); ax.set_xlim(9.5, 52.5); ax.set_ylim(-0.6, 3.6)
ax.set_title("Fall-degree paper: data on record and data requested")
ax.legend(frameon=False, fontsize=8, loc="upper left", bbox_to_anchor=(1.0, 1.0))
fig.tight_layout()
for ext in ("svg", "png", "pdf"): fig.savefig(os.path.join(H, f"coverage.{ext}"), dpi=160)
