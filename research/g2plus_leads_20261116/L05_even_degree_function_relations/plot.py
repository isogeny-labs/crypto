"""Standard-library SVG plot: GM over runs of FPM per relation per arm, per (g, p) cell (log y)."""
import glob
import json
import math
import sys

raw = sys.argv[1] if len(sys.argv) > 1 else "raw"
out = sys.argv[2] if len(sys.argv) > 2 else "plot.svg"
arms = ["O", "E_0", "E_1a", "E_low", "O_T", "E_1a_T"]
colors = {"O": "#1f77b4", "E_0": "#d62728", "E_1a": "#2ca02c", "E_low": "#9467bd", "O_T": "#17becf", "E_1a_T": "#ff7f0e"}
cells = [(g, p) for g in (2, 3) for p in (1031, 4099, 16411, 65537)]
data = {}
for fn in glob.glob(raw + "/rel_*_root_*.jsonl"):
    for l in open(fn):
        r = json.loads(l)
        if r["type"] == "run" and r["status"] == "ok" and r["fpm_per_rel"]:
            data.setdefault((r["g"], r["p"], r["arm"]), []).append(r["fpm_per_rel"])
gm = {k: math.exp(sum(math.log(x) for x in v) / len(v)) for k, v in data.items()}
vals = list(gm.values()) or [1.0]
lo, hi = math.log10(min(vals)) - 0.1, math.log10(max(vals)) + 0.1
W, H, L, B, T = 900, 420, 70, 70, 30
pw = (W - L - 20) / len(cells)
def Y(v):
    return T + (H - T - B) * (1 - (math.log10(v) - lo) / (hi - lo))
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="11">',
     f'<rect width="{W}" height="{H}" fill="white"/>',
     f'<text x="{W/2}" y="16" text-anchor="middle" font-size="13">FPM per distinct relation, GM over root-curve runs (counts, I=10)</text>']
for e in range(math.floor(lo), math.ceil(hi) + 1):
    if lo <= e <= hi:
        y = Y(10 ** e)
        s.append(f'<line x1="{L}" y1="{y}" x2="{W-20}" y2="{y}" stroke="#ddd"/><text x="{L-6}" y="{y+4}" text-anchor="end">1e{e}</text>')
bw = pw / (len(arms) + 1)
for i, (g, p) in enumerate(cells):
    x0 = L + i * pw
    s.append(f'<text x="{x0+pw/2}" y="{H-B+18}" text-anchor="middle">g={g} p={p}</text>')
    for j, a in enumerate(arms):
        v = gm.get((g, p, a))
        if v is None:
            continue
        y = Y(v)
        s.append(f'<rect x="{x0+bw*(j+0.5)}" y="{y}" width="{bw*0.9}" height="{H-B-y}" fill="{colors[a]}"/>')
for j, a in enumerate(arms):
    s.append(f'<rect x="{L+j*110}" y="{H-24}" width="10" height="10" fill="{colors[a]}"/><text x="{L+j*110+14}" y="{H-15}">{a}</text>')
s.append("</svg>")
open(out, "w").write("\n".join(s))
