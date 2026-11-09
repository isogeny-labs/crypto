#!/usr/bin/env python3
"""Layout only: plots values recorded in runs/*.analysis.json (computed by `cmsem analyze`).
No result is computed here (AGENTS.md: Python only for plotting recorded values)."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
RUNS = os.path.join(HERE, "..", "runs")
FIG = os.path.join(HERE, "figures")
os.makedirs(FIG, exist_ok=True)

COL = {"E0": "#2a6fdb", "E0_AA": "#2a6fdb", "E0_ISO": "#7aa6ef", "SMALL": "#d1495b", "SMALL_ISO": "#f08a95",
       "A3": "#edae49", "RAND": "#5c5c5c", "PLANTED": "#00a676", "W": "#2a6fdb", "MONT": "#d1495b", "ED": "#edae49"}


def colour(a):
    if a == "PLANTED": return COL["PLANTED"]
    if a.startswith("SMALL") and a.endswith("_ISO"): return COL["SMALL_ISO"]
    if a.startswith("SMALL"): return COL["SMALL"]
    if a.startswith("E0_ISO"): return COL["E0_ISO"]
    if a.startswith("E0"): return COL["E0"]
    if a.startswith("A3"): return COL["A3"]
    if a.startswith("RAND"): return COL["RAND"]
    if a.startswith("MONT"): return COL["MONT"]
    if a.startswith("ED"): return COL["ED"]
    return COL["W"]


ORDER = ["E0", "E0_AA", "E0_ISO1", "E0_ISO2", "SMALL1", "SMALL2", "SMALL3", "SMALL4", "SMALL1_ISO", "SMALL2_ISO",
         "A3_1", "A3_2", "RAND1", "RAND2", "RAND3", "RAND4", "RAND5", "RAND6", "PLANTED",
         "W", "W_AA", "W_ISO", "MONT", "MONT_NEG", "ED_Y", "ED_YSYM"]


def load(cell):
    p = os.path.join(RUNS, f"{cell}.analysis.json")
    return json.load(open(p)) if os.path.exists(p) else None


def yield_panel(ax, cell, title):
    d = load(cell)
    if d is None:
        ax.set_title(title + " (pending)"); return
    arms = [a for a in ORDER if a in d["per_arm"]]
    m = [d["per_arm"][a]["ynorm_mean"] for a in arms]
    e = [1.96 * d["per_arm"][a]["ynorm_se"] for a in arms]
    ax.errorbar(range(len(arms)), m, yerr=e, fmt="none", ecolor="#999", capsize=2)
    ax.scatter(range(len(arms)), m, c=[colour(a) for a in arms], zorder=3, s=28)
    ax.axhline(1.0, color="#444", lw=0.8, ls="--")
    ax.set_xticks(range(len(arms))); ax.set_xticklabels(arms, rotation=70, fontsize=7)
    ax.set_ylabel("normalized yield  Y·#E/|F|^k  (mean ± 1.96 SE)", fontsize=8)
    ax.set_title(title, fontsize=9)


fig, axs = plt.subplots(1, 3, figsize=(15, 4.6))
yield_panel(axs[0], "C40-S3", "C40-S3: p≈2^40.3, B=2^20, m=2 (S3), T=1000/arm")
yield_panel(axs[1], "C48-S3", "C48-S3: p≈2^48, B=2^24, m=2, T=200/arm")
yield_panel(axs[2], "C40-SIDE", "Side M (4|#E): models, p≈2^40, B=2^20")
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(FIG, f"yield_per_arm.{ext}"), dpi=150)

# counted cost
fig, axs = plt.subplots(1, 3, figsize=(15, 4.2))
for ax, cell, key, lab in [(axs[0], "C40-S3", "muls_per_base_element", "Montgomery mults per base element (exact)"),
                           (axs[1], "C48-S3", "muls_per_base_element", "Montgomery mults per base element (exact)"),
                           (axs[2], "C40-S4", "muls_per_target_mean", "Montgomery mults per S4 attempt (exact)")]:
    d = load(cell)
    if d is None:
        ax.set_title(cell + " (pending)"); continue
    arms = [a for a in ORDER if a in d["per_arm"] and d["per_arm"][a][key]]
    v = [d["per_arm"][a][key] for a in arms]
    ax.bar(range(len(arms)), v, color=[colour(a) for a in arms])
    lo, hi = min(v), max(v)
    ax.set_ylim(lo * 0.995, hi * 1.003)
    ax.set_xticks(range(len(arms))); ax.set_xticklabels(arms, rotation=70, fontsize=7)
    ax.set_ylabel(lab, fontsize=8); ax.set_title(f"{cell}: range {lo:.4g} – {hi:.4g}", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(FIG, f"counted_cost_per_arm.{ext}"), dpi=150)

# reach cost: tested per class member vs 2p/h (recorded in meta)
pts = []
for cell in ("C40-S3", "C48-S3"):
    p = os.path.join(RUNS, f"{cell}.meta.json")
    if not os.path.exists(p): continue
    m = json.load(open(p))
    h = m.get("class_number") or m.get("class_number_estimate")
    for k in ("search_small", "search_rand", "search_a3"):
        if k in m and m[k]["found"]:
            pts.append((cell, k, m["p"], h, m[k]["tested"] / m[k]["found"]))
fig, ax = plt.subplots(figsize=(6, 4))
import math
for cell, k, p, h, tpm in pts:
    ax.scatter(math.log2(2 * p / h), math.log2(tpm), label=f"{cell} {k.replace('search_', '')}", s=30)
xs = [18, 70]
ax.plot(xs, xs, "k--", lw=0.8, label="tests = 2p/h")
ax.axvline(math.log2(2 * 2**130.19 / 2**63.6), color="#d1495b", lw=0.8)
ax.text(66.5, 25, "ECCp-131\n2p/h≈2^67.6", fontsize=7, color="#d1495b")
ax.set_xlabel("log2(2p/h)  (predicted filter tests per class member)")
ax.set_ylabel("log2(measured tests per member found)")
ax.legend(fontsize=7)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(FIG, f"reach_cost.{ext}"), dpi=150)
print("figures written")
