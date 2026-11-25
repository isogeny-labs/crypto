#!/usr/bin/env python3
"""Hand-written SVG figure for this study (no matplotlib here). Writes fig_sibling.svg."""
import json
import math
import os
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
an = json.load(open(os.path.join(HERE, "analysis.json")))
runs = [json.loads(l) for l in open(os.path.join(HERE, "raw", "runs.jsonl"))]


def cnt(m, i):
    return m + 10 * i


per = defaultdict(lambda: {"A": [], "B": []})
for r in runs:
    key = (r["g"], r["p"])
    if r["arm_label"] == "A":
        per[key]["A"].append(cnt(r["fwd_mul"] + r["restart_mul"], r["fwd_inv"] + r["restart_inv"]) / r["steps"])
    if r["arm_label"] == "B":
        per[key]["B"].append(cnt(r["fwd_mul"] + r["pm_mul"] + r["restart_mul"], r["fwd_inv"] + r["pm_inv"] + r["restart_inv"]) / (r["steps"] + r["sibs"]))

CA, CB, INK, MUTED, GRID = "#4a6fa5", "#d1823a", "#222", "#666", "#ddd"
W, H = 980, 480
s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif">',
     f'<rect width="{W}" height="{H}" fill="#fff"/>',
     f'<text x="20" y="28" font-size="17" fill="{INK}" font-weight="bold">Sibling divisors D − M in the genus 2/3 relation walk [pcc:unverified, L0, counted]</text>']

# panel 1: counted Cantor cost per candidate
x0, y0, pw, ph = 70, 70, 360, 290
cells = sorted(per)
ymax = 300
s.append(f'<text x="{x0}" y="{y0-12}" font-size="13" fill="{INK}">Cantor cost per relation candidate (mults + 10·inv)</text>')
for t in range(0, ymax + 1, 50):
    yy = y0 + ph - ph * t / ymax
    s.append(f'<line x1="{x0}" y1="{yy:.1f}" x2="{x0+pw}" y2="{yy:.1f}" stroke="{GRID}"/>')
    s.append(f'<text x="{x0-8}" y="{yy+4:.1f}" font-size="11" fill="{MUTED}" text-anchor="end">{t}</text>')
gw = pw / len(cells)
for ci, key in enumerate(cells):
    gx = x0 + ci * gw
    for ai, (arm, col) in enumerate((("A", CA), ("B", CB))):
        vals = per[key][arm]
        m = sum(vals) / len(vals)
        bx = gx + 18 + ai * 44
        bh = ph * m / ymax
        s.append(f'<rect x="{bx:.1f}" y="{y0+ph-bh:.1f}" width="38" height="{bh:.1f}" fill="{col}"/>')
        s.append(f'<text x="{bx+19:.1f}" y="{y0+ph-bh-5:.1f}" font-size="11" fill="{INK}" text-anchor="middle">{m:.0f}</text>')
    ratio = (sum(per[key]["B"]) / len(per[key]["B"])) / (sum(per[key]["A"]) / len(per[key]["A"]))
    s.append(f'<text x="{gx+gw/2:.1f}" y="{y0+ph+16}" font-size="11" fill="{INK}" text-anchor="middle">g={key[0]}, p={key[1]}</text>')
    s.append(f'<text x="{gx+gw/2:.1f}" y="{y0+ph+31}" font-size="11" fill="{MUTED}" text-anchor="middle">B/A = {ratio:.3f}</text>')
s.append(f'<rect x="{x0+10}" y="{y0+4}" width="12" height="12" fill="{CA}"/><text x="{x0+27}" y="{y0+14}" font-size="11" fill="{INK}">A: forward walk (lab Cantor)</text>')
s.append(f'<rect x="{x0+10}" y="{y0+22}" width="12" height="12" fill="{CB}"/><text x="{x0+27}" y="{y0+32}" font-size="11" fill="{INK}">B: + sibling D − M (shared xgcd, u₁u₂, e₁u₁)</text>')

