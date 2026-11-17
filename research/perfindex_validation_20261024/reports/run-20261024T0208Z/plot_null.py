"""Plots recorded values of the null run only; nothing here computes a result.

Reads ../../null-136x5/{compare.json,cost.json} and writes the two figures
(SVG, and a 150 dpi PNG from the same code) beside this file.
"""
import json
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).resolve().parent
run = here.parent.parent / "null-136x5"
cmp_ = json.loads((run / "compare.json").read_text())
cost = json.loads((run / "cost.json").read_text())
ks = sorted(cmp_["kernels"], key=lambda k: k["speedup"])
RUN = "null-136x5 (schedule a74b35928acc5791, seed 20260926)"
COL = {"neutral": "#6b7a8f", "slower": "#c0392b", "faster": "#1f7a4d"}


def save(fig, name):
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(here / f"{name}.{ext}", bbox_inches="tight", **kw)


# Figure 1: per-kernel speedup between identical binaries
fig, ax = plt.subplots(figsize=(9, 4.6))
x = range(len(ks))
ax.axhspan(1 / 1.02, 1.02, color="#dfe6ee", label="±2 % floor of the verdict rule")
ax.axhline(1.0, color="#333", lw=0.8)
for i, k in enumerate(ks):
    lo, hi = k["speedup_ci95"]
    c = COL[k["verdict"]]
    ax.vlines(i, lo, hi, color=c, lw=0.9, alpha=0.8)
    ax.plot(i, k["speedup"], "o", ms=2.6, color=c)
ax.set_yscale("log")
ax.set_xlabel("136 quick-tier kernels, sorted by measured ratio")
ax.set_ylabel("ratio, binary A / copy of A (log scale)")
ax.set_title(
    "Identical binaries: per-kernel wall-time ratio and 95 % bootstrap interval\n"
    f"5 rounds, 1 thread, pinned to one CPU; {RUN}",
    fontsize=9,
)
ax.plot([], [], "o", color=COL["neutral"], label="neutral (134)")
ax.plot([], [], "o", color=COL["slower"], label="called slower (2) — false")
ax.legend(fontsize=8, loc="upper left")
fig.text(0.01, -0.02, "Source: compare.json (kernels[].speedup, speedup_ci95, verdict).", fontsize=7)
save(fig, "fig1_null_ratios")
plt.close(fig)

# Figure 2: A/A spread per kernel, and what the run cost per area
import math

noise = [100 * (math.exp(k["aa_noise_ln"]) - 1) for k in cmp_["kernels"] if k.get("aa_noise_ln") is not None]
fig, (a, b) = plt.subplots(1, 2, figsize=(9, 3.8), gridspec_kw={"width_ratios": [1.1, 1]})
a.hist(noise, bins=24, color="#6b7a8f")
a.axvline(cmp_["aa_noise_median_pct"], color="#c0392b", lw=1.2)
a.text(cmp_["aa_noise_median_pct"] + 0.4, a.get_ylim()[1] * 0.9, f"median {cmp_['aa_noise_median_pct']:.1f} %", color="#c0392b", fontsize=8)
a.set_xlabel("A/A noise σ per kernel (%)")
a.set_ylabel(f"kernels (n = {len(noise)})")
a.set_title("Spread between two copies of the same arm", fontsize=9)
areas = sorted(cost["by_area_seconds"].items(), key=lambda t: -t[1])
b.barh([n for n, _ in areas][::-1], [v for _, v in areas][::-1], color="#6b7a8f")
b.set_xlabel("seconds inside the 2040 invocations")
b.set_title(
    f"Cost of the run: {cost['invocation_seconds']:.0f} s in invocations,\n"
    f"{100*cost['overhead_fraction']:.0f} % of it outside timed samples",
    fontsize=9,
)
fig.suptitle(RUN, fontsize=8, y=1.0)
fig.text(0.01, -0.03, "Source: compare.json (aa_noise_ln, aa_noise_median_pct), cost.json (by_area_seconds, invocation_seconds, overhead_fraction).", fontsize=7)
fig.tight_layout()
save(fig, "fig2_noise_and_cost")
plt.close(fig)
