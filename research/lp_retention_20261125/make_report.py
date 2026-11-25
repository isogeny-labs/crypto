"""Figure (SVG) and PDF for research/lp_retention_20261125. Reads results_explore/ (all six policies;
the frozen five there equal results/ exactly, checked in the README) and results_confirm/real.json."""
import json, os
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

here = os.path.dirname(os.path.abspath(__file__))
POL = [("SmallestLp", "smallest LP (textbook)", "#1b6ca8", "o"), ("Degree", "degree", "#8a8a8a", "s"),
       ("Spectral", "spectral (eigenvector)", "#b5651d", "^"), ("CoreThenRandom", "2-core, then random", "#6a994e", "v"),
       ("SinglesThenSpectral", "singles, then spectral (exploratory)", "#c1121f", "D")]
PANELS = [("results_explore/fp0.json", "F_p* toy, p ≈ 2^31 (FP0)"),
          ("results_explore/fp0r.json", "FP0 with LP labels hashed"),
          ("results_explore/syn.json", "synthetic stream (SYN)"),
          ("results_explore/synr.json", "SYN with LP labels hashed"),
          ("results_confirm/real.json", "real ECDLP stream, binary n = 20")]


def ratios(path):
    d = json.load(open(os.path.join(here, path)))
    fr = [b["fraction"] for b in d["repeats"][0]["budgets"]]
    out = {}
    for name, *_ in POL:
        rows = []
        for bi in range(len(fr)):
            v = []
            for r in d["repeats"]:
                o = {x["policy"]: x["rank"]["left_nullity"] for x in r["budgets"][bi]["outcomes"]}
                v.append(o[name] / max(o["Random"], 1))
            rows.append(np.mean(v))
        out[name] = rows
    return fr, out, d


fig, axes = plt.subplots(1, 5, figsize=(19, 4.4), sharey=True)
for ax, (path, title) in zip(axes, PANELS):
    fr, out, d = ratios(path)
    for name, label, color, m in POL:
        ax.plot(fr, out[name], marker=m, color=color, label=label, lw=1.6, ms=5)
    ax.axhline(1, color="black", lw=0.8, ls=":")
    ax.set_xscale("log"); ax.set_yscale("log")
    ax.set_xticks(fr); ax.set_xticklabels([f"{f:g}" for f in fr])
    ax.set_title(f"{title}\npool {d['repeats'][0]['pool']}", fontsize=10)
    ax.set_xlabel("budget (fraction of pool kept)")
    ax.grid(alpha=0.3, which="both")
axes[0].set_ylabel("useful rank kept / random policy\n(exact GainDsu left nullity)")
axes[0].legend(fontsize=8, loc="upper right")
fig.suptitle("Which partial relations to keep: exact useful rank under a row budget (mean of 5 repeats; random = 1)", fontsize=12)
fig.tight_layout()
fig.savefig(os.path.join(here, "figure.svg"))

with PdfPages(os.path.join(here, "report.pdf")) as pdf:
    pdf.savefig(fig)
    txt = open(os.path.join(here, "README.md")).read() if os.path.exists(os.path.join(here, "README.md")) else ""
    lines = txt.splitlines()
    for start in range(0, len(lines), 62):
        f2 = plt.figure(figsize=(8.5, 11))
        f2.text(0.05, 0.97, "\n".join(l[:120] for l in lines[start:start + 62]), va="top", family="monospace", fontsize=6.5)
        pdf.savefig(f2); plt.close(f2)
print("wrote figure.svg and report.pdf")
