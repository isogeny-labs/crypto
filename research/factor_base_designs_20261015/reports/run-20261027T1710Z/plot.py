"""Size and write time of each checkpoint and in-step snapshot of the V16 run, parsed from
runs/V16-A0-m4-t4.log (committed). Reads that file only."""
import os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
log = os.path.join(here, "..", "..", "runs", "V16-A0-m4-t4.log")
ev = []
for line in open(log, encoding="utf-8"):
    m = re.search(r"f4: checkpoint \S+ at the top of step (\d+), written inside it: (\d+) bytes in ([0-9.]+) s", line)
    if m: ev.append(("step-top (written inside a step)", int(m.group(1)), int(m.group(2)), float(m.group(3)))); continue
    m = re.search(r"f4: checkpoint \S+ at step (\d+): (\d+) bytes in ([0-9.]+) s", line)
    if m: ev.append(("step-top", int(m.group(1)), int(m.group(2)), float(m.group(3)))); continue
    m = re.search(r"f4: in-step snapshot of step (\d+) \(phase (\d), column (\d+), (\d+) pivots\): (\d+) bytes in ([0-9.]+) s", line)
    if m: ev.append((f"in-step snapshot, phase {m.group(2)}", int(m.group(1)), int(m.group(5)), float(m.group(6))))
colors = {"step-top": "#1f77b4", "step-top (written inside a step)": "#9ecae1",
          "in-step snapshot, phase 0": "#d62728", "in-step snapshot, phase 1": "#ff9896"}
fig, ax = plt.subplots(figsize=(8.4, 4.8))
for k, (kind, step, size, secs) in enumerate(ev):
    ax.bar(k, size / 1e9, color=colors[kind])
    ax.text(k, size / 1e9 + 0.8, f"{secs:.0f} s", ha="center", fontsize=8)
ax.set_xticks(range(len(ev))); ax.set_xticklabels([f"step {e[1]}" for e in ev], rotation=45, fontsize=8)
ax.set_ylabel("file size (GB); label: write time")
ax.set_title("V16 (A0, m = 4, t = 4): checkpoints and in-step snapshots, in log order", fontsize=10)
ax.legend(handles=[plt.Rectangle((0, 0), 1, 1, color=c) for c in colors.values()], labels=list(colors), fontsize=8, loc="upper left")
ax.grid(axis="y", alpha=0.25)
fig.tight_layout()
fig.savefig(os.path.join(here, "snapshots.svg")); fig.savefig(os.path.join(here, "snapshots.png"), dpi=150)
