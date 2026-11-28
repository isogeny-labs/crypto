"""Layout only: draws ops ratios recorded in comparisons/*.json into figure.svg. Computes nothing."""
import json, glob, math, re
rows = {}
for f in glob.glob("comparisons/*.json"):
    m = re.match(r"comparisons/d(\d)_(.+?)__(.+?)__ECC", f)
    d, a, b = int(m[1]), m[2], m[3]
    o = json.load(open(f))["ops"]
    rows[(d, a, b)] = (o["ratio_b_over_a"], o["ci95"])
W, H = 980, 420
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12">', f'<rect width="{W}" height="{H}" fill="white"/>']
def panel(x0, title, series, ylog, ymin, ymax, ticks, ref=None):
    y0, pw, ph = 50, 380, 290
    s.append(f'<text x="{x0}" y="30" font-weight="bold">{title}</text>')
    def Y(v): 
        t = (math.log10(v) - math.log10(ymin)) / (math.log10(ymax) - math.log10(ymin)) if ylog else (v - ymin) / (ymax - ymin)
        return y0 + ph - t * ph
    def X(N): return x0 + (N - 6) / 16 * pw
    s.append(f'<line x1="{x0}" y1="{y0+ph}" x2="{x0+pw}" y2="{y0+ph}" stroke="#333"/><line x1="{x0}" y1="{y0}" x2="{x0}" y2="{y0+ph}" stroke="#333"/>')
    for t in ticks:
        s.append(f'<line x1="{x0}" y1="{Y(t)}" x2="{x0+pw}" y2="{Y(t)}" stroke="#eee"/><text x="{x0-6}" y="{Y(t)+4}" text-anchor="end">{t:g}</text>')
    for N in (8, 12, 16, 20):
        s.append(f'<text x="{X(N)}" y="{y0+ph+16}" text-anchor="middle">N={N}</text>')
    if ref:
        s.append(f'<line x1="{x0}" y1="{Y(ref)}" x2="{x0+pw}" y2="{Y(ref)}" stroke="#c33" stroke-dasharray="4 3"/>')
    for k, (label, key, col) in enumerate(series):
        pts = [(4 * d, rows[(d,) + key]) for d in (2, 3, 4, 5) if (d,) + key in rows]
        s.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (col, " ".join(f"{X(N)},{Y(r)}" for N, (r, _) in pts)))
        for N, (r, ci) in pts:
            s.append(f'<line x1="{X(N)}" y1="{Y(ci[0])}" x2="{X(N)}" y2="{Y(ci[1])}" stroke="{col}"/><circle cx="{X(N)}" cy="{Y(r)}" r="4" fill="{col}"/>')
        s.append(f'<text x="{x0+10}" y="{y0+14+16*k}" fill="{col}">{label}</text>')
panel(60, "A. trace-selected / standard line (counted GAE)", [("selected / standard (95% CI)", ("standard", "selected"), "#3a6ea5"), ("A/A: standard-aa / standard", ("standard", "standard-aa"), "#999")], False, 0, 1.2, [0, 0.25, 0.5, 0.75, 1.0], ref=1.0)
panel(560, "B. gap to matched rho-negation (log scale)", [("selected / rho", ("rho-neg", "selected"), "#3a6ea5"), ("standard / rho", ("rho-neg", "standard"), "#e08a00")], True, 1, 400, [1, 3, 10, 30, 100, 300], ref=1.0)
s.append(f'<text x="60" y="{H-20}" font-size="11">RunPod cpu5c pod (L1, wall time not admitted); 2 curves x 8 targets x 6 rounds per arm; N=8 standard arms failed (no small-base columns).</text>')
s.append("</svg>")
open("figure.svg", "w").write("\n".join(s))
