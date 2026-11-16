#!/usr/bin/env python3
"""Write figures/route_closure_map.svg from data/routes.json (pure Python, no
plotting library). Computes nothing. PNG and PDF are rendered from the SVG and
the HTML report with the pre-installed Chromium (see render.js)."""
import json
import pathlib
from xml.sax.saxutils import escape

HERE = pathlib.Path(__file__).resolve().parent
data = json.loads((HERE / "data" / "routes.json").read_text())
(HERE / "figures").mkdir(exist_ok=True)

COL = {
    "closed (proof)": "#7a7a7a", "closed (F4)": "#7a7a7a", "closed (F6)": "#7a7a7a",
    "closed (cost)": "#9a9a9a", "null measured": "#4c72b0", "open lead": "#dd8452",
    "unexamined": "#c44e52",
}
W, H = 1500, 560
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
       'font-family="Helvetica, Arial, sans-serif" font-size="12">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       '<text x="20" y="26" font-size="15" font-weight="bold">Prime-field ECDLP: what Lemma A closes, '
       'what remains, and where the arity goalpost sits (desk note, 2026-11-10; nothing measured)</text>']

def text(x, y, s, size=12, color="#000", anchor="start", weight="normal", style="normal"):
    out.append(f'<text x="{x}" y="{y}" font-size="{size}" fill="{color}" text-anchor="{anchor}" '
               f'font-weight="{weight}" font-style="{style}">{escape(s)}</text>')

# Panel A
x0, y0 = 20, 60
text(x0, y0, "A. Transfer families closed by Lemma A", 14, weight="bold")
text(x0, y0 + 18, "degree < p/4 and additive on E(F_p)[N]  ⇒  algebraic homomorphism", 11, style="italic")
for i, f in enumerate(data["lemma_a_families"]):
    y = y0 + 45 + i * 62
    out.append(f'<rect x="{x0}" y="{y}" width="230" height="46" fill="#e8e8e8" rx="4"/>')
    text(x0 + 8, y + 28, f["family"], 11.5)
    out.append(f'<line x1="{x0+236}" y1="{y+23}" x2="{x0+268}" y2="{y+23}" stroke="#000" marker-end="url(#arr)"/>')
    text(x0 + 276, y + 18, f["target"], 11, "#333")
    text(x0 + 276, y + 36, f["obstruction"], 11, "#7a1f1f")

# Panel B
x1 = 560
text(x1, y0, "B. Non-algebraic operations on [0, p): status", 14, weight="bold")
text(x1, y0 + 18, "one row has no recorded obstruction and no lead: probe P2", 11, style="italic")
for i, o in enumerate(data["operations"]):
    y = y0 + 40 + i * 37
    out.append(f'<rect x="{x1}" y="{y}" width="8" height="28" fill="{COL[o["status"]]}"/>')
    text(x1 + 16, y + 19, o["op"], 11.5)
    text(x1 + 215, y + 19, o["status"], 11, COL[o["status"]])
    text(x1 + 330, y + 19, o["lead"], 9.5, "#555")

# Panel C: exponent plot
x2, yb, pw, ph = 1120, 470, 330, 330
text(x2, y0, "C. Arity frontier vs linear algebra", 14, weight="bold")
text(x2, y0 + 18, "total-cost exponent with a free oracle", 11, style="italic")
ymin, ymax = 0.2, 0.72
def sy(v): return yb - (v - ymin) / (ymax - ymin) * ph
def sx(m): return x2 + 40 + (m - 2) * (pw - 80) / 2
out.append(f'<line x1="{x2}" y1="{yb}" x2="{x2+pw}" y2="{yb}" stroke="#000"/>')
out.append(f'<line x1="{x2}" y1="{yb}" x2="{x2}" y2="{yb-ph}" stroke="#000"/>')
for v in (0.25, 1/3, 0.4, 0.5, 2/3):
    out.append(f'<line x1="{x2-4}" y1="{sy(v)}" x2="{x2}" y2="{sy(v)}" stroke="#000"/>')
    text(x2 - 8, sy(v) + 4, f"{v:.3f}".rstrip("0"), 10, anchor="end")
out.append(f'<line x1="{x2}" y1="{sy(0.5)}" x2="{x2+pw}" y2="{sy(0.5)}" stroke="#000" stroke-dasharray="3,4"/>')
text(x2 + pw - 4, sy(0.5) - 5, "rho exponent 1/2", 10, anchor="end")
exps = {"p^(2/3)": 2/3, "p^(1/2)": 1/2, "p^(1/3)": 1/3, "p^(2/5)": 2/5, "p^(1/4)": 1/4}
af = data["arity_frontier"]
for key, col, dash, lab in (("total_B2", "#7a7a7a", "", "LA = B² (program note §2.4)"),
                             ("total_Bquasi", "#c44e52", "6,4", "LA = Õ(B) (hypothetical)")):
    pts = " ".join(f"{sx(r['m'])},{sy(exps[r[key]])}" for r in af)
    out.append(f'<polyline points="{pts}" fill="none" stroke="{col}" stroke-width="2" stroke-dasharray="{dash}"/>')
    for r in af:
        out.append(f'<circle cx="{sx(r["m"])}" cy="{sy(exps[r[key]])}" r="4" fill="{col}"/>')
        text(sx(r["m"]) + 7, sy(exps[r[key]]) + (-5 if key == "total_B2" else 14), r[key], 10, col)
for r in af:
    text(sx(r["m"]), yb + 16, f"m={r['m']}", 11, anchor="middle")
    text(sx(r["m"]), yb + 30, f"free S{r['m']+1}", 10, "#555", anchor="middle")
text(x2 + 10, y0 + 45, "— LA = B² (program note §2.4)", 10.5, "#7a7a7a")
text(x2 + 10, y0 + 60, "- - LA = Õ(B) (hypothetical)", 10.5, "#c44e52")
out.append('<defs><marker id="arr" markerWidth="8" markerHeight="8" refX="6" refY="4" orient="auto">'
           '<path d="M0,0 L8,4 L0,8 z" fill="#000"/></marker></defs></svg>')
(HERE / "figures" / "route_closure_map.svg").write_text("\n".join(out))
print("wrote figures/route_closure_map.svg")
