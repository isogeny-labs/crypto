"""Layout only: draw fig2_charts.svg from rows.jsonl and a2_rows.jsonl.

For curve 1's four conj targets with a short decomposition, every transformed
solve (the first A plus A2's three) is one point: x = rational solutions missing
from this chart (the target's largest count over its charts minus this one),
y = 4,096 - quotient dimension. The failed solve has no rational output and is
drawn on the right margin.
"""
import json
import sys

run = sys.argv[1]
TARGETS = ["planted[5, 6, 12, 19]", "common0", "planted3_0[8, 13, 26]", "planted3_1[0, 2, 15]"]
rows = []
for f in ("rows.jsonl", "a2_rows.jsonl"):
    for l in open(f"{run}/{f}"):
        r = json.loads(l)
        if r.get("kind") == "inf" and r["transform"] and r["curve"] == 1 and r["arm"] == "conj" and r["target"] in TARGETS:
            rows.append(r)
best = {t: max(r["e_solutions"] for r in rows if r["target"] == t) for t in TARGETS}
COL = ["#2f6fb0", "#d9730d", "#3a9a5b", "#8a4fb0"]
W, H, L, T, B = 640, 420, 70, 60, 70
PW, PH = 400, H - T - B
def sx(v): return L + v / 3 * PW
def sy(v): return T + PH - v / 3 * PH
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="11">',
     '<rect width="100%" height="100%" fill="white"/>',
     '<text x="20" y="22" font-size="13" font-weight="bold">Curve 1 conj targets, 16 random charts: lost dimension = lost rational solutions</text>',
     '<text x="20" y="40" fill="#555">Each point is one transformed solve at Macaulay degree 29 (first A and A2). Post-hoc view; jitter only to separate ties.</text>']
for v in range(4):
    o.append(f'<line x1="{L}" y1="{sy(v)}" x2="{L+PW}" y2="{sy(v)}" stroke="#eee"/><text x="{L-6}" y="{sy(v)+4}" text-anchor="end">{v}</text>')
    o.append(f'<line x1="{sx(v)}" y1="{T}" x2="{sx(v)}" y2="{T+PH}" stroke="#eee"/><text x="{sx(v)}" y="{T+PH+16}" text-anchor="middle">{v}</text>')
o.append(f'<line x1="{sx(0)}" y1="{sy(0)}" x2="{sx(3)}" y2="{sy(3)}" stroke="#999" stroke-dasharray="4 3"/>')
o.append(f'<text x="{L+PW/2}" y="{T+PH+36}" text-anchor="middle">rational solutions missing from this chart</text>')
o.append(f'<text x="18" y="{T+PH/2}" transform="rotate(-90 18 {T+PH/2})" text-anchor="middle">4,096 − dim</text>')
seen = {}
for r in rows:
    ti = TARGETS.index(r["target"])
    d = 4096 - r["dim"]
    if not r["solved"]:
        cx, cy = L + PW + 40, sy(d)
        o.append(f'<text x="{cx}" y="{cy+4}" text-anchor="middle" font-size="14" fill="{COL[ti]}">×</text>'
                 f'<text x="{cx+10}" y="{cy+4}" font-size="9">solve failed (no output)</text>')
        continue
    x = best[r["target"]] - r["e_solutions"]
    k = seen.get((x, d), 0)
    seen[(x, d)] = k + 1
    cx, cy = sx(x) + (k % 4 - 1.5) * 7, sy(d) - (k // 4) * 7
    o.append(f'<circle cx="{cx}" cy="{cy}" r="4.5" fill="{COL[ti]}" fill-opacity="0.85"/>')
for i, t in enumerate(TARGETS):
    o.append(f'<circle cx="{L+PW+30}" cy="{T+20+i*18}" r="4.5" fill="{COL[i]}"/><text x="{L+PW+40}" y="{T+24+i*18}">{t.split("[")[0]}</text>')
o.append("</svg>")
open(f"{run}/fig2_charts.svg", "w").write("\n".join(o))
