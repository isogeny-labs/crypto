"""Lay out recorded values from ../summary.json and the per-run price records
as SVG (and 150 dpi PNG) graphs.  Plotting only: every number is read from a
committed record; the only arithmetic is a per-round ratio of two recorded
medians, as summarise.sh computes it."""
import json
import pathlib
import statistics

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = pathlib.Path(__file__).resolve().parent
study = here.parent
summary = json.loads((study / "summary.json").read_text())
BASE, CAND = "#4c72b0", "#55a868"


def save(fig, name):
    fig.savefig(here / f"{name}.svg")
    fig.savefig(here / f"{name}.png", dpi=150)
    plt.close(fig)


def med_collect(path):
    reps = json.loads(path.read_text())["repetitions"]
    return statistics.median(r["phases_ns"]["collect"] for r in reps)


# 1. Paired collect-phase ratio per A/B round, and the A/A pairs.
ab = study / "runs/time/ab"
ratios = [med_collect(ab / f"baseline-r{r}.json") / med_collect(ab / f"candidate-r{r}.json") for r in range(1, 31)]
aa = study / "runs/time/aa"
aa_ratios = [med_collect(aa / f"baseline-r{2 * k - 1}.json") / med_collect(aa / f"baseline-r{2 * k}.json")
             for k in range(1, 6)]
n = summary["native"]
fig, ax = plt.subplots(figsize=(7.2, 3.6))
ax.axhline(1.0, color="#888", lw=1)
ax.scatter(range(1, 31), ratios, color=CAND, s=18, label="A/B round: main / candidate")
ax.scatter([31.5 + i * 0.5 for i in range(5)], aa_ratios, color=BASE, marker="s", s=18, label="A/A pair: main / main")
c, a = n["ab_collect"], n["aa_collect"]
ax.axhspan(c["ci95"]["lo"], c["ci95"]["hi"], xmax=0.82, color=CAND, alpha=0.15)
ax.axhspan(a["ci95"]["lo"], a["ci95"]["hi"], xmin=0.85, color=BASE, alpha=0.15)
ax.axhline(c["median"], xmax=0.82, color=CAND, lw=1.5)
ax.set_xlabel("round (shaded: 95% bootstrap interval of the median)")
ax.set_ylabel("collect-phase ratio")
ax.set_title(f"ic price k0n53 T01: paired collect ratio, median {c['median']:.3f} "
             f"[{c['ci95']['lo']:.3f}, {c['ci95']['hi']:.3f}], {c['above_one']}/30 above 1", fontsize=9)
ax.legend(fontsize=7, loc="lower right")
fig.tight_layout()
save(fig, "paired-collect-ratio")

# 2. The three measures side by side.
cg = {r["params"]: r for r in summary["callgrind"]}
perf = summary["perf"]
fam = {arm: statistics.median(p["scan_family_pct"] for p in perf if p["arm"] == arm) for arm in ("baseline", "candidate")}
gat = {arm: statistics.median(p["gather_closure_pct"] for p in perf if p["arm"] == arm) for arm in ("baseline", "candidate")}
fig, axes = plt.subplots(1, 3, figsize=(8.4, 3.2))
ax = axes[0]
for i, k in enumerate(["k0n53", "k0n61"]):
    ax.bar(i * 2.5, cg[k]["baseline_ir"] / 1e9, color=BASE, label="main" if i == 0 else None)
    ax.bar(i * 2.5 + 1, cg[k]["candidate_ir"] / 1e9, color=CAND, label="candidate" if i == 0 else None)
    ax.text(i * 2.5 + 0.5, cg[k]["baseline_ir"] / 1e9 * 1.02, f"{cg[k]['ratio']:.3f}x", ha="center", fontsize=7)
ax.set_xticks([0.5, 3.0])
ax.set_xticklabels(["n53", "n61"])
ax.set_ylabel("Callgrind instructions (10^9)")
ax.set_title("1. instructions / process", fontsize=9)
ax.legend(fontsize=7)
ax = axes[1]
ax.bar([0, 1], [fam["baseline"], fam["candidate"]], color=[BASE, CAND])
ax.bar([0, 1], [gat["baseline"], gat["candidate"]], color="#c44e52", label="gather closure")
ax.set_xticks([0, 1])
ax.set_xticklabels(["main", "candidate"])
ax.set_ylabel("% of samples (median of 3)")
ax.set_title("2. perf: scan family share", fontsize=9)
ax.legend(fontsize=7)
ax = axes[2]
for i, (key, lab) in enumerate([("ab_collect", "collect"), ("ab_total", "set-up")]):
    s = n[key]
    ax.errorbar(i, s["median"], yerr=[[s["median"] - s["ci95"]["lo"]], [s["ci95"]["hi"] - s["median"]]],
                fmt="o", color=CAND, capsize=4)
    aa_s = n[key.replace("ab", "aa")]
    ax.errorbar(i + 0.25, aa_s["median"], yerr=[[aa_s["median"] - aa_s["ci95"]["lo"]], [aa_s["ci95"]["hi"] - aa_s["median"]]],
                fmt="s", color=BASE, capsize=4)
ax.axhline(1.0, color="#888", lw=1)
ax.set_xticks([0.12, 1.12])
ax.set_xticklabels(["collect", "set-up"])
ax.set_ylabel("paired ratio (A/B green, A/A blue)")
ax.set_title("3. native, 30 rounds", fontsize=9)
fig.tight_layout()
save(fig, "three-measures")
