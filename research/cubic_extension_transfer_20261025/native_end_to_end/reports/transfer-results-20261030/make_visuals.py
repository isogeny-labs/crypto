#!/usr/bin/env python3
"""Render the existing cubic-transfer receipts as SVG; this runs no experiments."""

import json
from pathlib import Path
from xml.sax.saxutils import escape


HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
RESULTS = STUDY / "results"
OUT = HERE


def read_json(path):
    return json.loads(path.read_text())


def svg_text(x, y, value, size=14, anchor="start", color="#17212b", weight="normal"):
    return (
        f'<text x="{x}" y="{y}" font-family="DejaVu Sans, sans-serif" '
        f'font-size="{size}" text-anchor="{anchor}" fill="{color}" '
        f'font-weight="{weight}">{escape(str(value))}</text>'
    )


def make_attempts_chart():
    panel = read_json(RESULTS / "panel-summary-20261030.json")
    if panel.get("source_verified") is not True:
        raise ValueError("panel summary does not certify source verification")
    if panel.get("no_synthetic_targets") is not True or panel.get("no_source_log_read") is not True:
        raise ValueError("panel summary violates the public-input contract")
    workloads = {}
    for p in (109, 157):
        known = read_json(RESULTS / f"p{p}-known-answer-panel.json")
        for i, item in enumerate(known["workloads"]):
            workloads[(p, item["scalar"])] = f"p{p} target{i:02d}"

    cases = panel["cases"]
    if panel["instances"] != 16 or len(cases) != 16:
        raise ValueError("the source receipt no longer describes 16 cases")
    rows = []
    for case in cases:
        name = workloads.get((case["p"], case["recovered_scalar"]))
        if name is None:
            raise ValueError(f"no public known-answer workload for {case}")
        rows.append((name, case))
    if len({name for name, _ in rows}) != 16:
        raise ValueError("target mapping is not one-to-one")
    verification_receipts = sorted((RESULTS / "panel-development-attempt2").glob("*-independent-verification.json"))
    if len(verification_receipts) != 16:
        raise ValueError("expected exactly 16 independent panel verification receipts")
    for path in verification_receipts:
        if read_json(path).get("status") != "passed":
            raise ValueError(f"independent verification did not pass: {path.name}")
    for p in (109, 157):
        for arm in ("ic", "rho"):
            receipt = read_json(RESULTS / f"p{p}-transfer-{arm}-final-20261030.json")
            if (
                receipt.get("status") != "passed"
                or receipt.get("verified_on_original_source") is not True
                or receipt.get("source_log_read") is not False
                or receipt.get("synthetic_target_used") is not False
                or receipt.get("benchmark") is not False
            ):
                raise ValueError(f"original public {p} {arm} receipt failed its recorded contract")

    width, height = 1120, 740
    left, right, top, row_h = 300, 55, 116, 34
    plot_w = width - left - right
    x_max = 25
    plot_bottom = top + row_h * len(rows)
    colors = {
        "jacobian_base_point_attempts": "#e09f3e",
        "verification_split_attempts": "#3a9278",
    }
    labels = {
        "jacobian_base_point_attempts": "Jacobian base point attempts",
        "verification_split_attempts": "Verification split attempts",
    }
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        '<rect width="100%" height="100%" fill="#ffffff"/>',
        svg_text(left, 34, "Recorded recovery/setup attempts — 16 supplemental public targets", 20, weight="bold"),
        svg_text(left, 59, "Counts only; source randomization multiplier is in each row label. Not time or complete solver cost.", 13),
    ]
    for tick in range(0, 26, 5):
        x = left + plot_w * tick / x_max
        parts.append(f'<line x1="{x:.1f}" y1="{top-8}" x2="{x:.1f}" y2="{plot_bottom}" stroke="#d8dee5" stroke-width="1"/>')
        parts.append(svg_text(x, plot_bottom + 25, tick, 12, anchor="middle", color="#52606d"))
    metrics = list(colors)
    for row, (name, case) in enumerate(rows):
        y0 = top + row_h * row + 3
        if row % 2 == 0:
            parts.append(f'<rect x="0" y="{y0-5}" width="{width}" height="{row_h}" fill="#f6f8fa"/>')
        parts.append(svg_text(left - 14, y0 + 16, f"{name} (mult={case['source_randomization_multiplier']})", 12, anchor="end"))
        for j, metric in enumerate(metrics):
            value = case[metric]
            y = y0 + j * 8
            w = plot_w * value / x_max
            parts.append(f'<rect x="{left}" y="{y}" width="{w:.2f}" height="6" fill="{colors[metric]}"/>')
            parts.append(svg_text(left + w + 5, y + 6, value, 10, color="#263238"))
    parts.append(svg_text(left + plot_w / 2, height - 48, "Recorded attempt count per target", 13, anchor="middle", weight="bold"))
    legend_y = height - 18
    legend_x = left + 30
    for metric in metrics:
        parts.append(f'<rect x="{legend_x}" y="{legend_y-11}" width="12" height="12" fill="{colors[metric]}"/>')
        parts.append(svg_text(legend_x + 17, legend_y, labels[metric], 11))
        legend_x += 360
    parts.append("</svg>")
    (OUT / "panel-attempts.svg").write_text("\n".join(parts) + "\n")


