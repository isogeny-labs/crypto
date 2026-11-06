#!/usr/bin/env python3
"""Draw figures/impact.svg from the committed compare/*.json (layout only).

Every number plotted is a ratio and interval `ecbench compare` wrote; this
script computes nothing.  Run: python3 -I figures/plot.py
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CMP = os.path.join(HERE, "..", "compare")

AUT = [  # (file stem, label, theory)
    ("prime_rho_automorphism.quartic.rho-quartic__rho-neg", "prime-aut quartic (j=1728)", 0.7071),
    ("prime_rho_automorphism.cubic.rho-cubic__rho-neg", "prime-aut cubic (j=0)", 0.5774),
    ("isogeny_volcano_cost.j1728.rho-quartic__rho-neg-crater", "iso-cost j=1728 crater", 0.7071),
    ("isogeny_volcano_cost.j0.rho-cubic__rho-neg-crater", "iso-cost j=0 crater", 0.5774),
    ("volcano_ecdlp_census.V3-D-L0-c0.rho-cubic__rho-neg", "census V3 crater D", 0.5774),
    ("volcano_ecdlp_census.V3-N-L0-c0.rho-cubic__rho-neg", "census V3 crater N", 0.5774),
    ("volcano_ecdlp_census.V3-ctl-L0-c0.rho-cubic__rho-neg", "census V3 crater ctl", 0.5774),
    ("volcano_ecdlp_census_followup.V5-D-L0-c0.rho-cubic__rho-neg", "follow-up V5 crater D", 0.5774),
    ("volcano_ecdlp_census_followup.V5-N-L0-c0.rho-cubic__rho-neg", "follow-up V5 crater N", 0.5774),
    ("volcano_ecdlp_census_followup.V5-ctl-L0-c0.rho-cubic__rho-neg", "follow-up V5 crater ctl", 0.5774),
]
IC = [
    ("volcano_ecdlp_census.V3-D-L0-c0.ic__rho-cubic", "census V3 crater D"),
    ("volcano_ecdlp_census.V3-N-L0-c0.ic__rho-cubic", "census V3 crater N"),
    ("volcano_ecdlp_census.V3-ctl-L0-c0.ic__rho-cubic", "census V3 crater ctl"),
    ("volcano_ecdlp_census_followup.V5-D-L0-c0.ic__rho-cubic", "follow-up V5 crater D"),
    ("volcano_ecdlp_census_followup.V5-N-L0-c0.ic__rho-cubic", "follow-up V5 crater N"),
    ("volcano_ecdlp_census_followup.V5-ctl-L0-c0.ic__rho-cubic", "follow-up V5 crater ctl"),
]

INK, MUTED, GRID = "#1f2328", "#59636e", "#d8dee4"
REC, COR, THEORY = "#8c959f", "#0969da", "#cf222e"


def ops(stem, variant):
    with open(os.path.join(CMP, f"{stem}.{variant}.json")) as f:
        o = json.load(f)["ops"]
    return o["ratio_b_over_a"], o["ci95"][0], o["ci95"][1], o["pairs"]


def panel(out, x0, y0, w, rows, lo, hi, ticks, title, theory=None):
    label_w, row_h = 190, 26
    px = lambda v: x0 + label_w + (v - lo) / (hi - lo) * (w - label_w - 10)
    h = row_h * len(rows) + 40
    out.append(f'<text x="{x0}" y="{y0}" font-size="13" font-weight="600" fill="{INK}">{title}</text>')
    top = y0 + 14
    for t in ticks:
        x = px(t)
        out.append(f'<line x1="{x:.1f}" y1="{top}" x2="{x:.1f}" y2="{top + row_h * len(rows)}" stroke="{GRID}"/>')
        out.append(f'<text x="{x:.1f}" y="{top + row_h * len(rows) + 14}" font-size="10" fill="{MUTED}" text-anchor="middle">{t:g}</text>')
    for i, row in enumerate(rows):
        stem, label = row[0], row[1]
        cy = top + row_h * i + row_h / 2
        out.append(f'<text x="{x0}" y="{cy + 4:.1f}" font-size="11" fill="{INK}">{label}</text>')
        if theory:
            x = px(row[2])
            out.append(f'<line x1="{x:.1f}" y1="{cy - 9:.1f}" x2="{x:.1f}" y2="{cy + 9:.1f}" stroke="{THEORY}" stroke-width="2"/>')
        for dy, variant, color, fill in ((-4, "recorded", REC, "#ffffff"), (4, "corrected", COR, COR)):
            r, a, b, _ = ops(stem, variant)
            y = cy + dy
            out.append(f'<line x1="{px(a):.1f}" y1="{y:.1f}" x2="{px(b):.1f}" y2="{y:.1f}" stroke="{color}" stroke-width="2"/>')
            out.append(f'<circle cx="{px(r):.1f}" cy="{y:.1f}" r="3.5" fill="{fill}" stroke="{color}" stroke-width="1.5"/>')
    return h


def main():
    W = 760
    out = []
    y = 72
    y += panel(out, 16, y, W - 32, AUT, 0.5, 1.0, [0.5, 0.6, 0.7, 0.8, 0.9, 1.0],
               "A. Automorphism rho / rho.negation: mean-S ratio, 95% interval (red: asymptotic theory)", theory=True)
    y += 30
    y += panel(out, 16, y, W - 32, IC, 75, 110, [75, 80, 85, 90, 95, 100, 105, 110],
               "B. IC / matched rho.cubic: mean-S ratio at 0 GAE, 95% interval (distance remaining)")
    H = y + 40
    head = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica, Arial, sans-serif">',
        f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
        f'<text x="16" y="22" font-size="15" font-weight="700" fill="{INK}">Class set-up undercharge: recorded vs corrected ratios</text>',
        f'<circle cx="22" cy="34" r="3.5" fill="#ffffff" stroke="{REC}" stroke-width="1.5"/><text x="30" y="38" font-size="11" fill="{MUTED}">as recorded (pre-2026-10-18 binary)</text>',
        f'<circle cx="252" cy="34" r="3.5" fill="{COR}" stroke="{COR}" stroke-width="1.5"/><text x="260" y="38" font-size="11" fill="{MUTED}">class set-up charged in the total</text>',
    ]
    foot = (f'<text x="16" y="{H - 14}" font-size="10" fill="{MUTED}">ecbench compare (ops, linearised-t interval) on each committed session and on a corrected scratch copy; '
            f'data: research/class_setup_impact_20261024/compare/</text>')
    with open(os.path.join(HERE, "impact.svg"), "w") as f:
        f.write("\n".join(head + out + [foot, "</svg>"]) + "\n")


main()
