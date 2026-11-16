#!/usr/bin/env python3
"""Dependency-free one-page PDF for the exact R103 norm identity."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
data = json.loads((root / "independent-replay.json").read_text())
assert data["check"] == "R103_INDEPENDENT_NORM_MATCH_PASS"
assert data["direct_numerator_degree"] == 123 and data["direct_denominator_exponent"] == 163


def label(x, y, size, value):
    value = value.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")
    return f"BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n"


content = label(52, 748, 18, "R103: direct marked-23 lambda derivative norm")
content += label(52, 720, 10, "5 November 2026 | Koblitz modular IC research")
intro = [
    "The actual G51 has lambda exponents 16, 8, 4, 1, 0. In F2 its",
    "derivative is the l-linear coefficient C(x,y). Since G51 is monic",
    "degree 16, Res_l(G51,C)=C^16. The direct norm through f51 is",
    "Res_y(f51,C)=P123(x)/x^163, with degree(P123)=123 and 74 terms.",
    "Its sixteenth power equals the accepted R99 full norm exactly.",
]
for i, line in enumerate(intro):
    content += label(52, 686 - 16 * i, 10, line)
content += label(52, 574, 12, "Exact direct-to-full resultant relation")
boxes = [(52, "dG/dl = C", "actual source"), (225, "Res_y(f,C)", "P123/x^163"), (398, "R99 full norm", "P123^16/x^2608")]
for x, title, sub in boxes:
    content += f"0.86 0.92 0.98 rg {x} 418 155 105 re f 0.16 0.37 0.62 RG {x} 418 155 105 re S\n"
    content += label(x + 10, 478, 12, title)
    content += label(x + 10, 449, 10, sub)
content += "0.16 0.37 0.62 RG 207 470 m 225 470 l S 380 470 m 398 470 l S\n"
tail = [
    "Source-bound Singular exited zero without native errors and rejected a",
    "changed norm. Fresh replay and an independent GF(2) comparison matched",
    "the R99/R100 hashes, numerator exponents, and denominator powers.",
    "Lean 4.34.1 checked exponent parity and degree arithmetic.",
    "This identity concerns the displayed monogenic order; local integral",
    "closure is still required for normalized branch valuations.",
    "Raw archive SHA-256:",
    "1136fb2502eae1d22a4d5b9604e01bd2163d9438725ec7e6d19891c3c1b49e4d",
]
for i, line in enumerate(tail):
    content += label(52, 380 - 18 * i, 10, line)

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
