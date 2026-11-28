"""Lay out the recorded confirmation and warm-up values; computes nothing beyond ratios.

    python3 scripts/plot_report.py DATA_DIR OUT_DIR
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
pilot = {r["run"]: r for r in (json.loads(line) for line in open(f"{data}/pilot_runs.jsonl"))}
fig, (a1, a2) = plt.subplots(1, 2, figsize=(13, 4.6))
for arm, col, lab in [("B0", "#555555", "B0: branches off"), ("B1", "#c8553d", "B1: branches on"),
                      ("B1a", "#e8a08f", "B1a: A/A copy")]:
    a1.plot(rounds, [w[(arm, k)] for k in rounds], "o-", color=col, label=lab)
a1.set_xlabel("round")
a1.set_ylabel("wall time (s)")
a1.set_title("jv_cover --exp dlp --trace, p = 53, 4 threads")
a1.legend(fontsize=9)
ws = [("256", "on1"), ("1024", "w1024"), ("4096", "w4096"), ("19020", "w19020")]
xs = [int(a) for a, _ in ws]
a2.plot(xs, [pilot[b]["s"] for _, b in ws], "o-", color="#c8553d", label="branches on")
a2.axhline(pilot["off"]["s"], color="#555555", ls="--", label="branches off")
a2.set_xscale("log")
a2.set_xlabel("warm-up residuals (sequential)")
a2.set_ylabel("counted S")
a2.set_title("counted S vs warm-up length (deterministic)")
a2.legend(fontsize=9)
fig.suptitle("Branch traces: counted S 1.042x lower, wall in noise; identical relations [pcc:unverified]", fontsize=11)
fig.tight_layout(rect=(0, 0, 1, 0.93))
fig.savefig(f"{out}/confirm.svg")
print("wrote", out)
