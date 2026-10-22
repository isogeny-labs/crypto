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
    "rho_GAE", "candidate_over_external", "candidate_over_rho", "baseline_GAE",
    "candidate_over_baseline", "status", "source_grid",
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
                "baseline_GAE": "", "candidate_over_baseline": "",
                "status": "priced_source_verified", "source_grid": source,
            })

# The pencil paired grids use the online identity arm as their baseline. Copy
# both sides of each comparison so reviewers can regroup the price band.
PENCIL_CURVES = {739: 16, 1151: 16, 1187: 16, 3371: 20, 3527: 20,
                 3931: 20, 11731: 24, 11887: 24, 12143: 24, 48383: 28}
for p, bits in PENCIL_CURVES.items():
    source = f"research/pencil_sparse_20261022/results/p{p}-peel-vs-online.csv"
    with (ROOT / source).open(newline="") as handle:
        for original in csv.DictReader(handle):
            baseline = float(original["baseline_GAE"])
            candidate = float(original["candidate_GAE"])
            rho = float(original["rho_GAE"])
            assert math.isclose(candidate / baseline, float(original["candidate_over_baseline"]), rel_tol=1e-9)
            assert math.isclose(baseline / rho, float(original["baseline_over_rho"]), rel_tol=1e-9)
            assert math.isclose(candidate / rho, float(original["candidate_over_rho"]), rel_tol=1e-9)
            for arm, priced, ratio_rho, ratio_base in (
                ("online_identity", original["baseline_GAE"], original["baseline_over_rho"], "1"),
                ("degree_peel", original["candidate_GAE"], original["candidate_over_rho"], original["candidate_over_baseline"]),
            ):
                rows.append({
                    "lead": "pencil_collector", "arm": arm, "slice": f"p{p}",
                    "targets": original["targets"], "verified": original["targets"],
                    "candidate_failures": "0", "rho_failures": "0",
                    "S_per_M": original["S_per_M"], "I_per_M": original["I_per_M"],
                    "native_GAE_per_logical": original["native_GAE_per_logical"],
                    "candidate_GAE": priced, "external_GAE": "", "rho_GAE": original["rho_GAE"],
                    "candidate_over_external": "", "candidate_over_rho": ratio_rho,
                    "baseline_GAE": original["baseline_GAE"], "candidate_over_baseline": ratio_base,
                    "status": "priced_source_verified", "source_grid": source,
                })

for bits in (16, 20, 24, 28, 32, 36, 40):
    rows.append({
        "lead": "kummer_walk", "arm": "coordinate_identity_gate", "slice": str(bits),
        "targets": "180", "verified": "180", "candidate_failures": "0", "rho_failures": "",
        "S_per_M": "", "I_per_M": "", "native_GAE_per_logical": "", "candidate_GAE": "",
        "external_GAE": "", "rho_GAE": "", "candidate_over_external": "",
        "candidate_over_rho": "", "baseline_GAE": "", "candidate_over_baseline": "",
        "status": "coordinate_and_recover_gate_no_dlp_collector",
        "source_grid": f"research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl",
    })

