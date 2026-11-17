"""Lay out recorded Q5 (msolve) and Q2/Q4 (Buchberger) ratios as SVG. Computes nothing:
every plotted value is a column of q5_cells.csv, q2_cells.csv or q4_cells.csv."""
import csv, math, os, sys
q5, q2, q4, figs = sys.argv[1:5]
rd = lambda p: list(csv.DictReader(open(p)))
f5 = rd(os.path.join(q5, "q5_cells.csv")); b2 = rd(os.path.join(q2, "q2_cells.csv")); b4 = rd(os.path.join(q4, "q4_cells.csv"))
def get5(part, num, den, p, k, s):
    for r in f5:
        if (r["part"], r["num"], r["den"], r["p"], r["k"], r["stratum"]) == (part, num, den, p, k, s): return float(r["ratio"])
cells = [(r["p"], r["k"], r["stratum"]) for r in b2 if r["order"] == "grevlex"]
series = [
  ("A2/B Buchberger", "#d08a1c", [float(next(r for r in b2 if r["order"]=="grevlex" and (r["p"],r["k"],r["stratum"])==c)["A2_over_B"]) for c in cells], "o"),
  ("A2/B msolve F4", "#d08a1c", [get5("Q2","A2","B",*c) for c in cells], "s"),
  ("B/A1 Buchberger", "#b8322d", [float(next(r for r in b2 if r["order"]=="grevlex" and (r["p"],r["k"],r["stratum"])==c)["B_over_A1"]) for c in cells], "o"),
  ("B/A1 msolve F4", "#b8322d", [get5("Q2","B","A1",*c) for c in cells], "s"),
  ("sel/native Buchberger", "#2a6fbb", [float(next(r for r in b4 if r["order"]=="grevlex" and (r["p"],r["k"],r["stratum"])==c)["sel_over_nat"]) for c in cells], "o"),
  ("sel/native msolve F4", "#2a6fbb", [get5("Q4","C_sel_grevlex","C_nat",*c) for c in cells], "s"),
]
W, H, L, T, B = 920, 400, 60, 40, 95
lo, hi = -0.5, 1.5
ys = lambda v: T + (H - T - B) * (1 - (min(max(math.log2(v), lo), hi) - lo) / (hi - lo))
body = f'<text x="{L}" y="22" font-size="13">Grevlex cost ratios, Buchberger termops (circles) vs msolve F4 matrix cells (squares); log2 axis; values from q2/q4/q5_cells.csv</text>'
for g in (0.75, 1, 1.5, 2, 2.5):
    y = ys(g); body += f'<line x1="{L}" x2="{W-10}" y1="{y}" y2="{y}" stroke="{"#999" if g == 1 else "#ddd"}"/><text x="{L-6}" y="{y+4}" text-anchor="end">{g}x</text>'
slot = (W - L - 20) / len(cells)
for i, c in enumerate(cells):
    x0 = L + 10 + i * slot
    for j, (lab, col, vals, mk) in enumerate(series):
        x = x0 + (j // 2) * slot / 3.2 + (j % 2) * 7 + 8; y = ys(vals[i])
        body += (f'<circle cx="{x}" cy="{y}" r="3.5" fill="{col}"><title>{lab} {vals[i]:.3f}</title></circle>' if mk == "o"
                 else f'<rect x="{x-3.5}" y="{y-3.5}" width="7" height="7" fill="{col}"><title>{lab} {vals[i]:.3f}</title></rect>')
    body += f'<text x="{x0+slot/2}" y="{H-B+14}" text-anchor="middle" font-size="9">p={c[0]}</text><text x="{x0+slot/2}" y="{H-B+25}" text-anchor="middle" font-size="9">k={c[1]} {c[2]}</text>'
for j, (lab, col, _, mk) in enumerate(series):
    x = L + (j % 3) * 280; y = H - 45 + (j // 3) * 18
    body += (f'<circle cx="{x+5}" cy="{y-4}" r="4" fill="{col}"/>' if mk == "o" else f'<rect x="{x+1}" y="{y-8}" width="8" height="8" fill="{col}"/>') + f'<text x="{x+14}" y="{y}" font-size="11">{lab}</text>'
open(os.path.join(figs, "q5_buchberger_vs_f4.svg"), "w").write(f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11"><rect width="100%" height="100%" fill="white"/>{body}</svg>\n')
print("ok")
