"""Figures for README.md / REPORT.typ, from analysis/ only (no recomputation).

    python3 research/ic_acct2_setup_20261021/make_figures.py
"""
import csv
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent
bounds = here / "analysis" / "bounds"
fig_dir = here / "figures"
fig_dir.mkdir(exist_ok=True)
COL = {"total": "#4C6EF5", "per-target": "#868E96", "search": "#E8590C"}


def load(arm, basis, rng):
    return json.loads((bounds / f"{arm}-{basis}-{rng}.json").read_text())


# 1. Forest plot: alpha in additions, all generic arms, total vs search.
arms = ["rho-neg", "bsgs-neg", "kangaroo"]
ranges = ["16-22", "23-32", "16-26", "33-40"]
fig, ax = plt.subplots(figsize=(7.5, 5.2))
y, labels = 0, []
for arm in arms:
    for rng in ranges:
        for k, basis in enumerate(["total", "search"]):
            f = load(arm, basis, rng)["fit"]
            a, (lo, hi) = f["alpha_in_additions"], f["alpha_in_additions_ci95"]
            yy = y + (0.18 if k == 0 else -0.18)
            ax.errorbar(a, yy, xerr=[[a - lo], [hi - a]], fmt="o", ms=4,
                        color=COL[basis], capsize=2,
                        label=basis if (arm, rng) == ("rho-neg", "16-22") else None)
        labels.append((y, f"{arm} {rng}"))
        y -= 1
    y -= 0.5
ax.axvline(0.5, color="black", lw=0.8, ls="--")
ax.set_yticks([p for p, _ in labels], [t for _, t in labels], fontsize=8)
ax.set_xlabel("exponent α in additions (95% CI)")
ax.set_title("Generic arms: total vs search basis (bitop/v1)")
ax.legend(loc="lower right", fontsize=8)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(fig_dir / f"alpha_forest.{ext}", dpi=160)

# 2. rho-neg per-size mean cost by basis, and the set-up share.
tot = load("rho-neg", "total", "16-26")["sizes"]
sea = load("rho-neg", "search", "16-26")["sizes"]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(9, 3.6))
bits = [s["log2_r"] if "log2_r" in s else s.get("bits") for s in tot]
a1.semilogy(bits, [s["mean_gae"] for s in tot], "o-", color=COL["total"], label="total")
a1.semilogy(bits, [s["mean_gae"] for s in sea], "s-", color=COL["search"], label="search")
a1.axvline(22.5, color="grey", lw=0.8, ls=":")
a1.text(22.6, a1.get_ylim()[0] * 1.5, "8 → 16 jumps", fontsize=8, color="grey")
a1.set_xlabel("log2 r"); a1.set_ylabel("mean gates per solve"); a1.legend(fontsize=8)
a1.set_title("rho-neg, 16–26 bits")
share = [1 - s2["mean_gae"] / s1["mean_gae"] for s1, s2 in zip(tot, sea)]
a2.plot(bits, share, "o-", color="#2B8A3E")
a2.axvline(22.5, color="grey", lw=0.8, ls=":")
a2.set_xlabel("log2 r"); a2.set_ylabel("non-search share of total")
a2.set_title("set-up + verification + natives")
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(fig_dir / f"rho_neg_cost_by_basis.{ext}", dpi=160)

# 3. IC/rho per-target ratio, stored vs target-aware split.
rows = list(csv.DictReader(open(here / "analysis/amortize/summary.tsv"), delimiter="\t"))
x = [float(r["log2r"]) for r in rows]
fig, ax = plt.subplots(figsize=(6.5, 3.6))
ax.semilogy(x, [float(r["cold_ratio"]) for r in rows], "o-", color="black", ms=3, label="cold (total/total)")
ax.semilogy(x, [float(r["ptr_stored"]) for r in rows], "s--", color=COL["per-target"], ms=3, label="per-target, stored split")
ax.semilogy(x, [float(r["ptr_aware"]) for r in rows], "^-", color=COL["search"], ms=3, label="per-target, target-aware")
ax.axhline(1, color="grey", lw=0.8)
ax.set_xlabel("log2 r"); ax.set_ylabel("ic-pair-rule / rho-neg (gates)")
ax.set_title("IC over rho: the correction lowers the per-target ratio")
ax.legend(fontsize=8)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(fig_dir / f"ic_rho_ratios.{ext}", dpi=160)
print("figures written to", fig_dir)
