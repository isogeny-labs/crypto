#!/usr/bin/env python3
"""Render R105's two exact Newton edges to SVG and one-page PDF."""
from pathlib import Path

root = Path(__file__).resolve().parent
series = [
    ("y=0", [2, 4, 1, 0, 1, 1, 2, 0], 3, "#245b9b"),
    ("y=1", [3, 3, 2, 1, 0, 0, 0, 0], 4, "#b45b27"),
]
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="820" height="340" viewBox="0 0 820 340">',
       '<rect width="820" height="340" fill="white"/>',
       '<text x="35" y="32" font-size="20" font-family="sans-serif">R105: x=1 base Newton edges</text>',
       '<text x="35" y="318" font-size="14" font-family="sans-serif">Exact R51 donor; R105 jets105-02 and fresh replay</text>']
for idx, (name, vals, edge, color) in enumerate(series):
    ox = 40 + idx * 390
    svg += [f'<text x="{ox+8}" y="65" font-size="17" font-family="sans-serif" fill="{color}">{name}: slope {vals[0]}/{edge}</text>',
            f'<path d="M{ox+38} 270 L{ox+345} 270 M{ox+38} 270 L{ox+38} 80" stroke="black" fill="none"/>',
            f'<path d="M{ox+38} {270-48*vals[0]} L{ox+38+42*edge} 270" stroke="{color}" stroke-width="3" fill="none"/>']
    for z, v in enumerate(vals):
        svg.append(f'<circle cx="{ox+38+42*z}" cy="{270-48*v}" r="4" fill="{color}"/>')
    svg.append(f'<text x="{ox+255}" y="289" font-size="13" font-family="sans-serif">local y degree</text>')
svg.append('</svg>')
(root / "diagram.svg").write_text("\n".join(svg) + "\n")


def label(x, y, size, value):
    value = value.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")
    return f"BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n"


c = "0 0 0 rg\n" + label(42, 748, 18, "R105: x=1 Hasse jets and base Newton edges")
c += label(42, 723, 10, "5 November 2026 | marked-23 Koblitz source-order experiment")
for i, line in enumerate([
    "The exact f51 donor gives local Newton edges (0,2)-(3,0) near y=0",
    "and (0,3)-(4,0) near y=1. The slopes are 2/3 and 3/4.",
    "The full G51 jets at orders 1, 2 and 3 vanish. Its order-4 term is",
    "y^3(y+1)^3 l(l+1)(l^2+l+1); endpoint evaluation hides it.",
]): c += label(42, 687-18*i, 11, line)
for idx, (name, vals, edge, _) in enumerate(series):
    ox = 48 + idx * 280
    c += "0 0 0 RG 1 w\n"
    c += f"{ox+25} 340 m {ox+230} 340 l S {ox+25} 340 m {ox+25} 520 l S\n"
    c += "0.15 0.35 0.61 RG 2 w\n" if idx == 0 else "0.68 0.32 0.13 RG 2 w\n"
    c += f"{ox+25} {340+40*vals[0]} m {ox+25+28*edge} 340 l S\n"
    for z, v in enumerate(vals):
        x, y = ox+25+28*z, 340+40*v
        c += f"{x-2} {y-2} 4 4 re f\n"
    c += "0 0 0 rg\n" + label(ox+25, 530, 12, f"{name}: slope {vals[0]}/{edge}")
for i, line in enumerate([
    "Fresh replay matched exactly; a changed donor was rejected.",
    "Independent bit arithmetic and Lean checked the displayed data.",
    "The degree-112 lambda branch structure remains to be normalized.",
    "Raw archive SHA-256:",
    "bbb4005d3a3a747f3ff15b8d887e1c905d4a40d8d19856f135cbf6d5d178933c",
]): c += label(42, 300-18*i, 10, line)
stream = c.encode("ascii")
objects = [b"<< /Type /Catalog /Pages 2 0 R >>",
           b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
           b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>",
           b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
           f"<< /Length {len(stream)} >>\nstream\n".encode()+stream+b"endstream"]
pdf = bytearray(b"%PDF-1.4\n")
offsets = [0]
for n, obj in enumerate(objects, 1):
    offsets.append(len(pdf)); pdf += f"{n} 0 obj\n".encode()+obj+b"\nendobj\n"
xref = len(pdf)
pdf += f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode()
for off in offsets[1:]: pdf += f"{off:010d} 00000 n \n".encode()
pdf += f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
(root / "report.pdf").write_bytes(pdf)
