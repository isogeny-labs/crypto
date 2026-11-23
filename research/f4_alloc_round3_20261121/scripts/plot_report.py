"""Lay out the recorded values of this study as three graphs; it computes nothing new beyond ratios.

    python3 scripts/plot_report.py DATA_DIR OUT_DIR

screen.svg: per-round wall time of glibc vs jemalloc on jv_quartic and jv_quintic (and the decay/oversize arms).
confirm.svg: per-round wall times of B0, B1, B1a on jv_cover and the paired ratios.
instructions.svg: callgrind instruction counts of the four builds.
"""
import json
import sys

import matplotlib

matplotlib.use("svg")
matplotlib.rcParams["svg.fonttype"] = "path"
matplotlib.rcParams["font.size"] = 11
import matplotlib.pyplot as plt  # noqa: E402

data, out = sys.argv[1], sys.argv[2]
rows = lambda p: [json.loads(l) for l in open(f"{data}/{p}")]  # noqa: E731

# screen
scr = rows("screen/runs.jsonl")
qc = rows("quintic_conf/runs.jsonl")
fig, axes = plt.subplots(1, 2, figsize=(13, 4.6))
for ax, c in zip(axes, ["jv_quartic", "jv_quintic"]):
    for arm, col in [("glibc", "#555555"), ("jemalloc", "#c8553d")]:
        rs = sorted((r["round"], r["wall_ms"] / 1e3) for r in scr if r["consumer"] == c and r["arm"] == arm)
        ax.plot([x for x, _ in rs], [y for _, y in rs], "o-", color=col, label=arm)
    if c == "jv_quintic":
        for arm, col, mk in [("je_nodecay", "#e39a3b", "s"), ("je_nooversize", "#7fa7d6", "^"), ("je_both", "#3b6ea8", "v")]:
            rs = sorted((r["round"], r["wall_ms"] / 1e3) for r in qc if r["arm"] == arm)
            ax.plot([x for x, _ in rs], [y for _, y in rs], mk + "--", color=col, label=f"jemalloc, {arm[3:]}", alpha=0.8)
    ax.set_title({"jv_quartic": "jv_quartic --exp cprime (65-row F4 systems)",
                  "jv_quintic": "jv_quintic weierstrass (~1e10 mults per test)"}[c], fontsize=10)
    ax.set_xlabel("round")
    ax.set_ylabel("wall time (s)")
    ax.legend(fontsize=8)
fig.suptitle("Allocator screen, LD_PRELOAD jemalloc vs glibc, identical work in every run [pcc:unverified]", fontsize=11)
fig.tight_layout(rect=(0, 0, 1, 0.93))
fig.savefig(f"{out}/screen.svg")

# confirmation
cf = rows("confirm/runs.jsonl")
w = {(r["arm"], r["round"]): r["wall_ms"] / 1e3 for r in cf}
rounds = sorted({r["round"] for r in cf})
fig, (a1, a2) = plt.subplots(1, 2, figsize=(13, 4.6))
for arm, col, lab in [("B0", "#555555", "B0: main + jemalloc"), ("B1", "#c8553d", "B1: f4_fp changes"),
                      ("B1a", "#e8a08f", "B1a: A/A copy")]:
    a1.plot(rounds, [w[(arm, k)] for k in rounds], "o-", color=col, label=lab)
a1.set_xlabel("round")
a1.set_ylabel("wall time (s)")
a1.set_title("jv_cover --exp dlp, p = 53, 4 threads")
a1.legend(fontsize=9)
a2.plot(rounds, [w[("B0", k)] / w[("B1", k)] for k in rounds], "o-", color="#c8553d", label="B0 / B1")
a2.plot(rounds, [w[("B1", k)] / w[("B1a", k)] for k in rounds], "s--", color="#3b6ea8", label="A/A: B1 / B1a")
a2.axhline(1.0, color="#999", lw=0.8)
a2.axhspan(1.02, 1.10, color="#c8553d", alpha=0.08, label="pre-registered band [1.02, 1.10]")
a2.set_xlabel("round")
a2.set_ylabel("wall-time ratio")
a2.set_title("per-round paired ratios")
a2.legend(fontsize=9)
fig.suptitle("run-20261121T0100Z: 12 rounds x 3 arms, identical work in all 36 runs [pcc:unverified]", fontsize=11)
fig.tight_layout(rect=(0, 0, 1, 0.93))
fig.savefig(f"{out}/confirm.svg")

# instructions
ins = json.load(open(f"{data}/callgrind/instructions.json"))["arms"]
fig, ax = plt.subplots(figsize=(9, 3.8))
labels = [a["arm"] + ("" if a["arm"] != "new2" else " (rejected)") for a in ins]
vals = [a["f4_ir"] / 1e9 for a in ins]
ax.barh(labels, vals, color=["#555555", "#3b6ea8", "#bbbbbb", "#c8553d"])
for i, v in enumerate(vals):
    ax.text(v + 0.01, i, f"{v:.3f} G ({100 * (v / vals[0] - 1):+.1f} %)", va="center", fontsize=10)
ax.invert_yaxis()
ax.set_xlim(0, max(vals) * 1.3)
ax.set_xlabel("F4 instructions, 104 Nagao tests (callgrind, 1 thread)")
ax.set_title("Instructions inside F4 by build (output identical)")
fig.tight_layout()
fig.savefig(f"{out}/instructions.svg")
print("wrote", out)