with (OUT / "standard-cost-grid.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(handle, fieldnames=FIELDS, lineterminator="\n")
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
            "candidate_over_external": float(row["candidate_over_external"]) if row["candidate_over_external"] else None,
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
        {"lead": "pencil_collector", "status": "partial", "candidate_distinct_targets": 543,
         "rho_distinct_targets": 544, "failed_distinct_targets": 1,
         "open_distinct_targets": 717,
         "remaining": "717 original target workloads have no completed candidate solve; one p=186379 target has two cap failures and source-verified rho.",
         "source": "research/pencil_sparse_20261022/REPORT.md"},
        {"lead": "kummer_walk", "status": "partial", "coordinate_label_cases": 8820,
         "nonzero_matches": 8813, "zero_outputs": 7, "wrong_nonzero_outputs": 0,
         "candidate_dlp_targets": 0, "candidate_over_rho": None,
         "signed_recoveries_verified": 1260, "zero_coordinate_repairs_verified": 175,
         "remaining": "A closed labeled DLP collector with complete cost.",
         "source": "research/kummer_recover_20261022/REPORT.md"},
    ],
}
(OUT / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")

target_fields = [
    "lead", "arm", "bits", "p", "target_index", "workload_id", "source_verified",
    "candidate_dlp_verified", "rho_verified", "candidate_GAE", "rho_GAE",
    "candidate_over_rho", "coordinate_nonzero_matches", "coordinate_zero_outputs",
    "coordinate_failures", "signed_recover_verified", "recover_method", "recover_record",
    "status", "source_record", "failure", "solver_mode", "order",
]
targets = []

def append_source_targets(path, lead, arm, p_key):
    with (ROOT / path).open() as handle:
        for line in handle:
            source = json.loads(line)
            verified = source.get("candidate_verified", source.get("verified", False))
            targets.append({
                "lead": lead, "arm": arm, "bits": source["bits"], "p": source[p_key],
                "target_index": source["target_index"], "workload_id": source["workload_id"],
                "source_verified": bool(verified), "candidate_dlp_verified": bool(verified),
                "rho_verified": bool(source.get("rho_verified", verified)),
                "candidate_GAE": source["candidate_GAE"], "rho_GAE": source["rho_GAE"],
                "candidate_over_rho": source["candidate_over_rho"],
                "coordinate_nonzero_matches": "", "coordinate_zero_outputs": "",
                "coordinate_failures": "", "signed_recover_verified": "", "recover_method": "",
                "recover_record": "", "status": "priced_source_verified",
                "source_record": source.get("raw_record", path),
                "failure": "", "solver_mode": "", "order": "",
            })

factor_path = "research/four_followups_20261021/results/matched/targets.jsonl"
with (ROOT / factor_path).open() as handle:
    for line in handle:
        source = json.loads(line)
        targets.append({
            "lead": "factor_base", "arm": {
                "fb1-j16": "full_16", "fb1-j8": "full_8", "fb1-j32": "full_32",
                "fb2-j16": "half_16", "fb4-j16": "quarter_16",
            }[source["arm"]],
            "bits": source["bits"], "p": source["curve_p"],
            "target_index": source["target_index"], "workload_id": source["workload_id"],
            "source_verified": bool(source["verified"]), "candidate_dlp_verified": bool(source["verified"]),
            "rho_verified": bool(source["verified"]),
            "candidate_GAE": source["candidate_GAE"], "rho_GAE": source["rho_GAE"],
            "candidate_over_rho": source["candidate_over_rho"],
            "coordinate_nonzero_matches": "", "coordinate_zero_outputs": "",
            "coordinate_failures": "", "signed_recover_verified": "", "recover_method": "",
            "recover_record": "", "status": "priced_source_verified",
            "source_record": source["raw_record"],
            "failure": "", "solver_mode": "", "order": "",
        })
append_source_targets("research/projective_law_followup_20261022/results/targets-projective.jsonl", "projective_law", "wrapper", "p")
append_source_targets("research/projective_law_followup_20261022/results/targets-state.jsonl", "projective_law", "retained_state", "p")
PENCIL_TARGETS = "research/pencil_sparse_20261022/results/targets.jsonl"
with (ROOT / PENCIL_TARGETS).open() as handle:
    for line in handle:
        source = json.loads(line)
        priced = source["representative"]
        verified = bool(source["candidate_verified"])
        targets.append({
            "lead": "pencil_collector", "arm": f'{source["solver_mode"]}_{source["order"]}',
            "bits": source["bits"], "p": source["curve_p"],
            "target_index": source["target_index"], "workload_id": source["workload_id"],
            "source_verified": bool(source["rho_verified"]),
            "candidate_dlp_verified": verified, "rho_verified": bool(source["rho_verified"]),
            "candidate_GAE": priced["pencil_GAE"] if priced else "",
            "rho_GAE": priced["rho_GAE"] if priced else "",
            "candidate_over_rho": priced["pencil_over_rho"] if priced else "",
            "coordinate_nonzero_matches": "", "coordinate_zero_outputs": "",
            "coordinate_failures": "", "signed_recover_verified": "", "recover_method": "",
            "recover_record": "", "status": "priced_source_verified" if verified else "incomplete_cap_unpriced",
            "source_record": source["raw_path"], "failure": source.get("failure") or "",
            "solver_mode": source["solver_mode"], "order": source["order"],
        })
for bits in (16, 20, 24, 28, 32, 36, 40):
    path = f"research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
    recover_path = f"research/kummer_recover_20261022/results/recover-target-{bits}.jsonl"
    with (ROOT / recover_path).open() as handle:
        recover = {(item["p"], item["target_index"]): item for item in map(json.loads, handle)}
    with (ROOT / path).open() as handle:
        for line in handle:
            source = json.loads(line)
            signed = recover[(source["p"], source["target_index"])]
            assert signed["workload_id"] == source["workload_id"]
            statuses = [case["coordinate_status"] for case in source["cases"]]
            passed = statuses.count("pass")
            zero = statuses.count("zero_output")
            failures = len(statuses) - passed - zero
            assert passed + zero + failures == source["case_count"]
            targets.append({
                "lead": "kummer_walk", "arm": "coordinate_identity_gate", "bits": bits, "p": source["p"],
                "target_index": source["target_index"], "workload_id": source["workload_id"],
                "source_verified": bool(source["all_source_checks"] and source["source_scalar_verified"]),
                "candidate_dlp_verified": "", "rho_verified": "", "candidate_GAE": "", "rho_GAE": "",
                "candidate_over_rho": "", "coordinate_nonzero_matches": passed,
                "coordinate_zero_outputs": zero, "coordinate_failures": failures,
                "signed_recover_verified": bool(signed["correct"] and signed["source_scalar_verified"]),
                "recover_method": signed["status"], "recover_record": recover_path,
                "status": "coordinate_and_recover_gate_no_dlp_collector", "source_record": path,
                "failure": "", "solver_mode": "", "order": "",
            })

assert len(targets) == 11528, len(targets)
assert sum(int(row["coordinate_zero_outputs"]) for row in targets if row["lead"] == "kummer_walk") == 7
assert sum(int(row["coordinate_failures"]) for row in targets if row["lead"] == "kummer_walk") == 0
assert all(row["signed_recover_verified"] for row in targets if row["lead"] == "kummer_walk")
assert sum(row["candidate_dlp_verified"] is True for row in targets if row["lead"] == "pencil_collector") == 1446
assert sum(row["status"] == "incomplete_cap_unpriced" for row in targets) == 2
by_input = {}
for row in targets:
    key = (row["bits"], row["p"], row["target_index"])
    by_input.setdefault(key, set()).add(row["workload_id"])
assert len(by_input) == 1260 and all(len(workloads) == 1 for workloads in by_input.values())
assert all(bool(row["source_verified"]) for row in targets)
assert all(
    math.isclose(float(row["candidate_GAE"]) / float(row["rho_GAE"]), float(row["candidate_over_rho"]), rel_tol=1e-9)
    for row in targets if row["candidate_dlp_verified"]
)
with (OUT / "standard-targets.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(handle, fieldnames=target_fields, lineterminator="\n")
    writer.writeheader()
    writer.writerows(targets)
summary["target_rows"] = len(targets)
summary["distinct_target_keys"] = len(by_input)
(OUT / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")

COLORS = {"full_16": "#1764aa", "full_8": "#16a0a0", "full_32": "#3946a5", "half_16": "#b2691b", "quarter_16": "#a03543", "wrapper": "#75832a", "retained_state": "#8647a3"}
grouped = {}
for row in representative:
    if row["slice"] != "all" and row["lead"] != "pencil_collector":
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

# A paired price-band view keeps the pencil's losses visible; each vertical
# line spans all 36 source cells and its point is the declared native price.
pencil = [row for row in rows if row["lead"] == "pencil_collector" and row["arm"] == "degree_peel"]
native = {(row["slice"]): row for row in pencil
          if row["S_per_M"] == "0.5" and row["I_per_M"] == "10"
          and row["native_GAE_per_logical"] == "0.1"}
assert len(pencil) == 360 and len(native) == 10
px = lambda index: 100 + index * 76
py = lambda ratio: 270 - (ratio - 0.2) / 1.0 * 215
psvg = ['<svg xmlns="http://www.w3.org/2000/svg" width="920" height="355" viewBox="0 0 920 355">',
        '<rect width="100%" height="100%" fill="white"/>',
        '<text x="100" y="27" font-family="sans-serif" font-size="17">Degree-peel / matched online: 36-cell range and native price</text>']
for ratio in (0.2, 0.5, 1.0, 1.2):
    yy = py(ratio)
    psvg += [f'<line x1="100" y1="{yy:.1f}" x2="815" y2="{yy:.1f}" stroke="#d8dfe7"/>',
             f'<text x="88" y="{yy+4:.1f}" text-anchor="end" font-family="sans-serif" font-size="12">{ratio}</text>']
for i, p in enumerate(PENCIL_CURVES):
    slice_name = f"p{p}"
    band = [float(row["candidate_over_baseline"]) for row in pencil if row["slice"] == slice_name]
    assert len(band) == 36
    yy1, yy2 = py(min(band)), py(max(band))
    yyn = py(float(native[slice_name]["candidate_over_baseline"]))
    xx = px(i)
    psvg += [f'<line x1="{xx}" y1="{yy1:.1f}" x2="{xx}" y2="{yy2:.1f}" stroke="#426b9d" stroke-width="4"/>',
             f'<circle cx="{xx}" cy="{yyn:.1f}" r="5" fill="#bd5b24"/>',
             f'<text x="{xx}" y="304" text-anchor="middle" font-family="sans-serif" font-size="11">{p}</text>']
psvg += ['<text x="460" y="329" text-anchor="middle" font-family="sans-serif" font-size="12">Curve prime p; last column is one target, others are 60 targets</text>',
         '</svg>']
(HERE / "pencil-ratios.svg").write_text("\n".join(psvg) + "\n")
print(f"wrote {len(rows)} normalized rows and {len(representative)} representative rows")
