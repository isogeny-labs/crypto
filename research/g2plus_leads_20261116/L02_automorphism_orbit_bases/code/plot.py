import json, os, math
os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
A = json.load(open("raw/analysis.json"))
arms = [("IC_A", "#1f77b4"), ("IC_B", "#d62728"), ("R_neg", "#2ca02c"), ("R_aut", "#9467bd")]
panels = [("F5", [10, 11, 12, 13, 14]), ("F7", [8, 9, 10])]
W, H = 900, 380
out = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" font-family="sans-serif" font-size="12">' % (W, H),
       '<rect width="100%" height="100%" fill="white"/>',
       '<text x="450" y="18" text-anchor="middle" font-size="14">Mean FPM (I=10, kappa=g^2) per arm, log scale; unverified, Python instrument</text>']
for pi, (fam, ks) in enumerate(panels):
    x0 = 60 + pi * 440; pw = 360; y0 = 40; ph = 280
    vals = [A["cells"]["%s|%d" % (fam, k)]["readings"]["I10_kg2"][a] for k in ks for a, _ in arms]
    lo, hi = math.floor(math.log10(min(vals))), math.ceil(math.log10(max(vals)))
    def X(i): return x0 + (i + 0.5) * pw / len(ks)
    def Y(v): return y0 + ph - (math.log10(v) - lo) / (hi - lo) * ph
    out.append('<rect x="%d" y="%d" width="%d" height="%d" fill="none" stroke="#888"/>' % (x0, y0, pw, ph))
    out.append('<text x="%d" y="%d" text-anchor="middle">%s (y^2 = x^%d + a)</text>' % (x0 + pw / 2, y0 - 6, fam, 5 if fam == "F5" else 7))
    for e in range(lo, hi + 1):
        out.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="#ddd"/><text x="%d" y="%.1f" text-anchor="end">1e%d</text>' % (x0, x0 + pw, Y(10 ** e), Y(10 ** e), x0 - 4, Y(10 ** e) + 4, e))
    for i, k in enumerate(ks):
        out.append('<text x="%.1f" y="%d" text-anchor="middle">k=%d</text>' % (X(i), y0 + ph + 16, k))
    for a, col in arms:
        pts = " ".join("%.1f,%.1f" % (X(i), Y(A["cells"]["%s|%d" % (fam, k)]["readings"]["I10_kg2"][a])) for i, k in enumerate(ks))
        out.append('<polyline points="%s" fill="none" stroke="%s" stroke-width="2"/>' % (pts, col))
        for i, k in enumerate(ks):
            out.append('<circle cx="%.1f" cy="%.1f" r="3" fill="%s"/>' % (X(i), Y(A["cells"]["%s|%d" % (fam, k)]["readings"]["I10_kg2"][a]), col))
for j, (a, col) in enumerate(arms):
    out.append('<rect x="%d" y="%d" width="12" height="12" fill="%s"/><text x="%d" y="%d">%s</text>' % (200 + j * 130, 345, col, 218 + j * 130, 356, a))
out.append("</svg>")
open("plot.svg", "w").write("\n".join(out))
