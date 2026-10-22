#!/usr/bin/env python3
"""Layout and schema conversion of independently generated study price grids.

This script does not run a collector, reprice arithmetic, or validate scalars.
Every emitted numeric cost is copied from the cited study grid. Ratio fields
are checked against copied numerator and denominator before being emitted.
"""

import csv
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
OUT = HERE / "results"
OUT.mkdir(exist_ok=True)

SOURCES = [
    ("factor_base", "full_16", "research/four_followups_20261021/results/matched/price-grid.csv", "fb1-j16"),
    ("factor_base", "full_8", "research/four_followups_20261021/results/matched/price-grid.csv", "fb1-j8"),
    ("factor_base", "full_32", "research/four_followups_20261021/results/matched/price-grid.csv", "fb1-j32"),
    ("factor_base", "half_16", "research/four_followups_20261021/results/matched/price-grid.csv", "fb2-j16"),
    ("factor_base", "quarter_16", "research/four_followups_20261021/results/matched/price-grid.csv", "fb4-j16"),
    ("projective_law", "wrapper", "research/projective_law_followup_20261022/results/price-grid-projective.csv", None),
    ("projective_law", "retained_state", "research/projective_law_followup_20261022/results/price-grid-state.csv", None),
]

FIELDS = [
    "lead", "arm", "slice", "targets", "verified", "candidate_failures", "rho_failures",
    "S_per_M", "I_per_M", "native_GAE_per_logical", "candidate_GAE", "external_GAE",
    "rho_GAE", "candidate_over_external", "candidate_over_rho", "status", "source_grid",
]

rows = []
for lead, arm, source, source_arm in SOURCES:
    with (ROOT / source).open(newline="") as handle:
        for original in csv.DictReader(handle):
            if source_arm is not None and original["arm"] != source_arm:
                continue
            candidate = float(original["candidate_GAE"])
            rho = float(original["rho_GAE"])
            external = float(original["external_GAE"])
            assert math.isclose(candidate / rho, float(original["candidate_over_rho"]), rel_tol=1e-9)
            assert math.isclose(candidate / external, float(original["candidate_over_external"]), rel_tol=1e-9)
            rows.append({
                "lead": lead, "arm": arm, "slice": original["slice"],
                "targets": original["targets"], "verified": original["verified"],
                "candidate_failures": original.get("candidate_failures", original.get("failures", "")),
                "rho_failures": original.get("rho_failures", original.get("failures", "")),
                "S_per_M": original["S_per_M"], "I_per_M": original["I_per_M"],
                "native_GAE_per_logical": original["native_GAE_per_logical"],
                "candidate_GAE": original["candidate_GAE"], "external_GAE": original["external_GAE"],
                "rho_GAE": original["rho_GAE"],
                "candidate_over_external": original["candidate_over_external"],
                "candidate_over_rho": original["candidate_over_rho"],
                "status": "priced_source_verified", "source_grid": source,
            })

for bits in (16, 20, 24, 28, 32, 36, 40):
    rows.append({
        "lead": "kummer_walk", "arm": "coordinate_identity_gate", "slice": str(bits),
        "targets": "180", "verified": "180", "candidate_failures": "0", "rho_failures": "",
        "S_per_M": "", "I_per_M": "", "native_GAE_per_logical": "", "candidate_GAE": "",
        "external_GAE": "", "rho_GAE": "", "candidate_over_external": "",
        "candidate_over_rho": "", "status": "identity_only_no_dlp_collector",
        "source_grid": f"research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl",
    })

