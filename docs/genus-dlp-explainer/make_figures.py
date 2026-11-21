#!/usr/bin/env python3
"""Generate the SVG graphs for the genus >= 2 DLP explainer.

Pure Python, no plotting library: the values are closed-form exponents from
the cited papers, not measurements. Output: fig_exponents.svg beside this
script, plus fig_exponents.json with the plotted values.
"""
import json
import os
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))

# Exponent e such that the method costs q^{e+o(1)} on a genus-g Jacobian over F_q.
SERIES = [
    ("Pollard rho  g/2", lambda g: Fr(g, 2), "#111111", "4 2", 2),
    ("Gaudry 2000  2", lambda g: Fr(2), "#9a9a9a", "1 3", 2),
    ("reduced factor base  2-2/(g+1)", lambda g: 2 - Fr(2, g + 1), "#6a994e", "", 2),
    ("Theriault 2003  2-4/(2g+1)", lambda g: 2 - Fr(4, 2 * g + 1), "#bc6c25", "", 2),
    ("GTTD / Nagao 2007  2-2/g", lambda g: 2 - Fr(2, g), "#1d3557", "", 2),
    ("Diem 2006, non-hyperelliptic  2-2/(g-1)", lambda g: 2 - Fr(2, g - 1), "#9d0208", "", 3),
]
GENERA = list(range(2, 9))


def svg():
    W, H = 760, 460
    L, R, T, B = 70, 300, 30, 60  # margins (right margin holds the legend)
    pw, ph = W - L - R, H - T - B
    xmin, xmax, ymin, ymax = 2, 8, 0.5, 4.0

    def X(g):
        return L + (g - xmin) / (xmax - xmin) * pw

    def Y(e):
        return T + (ymax - float(e)) / (ymax - ymin) * ph

    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
           f'viewBox="0 0 {W} {H}" font-family="DejaVu Sans, Arial, sans-serif" font-size="12">',
           f'<rect width="{W}" height="{H}" fill="white"/>']
    # grid and axes
    for e in [0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4]:
        out.append(f'<line x1="{L}" y1="{Y(e):.1f}" x2="{L+pw}" y2="{Y(e):.1f}" stroke="#e6e6e6"/>')
        out.append(f'<text x="{L-8}" y="{Y(e)+4:.1f}" text-anchor="end">{e:g}</text>')
    for g in GENERA:
        out.append(f'<line x1="{X(g):.1f}" y1="{T}" x2="{X(g):.1f}" y2="{T+ph}" stroke="#e6e6e6"/>')
        out.append(f'<text x="{X(g):.1f}" y="{T+ph+18}" text-anchor="middle">{g}</text>')
    out.append(f'<rect x="{L}" y="{T}" width="{pw}" height="{ph}" fill="none" stroke="#333"/>')
    out.append(f'<text x="{L+pw/2:.1f}" y="{H-14}" text-anchor="middle">genus g of the Jacobian over F_q</text>')
    out.append(f'<text transform="translate(18,{T+ph/2:.1f}) rotate(-90)" text-anchor="middle">'
               f'exponent e : cost = q^(e+o(1))</text>')
    # shaded region where index calculus beats rho (below the rho line)
    pts = " ".join(f"{X(g):.1f},{Y(min(ymax, g/2)):.1f}" for g in GENERA)
    out.append(f'<polygon points="{X(2):.1f},{Y(ymin):.1f} {pts} {X(8):.1f},{Y(ymin):.1f}" '
               f'fill="#1d3557" fill-opacity="0.05"/>')
    out.append(f'<text x="{X(6.9):.1f}" y="{Y(1.0):.1f}" fill="#1d3557" font-size="11">'
               f'below the rho line: index calculus wins asymptotically</text>')
    data = {}
    ly = T + 14
    for name, f, color, dash, g0 in SERIES:
        gs = [g for g in GENERA if g >= g0]
        vals = [f(g) for g in gs]
        data[name] = {str(g): str(v) for g, v in zip(gs, vals)}
        pl = " ".join(f"{X(g):.1f},{Y(min(ymax, v)):.1f}" for g, v in zip(gs, vals))
        da = f' stroke-dasharray="{dash}"' if dash else ""
        out.append(f'<polyline points="{pl}" fill="none" stroke="{color}" stroke-width="2.2"{da}/>')
        for g, v in zip(gs, vals):
            if v <= ymax:
                out.append(f'<circle cx="{X(g):.1f}" cy="{Y(v):.1f}" r="3" fill="{color}"/>')
        lx = L + pw + 14
        out.append(f'<line x1="{lx}" y1="{ly}" x2="{lx+26}" y2="{ly}" stroke="{color}" stroke-width="2.2"{da}/>')
        out.append(f'<text x="{lx+32}" y="{ly+4}" font-size="11">{name}</text>')
        ly += 20
    # crossover annotations
    out.append(f'<circle cx="{X(2):.1f}" cy="{Y(1):.1f}" r="7" fill="none" stroke="#9d0208" stroke-width="1.5"/>')
    out.append(f'<text x="{X(2)+10:.1f}" y="{Y(1)-10:.1f}" font-size="11" fill="#9d0208">g = 2: GTTD ties rho at q^1</text>')
    out.append(f'<circle cx="{X(3):.1f}" cy="{Y(1):.1f}" r="7" fill="none" stroke="#9d0208" stroke-width="1.5"/>')
    out.append(f'<text x="{X(3)+10:.1f}" y="{Y(1)+16:.1f}" font-size="11" fill="#9d0208">g = 3 plane quartic: q^1 vs rho q^1.5</text>')
    out.append('</svg>')
    return "\n".join(out), data


if __name__ == "__main__":
    s, data = svg()
    with open(os.path.join(HERE, "fig_exponents.svg"), "w") as fh:
        fh.write(s)
    with open(os.path.join(HERE, "fig_exponents.json"), "w") as fh:
        json.dump({"unit": "exponent e with cost q^(e+o(1))", "series": data}, fh, indent=1)
    print("wrote fig_exponents.svg / .json")
