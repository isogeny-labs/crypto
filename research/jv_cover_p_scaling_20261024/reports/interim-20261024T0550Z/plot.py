#!/usr/bin/env python3
"""Layout only: plot S/rho as recorded by the jv_cover binary (field s_over_rho) against p.
No result is computed here; values are read from results/dlp_p*_s*.json."""
import glob, json, math, sys, os
here = os.path.dirname(os.path.abspath(__file__))
res = os.path.join(here, "..", "..", "results")
rows = []
for f in sorted(glob.glob(os.path.join(res, "dlp_p*_s*.json"))):
    try:
        d = json.load(open(f))[0]
    except Exception:
        continue
    rows.append((d["p"], d["seed"], d["s_over_rho"], d["solved"] and d["correct"], d.get("predicted_s_over_rho")))
rows.sort()
with open(os.path.join(here, "data.csv"), "w") as fh:
    fh.write("p,seed,s_over_rho_recorded,solved_and_correct,predicted_s_over_rho_binary\n")
    for r in rows:
        fh.write(",".join(str(x) for x in r) + "\n")
W, H, L, R, T, B = 640, 420, 70, 20, 30, 60
xmin, xmax = math.log10(40), math.log10(1200)
ymin, ymax = math.log10(0.5), math.log10(3000)
X = lambda p: L + (math.log10(p) - xmin) / (xmax - xmin) * (W - L - R)
Y = lambda v: T + (ymax - math.log10(v)) / (ymax - ymin) * (H - T - B)
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="monospace" font-size="11">',
     f'<rect width="{W}" height="{H}" fill="#ffffff"/>']
for v in (1, 10, 100, 1000):
    style = 'stroke="#b8480f" stroke-width="1.5" stroke-dasharray="6 4"' if v == 1 else 'stroke="#dde1e8"'
    s.append(f'<line x1="{L}" x2="{W-R}" y1="{Y(v):.1f}" y2="{Y(v):.1f}" {style}/>')
    s.append(f'<text x="{L-6}" y="{Y(v)+4:.1f}" text-anchor="end" fill="#444">{v}</text>')
for p in (53, 101, 151, 251, 503, 1009):
    s.append(f'<line x1="{X(p):.1f}" x2="{X(p):.1f}" y1="{T}" y2="{H-B}" stroke="#eef0f4"/>')
    s.append(f'<text x="{X(p):.1f}" y="{H-B+16}" text-anchor="middle" fill="#444">{p}</text>')
s.append(f'<text x="{W-R-4}" y="{Y(1)-6:.1f}" text-anchor="end" fill="#b8480f">S/rho = 1</text>')
for p, seed, v, ok, pred in rows:
    s.append(f'<circle cx="{X(p):.1f}" cy="{Y(v):.1f}" r="4.5" fill="{"#2c5cc5" if ok else "#ffffff"}" stroke="#2c5cc5"/>')
preds = sorted({(p, pred) for p, _, _, _, pred in rows if pred})
for p, pred in preds:
    s.append(f'<path d="M{X(p)-5:.1f},{Y(pred):.1f} h10" stroke="#1f7a4d" stroke-width="2"/>')
s.append(f'<text x="{(L+W-R)/2}" y="{H-18}" text-anchor="middle" fill="#222">p  (E over F_(p^6), genus-3 cover over F_(p^2)); log scale</text>')
s.append(f'<text x="16" y="{(T+H-B)/2}" transform="rotate(-90 16 {(T+H-B)/2})" text-anchor="middle" fill="#222">S / rho  (log scale)</text>')
s.append(f'<circle cx="{W-R-300}" cy="{T+8}" r="4.5" fill="#2c5cc5"/><text x="{W-R-290}" y="{T+12}" fill="#222">measured, one (p, seed), solved and correct</text>')
s.append(f'<path d="M{W-R-305},{T+26} h10" stroke="#1f7a4d" stroke-width="2"/><text x="{W-R-290}" y="{T+30}" fill="#222">binary predicted_s_over_rho (model)</text>')
s.append('</svg>')
open(os.path.join(here, "s_over_rho_vs_p.svg"), "w").write("\n".join(s))
print(len(rows), "rows")
