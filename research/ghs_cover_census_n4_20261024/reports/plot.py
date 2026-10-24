"""Render recorded values only (research-visuals skill; AGENTS.md "no Python" for results: nothing here computes a result).
Every bar is a `group` record (scope=self, by=mod4_E2: curves per minimal genus) written by cover_census_n4.rs; the one
arithmetic line is the model exponent 2 - 2/g printed under each cell, labelled model.
Usage: python3 -I plot.py <results-dir> <outdir>
Writes SVG, PDF (reportlab graphics) and PNG (pdftoppm of that PDF; no rsvg/cairo PNG backend on the host)."""
import json, sys, os, subprocess
from reportlab.graphics.shapes import Drawing, Rect, String, Line
from reportlab.graphics import renderSVG, renderPDF
from reportlab.lib.colors import HexColor
src, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
PANELS = [("P1  q = 3, exhaustive (all curve classes)", "R4-n4-q3-cosets.jsonl"), ("P2  q = 5, exhaustive (all curve classes)", "P2-n4-q5.jsonl"), ("P3  q = 7, seeded sample of 12 curves (neighbour models not in these counts)", "P3-n4-q7-sample12.jsonl")]
CLASSES = [("odd/1", "#E odd"), ("2mod4/2", "#E = 2 mod 4  (E[2](K) = Z/2)"), ("0mod4/2", "4 | #E, E[2](K) = Z/2"), ("0mod4/4", "E[2](K) = (Z/2)^2")]
INK, MUTED, BAR = HexColor("#1a1a19"), HexColor("#6b6a63"), HexColor("#1f6fd1")
def load(f):
    p = os.path.join(src, f)
    return [json.loads(l) for l in open(p)] if os.path.exists(p) and os.path.getsize(p) > 0 else []
rows = [(t, load(f)) for t, f in PANELS]; rows = [(t, r) for t, r in rows if any(x.get("rec") == "group" for x in r)]
W, CW, CH, LM, ROWH = 900, 205, 110, 30, 185
H = 70 + len(rows) * ROWH + 30
d = Drawing(W, H)
d.add(Rect(0, 0, W, H, fillColor=HexColor("#ffffff"), strokeColor=None))
def txt(x, y, t, size=9, fill=INK, anchor="start", bold=False):
    d.add(String(x, H - y, t, fontName="Helvetica-Bold" if bold else "Helvetica", fontSize=size, fillColor=fill, textAnchor=anchor))
txt(15, 20, "GHS cover census, n = 4 (K = F_(q^4)): minimal admissible genus per curve, by 2-torsion class", 12, bold=True)
txt(15, 35, "measured counts of curve isomorphism classes; x axis: the curve's minimum genus over all F_q-structures (sigma-orbit length 4); run 20261024", 8, MUTED)
for ri, (title, recs) in enumerate(rows):
    y0 = 60 + ri * ROWH
    txt(15, y0, title, 10, bold=True)
    for ci, (key, label) in enumerate(CLASSES):
        x0 = LM + ci * (CW + 10); base = y0 + 22 + CH
        txt(x0, y0 + 14, label, 8.5, bold=True)
        d.add(Line(x0, H - base, x0 + CW, H - base, strokeColor=MUTED, strokeWidth=0.6))
        rec = next((r for r in recs if r.get("rec") == "group" and r["scope"] == "self" and r["by"] == "mod4_E2" and r["key"] == key), None)
        if rec is None:
            txt(x0 + 8, base - CH / 2, "no curves in this class", 8, MUTED); continue
        h = {int(k): v for k, v in rec["curves_by_min_genus"].items()}; gs = sorted(h); mx = max(h.values()); bw = min(34, (CW - 8) / len(gs) - 4)
        for i, g in enumerate(gs):
            bh = (CH - 16) * h[g] / mx; bx = x0 + 4 + i * (bw + 4)
            d.add(Rect(bx, H - base, bw, bh, fillColor=BAR, strokeColor=None))
            txt(bx + bw / 2, base - bh - 3, str(h[g]), 8, anchor="middle")
            txt(bx + bw / 2, base + 11, str(g), 8, MUTED, "middle")
        lo = gs[0]
        txt(x0, base + 24, f"n = {rec['curves']} curves; class min g = {lo}", 8)
        txt(x0, base + 35, f"model exponent 2 - 2/g = {2 - 2 / lo:.3f}  (rho: 2)", 8, MUTED)
txt(15, H - 8, "Model exponents are asymptotic formulas (constants ~g! and descent costs not included), not measurements. Source: results/run-20261024/*.jsonl", 7, MUTED)
renderSVG.drawToFile(d, os.path.join(out, "n4_min_genus_by_2torsion.svg"))
renderPDF.drawToFile(d, os.path.join(out, "n4_min_genus_by_2torsion.pdf"))   # vector copy
subprocess.run(["pdftoppm", "-png", "-r", "150", "-singlefile", os.path.join(out, "n4_min_genus_by_2torsion.pdf"), os.path.join(out, "n4_min_genus_by_2torsion")], check=True)
print("wrote svg, pdf, png to", out)
