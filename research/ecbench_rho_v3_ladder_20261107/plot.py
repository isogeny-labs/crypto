#!/usr/bin/env python3
"""Lay out recorded values as SVG. Computes nothing beyond dividing recorded
means and interval ends by the recorded `declared_s` (S / theory).

Usage: plot.py OUT.svg TITLE --bound LABEL=bound.json ... [--compare LABEL=comparison.json ...]

Panel 1: per-size mean S / declared S with the bound record's 95 % interval.
Panel 2 (when --compare is given): per-curve paired ratio with its 95 % interval.
"""
import json
import sys

COLORS = ["#2a6fdb", "#d1495b", "#00798c", "#edae49", "#66a182", "#8d6a9f"]


def parse(argv):
    out, title = argv[1], argv[2]
    bounds, comps, mode = [], [], None
    for a in argv[3:]:
        if a in ("--bound", "--compare"):
            mode = a
            continue
        label, path = a.rsplit("=", 1)
        (bounds if mode == "--bound" else comps).append((label, json.load(open(path))))
    return out, title, bounds, comps


def axes(x0, y0, w, h, xs, ys, xlabel, ylabel, ref=None):
    xmin, xmax = min(xs), max(xs)
    ymin, ymax = min(ys), max(ys)
    pad = (ymax - ymin) * 0.08 or 0.05
    ymin, ymax = ymin - pad, ymax + pad
    sx = lambda x: x0 + (x - xmin) / ((xmax - xmin) or 1) * w
    sy = lambda y: y0 + h - (y - ymin) / ((ymax - ymin) or 1) * h
    g = [f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="none" stroke="#999"/>']
    for i in range(6):
        yv = ymin + (ymax - ymin) * i / 5
        g.append(f'<line x1="{x0}" x2="{x0 + w}" y1="{sy(yv):.1f}" y2="{sy(yv):.1f}" stroke="#eee"/>')
        g.append(f'<text x="{x0 - 6}" y="{sy(yv) + 4:.1f}" text-anchor="end" font-size="11">{yv:.2f}</text>')
    for xv in sorted(set(round(x, 1) for x in xs)):
        g.append(f'<text x="{sx(xv):.1f}" y="{y0 + h + 16}" text-anchor="middle" font-size="11">{xv:.1f}</text>')
    if ref is not None and ymin <= ref <= ymax:
        g.append(f'<line x1="{x0}" x2="{x0 + w}" y1="{sy(ref):.1f}" y2="{sy(ref):.1f}" stroke="#333" stroke-dasharray="5,4"/>')
    g.append(f'<text x="{x0 + w / 2}" y="{y0 + h + 34}" text-anchor="middle" font-size="12">{xlabel}</text>')
    g.append(f'<text x="{x0 - 46}" y="{y0 + h / 2}" text-anchor="middle" font-size="12" transform="rotate(-90 {x0 - 46} {y0 + h / 2})">{ylabel}</text>')
    return g, sx, sy


def main():
    ylog = "--ylog" in sys.argv
    sys.argv = [a for a in sys.argv if a != "--ylog"]
    out, title, bounds, comps = parse(sys.argv)
    W = 760
    H = 360 if not comps else 700
    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif">',
           f'<rect width="{W}" height="{H}" fill="white"/>',
           f'<text x="{W / 2}" y="22" text-anchor="middle" font-size="15">{title}</text>']
    pts = []
    for label, b in bounds:
        for s in b["sizes"]:
            d = s.get("declared_s") or s["floor_s"]
            v = (s["mean_s"] / d, s["s_ci95"][0] / d, s["s_ci95"][1] / d)
            if ylog:
                import math
                v = tuple(math.log10(x) for x in v)
            pts.append((label, s["log2_r"]) + v)
    xs = [p[1] for p in pts]
    ref = 0.0 if ylog else 1.0
    ys = [p[3] for p in pts] + [p[4] for p in pts] + [ref]
    ylabel = "log10(S / floor or theory)" if ylog else "S / theory (recorded)"
    g, sx, sy = axes(80, 40, 520, 250, xs, ys, "log2 r", ylabel, ref=ref)
    svg += g
    for i, (label, _) in enumerate(bounds):
        c = COLORS[i % len(COLORS)]
        row = sorted((p for p in pts if p[0] == label), key=lambda p: p[1])
        off = (i - (len(bounds) - 1) / 2) * 0.12
        path = " ".join(f"{sx(p[1]) + off * 20:.1f},{sy(p[2]):.1f}" for p in row)
        svg.append(f'<polyline points="{path}" fill="none" stroke="{c}" stroke-width="1.5"/>')
        for p in row:
            x = sx(p[1]) + off * 20
            svg.append(f'<line x1="{x:.1f}" x2="{x:.1f}" y1="{sy(p[3]):.1f}" y2="{sy(p[4]):.1f}" stroke="{c}"/>')
            svg.append(f'<circle cx="{x:.1f}" cy="{sy(p[2]):.1f}" r="3.5" fill="{c}"/>')
        svg.append(f'<rect x="615" y="{50 + i * 20}" width="12" height="12" fill="{c}"/>')
        svg.append(f'<text x="632" y="{60 + i * 20}" font-size="12">{label}</text>')
    svg.append('<text x="615" y="{}" font-size="11" fill="#333">dashed: {}</text>'.format(60 + len(bounds) * 20, "floor (log 0)" if ylog else "theory (1.0)"))
    if comps:
        cp = []
        for label, c in comps:
            for cv in c["curves"]:
                cp.append((label, cv["log2_r"], cv["ratio_b_over_a"], cv["ci95"][0], cv["ci95"][1]))
        xs = [p[1] for p in cp]
        ys = [p[3] for p in cp] + [p[4] for p in cp] + [1.0]
        g, sx, sy = axes(80, 390, 520, 250, xs, ys, "log2 r", "paired ratio B / A (recorded)", ref=1.0)
        svg += g
        for i, (label, _) in enumerate(comps):
            c = COLORS[(i + 3) % len(COLORS)]
            off = (i - (len(comps) - 1) / 2) * 0.12
            for p in (q for q in cp if q[0] == label):
                x = sx(p[1]) + off * 20
                svg.append(f'<line x1="{x:.1f}" x2="{x:.1f}" y1="{sy(p[3]):.1f}" y2="{sy(p[4]):.1f}" stroke="{c}"/>')
                svg.append(f'<rect x="{x - 3:.1f}" y="{sy(p[2]) - 3:.1f}" width="6" height="6" fill="{c}"/>')
            svg.append(f'<rect x="615" y="{400 + i * 20}" width="12" height="12" fill="{c}"/>')
            svg.append(f'<text x="632" y="{410 + i * 20}" font-size="12">{label}</text>')
    svg.append("</svg>")
    open(out, "w").write("\n".join(svg))


if __name__ == "__main__":
    main()
