"""Lay out recorded values as SVG and 150 dpi PNG graphs: per-round ratios
from ../summary.json, and both sessions' medians, corrected intervals and
verdicts from ../summary-corrected.json (CORRECTION.md). Plotting only:
every number is read from a committed summary."""
import json
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = pathlib.Path(__file__).resolve().parent
rounds_src = json.loads((here.parent / "summary.json").read_text())
corr = json.loads((here.parent / "summary-corrected.json").read_text())["sessions"]
s1, s2 = corr["2026-10-23"], corr["2026-10-29"]
AB, AA = "#55a868", "#4c72b0"


def save(fig, name):
    fig.savefig(here / f"{name}.svg")
    fig.savefig(here / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Per-round ratios of this session, collect phase.
rounds = rounds_src["per_round"]
fig, ax = plt.subplots(figsize=(7.4, 3.6))
ax.axhline(1.0, color="#888", lw=1)
x = [r["round"] for r in rounds]
ax.scatter(x, [r["ab_collect"] for r in rounds], color=AB, s=16, label="A/B: main / candidate")
ax.scatter(x, [r["aa_collect"] for r in rounds], color=AA, marker="s", s=14, label="A/A: main / main")
for key, col in (("ab_collect", AB), ("aa_collect", AA)):
    st = s2[key]
    ax.axhspan(st["ci95"]["lo"], st["ci95"]["hi"], color=col, alpha=0.15)
    ax.axhline(st["median"], color=col, lw=1.4)
ab, aa = s2["ab_collect"], s2["aa_collect"]
ax.set_title(f"collect phase, 42 rounds: A/B {ab['median']:.3f} [{ab['ci95']['lo']:.3f}, {ab['ci95']['hi']:.3f}]"
             f" vs A/A {aa['median']:.3f} [{aa['ci95']['lo']:.3f}, {aa['ci95']['hi']:.3f}]", fontsize=9)
ax.set_xlabel("round (shaded: 95% bootstrap interval of the median, corrected)")
ax.set_ylabel("paired collect-phase ratio")
ax.legend(fontsize=7, loc="upper right")
fig.tight_layout()
save(fig, "per-round-collect")

# 2. Intervals of both sessions, both metrics.
fig, axes = plt.subplots(1, 2, figsize=(8.0, 3.4), sharey=True)
for ax, metric, title in ((axes[0], "collect", "collect phase"), (axes[1], "total", "whole set-up")):
    ax.axhline(1.0, color="#888", lw=1)
    for i, (lab, s, n_aa) in enumerate((("2026-10-23\n30 A/B, 5 A/A", s1, 5), ("2026-10-29\n42 A/B, 42 A/A", s2, 42))):
        for dx, kind, col, mk in ((-0.08, "ab", AB, "o"), (0.08, "aa", AA, "s")):
            st = s[f"{kind}_{metric}"]
            ax.errorbar(i + dx, st["median"], yerr=[[st["median"] - st["ci95"]["lo"]], [st["ci95"]["hi"] - st["median"]]],
                        fmt=mk, color=col, capsize=4, label=("A/B" if kind == "ab" else "A/A") if i == 0 else None)
        est = s[f"established_{metric}"]
        marginal = est and s[f"margin_{metric}"] < 0.01
        ax.text(i, 0.925, "marginal, not claimed" if marginal else ("established" if est else "not established"),
                ha="center", fontsize=6)
    ax.set_xticks([0, 1])
    ax.set_xticklabels(["2026-10-23\n30 A/B, 5 A/A", "2026-10-29\n42 A/B, 42 A/A"], fontsize=7)
    ax.set_title(title, fontsize=9)
    ax.set_ylim(0.91, 1.18)
axes[0].set_ylabel("median paired ratio, 95% interval (corrected)")
axes[0].legend(fontsize=7, loc="upper left")
fig.tight_layout()
save(fig, "sessions-intervals")