with (OUT / "standard-cost-grid.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(handle, fieldnames=FIELDS)
    writer.writeheader()
    writer.writerows(rows)

representative = []
for row in rows:
    if row["status"] != "priced_source_verified":
        continue
    if row["S_per_M"] == "0.5" and row["I_per_M"] == "10" and row["native_GAE_per_logical"] == "0.1":
        representative.append({
            "lead": row["lead"], "arm": row["arm"], "slice": row["slice"],
            "targets": int(row["targets"]), "candidate_over_rho": float(row["candidate_over_rho"]),
            "candidate_over_external": float(row["candidate_over_external"]),
            "candidate_GAE": float(row["candidate_GAE"]), "rho_GAE": float(row["rho_GAE"]),
            "source_grid": row["source_grid"],
        })

summary = {
    "schema": "four-followups-review/v1", "price_unit": "GAE",
    "representative_price": {"S_per_M": 0.5, "I_per_M": 10, "native_GAE_per_logical": 0.1},
    "scope": "Frozen 21-model, 1,260-public-target cohort; 60 targets/model; seven order sizes.",
    "rows": len(rows), "representative_rows": representative,
    "requirements": [
        {"lead": "factor_base", "status": "verified_complete_for_five_frozen_arms", "candidate_targets_per_arm": 1260,
         "rho_targets_per_arm": 1260, "source": "research/four_followups_20261021/REPORT.md"},
        {"lead": "projective_law", "status": "verified_complete_for_two_frozen_arms", "candidate_targets_per_arm": 1260,
         "rho_targets_per_arm": 1260, "source": "research/projective_law_followup_20261022/REPORT.md"},
        {"lead": "pencil_collector", "status": "partial", "candidate_targets": 541,
         "rho_targets": 541, "remaining": "719 distinct targets from the original cohort are unpriced in the integrated online pilot; sparse rank study is in progress.",
         "source": "research/four_followups_20261021/REPORT.md"},
        {"lead": "kummer_walk", "status": "partial", "coordinate_label_cases": 8820,
         "nonzero_matches": 8813, "zero_outputs": 7, "wrong_nonzero_outputs": 0,
         "candidate_dlp_targets": 0, "candidate_over_rho": None,
         "remaining": "Published fast Recover and a labeled DLP collector with complete cost.",
         "source": "research/kummer_coordinates_20261022/REPORT.md"},
    ],
}
(OUT / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")

COLORS = {"full_16": "#1764aa", "full_8": "#16a0a0", "full_32": "#3946a5", "half_16": "#b2691b", "quarter_16": "#a03543", "wrapper": "#75832a", "retained_state": "#8647a3"}
grouped = {}
for row in representative:
    if row["slice"] != "all":
        grouped.setdefault(row["arm"], []).append(row)

width, height = 920, 560
left, right, top, bottom = 90, 690, 40, 465
def x(bits): return left + (int(bits) - 16) / 24 * (right - left)
def y(ratio): return bottom - (math.log10(ratio) / math.log10(100)) * (bottom - top)
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
       '<rect width="100%" height="100%" fill="white"/>',
       '<text x="90" y="25" font-size="17" font-family="sans-serif">Cold candidate / matched rho, representative price</text>']
for ratio in (1, 2, 5, 10, 20, 50, 100):
    yy = y(ratio)
    svg += [f'<line x1="{left}" y1="{yy:.1f}" x2="{right}" y2="{yy:.1f}" stroke="#d8dfe7"/>',
            f'<text x="{left-12}" y="{yy+4:.1f}" text-anchor="end" font-size="12" font-family="sans-serif">{ratio}</text>']
for bits in (16, 20, 24, 28, 32, 36, 40):
    xx = x(bits)
    svg += [f'<line x1="{xx:.1f}" y1="{top}" x2="{xx:.1f}" y2="{bottom}" stroke="#e9edf1"/>',
            f'<text x="{xx:.1f}" y="{bottom+22}" text-anchor="middle" font-size="12" font-family="sans-serif">{bits}</text>']
for i, (arm, data) in enumerate(grouped.items()):
    data.sort(key=lambda row: int(row["slice"]))
    points = " ".join(f'{x(row["slice"]):.1f},{y(row["candidate_over_rho"]):.1f}' for row in data)
    color = COLORS[arm]
    svg.append(f'<polyline points="{points}" fill="none" stroke="{color}" stroke-width="2.4"/>')
    for row in data:
        svg.append(f'<circle cx="{x(row["slice"]):.1f}" cy="{y(row["candidate_over_rho"]):.1f}" r="3.5" fill="{color}"/>')
    svg.append(f'<line x1="715" y1="{75+i*27}" x2="743" y2="{75+i*27}" stroke="{color}" stroke-width="3"/>')
    svg.append(f'<text x="753" y="{79+i*27}" font-size="13" font-family="sans-serif">{arm}</text>')
svg += ['<text x="390" y="518" text-anchor="middle" font-size="13" font-family="sans-serif">Order bits</text>',
        '<text x="90" y="544" font-size="11" font-family="sans-serif">Kummer DLP has no priced collector and is intentionally absent. Source: standard-cost-grid.csv.</text>',
        '</svg>']
(HERE / "cost-ratios.svg").write_text("\n".join(svg) + "\n")
print(f"wrote {len(rows)} normalized rows and {len(representative)} representative rows")
