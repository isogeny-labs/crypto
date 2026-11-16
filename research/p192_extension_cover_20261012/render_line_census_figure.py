#!/usr/bin/env python3
"""Render a one-page visual from the frozen P-192 line and tangency censuses."""

import html
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
WIDTH, HEIGHT = 1200, 760
BG = "#f5f7fb"
INK = "#17243b"
MUTED = "#4c5b73"
BLUE = "#dceafe"
BLUE_EDGE = "#3973bd"
PURPLE = "#e9e2fb"
PURPLE_EDGE = "#7653b5"
GREEN = "#dff3e9"
GREEN_EDGE = "#31845d"
AMBER = "#fff0d5"
AMBER_EDGE = "#ba7620"


def load_and_check():
    lines = json.loads((ROOT / "type_i_line_full_collinearity.json").read_text())
    tangencies = json.loads((ROOT / "type_i_line_full_selected_tangencies.json").read_text())
    assert lines["status"] == "FULL_9925_POINT_COLLINEARITY_CENSUS"
    assert tangencies["status"] == "FULL_SELECTED_POINT_TANGENCY_CENSUS"
    assert lines["point_count"] == tangencies["point_count"] == 9_925
    assert lines["unordered_pairs_visited"] == tangencies["unordered_pairs_visited"] == 49_247_850
    assert len(lines["line_records"]) == lines["source_quartets_found"] == lines["source_quartets"] == 4_892
    assert all(len(row["point_indices_zero_based"]) == 4 for row in lines["line_records"])
    assert lines["novel_three_point_lines"] == lines["novel_four_point_lines"] == 0
    assert tangencies["tangent_pair_count"] == 0 and not tangencies["witnesses"]
    return lines, tangencies


def svg_figure(lines, tangencies):
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">']
    out.append(f'<rect width="{WIDTH}" height="{HEIGHT}" fill="{BG}"/>')

    def rect(x, y, w, h, fill, edge):
        out.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{fill}" stroke="{edge}" stroke-width="2"/>')

    def text(x, y, value, size, color=INK, weight="400"):
        out.append(f'<text x="{x}" y="{y}" font-family="Arial, sans-serif" font-size="{size}" font-weight="{weight}" fill="{color}">{html.escape(str(value))}</text>')

    def arrow(x1, x2, y):
        out.append(f'<path d="M{x1} {y} H{x2-12} M{x2-20} {y-8} L{x2-10} {y} L{x2-20} {y+8}" fill="none" stroke="{MUTED}" stroke-width="3"/>')

    text(60, 74, "P-192 quartic: complete pair census", 36, INK, "700")
    text(60, 112, "Frozen Type I point set | exact geometry checks", 20, MUTED)

    cards = [
        (60, 190, 300, 205, BLUE, BLUE_EDGE),
        (450, 190, 300, 205, PURPLE, PURPLE_EDGE),
        (840, 190, 300, 205, GREEN, GREEN_EDGE),
    ]
    for card in cards:
        rect(*card)
    arrow(360, 450, 286)
    arrow(750, 840, 286)

    text(88, 230, "FROZEN POINT SET", 15, BLUE_EDGE, "700")
    text(88, 292, f'{lines["point_count"]:,}', 46, INK, "700")
    text(88, 345, "rational quartic points", 20, MUTED)
    text(88, 375, "same input for both full scans", 15, MUTED)

    text(478, 230, "UNORDERED PAIRS SCANNED", 15, PURPLE_EDGE, "700")
    text(478, 292, f'{lines["unordered_pairs_visited"]:,}', 34, INK, "700")
    text(478, 345, "collinearity and tangency", 18, MUTED)
    text(478, 375, "two independently checked passes", 15, MUTED)

    text(868, 230, "COLLINEAR SOURCE QUARTETS", 15, GREEN_EDGE, "700")
    text(868, 292, f'{len(lines["line_records"]):,}', 46, INK, "700")
    text(868, 345, "each line has exactly 4 selected points", 15, MUTED)
    text(868, 375, "0 novel 3-point or 4-point lines", 16, GREEN_EDGE, "700")

    rect(250, 465, 700, 135, AMBER, AMBER_EDGE)
    text(282, 505, "SELECTED-ENDPOINT TANGENCY CHECK", 15, AMBER_EDGE, "700")
    text(282, 538, f'{tangencies["tangent_pair_count"]:,}', 42, INK, "700")
    text(368, 556, "tangent pairs found", 22, INK, "700")
    text(650, 556, f'of {tangencies["unordered_pairs_visited"]:,} tested', 17, MUTED)

    text(60, 660, "Geometric incidence alone is not a known-scalar relation or a measured index-calculus cost.", 16, MUTED)
    text(60, 690, "Tangency result covers selected endpoints only; residual-point ramification is separate.", 16, MUTED)
    text(60, 728, "Source: type_i_line_full_collinearity.json and type_i_line_full_selected_tangencies.json", 12, MUTED)
    out.append("</svg>\n")
    return "\n".join(out)


