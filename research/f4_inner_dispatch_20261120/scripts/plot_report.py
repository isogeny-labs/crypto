"""Lay out the recorded values of this study as two graphs; it computes nothing new.

    python3 scripts/plot_report.py data/confirm/runs.jsonl data/callgrind/jv53_f4only_self.txt reports/run-20261120T2241Z

walls.svg: per-round wall time of the four confirmation arms, and the per-round ratios baseline/jemalloc
and the A/A ratio. f4_breakdown.svg: the self-instruction share of each part of F4 inside jv_cover's
Nagao tests, read from the callgrind summary.
"""
import json
import re
import sys

import matplotlib

matplotlib.use("svg")
matplotlib.rcParams["svg.fonttype"] = "path"
matplotlib.rcParams["font.size"] = 11
import matplotlib.pyplot as plt  # noqa: E402

runs_path, cg_path, out = sys.argv[1], sys.argv[2], sys.argv[3]
runs = [json.loads(line) for line in open(runs_path)]
arms = {"B0": "baseline (main)", "B1": "move fix", "B1a": "move fix (A/A copy)", "B2": "move fix + jemalloc"}
colors = {"B0": "#555555", "B1": "#3b6ea8", "B1a": "#7fa7d6", "B2": "#c8553d"}
rounds = sorted({r["round"] for r in runs})
wall = {(r["arm"], r["round"]): r["wall_ms"] / 1e3 for r in runs}

fig, (a1, a2) = plt.subplots(1, 2, figsize=(13, 5))
for arm, label in arms.items():
    a1.plot(rounds, [wall[(arm, k)] for k in rounds], "o-", color=colors[arm], label=label)
a1.set_xlabel("round")
a1.set_ylabel("wall time (s)")
a1.set_title("jv_cover --exp dlp, p = 53, 4 threads")
a1.legend(fontsize=9)
ratio = [wall[("B0", k)] / wall[("B2", k)] for k in rounds]
aa = [wall[("B1", k)] / wall[("B1a", k)] for k in rounds]
a2.plot(rounds, ratio, "o-", color=colors["B2"], label="baseline / jemalloc")
a2.plot(rounds, aa, "s--", color=colors["B1"], label="A/A (move fix / its copy)")
a2.axhline(1.0, color="#999", lw=0.8)
a2.axhspan(1.10, 1.30, color=colors["B2"], alpha=0.08, label="pre-registered band [1.10, 1.30]")
a2.set_xlabel("round")
a2.set_ylabel("wall-time ratio (>1: second arm faster)")
a2.set_title("per-round paired ratios")
a2.legend(fontsize=9)
fig.suptitle("run-20261120T2241Z: 8 rounds x 4 arms, identical work in all 32 runs [pcc:unverified]", fontsize=11)
fig.tight_layout(rect=(0, 0, 1, 0.94))
fig.savefig(f"{out}/walls.svg")

# F4 self-instruction breakdown from the callgrind summary (one F4-only collection)
total = None
parts = {"glibc allocator (malloc/free family)": 0, "monomial interning (symbolic)": 0,
         "step loop (f4_with_arithmetic)": 0, "elimination arithmetic": 0, "row/term conversion and copies": 0}
for line in open(cg_path):
    m = re.match(r"\s*([\d,]+)\s+\(\s*([\d.]+)%\)\s+(.*)", line)
    if not m:
        continue
    ir = int(m.group(1).replace(",", ""))
    name = m.group(3)
    if "PROGRAM TOTALS" in name:
        total = ir
    elif "malloc.c" in name or "__rdl_alloc" in name:
        parts["glibc allocator (malloc/free family)"] += ir
    elif "Interned>::id" in name:
        parts["monomial interning (symbolic)"] += ir
    elif re.search(r"f4_fp::f4_with_arithmetic\s*$", name.strip()):
        parts["step loop (f4_with_arithmetic)"] += ir
    elif "closure#42" in name or "rref32_deferred" in name or "add_lazy" in name:
        parts["elimination arithmetic"] += ir
    elif "SpecFromIterNested" in name or "memcpy" in name or "closure#31" in name or "closure#29" in name:
        parts["row/term conversion and copies"] += ir
fig, ax = plt.subplots(figsize=(9, 4))
names = list(parts)
shares = [100 * parts[k] / total for k in names]
ax.barh(names, shares, color=["#c8553d", "#3b6ea8", "#3b6ea8", "#555555", "#3b6ea8"])
for i, s in enumerate(shares):
    ax.text(s + 0.4, i, f"{s:.1f}%", va="center", fontsize=10)
ax.invert_yaxis()
ax.set_xlabel("share of F4 instructions (callgrind, 104 Nagao tests, 1 thread)")
ax.set_title(f"Inside F4 on jv_cover's systems: {total / 1e9:.2f} G instructions, rest unlisted")
fig.tight_layout()
fig.savefig(f"{out}/f4_breakdown.svg")
print("wrote", out)
