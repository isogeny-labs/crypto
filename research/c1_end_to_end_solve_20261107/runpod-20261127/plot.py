"""Layout only: draws the recorded ops ratios (from analyze.out) into figure.svg. Computes nothing."""
import re, math
rows = {}
for line in open("analyze.out"):
    m = re.match(r"ops \(ok\): (\S+) / (\S+) = ([\d.]+) \[([\d.]+), ([\d.]+)\].*?band \(w=0: ([\d.]+), w=0.1: ([\d.]+), w=1: ([\d.]+)\)", line)
    if m:
        b, a = m[1], m[2]
        rows[(b, a)] = dict(band=[float(m[6]), float(m[7]), float(m[8])])
dec = re.search(r"ic-tw-a / ic-std = ([\d.]+) \[([\d.]+), ([\d.]+)\]", open("analyze.out").read())
W, H = 980, 420
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12">', f'<rect width="{W}" height="{H}" fill="white"/>']
# Panel A: decision ratio and line A/A
x0, y0, pw, ph = 70, 60, 330, 280
def YA(v): return y0 + ph - v / 1.2 * ph
s.append(f'<text x="{x0}" y="35" font-weight="bold">A. Decision: twisted / standard line (counted GAE)</text>')
s.append(f'<line x1="{x0}" y1="{y0+ph}" x2="{x0+pw}" y2="{y0+ph}" stroke="#333"/><line x1="{x0}" y1="{y0}" x2="{x0}" y2="{y0+ph}" stroke="#333"/>')
for t in (0, 0.25, 0.5, 0.75, 1.0):
    s.append(f'<line x1="{x0}" y1="{YA(t)}" x2="{x0+pw}" y2="{YA(t)}" stroke="#eee"/><text x="{x0-6}" y="{YA(t)+4}" text-anchor="end">{t:g}</text>')
s.append(f'<line x1="{x0}" y1="{YA(1)}" x2="{x0+pw}" y2="{YA(1)}" stroke="#c33" stroke-dasharray="4 3"/>')
items = [("tw-a / std", float(dec[1]), float(dec[2]), float(dec[3]), "#3a6ea5"), ("tw-b / tw-a (line A/A)", 1.0306, 0.964, 1.102, "#999")]
for k, (lab, r, lo, hi, col) in enumerate(items):
    cx = x0 + pw * (0.3 + 0.4 * k)
    s.append(f'<line x1="{cx}" y1="{YA(lo)}" x2="{cx}" y2="{YA(hi)}" stroke="{col}" stroke-width="2"/><circle cx="{cx}" cy="{YA(r)}" r="6" fill="{col}"/>')
    s.append(f'<text x="{cx}" y="{y0+ph+18}" text-anchor="middle">{lab}</text><text x="{cx+10}" y="{YA(r)+4}">{r:.3f}</text>')
# Panel B: gap to rho across band
x1 = 560
s.append(f'<text x="{x1}" y="35" font-weight="bold">B. Gap to strong rho across the §5a band (log)</text>')
def YB(v): return y0 + ph - (math.log10(v) - 2) / 3 * ph
def XB(i): return x1 + 40 + i * 140
s.append(f'<line x1="{x1}" y1="{y0+ph}" x2="{x1+360}" y2="{y0+ph}" stroke="#333"/><line x1="{x1}" y1="{y0}" x2="{x1}" y2="{y0+ph}" stroke="#333"/>')
for t in (100, 300, 1000, 3000, 10000, 30000):
    s.append(f'<line x1="{x1}" y1="{YB(t)}" x2="{x1+360}" y2="{YB(t)}" stroke="#eee"/><text x="{x1-6}" y="{YB(t)+4}" text-anchor="end">{t:g}</text>')
for i, w in enumerate(("w=0", "w=0.1", "w=1")):
    s.append(f'<text x="{XB(i)}" y="{y0+ph+18}" text-anchor="middle">{w}</text>')
for arm, col in (("ic-std", "#e08a00"), ("ic-tw-a", "#3a6ea5")):
    vals = [max(rows[(arm, "rho-strong-l1")]["band"][i], rows[(arm, "rho-strong-l16")]["band"][i]) for i in range(3)]
    s.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (col, " ".join(f"{XB(i)},{YB(v)}" for i, v in enumerate(vals))))
    for i, v in enumerate(vals):
        s.append(f'<circle cx="{XB(i)}" cy="{YB(v)}" r="4" fill="{col}"/><text x="{XB(i)+6}" y="{YB(v)-6}" font-size="10">{v:.0f}</text>')
    s.append(f'<text x="{x1+200}" y="{y0+14+(16 if arm=="ic-tw-a" else 0)}" fill="{col}">{arm} / matched rho</text>')
s.append(f'<text x="70" y="{H-20}" font-size="11">f2m4_a0, m = 5; 6 workloads x 5 rounds; RunPod cpu5c (L1, wall time not admitted); freeze-commit ecbench 3c04b4be.</text>')
s.append("</svg>")
open("figure.svg", "w").write("\n".join(s))
