"""Lay out the recorded msolve traces of run 20261018T0627Z (pod zdec0w9xwevehz).
Reads runs/*.log (msolve -v 2 round table) and runs/*.mem; computes nothing
beyond summing the recorded per-round times and converting units."""
import glob, os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROW = re.compile(r"\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+([\d.]+)%\s+(\d+) new\s+(\d+) zero\s+([\d.]+) \|")
runs = []
for log in sorted(glob.glob("../../runs/*.log")):
    name = os.path.basename(log)[:-4]
    rounds = [ROW.match(l) for l in open(log, errors="replace")]
    rounds = [(int(m.group(1)), float(m.group(9))) for m in rounds if m]
    mem = dict(kv.split("=") for kv in open(log[:-4] + ".mem").read().split())
    runs.append((name, rounds, int(mem["VmHWM_kB"]) / 1e6, mem["exit"]))

walls = {  # seconds, from progress.txt (start/end stamps) and the WALL values
    "pkm_kummer_p2013265921_m3_t5_x1289903598": (1299, "finished"),
    "pkm_kummer_p2013265921_m3_t7_x221026520": (15458, "wall 15,458 s"),
    "pkm_kummer_p2013265921_m3_t7_x704078485": (7728, "wall 7,728 s"),
    "pkm_kummer_p2013265921_m3_t8_x1227422598": (8328, "wall 8,328 s"),
}
short = {"m3_t5_x1289903598": "t5 (N=15) control", "m3_t7_x221026520": "t7 target 0 (N=21)",
         "m3_t7_x704078485": "t7 target 1 (N=21)", "m3_t8_x1227422598": "t8 target 0 (N=24)"}
label = lambda n: short[n.split("p2013265921_")[1]]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(11.5, 4.6), gridspec_kw={"width_ratios": [1.5, 1]})
colors = ["#7f7f7f", "#1f77b4", "#ff7f0e", "#2ca02c"]
for (name, rounds, hwm, rc), c in zip(runs, colors):
    t, deg, cum = [0.0], [rounds[0][0]], 0.0
    for d, secs in rounds:
        t.append(cum); deg.append(d); cum += secs; t.append(cum); deg.append(d)
    wall, how = walls[name]
    t.append(wall); deg.append(deg[-1])
    ax1.step(t, deg, where="post", color=c, lw=1.8, label=f"{label(name)}: max degree {max(deg)}, {how}")
    ax1.plot([wall], [deg[-1]], marker="x" if rc != "0" else "o", color=c, ms=8)
ax1.set_xscale("symlog", linthresh=60)
ax1.set_xlabel("cumulative msolve round time (s, symlog)")
ax1.set_ylabel("degree of the pairs selected in the round")
ax1.set_yticks(range(2, 9)); ax1.set_ylim(1.5, 8.5)
ax1.axhline(8, color="#d62728", ls=":", lw=1); ax1.text(70, 8.08, "a productive degree-8 step would be H0", fontsize=7, color="#d62728")
ax1.set_title("msolve 0.6.5 F4 round degrees per system (x = wall stop, o = finished)")
ax1.legend(fontsize=7.5, loc="lower right")

names = [label(r[0]) for r in runs]
x = range(len(runs))
ax2.bar([i - 0.2 for i in x], [r[2] for r in runs], width=0.4, color="#2ca02c", label="peak VmHWM (GB)")
ax2.axhline(27.2, color="k", ls="--", lw=1); ax2.text(1.55, 27.8, "prlimit cap 27.2 GB", fontsize=7)
ax2.set_ylabel("GB"); ax2.set_ylim(0, 32)
ax3 = ax2.twinx()
ax3.bar([i + 0.2 for i in x], [walls[r[0]][0] / 3600 for r in runs], width=0.4, color="#9467bd", label="wall (h)")
ax3.set_ylabel("hours"); ax3.set_ylim(0, 5)
ax2.set_xticks(list(x)); ax2.set_xticklabels([n.replace(" (", "\n(") for n in names], fontsize=7.5)
ax2.set_title("resources per run (8 vCPU pod, 1 thread, cgroup 32 GB)")
h1, l1 = ax2.get_legend_handles_labels(); h2, l2 = ax3.get_legend_handles_labels()
ax2.legend(h1 + h2, l1 + l2, fontsize=8, loc="upper left", bbox_to_anchor=(0.0, 0.84))
fig.suptitle("pkm_m3_n21_msolve_20261018 · run 20261018T0627Z · pod zdec0w9xwevehz (commit 6667c066)", fontsize=10)
fig.tight_layout()
fig.savefig("msolve-degree.svg"); fig.savefig("msolve-degree.png", dpi=150)
for name, rounds, hwm, rc in runs:
    print(name, "rounds", len(rounds), "maxdeg", max(d for d, _ in rounds), "sum round secs", round(sum(s for _, s in rounds), 1), "hwm GB", round(hwm, 2), "exit", rc)
