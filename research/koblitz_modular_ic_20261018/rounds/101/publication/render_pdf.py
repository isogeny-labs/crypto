#!/usr/bin/env python3
"""Dependency-free one-page PDF report from the checked R101 data."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
data = json.loads((root / "independent-replay.json").read_text())
left, right = data["component_dimensions_over_F2"]
assert (left, right) == (48, 64) and left + right == 112


def pdf_text(x: int, y: int, size: int, value: str) -> str:
    value = value.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")
    return f"BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n"


content = ""
content += pdf_text(56, 748, 18, "R101: marked-23 source-order fiber at x=1")
content += pdf_text(56, 720, 10, "5 November 2026 | Koblitz modular IC research")
lines = [
    "The SHA-256-bound R51 source gives f51(1,y)=y^3(y+1)^4 and",
    "G51(1,y,l)=l^16+l^8+1=(l^2+l+1)^8. All 28 rational lambda",
    "coefficients were accounted for. The lambda derivative vanishes at x=1.",
    "Singular checked both y=0 and y=1 lambda factors, full products, and",
    "changed-product controls. A fresh process rebound the donor and",
    "reproduced every export byte-for-byte. Independent GF(2) arithmetic",
    "and Lean 4.34.1 checked the polynomial and length identities.",
]
for i, line in enumerate(lines):
    content += pdf_text(56, 687 - 15 * i, 10, line)
content += pdf_text(56, 561, 12, "F2 dimension of the x=1 source-order components")
content += "0.1 0.1 0.1 RG 96 340 m 516 340 l S\n"
for x, height, label, value in [(145, left * 3, "y=0, F4 residue", left), (385, right * 3, "y=1, F4 residue", right)]:
    content += f"0.16 0.37 0.62 rg {x} 340 110 {height} re f\n"
    content += pdf_text(x + 38, 349 + height, 12, str(value))
    content += pdf_text(x, 320, 10, label)
tail = [
    "For F2[x,y,l]/(f51,G51), the x=1 fiber has two maximal ideals",
    "with F4 residue field. Local F2 dimensions are 48 and 64, totaling 112.",
    "These are source-order lengths; N51 was not used in this order's fiber.",
    "Local normalization is required for branch counts and valuations.",
    "Attempts 01 and 02 were retained as preparation failures; attempt 03",
    "and its fresh replay passed. The independent Python 3.9 verifier passed",
    "after its recorded bit-count compatibility correction.",
    "Raw archive SHA-256:",
    "6d672fe4f5ea9dc4ff66d7334586959cb120bb09e7521332bfcc85e2cc731c99",
]
for i, line in enumerate(tail):
    content += pdf_text(56, 278 - 16 * i, 10, line)

stream = content.encode("ascii")
objects = [
    b"<< /Type /Catalog /Pages 2 0 R >>",
    b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
    b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>",
    b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
    f"<< /Length {len(stream)} >>\nstream\n".encode() + stream + b"endstream",
]
pdf = bytearray(b"%PDF-1.4\n")
offsets = [0]
for number, obj in enumerate(objects, 1):
    offsets.append(len(pdf))
    pdf += f"{number} 0 obj\n".encode() + obj + b"\nendobj\n"
xref = len(pdf)
pdf += f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode()
for offset in offsets[1:]:
    pdf += f"{offset:010d} 00000 n \n".encode()
pdf += f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
(root / "publication" / "report.pdf").write_bytes(pdf)
