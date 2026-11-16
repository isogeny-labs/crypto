#!/usr/bin/env python3
"""Render figures/hnf_finish.{svg,png} from the committed dev records (layout only).

Inputs: dev/m5p331/hnf.log, dev/m5p331/kernel_oom.txt, dev/m5p331/msolve.run.json and
the W0 baseline at p = 4111 from ../ecgfp5_quotient_solve_20261101/analysis.json.
"""
import json
import re
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
DEV = HERE / "dev" / "m5p331"
FIG = HERE / "figures"
FIG.mkdir(exist_ok=True)
GIB = 1024 ** 2  # KiB per GiB

phases = []
for line in (DEV / "hnf.log").read_text().splitlines():
    m = re.match(r"\[hnfsolve\] (\w+): ([\d.]+) s wall, ([\d.]+) s cpu, maxrss (\d+) KiB", line)
    if m:
        phases.append((m.group(1), float(m.group(2)), float(m.group(3)), int(m.group(4)) / GIB))
oom = re.search(r"anon-rss:(\d+)kB", (DEV / "kernel_oom.txt").read_text())
oom_gib = int(oom.group(1)) / GIB
ms = json.loads((DEV / "msolve.run.json").read_text())

fig, ax = plt.subplots(1, 2, figsize=(12.5, 4.6))

# (a) hnfsolve resident memory by phase, with the kill point and the caps
a = ax[0]
names = [p[0] for p in phases] + ["determinant\n(killed)"]
rss = [p[3] for p in phases] + [oom_gib]
cols = ["#0072B2"] * len(phases) + ["#D55E00"]
a.bar(range(len(names)), rss, color=cols)
for i, v in enumerate(rss):
    a.text(i, v + 0.2, f"{v:.2f}", ha="center", fontsize=8)
a.axhline(12, color="k", ls="--", lw=1)
a.text(-0.4, 12.15, "protocol envelope 12 GiB", fontsize=8)
a.axhline(16477352 / GIB, color="gray", ls=":", lw=1)
a.text(-0.4, 15.85, "host RAM 15.7 GiB", fontsize=8, color="gray")
a.set_xticks(range(len(names)), names, fontsize=8)
a.set_ylabel("peak RSS (GiB)")
a.set_ylim(0, 17.5)
a.set_title("(a) m = 5, p = 331: hnfsolve memory (measured)\n"
            "determinant = PML LNZ17, killed by the memory cgroup", fontsize=10)

# (b) CPU seconds of the finish: what is measured, what is predicted
b = ax[1]
w0 = json.loads((HERE.parent / "ecgfp5_quotient_solve_20261101" / "analysis.json").read_text())
ps = w0["phase_split"]["m5_p4111"]
fin = ps["solve_cpu_s"] - ps["f4_cpu_s"]  # finish = whole solve minus F4 (20,655 CPU-s)
labels = ["W0 finish\nmsolve Wiedemann\n(measured, p=4111)",
          "BNS22 model\nat t = 4920\n(predicted)",
          "hnfsolve pre-det\nparse+stair+P\n(measured, p=331)"]
vals = [fin, 3000.0, sum(p[2] for p in phases)]
cl = ["#555555", "#BBBBBB", "#0072B2"]
b.bar(range(3), vals, color=cl, hatch=["", "//", ""])
for i, v in enumerate(vals):
    b.text(i, v * 1.15, f"{v:,.0f} s", ha="center", fontsize=8)
b.set_yscale("log")
b.set_ylim(50, 1e5)
b.set_xticks(range(3), labels, fontsize=8)
b.set_ylabel("CPU-s (log)")
b.set_title("(b) finish CPU: no HNF finish completed\n"
            f"(msolve F4 + basis print at p = 331: {ms['user_seconds'] + ms['sys_seconds']:,.0f} CPU-s)",
            fontsize=10)

fig.tight_layout()
fig.savefig(FIG / "hnf_finish.svg")
fig.savefig(FIG / "hnf_finish.png", dpi=130)
print("wrote figures/hnf_finish.svg and .png")
