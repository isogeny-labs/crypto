#!/usr/bin/env python3
"""Render the checked weighted-Proj-head status with only the stdlib."""

from __future__ import annotations

import csv
from pathlib import Path
from xml.sax.saxutils import escape

ROOT = Path(__file__).resolve().parent
STEM = "weighted-proj-head-verified"

ROWS = [
    ("degree-six sextic", "checked", "equation_weighted_homogeneous", "Lean exit 0; standard axioms"),
    ("homogeneous principal ideal", "checked", "relationIdeal_homogeneous", "Lean exit 0; standard axioms"),
    ("Z=1 affine equation", "checked", "affine_equation", "substitution into the sextic only"),
    ("X=1 infinity equation", "checked", "infinity_equation", "substitution into the sextic only"),
    ("graded quotient", "open", "none", "homogeneous-ideal quotient grading required"),
    ("D_+(Z) affine chart", "open", "none", "degree-zero localization identification required"),
    ("infinity chart and overlap", "open", "none", "chart isomorphism and gluing comparison required"),
    ("smooth projective completion", "open", "none", "smoothness and projectivity required"),
    ("Pic0 transfer", "open", "none", "Jacobian compatibility required"),
]


def render_csv() -> None:
    with (ROOT / f"{STEM}.csv").open("w", newline="", encoding="utf-8") as file:
        writer = csv.writer(file, lineterminator="\n")
        writer.writerow(("obligation", "status", "evidence", "scope_or_missing_step"))
        writer.writerows(ROWS)


def render_svg() -> None:
    width, height = 1040, 110 + len(ROWS) * 62
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        '<rect width="100%" height="100%" fill="#fff"/>',
        '<text x="32" y="38" font-family="sans-serif" font-size="23" font-weight="bold">Weighted Proj bridge: verified status</text>',
        '<text x="32" y="64" font-family="sans-serif" font-size="13" fill="#4b5563">Pinned Lean 4.34.1, Mathlib d13f23b; logical dependency, not a performance graph</text>',
    ]
    for index, (name, status, evidence, _) in enumerate(ROWS):
        y = 84 + index * 62
        color = "#d9f4e2" if status == "checked" else "#f3f4f6"
        edge = "#207244" if status == "checked" else "#7c8491"
        parts.append(f'<rect x="32" y="{y}" width="976" height="48" rx="7" fill="{color}" stroke="{edge}"/>')
        parts.append(f'<text x="48" y="{y + 28}" font-family="sans-serif" font-size="15">{escape(name)}</text>')
        parts.append(f'<text x="405" y="{y + 28}" font-family="monospace" font-size="13" fill="{edge}">{status.upper()}</text>')
        parts.append(f'<text x="520" y="{y + 28}" font-family="monospace" font-size="12">{escape(evidence)}</text>')
        if index < len(ROWS) - 1:
            parts.append(f'<path d="M 520 {y + 48} V {y + 62}" stroke="#8a919d" stroke-width="2"/>')
    parts.append('</svg>')
    (ROOT / f"{STEM}.svg").write_text("\n".join(parts) + "\n", encoding="utf-8")


def pdf_escape(value: str) -> str:
    return value.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")


def render_pdf() -> None:
    lines = [
        "Weighted Proj bridge: checked homogeneity and chart equations",
        "Pinned Lean 4.34.1; Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612",
        "",
        "Weights: X=1, Y=3, Z=1. Coefficients a3,a2,a1,a0 are arbitrary in a field.",
        "Equation: Y^2 - X^5 Z - a3 X^3 Z^3 - a2 X^2 Z^4",
        "          - a1 X Z^5 - a0 Z^6.",
        "",
        "CHECKED: equation_weighted_homogeneous, weighted degree six.",
        "CHECKED: relationIdeal_homogeneous, principal ideal homogeneous.",
        "CHECKED: Z=1 gives y^2 - (x^5+a3*x^3+a2*x^2+a1*x+a0).",
        "CHECKED: X=1 gives w^2-z(1+a3*z^2+a2*z^3+a1*z^4+a0*z^5).",
        "Compiler exit: 0. Four axiom audits: propext, Classical.choice, Quot.sound.",
        "No sorryAx in any audit.",
        "",
        "OPEN: grading on the quotient by the homogeneous ideal.",
        "OPEN: degree-zero localization at Z equals the affine coordinate ring.",
        "OPEN: infinity chart and overlap comparison with constructed two-chart curve.",
        "OPEN: smooth projective completion and Pic0 transfer.",
        "",
        "The graph and CSV enumerate these obligations without a ranking claim.",
        "Compiler receipt: weighted-proj-head-final-build.log",
        "Chart receipt: weighted-proj-charts-build.log",
        "Copy timeout receipt: weighted-proj-head-copy.txt",
    ]
    stream = ["BT", "/F1 10 Tf", "50 748 Td", "14 TL"]
    for line in lines:
        stream.append(f"({pdf_escape(line)}) Tj")
        stream.append("T*")
    stream.append("ET")
    data = "\n".join(stream).encode("ascii")
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>",
        b"<< /Length " + str(len(data)).encode("ascii") + b" >>\nstream\n" + data + b"\nendstream",
    ]
    payload = bytearray(b"%PDF-1.4\n")
    offsets = [0]
    for index, obj in enumerate(objects, 1):
        offsets.append(len(payload))
        payload += f"{index} 0 obj\n".encode("ascii") + obj + b"\nendobj\n"
    xref_offset = len(payload)
    payload += f"xref\n0 {len(offsets)}\n0000000000 65535 f \n".encode("ascii")
    for offset in offsets[1:]:
        payload += f"{offset:010d} 00000 n \n".encode("ascii")
    payload += f"trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n".encode("ascii")
    (ROOT / f"{STEM}.pdf").write_bytes(payload)


if __name__ == "__main__":
    render_csv()
    render_svg()
    render_pdf()
