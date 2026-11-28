"""Lay out the recorded values of round 4 as graphs; it computes nothing beyond ratios.

    python3 scripts/plot_report.py DATA_DIR OUT_DIR

confirm.svg: per-round wall times of the five confirmation arms and the paired ratios.
instructions.svg: callgrind F4 instructions of each build/mode measured in this round.
"""
import json
import sys

import matplotlib

matplotlib.use("svg")
matplotlib.rcParams["svg.fonttype"] = "path"
matplotlib.rcParams["font.size"] = 11
import matplotlib.pyplot as plt  # noqa: E402

data, out = sys.argv[1], sys.argv[2]
runs = [json.loads(line) for line in open(f"{data}/confirm/runs.jsonl")]
w = {(r["arm"], r["round"]): r["wall_ms"] / 1e3 for r in runs}
rounds = sorted({r["round"] for r in runs})
arms = [("H0", "#555555", "Hilbert, before round 4"), ("H1", "#c8553d", "Hilbert + trace, round 4"),
        ("H1a", "#e8a08f", "H1 A/A copy"), ("F0", "#3b6ea8", "full F4, before"), ("F1", "#7fa7d6", "full F4, round 4")]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(13, 4.8))
for arm, col, lab in arms:
    a1.plot(rounds, [w[(arm, k)] for k in rounds], "o-", color=col, label=lab)
a1.set_xlabel("round")
a1.set_ylabel("wall time (s)")
a1.set_title("jv_cover --exp dlp, p = 53, 4 threads")
a1.legend(fontsize=8)
a2.plot(rounds, [w[("H0", k)] / w[("H1", k)] for k in rounds], "o-", color="#c8553d", label="H0 / H1")
a2.plot(rounds, [w[("F0", k)] / w[("F1", k)] for k in rounds], "^-", color="#3b6ea8", label="F0 / F1")
a2.plot(rounds, [w[("H1", k)] / w[("H1a", k)] for k in rounds], "s--", color="#999999", label="A/A H1 / H1a")
a2.axhline(1.0, color="#999", lw=0.8)
a2.axhspan(1.15, 1.50, color="#c8553d", alpha=0.07, label="pre-registered H band")
a2.axhspan(1.03, 1.15, color="#3b6ea8", alpha=0.07, label="pre-registered F band")
a2.set_xlabel("round")
a2.set_ylabel("wall-time ratio")
a2.set_title("per-round paired ratios")
a2.legend(fontsize=8)
fig.suptitle("F4 round 4 confirmation: identical relations and logarithm in every run [pcc:unverified]", fontsize=11)
fig.tight_layout(rect=(0, 0, 1, 0.93))
fig.savefig(f"{out}/confirm.svg")

ins = json.load(open(f"{data}/instructions.json"))["rows"]
fig, ax = plt.subplots(figsize=(10, 4.6))
labels = [r["label"] for r in ins]
vals = [r["f4_ir"] / 1e9 for r in ins]
ax.barh(labels, vals, color=[r.get("color", "#3b6ea8") for r in ins])
for i, v in enumerate(vals):
    ax.text(v + 0.01, i, f"{v:.3f} G", va="center", fontsize=9)
ax.invert_yaxis()
ax.set_xlim(0, max(vals) * 1.25)
ax.set_xlabel("F4 instructions, jv_cover --exp ccov p = 53, 104 tests (callgrind, 1 thread)")
ax.set_title("Round 4, step by step (output identical within each mode)")
fig.tight_layout()
fig.savefig(f"{out}/instructions.svg")
print("wrote", out)
