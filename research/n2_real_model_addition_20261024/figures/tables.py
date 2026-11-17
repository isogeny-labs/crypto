"""Markdown tables for REPORT.md, parsed from the recorded gae_band.txt files
(layout only). Usage: python tables.py <results-dir>"""

import re
import sys
from pathlib import Path

res = Path(sys.argv[1])
pt = re.compile(r"GAE point bits (\d+) h (\d+) (Jglue2?) w ([\d.]+): ic ([\d.e+-]+) GAE, rho ([\d.e+-]+) GAE, ratio ([\d.]+)")
bd = re.compile(r"GAE band bits (\d+) h (\d+) (Jglue2?): \S+ in \[([\d.]+), ([\d.]+)\] over 12 cells \(IC cheaper in (\d+)\) -> (.*)")
pts, bands, ic = {}, {}, {}
for d in sorted(res.iterdir()):
    if d.name.startswith(("N2-glue2", "N3-glue2")):
        t = (d / "gae_band.txt").read_text()
        for m in pt.finditer(t):
            k = (int(m.group(1)), int(m.group(2)), m.group(3), m.group(4))
            pts[k] = float(m.group(7))
            ic[k] = float(m.group(5))
        for m in bd.finditer(t):
            bands[(int(m.group(1)), int(m.group(2)), m.group(3))] = (m.group(4), m.group(5), m.group(6), m.group(7))
sizes = sorted({k[0] for k in pts})
for h in (1, 4):
    print(f"\n**h = {h}**: IC/rho at the gate's three points (I = 3M, measured Jacobian cost; native work at 0 / 0.1 / 1 GAE), baseline then new, and the 12-cell band\n")
    print("| bits | baseline Jglue/rho | Jglue2/rho | IC cost change | 12-cell band, baseline | 12-cell band, new |")
    print("|:--|:--|:--|:--|:--|:--|")
    for b in sizes:
        if (b, h, "Jglue", "0") not in pts:
            continue
        o = " / ".join(f"{pts[(b, h, 'Jglue', w)]:.2f}" for w in ("0", "0.1", "1"))
        n = " / ".join(f"{pts[(b, h, 'Jglue2', w)]:.2f}" for w in ("0", "0.1", "1"))
        dl = " / ".join(f"{100 * (ic[(b, h, 'Jglue2', w)] / ic[(b, h, 'Jglue', w)] - 1):+.1f}%" for w in ("0", "0.1", "1"))
        bo, bn = bands[(b, h, "Jglue")], bands[(b, h, "Jglue2")]
        print(f"| {b} | {o} | {n} | {dl} | {bo[0]}–{bo[1]}, IC cheaper in {bo[2]}/12 ({bo[3]}) | {bn[0]}–{bn[1]}, IC cheaper in {bn[2]}/12 ({bn[3]}) |")
