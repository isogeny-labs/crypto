"""Figure (SVG) and PDF for the v2 admission study (results_v2/). v1 used a different design; see README."""
import json, math, os
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker
from matplotlib.backends.backend_pdf import PdfPages

here = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(here, "results_v2")
STYLE = {"first_come": ("#8a8a8a", "s", "first come"), "singles_only": ("#c1121f", "D", "singles only"),
         "seen_lp": ("#b5651d", "^", "seen LP"), "max_lp_16384": ("#1b6ca8", "o", "max LP ≤ L/4"),
         "max_lp_4096": ("#0b3d61", "v", "max LP ≤ L/16")}


def name(o):
    r = o["rule"]
    key = r if isinstance(r, str) else next(iter(r))
    base = {"All": "all", "CompleteOnly": "complete_only", "FirstCome": "first_come", "SinglesOnly": "singles_only",
            "SeenLp": "seen_lp", "MaxLp": "max_lp"}[key]
    return base + (f"_{r['MaxLp']}" if key == "MaxLp" else "")


def ratios(path):
    d = json.load(open(path))
    per = {}
    for run in d["runs"]:
        b = run["draws_available"]
        sc = {(name(o), o["cap"]): (o["draws_to_target"] or b) for o in run["outcomes"]}
        a = sc[("all", None)]
        for (r, c), v in sc.items():
            per.setdefault((r, c), []).append(v / a)
    fb = d["runs"][0]["fb_columns"]
    return per, fb, d


panels = [("fp6", "F_p* toy, p ≈ 2^31 (FP6)"), ("fp7", "FP7"), ("fp8", "FP8"),
          ("real20a", "real ECDLP, binary n = 20 (a)"), ("real20b", "n = 20 (b)"),
          ("real20c", "n = 20 (c)"), ("real24", "binary n = 24")]
fig, axes = plt.subplots(1, len(panels), figsize=(24, 4.6), sharey=False)
for ax, (key, title) in zip(axes, panels):
    per, fb, d = ratios(os.path.join(R, key + ".json"))
    caps = sorted({c for (_, c) in per if c})
    mult = [c / fb for c in caps]
    for rule, (col, m, label) in STYLE.items():
        if (rule, caps[0]) not in per:
            continue
        means = [np.mean(per[(rule, c)]) for c in caps]
        ax.plot(mult, means, marker=m, color=col, label=label, lw=1.6, ms=5)
    co = np.mean(per[("complete_only", None)])
    ax.axhline(co, color="black", lw=1, ls="--", label="complete relations only")
    ax.axhline(1, color="black", lw=0.8, ls=":", label="all partials (no cap)")
    ax.set_yscale("log")
    ax.yaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda v, _: f"{v:g}"))
    ax.yaxis.set_minor_formatter(matplotlib.ticker.NullFormatter())
    top = max(co, max(np.mean(v) for v in per.values()))
    if top < 3:
        ax.set_yticks([t for t in (1, 1.25, 1.5, 1.75, 2, 2.5) if t <= top * 1.05])
    ax.set_xticks(mult); ax.set_xticklabels([f"{x:g}×" for x in mult])
    ax.set_xlabel("memory cap (× factor-base size)")
    n = len(d["runs"])
    ax.set_title(f"{title}\n|FB| {fb}, {n} {'seeds' if n > 1 else 'stream'}", fontsize=10)
    ax.grid(alpha=0.3, which="both")
axes[0].set_ylabel("draws to full useful rank\n÷ draws with all partials")
handles, labels = axes[0].get_legend_handles_labels()
fig.legend(handles, labels, loc="lower center", ncol=7, fontsize=9, frameon=False)
fig.suptitle("Which partial relations to store under a memory cap: collection cost to reach full factor-base rank "
             "(lower is better; mean over seeds)", fontsize=12)
fig.tight_layout(rect=(0, 0.07, 1, 1))
fig.savefig(os.path.join(here, "figure.svg"))

with PdfPages(os.path.join(here, "report.pdf")) as pdf:
    pdf.savefig(fig)
    for doc in ("README.md", "RESULTS_V2.md"):
        p = os.path.join(here, doc)
        if not os.path.exists(p):
            continue
        lines = open(p).read().splitlines()
        for start in range(0, len(lines), 66):
            f2 = plt.figure(figsize=(8.5, 11))
            f2.text(0.04, 0.97, "\n".join(l[:125] for l in lines[start:start + 66]), va="top",
                    family="monospace", fontsize=6.3)
            pdf.savefig(f2); plt.close(f2)
print("wrote figure.svg and report.pdf")
