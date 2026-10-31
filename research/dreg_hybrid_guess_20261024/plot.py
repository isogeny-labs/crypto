"""Plot recorded values only: reports/hybrid.svg/png from summary.json and runs/model-output.txt."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
S = json.load(open("summary.json"))
costs = sorted(S["costs"], key=lambda c: (c["cell"], c["draw"]))
fig, (a, b) = plt.subplots(1, 2, figsize=(12, 4.8), gridspec_kw={"width_ratios": [1.25, 1]})
x = range(len(costs))
lab = [f'{c["cell"].replace("m3-","")}\nd{c["draw"]}' for c in costs]
a.bar(x, [c["hybrid_ops"] for c in costs], color=["#3b7dd8" if c["kstar"] == 1 else "#d8803b" for c in costs], label="hybrid at k* (sum of 2^k* branches)")
a.scatter(x, [c["ops4_unspecialised"] * c["col_ratio_5_over_4"] ** 3.0 for c in costs], marker="_", s=400, color="k", label="degree 5 projected, e = 3 (floor)")
a.scatter(x, [c["ops4_unspecialised"] * c["col_ratio_5_over_4"] ** 4.1 for c in costs], marker="_", s=400, color="#b00", label="degree 5 projected, e = 4.1")
for i, c in enumerate(costs):
    a.text(i, c["hybrid_ops"] * 1.15, f'k*={c["kstar"]}', ha="center", fontsize=8)
a.set_yscale("log"); a.set_xticks(list(x)); a.set_xticklabels(lab, fontsize=7)
a.set_ylabel("closure word operations (64-bit)")
a.set_title("(3, 7): hybrid vs projected degree-5 closure\nmeasured (bars) vs projected (ticks); unspecialised degree 4 saturates on all", fontsize=9)
a.legend(fontsize=7, loc="center left")
rows = []
for l in open("runs/model-output.txt"):
    p = [t.strip() for t in l.split("|")]
    if len(p) > 8 and p[1] == "4" and p[4] == "1":
        rows.append((p[2], p[3], float(p[8])))
names = [f"{r[0].replace('hybrid, ','')}" + (f" kappa={r[1]}" if r[1] != "-" else "") for r in rows]
cols = ["#888" if r[1] == "-" else "#3b7dd8" for r in rows]
b.barh(range(len(rows)), [min(r[2], 60) for r in rows], color=cols)
b.set_yticks(range(len(rows))); b.set_yticklabels(names, fontsize=7); b.invert_yaxis()
for v, ref in [(0, "rho"), (7.62, "deg 4"), (10.32, "deg 5")]:
    b.axvline(v, color="k" if v == 0 else "#b00", ls="--", lw=0.8)
b.set_xlabel("bits over rho (w = 1, unconverted touches; clipped at 60)")
b.set_title("Implied m = 4 row (derived, not measured)", fontsize=9)
fig.text(0.01, 0.005, "Source: runs/*.jsonl via dreg_hybrid score; model: dreg_hybrid model. L0 shared VM; not isolated.", fontsize=6)
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig("reports/hybrid.svg"); fig.savefig("reports/hybrid.png", dpi=150)
