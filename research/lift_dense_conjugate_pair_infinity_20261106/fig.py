"""Layout only: draw fig1_infinity.svg from rows.jsonl (no computation beyond 4096 - dim)."""
import json
import sys

run = sys.argv[1]
rows = [json.loads(l) for l in open(f"{run}/rows.jsonl") if '"kind":"inf"' in l]
targets = []
for r in rows:
    k = (r["curve"], r["arm"], r["target"])
    if k not in targets:
        targets.append(k)
by = {(r["curve"], r["arm"], r["target"], r["transform"]): r for r in rows}

W, H = 960, 470
L, R, T, B = 70, 70, 60, 150
CAP = 20  # deficits beyond ±CAP are drawn at the cap and labelled
ymin, ymax = -8, CAP
def y(v):
    v = max(ymin, min(ymax, v))
    return T + (ymax - v) / (ymax - ymin) * (H - T - B)
step = (W - L - R) / len(targets)
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="11">',
     '<rect width="100%" height="100%" fill="white"/>',
     '<text x="20" y="22" font-size="13" font-weight="bold">Missing solutions (4,096 − quotient dimension) at Macaulay degree 29, plain and after a random projective change of coordinates</text>',
     '<text x="20" y="40" fill="#555">orange: R has a decomposition into ≤ 3 factor-base points; blue: it has none. Values beyond the axis are drawn at its edge and labelled.</text>']
for v in range(ymin, ymax + 1, 4):
    o.append(f'<line x1="{L}" y1="{y(v)}" x2="{W-R}" y2="{y(v)}" stroke="{"#999" if v == 0 else "#eee"}"/>'
             f'<text x="{L-6}" y="{y(v)+4}" text-anchor="end">{v}</text>')
o.append(f'<text x="18" y="{(T+H-B)/2}" transform="rotate(-90 18 {(T+H-B)/2})" text-anchor="middle">4,096 − dim</text>')
for i, (c, arm, t) in enumerate(targets):
    cx = L + step * (i + 0.5)
    p, q = by[(c, arm, t, False)], by[(c, arm, t, True)]
    short = p["short1"] + p["short2"] + p["short3"] > 0
    col = "#d9730d" if short else "#2f6fb0"
    for r, dx, shape in ((p, -7, "c"), (q, 7, "t")):
        d = 4096 - r["dim"]
        yy = y(d)
        ok = r["match"] and r["solved"]
        if shape == "c":
            o.append(f'<circle cx="{cx+dx}" cy="{yy}" r="5" fill="white" stroke="{col}" stroke-width="2"/>')
        else:
            o.append(f'<path d="M{cx+dx-5},{yy+4} L{cx+dx+5},{yy+4} L{cx+dx},{yy-5} Z" fill="{col}"/>')
        if not ok:
            o.append(f'<text x="{cx+dx if d >= 0 else cx+dx-10}" y="{yy-8 if d >= 0 else yy+4}" text-anchor="middle" font-size="9" fill="#b00">×</text>')
        if d > ymax or d < ymin:
            o.append(f'<text x="{cx+dx+9}" y="{yy+4}" font-size="9">{d}</text>')
    name = t.split("[")[0]
    o.append(f'<text transform="translate({cx+3},{H-B+8}) rotate(60)" font-size="10">c{c} {arm} {name}</text>')
lx = L
o.append(f'<circle cx="{lx+5}" cy="{H-12}" r="5" fill="white" stroke="#333" stroke-width="2"/><text x="{lx+15}" y="{H-8}">plain solve</text>')
o.append(f'<path d="M{lx+120},{H-8} L{lx+130},{H-8} L{lx+125},{H-17} Z" fill="#333"/><text x="{lx+136}" y="{H-8}">transformed solve</text>')
o.append(f'<text x="{lx+270}" y="{H-8}" fill="#b00">× solver missed oracle decompositions or returned nothing</text>')
o.append("</svg>")
open(f"{run}/fig1_infinity.svg", "w").write("\n".join(o))
