#!/usr/bin/env python3
"""Render the frozen genus-2 two-choice operation-cost grid as SVG and PNG.

This script only lays out recorded ratios from results/genus2-choice2-price-grid.csv.
It does not run solvers, estimate wall time, or calculate a new performance claim.
"""
from __future__ import annotations

import csv
import html
import math
import struct
import zlib
from pathlib import Path

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
GRID = STUDY / "results" / "genus2-choice2-price-grid.csv"
WIDTH, HEIGHT = 1200, 760  # 8.0 x 5.067 inches at 150 dpi
INK = "#18212b"
GRID_COLOR = "#d8dee6"
TEAL = "#087f72"
BLUE = "#315cbd"
POOLED = "#596579"
WHITE = "#ffffff"

# Five-by-seven bitmap alphabet used by the PNG renderer. The SVG and PDF use
# normal text; the same labels, points, scales and colors come from this source.
FONT = {
    "A": ("01110", "10001", "10001", "11111", "10001", "10001", "10001"),
    "B": ("11110", "10001", "10001", "11110", "10001", "10001", "11110"),
    "C": ("01111", "10000", "10000", "10000", "10000", "10000", "01111"),
    "D": ("11110", "10001", "10001", "10001", "10001", "10001", "11110"),
    "E": ("11111", "10000", "10000", "11110", "10000", "10000", "11111"),
    "F": ("11111", "10000", "10000", "11110", "10000", "10000", "10000"),
    "G": ("01111", "10000", "10000", "10111", "10001", "10001", "01111"),
    "H": ("10001", "10001", "10001", "11111", "10001", "10001", "10001"),
    "I": ("11111", "00100", "00100", "00100", "00100", "00100", "11111"),
    "J": ("00111", "00010", "00010", "00010", "10010", "10010", "01100"),
    "K": ("10001", "10010", "10100", "11000", "10100", "10010", "10001"),
    "L": ("10000", "10000", "10000", "10000", "10000", "10000", "11111"),
    "M": ("10001", "11011", "10101", "10101", "10001", "10001", "10001"),
    "N": ("10001", "11001", "10101", "10011", "10001", "10001", "10001"),
    "O": ("01110", "10001", "10001", "10001", "10001", "10001", "01110"),
    "P": ("11110", "10001", "10001", "11110", "10000", "10000", "10000"),
    "Q": ("01110", "10001", "10001", "10001", "10101", "10010", "01101"),
    "R": ("11110", "10001", "10001", "11110", "10100", "10010", "10001"),
    "S": ("01111", "10000", "10000", "01110", "00001", "00001", "11110"),
    "T": ("11111", "00100", "00100", "00100", "00100", "00100", "00100"),
    "U": ("10001", "10001", "10001", "10001", "10001", "10001", "01110"),
    "V": ("10001", "10001", "10001", "10001", "10001", "01010", "00100"),
    "W": ("10001", "10001", "10001", "10101", "10101", "10101", "01010"),
    "X": ("10001", "10001", "01010", "00100", "01010", "10001", "10001"),
    "Y": ("10001", "10001", "01010", "00100", "00100", "00100", "00100"),
    "Z": ("11111", "00001", "00010", "00100", "01000", "10000", "11111"),
    "0": ("01110", "10001", "10011", "10101", "11001", "10001", "01110"),
    "1": ("00100", "01100", "00100", "00100", "00100", "00100", "01110"),
    "2": ("01110", "10001", "00001", "00010", "00100", "01000", "11111"),
    "3": ("11110", "00001", "00001", "01110", "00001", "00001", "11110"),
    "4": ("00010", "00110", "01010", "10010", "11111", "00010", "00010"),
    "5": ("11111", "10000", "10000", "11110", "00001", "00001", "11110"),
    "6": ("01110", "10000", "10000", "11110", "10001", "10001", "01110"),
    "7": ("11111", "00001", "00010", "00100", "01000", "01000", "01000"),
    "8": ("01110", "10001", "10001", "01110", "10001", "10001", "01110"),
    "9": ("01110", "10001", "10001", "01111", "00001", "00001", "01110"),
    " ": ("00000",) * 7,
    ".": ("00000", "00000", "00000", "00000", "00000", "00110", "00110"),
    ",": ("00000", "00000", "00000", "00000", "00110", "00110", "00100"),
    ":": ("00000", "00110", "00110", "00000", "00110", "00110", "00000"),
    ";": ("00000", "00110", "00110", "00000", "00110", "00110", "00100"),
    "-": ("00000", "00000", "00000", "11111", "00000", "00000", "00000"),
    "/": ("00001", "00010", "00010", "00100", "01000", "01000", "10000"),
    "(": ("00010", "00100", "01000", "01000", "01000", "00100", "00010"),
    ")": ("01000", "00100", "00010", "00010", "00010", "00100", "01000"),
    "=": ("00000", "11111", "00000", "11111", "00000", "00000", "00000"),
    "+": ("00000", "00100", "00100", "11111", "00100", "00100", "00000"),
}