def make_pipeline_diagram():
    width, height = 1200, 440
    boxes = [
        (30, 112, 205, 210, "Public source inputs", ["p109.public: E/F₁₀₉³, r=1,293,169", "p157.public: E/F₁₅₇³, r=3,868,159", "fixtures contain no target scalar"]),
        (275, 112, 205, 210, "Kernel and map", ["rational 3-torsion lift", "p109: one 3-isogeny", "p157: two 3-isogenies"]),
        (520, 112, 205, 210, "Constructed target", ["Riemann quartic", "Jacobian over Fₚ", "divisors from actual source images"]),
        (765, 112, 205, 210, "Target solving", ["original inputs: IC and rho", "additional panel: 16 native IC solves", "no planted scalar in solver input"]),
        (1010, 112, 160, 210, "Verification", ["recovered k", "checked on original E", "16 independent Fₚ³ checks"]),
    ]
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        '<defs><marker id="arrow" markerWidth="10" markerHeight="8" refX="9" refY="4" orient="auto"><path d="M0,0 L10,4 L0,8 z" fill="#42637c"/></marker></defs>',
        '<rect width="100%" height="100%" fill="#ffffff"/>',
        svg_text(32, 42, "Recorded native elliptic-to-Jacobian transfer pipeline", 20, weight="bold"),
        svg_text(32, 67, "Description of the existing receipts; no new solving, timing, or performance claim.", 13),
    ]
    for x, y, w, h, title, lines in boxes:
        parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="12" fill="#f6f8fa" stroke="#42637c" stroke-width="2"/>')
        parts.append(svg_text(x + w / 2, y + 31, title, 14, anchor="middle", weight="bold"))
        for i, line in enumerate(lines):
            parts.append(svg_text(x + 12, y + 72 + i * 34, line, 11))
    for i in range(len(boxes) - 1):
        x0 = boxes[i][0] + boxes[i][2] + 7
        x1 = boxes[i + 1][0] - 7
        y = boxes[i][1] + boxes[i][3] / 2
        parts.append(f'<line x1="{x0}" y1="{y}" x2="{x1}" y2="{y}" stroke="#42637c" stroke-width="2" marker-end="url(#arrow)"/>')
    parts.append(svg_text(32, 380, "Evidence: fixtures/p109.public, fixtures/p157.public; NATIVE_MAP.md; panel-summary-20261030.json;", 12))
    parts.append(svg_text(32, 400, "p109/p157 final IC and rho receipts; 16 per-target independent-verification receipts.", 12))
    parts.append("</svg>")
    (OUT / "transfer-pipeline.svg").write_text("\n".join(parts) + "\n")


if __name__ == "__main__":
    make_attempts_chart()
    make_pipeline_diagram()
