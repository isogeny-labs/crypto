"""Lay out recorded values from a run directory as SVG. Computes nothing:
every plotted number is a column of q1_cells.csv, q2_cells.csv or s1_structure.csv
written by the Rust harness (`isomu_fb ... analyze`)."""
import csv, math, sys, os

run = sys.argv[1]; figs = sys.argv[2]
os.makedirs(figs, exist_ok=True)
rd = lambda n: list(csv.DictReader(open(os.path.join(run, n))))
COL = {"A0": "#2a6fbb", "A1": "#5aa0e0", "A2": "#d08a1c", "B": "#b8322d", "B2": "#6b2a8a", "R": "#555555", "PC": "#3a9a4a"}

def svg(path, w, h, body):
    open(path, "w").write(f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" font-family="Helvetica,Arial,sans-serif" font-size="11">'
                          f'<rect width="100%" height="100%" fill="white"/>{body}</svg>\n')

def legend(items, x, y):
    s = ""
    for i, (lab, c) in enumerate(items):
        s += f'<rect x="{x + i*120}" y="{y-9}" width="10" height="10" fill="{c}"/><text x="{x + i*120 + 14}" y="{y}">{lab}</text>'
    return s

# --- Fig 1: Q2 median-termops ratios vs A1, grevlex and lex, log2 axis
q2 = rd("q2_cells.csv")
for order in ("grevlex", "lex"):
    rows = [r for r in q2 if r["order"] == order]
    W, H, L, T, B_ = 900, 380, 60, 40, 80
    ymax = 6.0  # log2 ratio
    ys = lambda v: T + (H - T - B_) * (1 - min(max(math.log2(v), -1), ymax) / ymax) if v > 0 else H - B_
    body = f'<text x="{L}" y="22" font-size="14">Q2 ({order}): median termops relative to A1 (log2 scale), recorded in q2_cells.csv</text>'
    for g in range(0, 7):
        y = ys(2 ** g); body += f'<line x1="{L}" x2="{W-10}" y1="{y}" y2="{y}" stroke="#ddd"/><text x="{L-8}" y="{y+4}" text-anchor="end">{2**g}x</text>'
    y2 = ys(2); body += f'<line x1="{L}" x2="{W-10}" y1="{y2}" y2="{y2}" stroke="#999" stroke-dasharray="4,3"/>'
    n = len(rows); slot = (W - L - 20) / max(n, 1)
    for i, r in enumerate(rows):
        x0 = L + 10 + i * slot
        bars = [("A2", float(r["A2_over_A1"])), ("B", float(r["B_over_A1"])), ("B2", float(r["B2_over_B"]) * float(r["B_over_A1"]))]
        bw = slot / 4.5
        for j, (a, v) in enumerate(bars):
            y = ys(v); body += f'<rect x="{x0 + j*bw}" y="{y}" width="{bw-2}" height="{H-B_-y}" fill="{COL[a]}"><title>{a}/A1 = {v:.3f}</title></rect>'
        if order == "grevlex":
            for (a, lo, hi, j) in (("A2", "A2_over_A1_lo", "A2_over_A1_hi", 0), ("B", "B_over_A1_lo", "B_over_A1_hi", 1)):
                xc = x0 + j*bw + bw/2 - 1
                body += f'<line x1="{xc}" x2="{xc}" y1="{ys(float(r[hi]))}" y2="{ys(float(r[lo]))}" stroke="black"/>'
        body += f'<text x="{x0 + slot/3}" y="{H-B_+14}" text-anchor="middle" font-size="9">p={r["p"]}</text><text x="{x0 + slot/3}" y="{H-B_+25}" text-anchor="middle" font-size="9">k={r["k"]} {r["stratum"]}</text>'
    body += legend([("A2/A1 (Moebius mu_k)", COL["A2"]), ("B/A1 (Velu, order-4 term.)", COL["B"]), ("B2/A1 (Velu, 2-torsion term.)", COL["B2"])], L, H - 20)
    svg(os.path.join(figs, f"q2_ratios_{order}.svg"), W, H, body)