def read_points():
    rows = list(csv.DictReader(GRID.open(newline="", encoding="utf-8")))
    want = {"S_per_M": "0.5", "I_per_M": "10", "native_GAE_per_logical_operation": "0.1"}
    rows = [r for r in rows if all(r[k] == v for k, v in want.items())]
    sizes = {int(r["label"]): r for r in rows if r["kind"] == "bits"}
    pooled = next(r for r in rows if r["kind"] == "all")
    assert sorted(sizes) == [16, 20, 24, 28, 32, 36, 40]
    assert all(int(r["targets"]) == 180 for r in sizes.values())
    assert int(pooled["targets"]) == 1260
    return sizes, pooled


def f(v):
    return f"{v:.3f}"


def chart(sizes, pooled):
    # Figure elements are retained as a common source for SVG and PNG.
    shapes = []
    def line(x1, y1, x2, y2, color=INK, width=2, dash=False):
        shapes.append(("line", x1, y1, x2, y2, color, width, dash))
    def circle(x, y, r, color):
        shapes.append(("circle", x, y, r, color))
    def text(x, y, value, size=18, color=INK, anchor="start"):
        shapes.append(("text", x, y, value, size, color, anchor))

    text(WIDTH / 2, 42, "CHOICE2 OPERATION-COST RATIOS", 30, INK, "middle")
    text(WIDTH / 2, 75, "FIXED PRICE: M + 0.5S + 10I; NATIVE LOGICAL OP = 0.1M", 17, INK, "middle")
    panel_specs = [
        (90, 130, 475, 420, "CHOICE2 / INCUMBENT", "choice_over_incumbent", 0.90, 1.08, 0.05,
         float(pooled["choice_over_incumbent"]), TEAL),
        (635, 130, 475, 420, "CHOICE2 / RHO-NEG", "choice_over_rho_neg", 3.5, 5.0, 0.5,
         float(pooled["choice_over_rho_neg"]), BLUE),
    ]
    bits = sorted(sizes)
    for x0, y0, pw, ph, title, key, ymin, ymax, step, pooled_value, color in panel_specs:
        text(x0 + pw / 2, y0 - 25, title, 20, INK, "middle")
        # Center the scale label above the tick labels so it cannot collide
        # with the top tick at the left of either panel.
        text(x0 + pw / 2, y0 + 8, "RATIO", 12, INK, "middle")
        left, top, w, h = x0 + 62, y0 + 23, pw - 80, ph - 80
        ticks = int(round((ymax - ymin) / step))
        for j in range(ticks + 1):
            val = ymin + j * step
            yy = top + h - (val - ymin) / (ymax - ymin) * h
            line(left, yy, left + w, yy, GRID_COLOR, 1)
            text(left - 8, yy + 5, f(val), 13, INK, "end")
        line(left, top, left, top + h, INK, 2)
        line(left, top + h, left + w, top + h, INK, 2)
        px = [left + i * w / (len(bits) - 1) for i in range(len(bits))]
        for xx, b in zip(px, bits):
            text(xx, top + h + 25, str(b), 14, INK, "middle")
        text(left + w / 2, top + h + 52, "SOURCE ORDER BITS", 13, INK, "middle")
        py = [top + h - (float(sizes[b][key]) - ymin) / (ymax - ymin) * h for b in bits]
        for i in range(len(px) - 1):
            line(px[i], py[i], px[i + 1], py[i + 1], color, 4)
        for xx, yy in zip(px, py):
            circle(xx, yy, 7, color)
        yp = top + h - (pooled_value - ymin) / (ymax - ymin) * h
        # Dashed reference to the ratio pooled over all 1,260 paired targets.
        dash_len, gap, xx = 10, 7, left
        while xx < left + w:
            line(xx, yp, min(xx + dash_len, left + w), yp, POOLED, 2)
            xx += dash_len + gap
        text(left + w - 4, yp - 8, f"POOL {f(pooled_value)}", 12, POOLED, "end")
    text(WIDTH / 2, 685, "180 PAIRED TARGETS PER SIZE; 1,260 POOLED; NO WALL-TIME MEASUREMENT", 16, INK, "middle")
    text(WIDTH / 2, 720, "RATIO ABOVE 1.0 MEANS HIGHER OPERATION COST; VALUES ARE DETERMINISTIC COUNTS", 13, INK, "middle")
    return shapes


