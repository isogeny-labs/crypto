"""Plots recorded values only (reports/analysis.json, runs/rho_*.time); computes nothing."""
import json, glob, re, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = json.load(open("reports/analysis.json"))
ph = d["phases_s"]
fig, ax = plt.subplots(1, 3, figsize=(15, 5.2), gridspec_kw={"width_ratios": [1.15, 1.1, 1.0]})

# (a) wall by phase, three IC arms
names = [("O1", "O1 old"), ("N1", "N1 landed"), ("N2", "N2 landed (A/A)")]
parts = [("other (pre/post main, base load, basis, LA, target)", "#9aa5b1"),
         ("index build", "#e8a33d"), ("rank stage", "#3b7dd8")]
left = [0.0] * 3
vals = []
for tag, _ in names:
    p = ph[tag]
    idx, rank = p["index_build"], p["rank_stage"]
    vals.append([p["process_wall_bash_real_s"] - idx - rank, idx, rank])
for k, (lab, c) in enumerate(parts):
    ax[0].barh([n for _, n in names], [v[k] for v in vals], left=left, color=c, label=lab)
    left = [l + v[k] for l, v in zip(left, vals)]
for i, (tag, _) in enumerate(names):
    ax[0].text(left[i] + 60, i, f'{ph[tag]["process_wall_bash_real_s"]:.0f} s', va="center")
ax[0].invert_yaxis(); ax[0].set_xlim(0, 11000)
ax[0].set_xlabel("cold end-to-end wall, s (3 pinned cores)")
ax[0].set_title("(a) wall by phase"); ax[0].legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.18), ncol=1)

# (b) paired ratios with CI
r = d["old_over_new"]; aa = d["A_A_N1_over_N2"]
rows = [("O1 / mean(N1,N2)\nend-to-end wall", r["vs_mean_of_N1_N2"]["end_to_end_wall"]),
        ("O1 / mean(N1,N2)\nrank-stage wall", r["vs_mean_of_N1_N2"]["rank_stage_wall"]),
        ("O1 / N1", r["vs_N1"]["end_to_end_wall"]), ("O1 / N2", r["vs_N2"]["end_to_end_wall"]),
        ("A/A N1 / N2", aa["end_to_end_wall"])]
for i, (lab, v) in enumerate(rows):
    lo, hi = v["bootstrap_95"]["lo"], v["bootstrap_95"]["hi"]
    ax[1].plot([lo, hi], [i, i], color="#333", lw=2)
    ax[1].plot(v["point"], i, "o", color="#d62728")
    ax[1].text(hi + 0.03, i + 0.05, f'{v["point"]:.3f}', fontsize=8)
ax[1].set_yticks(range(len(rows))); ax[1].set_yticklabels([x for x, _ in rows], fontsize=8)
ax[1].axvline(1.0, color="#888", ls="--"); ax[1].axvline(1.1, color="#c33", ls=":")
ax[1].text(1.11, 2.5, "1.1 kill", color="#c33", fontsize=8)
ax[1].set_xlim(0.9, 2.3); ax[1].invert_yaxis()
ax[1].set_xlabel("ratio (95% bootstrap over columns, stratified by thread)")
ax[1].set_title("(b) paired old / new")

# (c) rho draws vs IC wall
rho = []
for f in sorted(glob.glob("runs/rho_*.time")):
    m = re.search(r"real=([\d.]+)", open(f).read()); rho.append(float(m.group(1)))
ax[2].plot(range(1, len(rho) + 1), rho, "o", color="#3b7dd8", alpha=.7, label="rho draws (1 core)")
ax[2].axhline(ph["N1"]["process_wall_bash_real_s"], color="#d62728", label="IC landed wall (3 cores)")
ax[2].axhline(ph["O1"]["process_wall_bash_real_s"], color="#7f7f7f", label="IC old wall (3 cores)")
ax[2].set_yscale("log"); ax[2].set_xlabel("rho draw (seed 910000000 + i)"); ax[2].set_ylabel("wall, s")
ax[2].set_title("(c) matched rho vs IC"); ax[2].legend(fontsize=7, loc="center right")
fig.suptitle("T20 n=73 K=600 Koblitz, cold end-to-end: old vs landed wide producer (4 vCPU KVM guest, run 2026-10-19/20)", fontsize=10)
fig.tight_layout(rect=(0, 0, 1, .95))
fig.savefig("reports/t20_e2e.svg"); fig.savefig("reports/t20_e2e.png", dpi=150)