# --- Fig 2: Q1 normalised yield r by arm per cell
q1 = rd("q1_cells.csv")
W, H, L, T, B_ = 900, 340, 60, 40, 70
lo_, hi_ = 0.0, 1.2
ys = lambda v: T + (H - T - B_) * (1 - (min(max(v, lo_), hi_) - lo_) / (hi_ - lo_))
body = f'<text x="{L}" y="22" font-size="14">Q1: median normalised yield r = |D|/E0 by arm (q1_cells.csv); B bar = 95% CI of median(B) - median(R), drawn at median(R)</text>'
for g in (0, 0.25, 0.5, 0.75, 1.0):
    y = ys(g); body += f'<line x1="{L}" x2="{W-10}" y1="{y}" y2="{y}" stroke="#ddd"/><text x="{L-8}" y="{y+4}" text-anchor="end">{g}</text>'
slot = (W - L - 20) / max(len(q1), 1)
for i, r in enumerate(q1):
    x0 = L + 10 + i * slot
    for j, a in enumerate(("A0", "A1", "B", "R", "PC")):
        v = float(r[f"median_ratio_{a}"])
        if math.isnan(v): continue
        body += f'<circle cx="{x0 + j*slot/6 + 4}" cy="{ys(v)}" r="3.5" fill="{COL[a]}"><title>{a} {v:.4f}</title></circle>'
    mr = float(r["median_ratio_R"]); lo, hi = float(r["ci_lo"]), float(r["ci_hi"])
    if not math.isnan(lo):
        xc = x0 + 2*slot/6 + 4
        body += f'<line x1="{xc}" x2="{xc}" y1="{ys(mr+hi)}" y2="{ys(mr+lo)}" stroke="{COL["B"]}" stroke-width="2"/>'
    body += f'<text x="{x0 + slot/3}" y="{H-B_+14}" text-anchor="middle" font-size="9">{r["p"]}/k={r["k"]}</text>'
body += legend([(a, COL[a]) for a in ("A0", "A1", "B", "R", "PC")], L, H - 18)
svg(os.path.join(figs, "q1_yield.svg"), W, H, body)

# --- Fig 3: S1 structure constants per product
s1 = rd("s1_structure.csv")
seen = {}
for r in s1:
    seen.setdefault((r["k"], r["arm"]), r["per_product"])
W, H, L, T, B_ = 520, 300, 60, 40, 60
ymax = 12.0
ys = lambda v: T + (H - T - B_) * (1 - float(v) / ymax)
body = f'<text x="{L}" y="22" font-size="14">S1: nonzero structure constants per product (s1_structure.csv)</text>'
for g in (0, 2, 4, 6, 8, 10, 12):
    y = ys(g); body += f'<line x1="{L}" x2="{W-10}" y1="{y}" y2="{y}" stroke="#ddd"/><text x="{L-8}" y="{y+4}" text-anchor="end">{g}</text>'
arms = ("A0", "A1", "A2", "B", "B2")
for gi, k in enumerate(("3", "4")):
    for j, a in enumerate(arms):
        if (k, a) not in seen: continue
        v = seen[(k, a)]; x = L + 20 + gi * 220 + j * 38; y = ys(v)
        body += f'<rect x="{x}" y="{y}" width="30" height="{H-B_-y}" fill="{COL[a]}"/><text x="{x+15}" y="{y-3}" text-anchor="middle" font-size="9">{float(v):.2f}</text><text x="{x+15}" y="{H-B_+12}" text-anchor="middle" font-size="9">{a}</text>'
    body += f'<text x="{L + 20 + gi*220 + 90}" y="{H-B_+28}" text-anchor="middle">k = {k} (first instance per arm, p = 257)</text>'
svg(os.path.join(figs, "s1_density.svg"), W, H, body)
print("figures written to", figs)
