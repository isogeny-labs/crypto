#!/usr/bin/env python3
"""Dependency-free R102 report PDF using the recorded histogram."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
data = json.loads((root / "result.json").read_text())
assert data["coefficient_count"] == 105
assert data["minimum_coefficient_order"] == -9
assert data["x_plus_one_clearing_exponent"] == 9
rows = sorted((int(order), count) for order, count in data["coefficient_order_histogram"].items())


def label(x, y, size, value):
    value = value.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")
    return f"BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n"


content = label(52, 748, 18, "R102: N51 coefficientwise clearing at x=1")
content += label(52, 720, 10, "5 November 2026 | Koblitz modular IC research")
intro = [
    "The actual R51 N51 relation has 105 distinct rational coefficients.",
    "Exact F2[x] division by x+1 found all 105 displayed orders negative.",
    "The minimum is -9, attained by 22 terms; the maximum is -1.",
    "Thus (x+1)^9 is the least coefficientwise clearing power; power 8 fails.",
    "The donor hash, monomial grid, malformed-coefficient control, and fresh",
    "byte-identical replay passed. Hasse coefficients independently matched",
    "all 105 orders. Lean 4.34.1 checked the reported bound and minimum.",
]
for i, line in enumerate(intro):
    content += label(52, 687 - 15 * i, 10, line)
content += label(52, 570, 12, "Orders of the 105 displayed N51 coefficients at x=1")
content += "0.1 0.1 0.1 RG 66 315 m 555 315 l S\n"
for i, (order, count) in enumerate(rows):
    x = 76 + i * 60
    height = count * 7
    content += f"0.16 0.37 0.62 rg {x} 315 37 {height} re f\n"
    content += label(x + 9, 322 + height, 10, str(count))
    content += label(x + 7, 294, 10, str(order))
content += label(200, 270, 10, "Coefficient order v_(x+1)")
tail = [
    "This coefficientwise bound is not a normalized function valuation.",
    "Next: reduce (x+1)^9 n at the two R101 source-order points, then",
    "compute integral closures and branch valuations.",
    "Raw archive SHA-256:",
    "554e9e6fab5c216b3f4964e9c505970544cac5a9ad0059fc6694c23c79ae38e4",
]
for i, line in enumerate(tail):
    content += label(52, 226 - 17 * i, 10, line)

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
