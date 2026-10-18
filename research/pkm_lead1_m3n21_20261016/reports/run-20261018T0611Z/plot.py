"""Lay out the recorded step traces of runs/M3-N21-t0.log and runs/M3-N21-t1.log
(pkm_tower_pilot --trace) and the recorded widths of the m = 3 Kummer line
(research/pkm_dickson_torus_20261015/README.md §5, recorded inputs). Computes
nothing: every plotted value is read from a committed trace or table."""
import re, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = sys.argv[1] if len(sys.argv) > 1 else "."
pat = re.compile(r"tower (step|stop in step) (\d+).*?: degree (\d+),.* x (\d+) cols,.*?([\d.]+) ms \(.*memory (\d+) MB \(peak (\d+) MB\)")
def trace(path):
    rows = []
    for line in open(path):
        m = pat.search(line)
        if m:
            rows.append(dict(stop=m.group(1) != "step", step=int(m.group(2)), degree=int(m.group(3)),
                             cols=int(m.group(4)), ms=float(m.group(5)), peak=int(m.group(7))))
    return rows
t = {k: trace(f"{here}/../../runs/M3-N21-t{k}.log") for k in (0, 1)}
recorded = {9: 1186, 12: 5019, 15: 31397, 18: 106288}  # width (max cols), recorded, m = 3 Kummer at p1

fig, axes = plt.subplots(1, 3, figsize=(14, 4.4))
colors = {0: "#2f6db3", 1: "#d1495b"}
ax = axes[0]
for k, rows in t.items():
    fin = [r for r in rows if not r["stop"]]; st = [r for r in rows if r["stop"]]
    ax.plot([r["step"] for r in fin], [r["cols"] for r in fin], "o-", ms=3, color=colors[k], label=f"target {k}, finished steps")
    if st:
        ax.plot([r["step"] for r in st], [r["cols"] for r in st], "x", ms=9, color=colors[k], label=f"target {k}, stopped step (cap)")
for k, rows in t.items():
    for r in rows:
        if r["degree"] >= 7 and (r["step"] == 21 or r["stop"]):
            ax.annotate(f"d={r['degree']}", (r["step"], r["cols"]), textcoords="offset points", xytext=(4, -10), fontsize=7, color=colors[k])
ax.set_yscale("log"); ax.set_xlabel("F4 step"); ax.set_ylabel("matrix columns"); ax.set_title("Width per step, N = 21 (recorded trace)"); ax.legend(fontsize=7)
ax = axes[1]
for k, rows in t.items():
    ax.plot([r["step"] for r in rows], [r["peak"] / 1024 for r in rows], "o-", ms=3, color=colors[k], label=f"target {k}")
ax.axhline(119416658 / 1024 / 1024, color="k", ls="--", lw=0.8); ax.text(1, 115, "A = ulimit -v (114 GB)", fontsize=7)
ax.set_xlabel("F4 step"); ax.set_ylabel("peak resident memory, GB"); ax.set_title("Peak memory per step (recorded)"); ax.legend(fontsize=7)
ax = axes[2]
ns = sorted(recorded); ax.plot(ns, [recorded[n] for n in ns], "s-", color="#555555", label="recorded, rounds 2–7 (D reached)")
ax.plot([21], [271278], "o", color=colors[0], label="N = 21, last finished step (degree 7)")
ax.plot([21], [667444], "x", ms=9, color=colors[0], label="N = 21, stopped degree-8 step (cap)")
ax.set_yscale("log"); ax.set_xlabel("N (m = 3, Kummer, p₁)"); ax.set_ylabel("max columns"); ax.set_title("Width against N"); ax.legend(fontsize=7)
fig.suptitle("PKM lead 1, m = 3, N = 21 (runs M3-N21-t0, M3-N21-t1; 2026-10-18, commit b7cafe044)", fontsize=9)
fig.tight_layout(); fig.savefig(f"{here}/steps.svg"); fig.savefig(f"{here}/steps.png", dpi=110)
for k, rows in t.items():
    print(k, [(r["step"], r["degree"], r["cols"], r["peak"]) for r in rows[-4:]])
