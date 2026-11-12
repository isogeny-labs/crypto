#!/usr/bin/env python3
"""census.svg from census.json: fraction with |d_K| <= 44 vs bit size (log y)."""
import json, math, sys
j = json.load(open("census.json"))
W, H, L, R, T, B = 760, 470, 70, 30, 50, 70
pops = {p["name"]: p for p in j["populations"]}
bits = sorted(int(n.split(":")[2]) for n in pops if n.startswith("c:null:"))
ymin, ymax = 1e-8, 2.0
def X(b): return L + (b - 10) / (42 - 10) * (W - L - R)
def Y(v): return T + (math.log10(ymax) - math.log10(max(v, ymin))) / (math.log10(ymax) - math.log10(ymin)) * (H - T - B)
o = []
o.append(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="sans-serif" font-size="12">')
o.append('<style>:root{--fg:#1a1a1a;--bg:#fff;--grid:#ddd;--obs:#1f6feb;--pred:#d1242f;--c:#2da44e}@media (prefers-color-scheme:dark){:root{--fg:#e6e6e6;--bg:#111;--grid:#333;--obs:#58a6ff;--pred:#ff7b72;--c:#56d364}}text{fill:var(--fg)}</style>')
o.append(f'<rect width="{W}" height="{H}" fill="var(--bg)"/>')
o.append(f'<text x="{L}" y="22" font-size="14" font-weight="bold">Fraction of random curves with |d_K| &lt;= 44 (endomorphism of degree &lt;= 11 in the class)</text>')
o.append(f'<text x="{L}" y="40" fill="currentColor">seed {j["seed"]}; unverified assertions, see CENSUS.md</text>')
for e in range(-8, 1):
    y = Y(10.0 ** e)
    o.append(f'<line x1="{L}" x2="{W-R}" y1="{y:.1f}" y2="{y:.1f}" stroke="var(--grid)"/><text x="{L-8}" y="{y+4:.1f}" text-anchor="end">1e{e}</text>')
for b in range(12, 41, 4):
    o.append(f'<text x="{X(b):.1f}" y="{H-B+18}" text-anchor="middle">{b}</text>')
o.append(f'<text x="{(L+W-R)/2}" y="{H-B+38}" text-anchor="middle">bit size of p</text>')
# C/sqrt(p) line, C = 4, p = 1.5*2^(b-1)
pts = " ".join(f"{X(b):.1f},{Y(4/math.sqrt(1.5*2**(b-1))):.1f}" for b in range(12, 41))
o.append(f'<polyline points="{pts}" fill="none" stroke="var(--pred)" stroke-dasharray="5 4"/>')
o.append(f'<text x="{X(30)}" y="{Y(4/math.sqrt(1.5*2**29))-10:.1f}" fill="var(--pred)" style="fill:var(--pred)">C/sqrt(p), C = 4</text>')
# exact predicted + observed
pp, op = [], []
for b in bits:
    r = pops[f"c:null:{b}"]
    pp.append(f"{X(b):.1f},{Y(r['expected_frac44']):.1f}")
    lo, hi = r["wilson95"]
    k = r["hit44"]
    x = X(b)
    if k > 0:
        o.append(f'<line x1="{x:.1f}" x2="{x:.1f}" y1="{Y(lo):.1f}" y2="{Y(hi):.1f}" stroke="var(--obs)" stroke-width="2"/>')
        o.append(f'<circle cx="{x:.1f}" cy="{Y(r["frac44"]):.1f}" r="4" fill="var(--obs)"/>')
    else:
        o.append(f'<path d="M{x:.1f},{Y(hi):.1f} l-5,-8 h10 z" fill="none" stroke="var(--obs)" stroke-width="2"/>')
    o.append(f'<text x="{x+7:.1f}" y="{Y(hi)-6:.1f}" font-size="10">{k}/{r["ordinary"]}</text>')
o.append(f'<polyline points="{" ".join(pp)}" fill="none" stroke="var(--pred)" stroke-width="2"/>')
for b in bits:
    r = pops[f"c:null:{b}"]
    o.append(f'<circle cx="{X(b):.1f}" cy="{Y(r["expected_frac44"]):.1f}" r="3" fill="var(--pred)"/>')
# controls at y = 1
ctl = {}
for c in j["controls"]:
    k = (c["bits"],)
    d = ctl.setdefault(c["bits"], [0, 0])
    d[0] += c["detected"]; d[1] += c["n"] - c["count_failed"]
for b, (k, n) in sorted(ctl.items()):
    o.append(f'<rect x="{X(b)-5:.1f}" y="{Y(1.0)-5:.1f}" width="10" height="10" fill="var(--c)"/>')
    o.append(f'<text x="{X(b)+9:.1f}" y="{Y(1.0)+4:.1f}" font-size="10">{k}/{n}</text>')
lx = L + 8; ly = H - 14
o.append(f'<circle cx="{lx}" cy="{ly-4}" r="4" fill="var(--obs)"/><text x="{lx+9}" y="{ly}">observed, Wilson 95% (hollow triangle: 0 hits, upper bound)</text>')
o.append(f'<line x1="{lx+395}" x2="{lx+420}" y1="{ly-4}" y2="{ly-4}" stroke="var(--pred)" stroke-width="2"/><text x="{lx+426}" y="{ly}">exact expected</text>')
o.append(f'<rect x="{lx+520}" y="{ly-9}" width="10" height="10" fill="var(--c)"/><text x="{lx+534}" y="{ly}">CM controls detected</text>')
o.append('</svg>')
open("census.svg", "w").write("\n".join(o))
print("wrote census.svg")