def to_svg(shapes):
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">',
           '<title>Five-lead two-choice operation-cost ratios</title>',
           '<desc>Fixed-price ratios at seven source-order sizes and the pooled 1260-target cohort. Deterministic operation counts, not wall timings.</desc>',
           f'<rect width="{WIDTH}" height="{HEIGHT}" fill="{WHITE}"/>']
    for s in shapes:
        if s[0] == "line":
            _, x1, y1, x2, y2, color, width, dash = s
            extra = ' stroke-dasharray="7 5"' if dash else ""
            out.append(f'<line x1="{x1:.2f}" y1="{y1:.2f}" x2="{x2:.2f}" y2="{y2:.2f}" stroke="{color}" stroke-width="{width}"{extra}/>')
        elif s[0] == "circle":
            _, x, y, r, color = s
            out.append(f'<circle cx="{x:.2f}" cy="{y:.2f}" r="{r}" fill="{color}"/>')
        else:
            _, x, y, value, size, color, anchor = s
            out.append(f'<text x="{x:.2f}" y="{y:.2f}" text-anchor="{anchor}" font-family="Arial, sans-serif" font-size="{size}" fill="{color}">{html.escape(value)}</text>')
    out.append("</svg>")
    return "\n".join(out) + "\n"


def hexrgb(color):
    color = color.lstrip("#")
    return tuple(int(color[i:i + 2], 16) for i in (0, 2, 4))


def draw_disc(buf, x, y, r, color):
    rgb = hexrgb(color)
    for yy in range(max(0, y - r), min(HEIGHT, y + r + 1)):
        for xx in range(max(0, x - r), min(WIDTH, x + r + 1)):
            if (xx - x) ** 2 + (yy - y) ** 2 <= r * r:
                i = (yy * WIDTH + xx) * 3
                buf[i:i + 3] = bytes(rgb)


