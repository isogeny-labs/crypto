# Lays out the recorded values of one run directory (argument): C_cov split by
# stage for the three arms and the end-to-end S / rho. Only ratios are computed.
import json, os, sys
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
run = os.path.abspath(sys.argv[1])
raw = lambda n: json.load(open(os.path.join(run, "raw", n)))
arms = ("dense", "l1a", "sparse")
names = {"dense": "dense (old)", "l1a": "L1a", "sparse": "sparse FGLM"}
cc = {a: raw(f"ccov_{a}.json") for a in arms}
dl = {a: raw(f"dlp_{a}.json") for a in arms}
parts = [("f4_muls", "F4 (f4_fp)", "#2a6fdb"), ("lin_muls", "linear stage", "#d9822b"),
         ("weil_muls", "Weil restriction", "#5a9e5a"), ("post_muls", "post-processing", "#a05a9e")]
fig, ax = plt.subplots(figsize=(9.5, 4.2))
x, ticks, labels = 0, [], []
for i in range(3):
    for a in arms:
        r = cc[a][i]; b = 0
        for k, lab, c in parts:
            v = r[k]["mean"]
            ax.bar(x, v, bottom=b, color=c, width=0.8, label=lab if x == 0 else None); b += v
        ax.text(x, b * 1.02, f"{r['c_cov']['mean']:.3g}", ha="center", fontsize=7)
        ticks.append(x); labels.append(f"p={r['p']}\n{names[a]}"); x += 1
    x += 0.6
ax.set_xticks(ticks); ax.set_xticklabels(labels, fontsize=7)
ax.set_ylabel("field multiplications per Nagao test (mean)"); ax.legend(fontsize=8, loc="lower right")
ax.set_title(f"C_cov by stage, jv_cover --exp ccov (120 tests per prime), {os.path.basename(run)}", fontsize=9)
fig.tight_layout(); fig.savefig(os.path.join(run, "ccov.svg")); fig.savefig(os.path.join(run, "ccov.png"), dpi=150)
fig, ax = plt.subplots(figsize=(9.5, 4.2))
w = 0.27; cols = {"dense": "#9aa3ad", "l1a": "#d9822b", "sparse": "#2a6fdb"}
for i in range(4):
    for j, a in enumerate(arms):
        ax.bar(i + (j - 1) * w, dl[a][i]["s_over_rho"], width=w, color=cols[a], label=names[a] if i == 0 else None)
    ax.text(i, dl["dense"][i]["s_over_rho"] * 1.15, f"{dl['dense'][i]['s']/dl['sparse'][i]['s']:.2f}x", ha="center", fontsize=8)
ax.axhline(1, color="#888", lw=0.8); ax.set_yscale("log"); ax.set_xticks(range(4))
ax.set_xticklabels([f"p={r['p']} seed {r['seed']}" for r in dl["dense"]], fontsize=8)
ax.set_ylabel("S_IC / S_rho (counted, log)"); ax.legend(fontsize=8, loc="lower left")
ax.set_title(f"End to end vs Pollard rho, jv_cover --exp dlp (4 rho runs), {os.path.basename(run)}", fontsize=9)
fig.tight_layout(); fig.savefig(os.path.join(run, "dlp.svg")); fig.savefig(os.path.join(run, "dlp.png"), dpi=150)
