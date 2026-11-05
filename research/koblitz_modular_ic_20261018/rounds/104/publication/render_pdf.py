#!/usr/bin/env python3
"""Dependency-free one-page PDF carrying the R104 diagram and result."""
from pathlib import Path

root = Path(__file__).resolve().parent


def label(x, y, size, value):
    value = value.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")
    return f"BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n"


content = label(42, 748, 18, "R104: cleared N51 leading residue at x=1")
content += label(42, 723, 10, "5 November 2026 | marked-23 Koblitz source-order experiment")
for i, line in enumerate([
    "R102 has 22 coefficients of order -9. Their leading term factors as",
    "H = y^2 (y+1)^4 (l^2+l+1)^7. The fiber equations are",
    "f = y^3(y+1)^4 and G = (l^2+l+1)^8.",
    "Pinned Singular checks H is nonzero in the rank-112 source order.",
]):
    content += label(42, 686 - 17 * i, 11, line)
boxes = [
    (42, "y=0 component", "length 48; H=y^2 q^7"),
    (224, "y=1 component", "length 64; H=0"),
    (406, "reduced points", "residues 0 and 0"),
]
for x, title, sub in boxes:
    content += f"0.88 0.94 1 rg {x} 380 164 126 re f 0.13 0.32 0.53 RG {x} 380 164 126 re S\n"
    content += label(x + 9, 465, 11, title)
    content += label(x + 9, 432, 9, sub)
for i, line in enumerate([
    "Fresh Singular replay matched exactly. Independent F2 support and Lean",
    "checks verified the factorization and the component dimensions.",
    "The least clearing exponent in this monogenic order is 9; normalized",
    "branch valuations still require its integral closure.",
    "Raw archive SHA-256:",
    "ac0d2364774167d2a79871484326308fd27d42b5c4b6d6fbd428a2a0aa3c3af4",
]):
    content += label(42, 340 - 18 * i, 10, line)

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
for n, obj in enumerate(objects, 1):
    offsets.append(len(pdf))
    pdf += f"{n} 0 obj\n".encode() + obj + b"\nendobj\n"
xref = len(pdf)
pdf += f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode()
for offset in offsets[1:]:
    pdf += f"{offset:010d} 00000 n \n".encode()
pdf += f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
(root / "report.pdf").write_bytes(pdf)