def draw_segment(buf, x1, y1, x2, y2, color, thickness=1):
    rgb = hexrgb(color)
    dx, dy = abs(x2 - x1), abs(y2 - y1)
    sx = 1 if x1 < x2 else -1
    sy = 1 if y1 < y2 else -1
    err = dx - dy
    while True:
        draw_disc(buf, x1, y1, max(0, thickness // 2), color)
        if x1 == x2 and y1 == y2:
            break
        e2 = 2 * err
        if e2 > -dy:
            err -= dy
            x1 += sx
        if e2 < dx:
            err += dx
            y1 += sy


def draw_text(buf, x, baseline, value, font_size, color, anchor):
    scale = max(1, round(font_size / 7))
    value = value.upper()
    width = max(0, len(value) * 6 * scale - scale)
    if anchor == "middle":
        x -= width // 2
    elif anchor == "end":
        x -= width
    y0 = baseline - 7 * scale
    for ci, ch in enumerate(value):
        glyph = FONT.get(ch, FONT[" "])
        gx = x + ci * 6 * scale
        for row, bits in enumerate(glyph):
            for col, bit in enumerate(bits):
                if bit == "1":
                    draw_disc(buf, gx + col * scale, y0 + row * scale, max(0, scale // 2), color)


def to_png(shapes):
    buf = bytearray(bytes((255, 255, 255)) * WIDTH * HEIGHT)
    for s in shapes:
        if s[0] == "line":
            _, x1, y1, x2, y2, color, width, dash = s
            dx, dy = x2 - x1, y2 - y1
            length = max(1, math.hypot(dx, dy))
            if dash:
                unit = 9
                n = int(length // unit) + 1
                for i in range(n):
                    if i % 2:
                        continue
                    a, b = i * unit / length, min((i * unit + 5) / length, 1)
                    if a <= 1:
                        draw_segment(buf, round(x1 + dx * a), round(y1 + dy * a), round(x1 + dx * b), round(y1 + dy * b), color, int(width))
            else:
                draw_segment(buf, round(x1), round(y1), round(x2), round(y2), color, int(width))
        elif s[0] == "circle":
            _, x, y, r, color = s
            draw_disc(buf, round(x), round(y), int(r), color)
        else:
            _, x, y, value, size, color, anchor = s
            draw_text(buf, round(x), round(y), value, size, color, anchor)
    raw = b"".join(b"\x00" + bytes(buf[y * WIDTH * 3:(y + 1) * WIDTH * 3]) for y in range(HEIGHT))
    def chunk(tag, data):
        return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data) & 0xffffffff)
    # PNG pHYs: 5906 pixels per metre is 150 dpi.
    png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", WIDTH, HEIGHT, 8, 2, 0, 0, 0))
    png += chunk(b"pHYs", struct.pack(">IIB", 5906, 5906, 1)) + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b"")
    return png


def report_tex(sizes, pooled):
    lines = [
        r"\documentclass{article}",
        r"\usepackage{graphicx}",
        r"\title{Five-lead genus-2 collector: operation-cost evidence}",
        r"\author{Crypto research evidence package}",
        r"\date{2026-10-19}",
        r"\begin{document}",
        r"\maketitle",
        r"\section*{Scope and method}",
        "This report summarizes the frozen two-choice genus-2 relation-collector cohort in the five-lead study. It contains 21 exact-order curves, seven source-order sizes (16 through 40 bits), 60 paired one-target workloads per curve, and 1,260 targets in total. All six arms across the cohort are source-verified and independently replayed (6,300 records); see the study replay-audit summary and packed session archive.",
        "",
        r"The graph uses deterministic operation-count costs from the frozen price grid at $M+0.5S+10I$, plus a $0.1M$ charge per declared native logical operation. Ratios are sums of paired costs, not averages of curve ratios. Ratios above 1 mean the two-choice candidate incurred higher operation cost. Each size point pools 180 paired targets. The pooled reference uses all 1,260 targets.",
        "",
        r"\section*{Observed ratios}",
        r"\begin{center}",
        r"\begin{tabular}{r|r|r|r}",
        r"Source order bits & Paired targets & Choice2 / incumbent & Choice2 / rho-neg \\",
        r"\hline",
    ]
    for b in sorted(sizes):
        row = sizes[b]
        lines.append(
            f"{b} & 180 & {float(row['choice_over_incumbent']):.6f} & "
            f"{float(row['choice_over_rho_neg']):.6f} \\\\"
        )
    lines.extend([
        r"\hline",
        f"Pooled & 1,260 & {float(pooled['choice_over_incumbent']):.6f} & "
        f"{float(pooled['choice_over_rho_neg']):.6f} \\\\",
        r"\end{tabular}",
        r"\end{center}",
        "",
        r"\begin{figure}[ht]",
        r"\centering",
        r"\includegraphics[width=\linewidth]{choice2-operation-cost-ratios.png}",
        r"\caption{Deterministic operation-cost ratios at the fixed price above. The dashed line is the pooled sum ratio. No statistical interval is applicable to these fixed counts.}",
        r"\end{figure}",
        "",
        f"At this fixed price, pooled two-choice/incumbent cost is {float(pooled['choice_over_incumbent']):.6f}; "
        f"two-choice/source-negation-rho cost is {float(pooled['choice_over_rho_neg']):.6f}. "
        "Across the 21 curves, the source report gives a two-choice/incumbent range of 0.933237 to 1.102391, "
        "so pooled parity does not establish a consistent per-curve improvement.",
        "",
        r"\section*{Limits and provenance}",
        "These are operation-count estimates (L0). No isolated wall timing, A/A timing, hardware-speed claim, "
        "fastest-external comparison, or end-to-end DLP advantage is established. The result is not entered into "
        "performance history or the IC leaderboard. Projective arithmetic and the other four leads remain incomplete "
        "as documented in the source report.",
        "",
        r"Source data (copies beside this report): \texttt{genus2-choice2-price-grid.csv} and \texttt{genus2-choice2-summary.json}. "
        r"Canonical source paths: \texttt{research/five\_leads\_e2e\_20261019/results/genus2-choice2-price-grid.csv} and "
        r"\texttt{research/five\_leads\_e2e\_20261019/results/genus2-choice2-summary.json}. "
        r"Replay receipts: \texttt{results/replay-audit-summary.json} and \texttt{sessions.tar.zst} in "
        r"\texttt{research/five\_leads\_e2e\_20261019/}. Reproduction command: "
        r"\texttt{python3 research/five\_leads\_e2e\_20261019/reports/choice2-operation-costs-20261019/render.py} "
        r"followed by \texttt{tectonic report.tex} from this directory.",
        r"\end{document}",
    ])
    return "\n".join(lines) + "\n"


def main():
    sizes, pooled = read_points()
    shapes = chart(sizes, pooled)
    (HERE / "choice2-operation-cost-ratios.svg").write_text(to_svg(shapes), encoding="utf-8")
    (HERE / "choice2-operation-cost-ratios.png").write_bytes(to_png(shapes))
    (HERE / "report.tex").write_text(report_tex(sizes, pooled), encoding="utf-8")
    print(f"rendered {len(sizes)} size rows + pooled n={pooled['targets']}; ratio={float(pooled['choice_over_incumbent']):.9f}")


if __name__ == "__main__":
    main()
