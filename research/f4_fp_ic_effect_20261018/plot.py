# Lays out the recorded values of one run directory (argument): the
# per-test cost split and the end-to-end S / rho of both builds.  No new
# computation beyond the ratios printed on the bars.
import json, os, sys
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

run = os.path.abspath(sys.argv[1])
raw = lambda n: json.load(open(os.path.join(run, "raw", n)))
cc = {b: raw(f"ccov_{b}.json") for b in ("main", "new")}
dl = {b: raw(f"dlp_{b}.json") for b in ("main", "new")}

fig, ax = plt.subplots(figsize=(9.5, 4.2))
parts = [("f4_muls", "F4 (f4_fp)", "#2a6fdb"), ("lin_muls", "linear stage", "#d9822b"),
         ("weil_muls", "Weil restriction", "#5a9e5a"), ("post_muls", "post-processing", "#a05a9e")]
labels, x = [], 0
for i, (m, n) in enumerate(zip(cc["main"], cc["new"])):
    for j, (b, r) in enumerate((("main", m), ("new", n))):
        bottom = 0
        for k, lab, c in parts:
            v = r[k]["mean"]
            ax.bar(x, v, bottom=bottom, color=c, label=lab if x == 0 else None, width=0.8)
            bottom += v
        ax.text(x, bottom * 1.02, f"{r['c_cov']['mean']:.2e}", ha="center", fontsize=7)
        labels.append(f"p={r['p']}\n{b}")
        x += 1
    x += 0.6
ticks = []
pos = 0
for i in range(len(cc["main"])):
    ticks += [pos, pos + 1]
    pos += 2.6
ax.set_xticks(ticks)
ax.set_xticklabels(labels, fontsize=8)
ax.set_ylabel("field multiplications per Nagao test (mean)")
ax.legend(fontsize=8)
ax.set_title(f"Cost of one decomposition test C_cov, jv_cover --exp ccov (120 tests per prime), {os.path.basename(run)}", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(run, "ccov.svg"))
fig.savefig(os.path.join(run, "ccov.png"), dpi=150)

fig, ax = plt.subplots(figsize=(9.5, 4.2))
rows = [(m, n) for m, n in zip(dl["main"], dl["new"])]
w = 0.38
for i, (m, n) in enumerate(rows):
    ax.bar(i - w / 2, m["s_over_rho"], width=w, color="#9aa3ad", label="f4_fp as on main before #1717" if i == 0 else None)
    ax.bar(i + w / 2, n["s_over_rho"], width=w, color="#2a6fdb", label="f4_fp after #1717" if i == 0 else None)
    ax.text(i, max(m["s_over_rho"], n["s_over_rho"]) * 1.15, f"{m['s']/n['s']:.2f}× cheaper", ha="center", fontsize=8)
ax.axhline(1, color="#888", lw=0.8)
ax.text(len(rows) - 0.5, 1.15, "rho", fontsize=8, color="#666", ha="right")
ax.set_yscale("log")
ax.set_xticks(range(len(rows)))
ax.set_xticklabels([f"p={m['p']} seed {m['seed']}" for m, _ in rows], fontsize=8)
ax.set_ylabel("S_IC / S_rho (counted, log scale)")
ax.legend(fontsize=8, loc="lower left")
ax.set_title(f"End-to-end index calculus vs Pollard rho, jv_cover --exp dlp (4 rho runs each), {os.path.basename(run)}", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(run, "dlp.svg"))
fig.savefig(os.path.join(run, "dlp.png"), dpi=150)