# panel 2: IC / matched rho (log), lab vs projected, w = 0, 0.1, 1
x1, pw2 = 540, 400
lo, hi = math.log10(0.04), math.log10(300)
def ly(v):
    return y0 + ph - ph * (math.log10(v) - lo) / (hi - lo)
s.append(f'<text x="{x1}" y="{y0-12}" font-size="13" fill="{INK}">IC / matched negation rho, §5a GAE band (log scale)</text>')
for t in (0.05, 0.1, 0.3, 1, 3, 10, 30, 100):
    yy = ly(t)
    s.append(f'<line x1="{x1}" y1="{yy:.1f}" x2="{x1+pw2}" y2="{yy:.1f}" stroke="{"#999" if t == 1 else GRID}" {"stroke-dasharray=\"4 3\"" if t == 1 else ""}/>')
    s.append(f'<text x="{x1-8}" y="{yy+4:.1f}" font-size="11" fill="{MUTED}" text-anchor="end">{t}</text>')
s.append(f'<text x="{x1+pw2-4}" y="{ly(1)-5:.1f}" font-size="11" fill="{MUTED}" text-anchor="end">rho parity</text>')
pj = an["projection"]
keys = [("g2_p65537", "g=2, p=65537"), ("g2_p524309", "g=2, p=524309"), ("g3_p65537", "g=3, p=65537 (SGE)")]
gw2 = pw2 / len(keys)
for ki, (k, lab) in enumerate(keys):
    gx = x1 + ki * gw2
    for wi, w in enumerate((0.0, 0.1, 1.0)):
        e = pj[f"{k}_w{w}"]
        xx = gx + 22 + wi * 38
        ya, yb = ly(e["ic_over_rho"]), ly(e["sib_over_rho"])
        s.append(f'<line x1="{xx:.1f}" y1="{ya:.1f}" x2="{xx:.1f}" y2="{yb:.1f}" stroke="{MUTED}" stroke-width="1.5"/>')
        s.append(f'<circle cx="{xx:.1f}" cy="{ya:.1f}" r="5" fill="{CA}"/>')
        s.append(f'<circle cx="{xx:.1f}" cy="{yb:.1f}" r="5" fill="none" stroke="{CB}" stroke-width="2.5"/>')
        s.append(f'<text x="{xx:.1f}" y="{y0+ph+14}" font-size="10" fill="{MUTED}" text-anchor="middle">w={w:g}</text>')
    s.append(f'<text x="{gx+gw2/2:.1f}" y="{y0+ph+31}" font-size="11" fill="{INK}" text-anchor="middle">{lab}</text>')
s.append(f'<circle cx="{x1+12}" cy="{y0+ph+50}" r="5" fill="{CA}"/><text x="{x1+22}" y="{y0+ph+54}" font-size="11" fill="{INK}">lab 2LP arm (recorded rows, gae_w)</text>')
s.append(f'<circle cx="{x1+212}" cy="{y0+ph+50}" r="5" fill="none" stroke="{CB}" stroke-width="2.5"/><text x="{x1+222}" y="{y0+ph+54}" font-size="11" fill="{INK}">+ siblings (derived projection, not a solve)</text>')
s.append(f'<text x="20" y="{H-26}" font-size="10.5" fill="{MUTED}">Python port of genus_ladder Cantor with identical ticks. Lab rows: genus_ladder_wallclock_vs_rho_20261122, genus_ladder_scale_20261115.</text>')
s.append(f'<text x="20" y="{H-11}" font-size="10.5" fill="{MUTED}">gae_w = group ops + w per native multiplication (relation phase, LA, rho canonicalisation), as genus_ladder_analyze; w = 0, 0.1, 1 GAE (AGENTS.md §5a).</text>')
s.append("</svg>")
open(os.path.join(HERE, "fig_sibling.svg"), "w").write("\n".join(s))
print("ok")
