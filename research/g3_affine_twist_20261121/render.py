#!/usr/bin/env python3
"""SVG figure from data/summary.json (layout only). Usage: python3 -I render.py"""
import json
import os

D = os.path.dirname(os.path.abspath(__file__))
s = json.load(open(os.path.join(D, "data", "summary.json")))
W, H = 860, 360
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="12">',
       '<rect width="100%" height="100%" fill="white"/>']


def panel(x0, title, xs, ys, lo, hi, xlab, ref=None, ticks=None):
    pw, ph, y0 = 360, 230, 60
    out.append(f'<text x="{x0}" y="30" font-size="14" font-weight="bold">{title}</text>')
    out.append(f'<rect x="{x0}" y="{y0}" width="{pw}" height="{ph}" fill="none" stroke="#999"/>')
    Y = lambda v: y0 + ph - v * ph
    for v in (0, 0.25, 0.5, 0.75, 1):
        out.append(f'<line x1="{x0}" x2="{x0 + pw}" y1="{Y(v)}" y2="{Y(v)}" stroke="#eee"/>')
        out.append(f'<text x="{x0 - 30}" y="{Y(v) + 4}">{v:.2f}</text>')
    out.append(f'<line x1="{x0}" x2="{x0 + pw}" y1="{Y(0.5)}" y2="{Y(0.5)}" stroke="#888" stroke-dasharray="4 3"/>')
    n = len(xs)
    X = lambda i: x0 + (i + 0.5) * pw / n
    if ref:
        pts = " ".join(f"{X(i)},{Y(r)}" for i, r in enumerate(ref))
        out.append(f'<polyline points="{pts}" fill="none" stroke="#d62728" stroke-dasharray="2 2"/>')
    for i, (xl, y, l, h) in enumerate(zip(xs, ys, lo, hi)):
        out.append(f'<line x1="{X(i)}" x2="{X(i)}" y1="{Y(l)}" y2="{Y(h)}" stroke="#1f77b4" stroke-width="2"/>')
        out.append(f'<circle cx="{X(i)}" cy="{Y(y)}" r="4" fill="#1f77b4"/>')
        out.append(f'<text x="{X(i) - 12}" y="{y0 + ph + 16}">{xl}</text>')
    out.append(f'<text x="{x0 + 100}" y="{y0 + ph + 36}">{xlab}</text>')


pp = s["main"]["per_prime"]
ks = sorted(pp, key=int)
panel(60, "Share of affine codomains that are Jacobians over F_p", ks,
      [pp[k]["r"] for k in ks], [pp[k]["wilson95"][0] for k in ks], [pp[k]["wilson95"][1] for k in ks], "p (Wilson 95%)")
bins = s["main"]["bins_N1_over_2p2"]
bk = [k for k in sorted(bins, key=int) if bins[k][1] >= 10]
import math


def wil(x, n, z=1.96):
    ph = x / n
    d = 1 + z * z / n
    c = (ph + z * z / (2 * n)) / d
    h = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n)) / d
    return c - h, c + h


panel(480, "Jacobian share vs N1/(2p+2) (bins with n >= 10)", [f"{int(k) / 10:.1f}" for k in bk],
      [bins[k][2] for k in bk], [wil(bins[k][0], bins[k][1])[0] for k in bk], [wil(bins[k][0], bins[k][1])[1] for k in bk],
      "bin of N1/(2p+2); red: N1/(2p+2) (pair-type law)", ref=[bins[k][3] for k in bk])
out.append('</svg>')
open(os.path.join(D, "fig_twist.svg"), "w").write("\n".join(out))
print("wrote fig_twist.svg")