def pdf_figure(lines, tangencies):
    ops = []

    def rgb(hex_color):
        return tuple(int(hex_color[i:i + 2], 16) / 255 for i in (1, 3, 5))

    def rect(x, y, w, h, fill, edge):
        fy = HEIGHT - y - h
        r, g, b = rgb(fill)
        er, eg, eb = rgb(edge)
        ops.append(f"{r:.4f} {g:.4f} {b:.4f} rg {er:.4f} {eg:.4f} {eb:.4f} RG 2 w {x} {fy} {w} {h} re B")

    def text(x, y, value, size, color=INK, bold=False):
        r, g, b = rgb(color)
        safe = str(value).replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")
        font = "F2" if bold else "F1"
        baseline = HEIGHT - y - size
        ops.append(f"{r:.4f} {g:.4f} {b:.4f} rg BT /{font} {size} Tf 1 0 0 1 {x} {baseline} Tm ({safe}) Tj ET")

    def arrow(x1, x2, y):
        py = HEIGHT - y
        r, g, b = rgb(MUTED)
        ops.append(f"{r:.4f} {g:.4f} {b:.4f} RG 3 w {x1} {py} m {x2-12} {py} l S")
        ops.append(f"{r:.4f} {g:.4f} {b:.4f} rg {x2-20} {py+8} m {x2-10} {py} l {x2-20} {py-8} l h f")

    r, g, b = rgb(BG)
    ops.append(f"{r:.4f} {g:.4f} {b:.4f} rg 0 0 {WIDTH} {HEIGHT} re f")
    text(60, 74, "P-192 quartic: complete pair census", 36, INK, True)
    text(60, 112, "Frozen Type I point set | exact geometry checks", 20, MUTED)
    rect(60, 190, 300, 205, BLUE, BLUE_EDGE)
    rect(450, 190, 300, 205, PURPLE, PURPLE_EDGE)
    rect(840, 190, 300, 205, GREEN, GREEN_EDGE)
    arrow(360, 450, 286)
    arrow(750, 840, 286)
    text(88, 230, "FROZEN POINT SET", 15, BLUE_EDGE, True)
    text(88, 292, f'{lines["point_count"]:,}', 46, INK, True)
    text(88, 345, "rational quartic points", 20, MUTED)
    text(88, 375, "same input for both full scans", 15, MUTED)
    text(478, 230, "UNORDERED PAIRS SCANNED", 15, PURPLE_EDGE, True)
    text(478, 292, f'{lines["unordered_pairs_visited"]:,}', 34, INK, True)
    text(478, 345, "collinearity and tangency", 18, MUTED)
    text(478, 375, "two independently checked passes", 15, MUTED)
    text(868, 230, "COLLINEAR SOURCE QUARTETS", 15, GREEN_EDGE, True)
    text(868, 292, f'{len(lines["line_records"]):,}', 46, INK, True)
    text(868, 345, "each line has exactly 4 selected points", 15, MUTED)
    text(868, 375, "0 novel 3-point or 4-point lines", 16, GREEN_EDGE, True)
    rect(250, 465, 700, 135, AMBER, AMBER_EDGE)
    text(282, 505, "SELECTED-ENDPOINT TANGENCY CHECK", 15, AMBER_EDGE, True)
    text(282, 538, f'{tangencies["tangent_pair_count"]:,}', 42, INK, True)
    text(368, 556, "tangent pairs found", 22, INK, True)
    text(650, 556, f'of {tangencies["unordered_pairs_visited"]:,} tested', 17, MUTED)
    text(60, 660, "Geometric incidence alone is not a known-scalar relation or a measured index-calculus cost.", 16, MUTED)
    text(60, 690, "Tangency result covers selected endpoints only; residual-point ramification is separate.", 16, MUTED)
    text(60, 728, "Source: type_i_line_full_collinearity.json and type_i_line_full_selected_tangencies.json", 12, MUTED)
    stream = ("\n".join(ops) + "\n").encode("ascii")
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 1200 760] /Resources << /Font << /F1 4 0 R /F2 5 0 R >> >> /Contents 6 0 R >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>",
        b"<< /Length " + str(len(stream)).encode("ascii") + b" >>\nstream\n" + stream + b"endstream",
    ]
    pdf = bytearray(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")
    offsets = [0]
    for i, obj in enumerate(objects, 1):
        offsets.append(len(pdf))
        pdf.extend(f"{i} 0 obj\n".encode("ascii") + obj + b"\nendobj\n")
    xref = len(pdf)
    pdf.extend(f"xref\n0 {len(objects)+1}\n0000000000 65535 f \n".encode("ascii"))
    for offset in offsets[1:]:
        pdf.extend(f"{offset:010d} 00000 n \n".encode("ascii"))
    pdf.extend(f"trailer\n<< /Size {len(objects)+1} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode("ascii"))
    return bytes(pdf)


def main():
    lines, tangencies = load_and_check()
    (ROOT / "type_i_line_census.svg").write_text(svg_figure(lines, tangencies))
    (ROOT / "type_i_line_census.pdf").write_bytes(pdf_figure(lines, tangencies))
    print("PASS: rendered and validated full P-192 collinearity and selected-endpoint tangency census")


if __name__ == "__main__":
    main()
