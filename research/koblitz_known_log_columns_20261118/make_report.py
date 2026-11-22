#!/usr/bin/env python3
"""Figures and REPORT.pdf from summary.json (recorded ecbench compare/band output and per-arm phase means only)."""
import json, os, textwrap
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__)); S = json.load(open(os.path.join(HERE, "summary.json")))
A1 = os.path.join(HERE, "a1", "summary.json"); S1 = json.load(open(A1)) if os.path.exists(A1) else None
os.makedirs(os.path.join(HERE, "figures"), exist_ok=True)
W = ["0", "0.1", "1"]; LOG2R = {"23": 22.00, "29": 15.37, "37": 27.78}
INC = {"23": "gen-c4", "29": "gen-c1", "37": "gen-c8"}; PRIM = {"23": "kl-c3", "37": "kl-c7"}
PH = ["factor_base", "oracle_setup", "relations", "linear_algebra", "verify"]; RH = ["setup", "search", "internal_verification"]
fig, ax = plt.subplots(1, 3, figsize=(14, 4.4))
cols = {"factor_base": "#8c564b", "oracle_setup": "#1f77b4", "relations": "#ff7f0e", "linear_algebra": "#2ca02c", "verify": "#7f7f7f",
        "setup": "#9467bd", "search": "#d62728", "internal_verification": "#bcbd22"}
for i, n in enumerate(["29", "23", "37"]):
    ph = (S1["phases"][n] if S1 and n in S1["phases"] else S["phases"][n]); arms = [a for a in sorted(ph) if not a.endswith(("-aa", "-m1"))]
    bottom = [0] * len(arms)
    for p in PH + RH:
        v = [ph[a].get(p, 0) for a in arms]
        if not any(v): continue
        ax[i].bar(arms, v, bottom=bottom, color=cols[p], label=p); bottom = [b + x for b, x in zip(bottom, v)]
    ax[i].set_title(f"n{n} (log2 r {LOG2R[n]}): mean GAE/solve, w=0" + (" [A1]" if S1 and n in S1["phases"] else " [A0]"), fontsize=9); ax[i].tick_params(axis="x", rotation=60, labelsize=7)
ax[0].legend(fontsize=6.5); plt.tight_layout()
for e in ("png", "svg"): fig.savefig(os.path.join(HERE, f"figures/fig1-phase-split.{e}"), dpi=150)
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
xs = [LOG2R[n] for n in ("29", "23", "37")]
def r(n, pair, w, src=S): return src["ratios"][f"n{n}"][pair][w]
for ref, c in (("rho-strong", "#1b6ca8"), ("rho-v5", "#c0392b")):
    y = [r("29", f"gen-c1/{ref}", "0.1")["ratio"], r("23", f"gen-c4/{ref}", "0.1", S1)["ratio"], r("37", f"gen-c8/{ref}", "0.1", S1)["ratio"]]
    ax[0].plot(xs, y, "o--", color=c, label=f"incumbent gen-orbit / {ref}")
    y2 = [r("29", f"gen-c1/{ref}", "0.1")["ratio"], r("23", f"sp-c4-d5/{ref}", "0.1", S1)["ratio"], r("37", f"sp-c8-d5/{ref}", "0.1", S1)["ratio"]]
    ax[0].plot(xs, y2, "s-", color=c, label=f"A1 primary sp-c<c0>-d5 (n29: unchanged) / {ref}")
ax[0].axhline(1, color="k", lw=0.6); ax[0].set_yscale("log"); ax[0].set_xlabel("log2 r"); ax[0].set_ylabel("paired cost ratio, w = 0.1"); ax[0].set_title("Distance to rho (strong and v5)"); ax[0].legend(fontsize=6.5)
labels, vals, los, his = [], [], [], []
for n, pair in (("23", "kl-c3/gen-c4"), ("23", "kl-c4/gen-c4"), ("29", "kl-c2/gen-c1"), ("37", "kl-c6/gen-c8"), ("37", "kl-c7/gen-c8"), ("37", "kl-c8/gen-c8")):
    for w in W:
        p = r(n, pair, w); labels.append(f"n{n} {pair.split('/')[0]} w{w}"); vals.append(p["ratio"]); los.append(p["ratio"] - p["lo"]); his.append(p["hi"] - p["ratio"])
if S1:
    for key, pairs in S1["ratios"].items():
        for pair, v in pairs.items():
            if pair.startswith("sp-") and ("/gen-" in pair or "/kl-" in pair):
                for w in W:
                    p = v[w]; labels.append(f"A1 {key} {pair} w{w}"); vals.append(p["ratio"]); los.append(p["ratio"] - p["lo"]); his.append(p["hi"] - p["ratio"])
ax[1].errorbar(vals, range(len(vals)), xerr=[los, his], fmt="o", ms=3, capsize=2)
ax[1].set_yticks(range(len(vals))); ax[1].set_yticklabels(labels, fontsize=5); ax[1].axvline(1, color="k", lw=0.6); ax[1].set_xscale("log")
ax[1].set_title("candidate / incumbent, paired 95% CI"); plt.tight_layout()
for e in ("png", "svg"): fig.savefig(os.path.join(HERE, f"figures/fig2-ratios.{e}"), dpi=150)
# PDF: README text + figures
txt = open(os.path.join(HERE, "README.md")).read() if os.path.exists(os.path.join(HERE, "README.md")) else ""
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    lines = []
    for para in txt.splitlines():
        lines += textwrap.wrap(para, 150) or [""]
    for i in range(0, len(lines), 70):
        f = plt.figure(figsize=(11, 8.5)); f.text(0.03, 0.97, "\n".join(lines[i:i + 70]), va="top", family="monospace", fontsize=6.2); pdf.savefig(f); plt.close(f)
    for name in ("fig1-phase-split.png", "fig2-ratios.png"):
        img = plt.imread(os.path.join(HERE, "figures", name)); f = plt.figure(figsize=(11, 8.5)); plt.imshow(img); plt.axis("off"); pdf.savefig(f); plt.close(f)
print("ok")
