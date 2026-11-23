#!/usr/bin/env python3
"""Recompute the evidence gates behind the 2026-10-31 frontier statement.

This is deliberately a stdlib-only cross-artifact audit.  It does not turn
historical records into a current wall-time measurement.  In particular, a
successful run means that the retained evidence supports the versioned claims
written to certificate.json; it does not discharge the draft EKS campaign.
"""

from __future__ import annotations

import hashlib
import json
import math
import statistics
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STUDY = ROOT / "research/ecbench_e2e_frontier_20261031"
OUT = STUDY / "establishment/certificate.json"


def load(path: Path):
    return json.loads(path.read_text())


def load_jsonl(path: Path):
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def sha256(path: Path):
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


checks = []


def check(name: str, condition: bool, detail):
    checks.append({"name": name, "ok": bool(condition), "detail": detail})


def close(a, b, tol=1e-9):
    return math.isclose(float(a), float(b), rel_tol=tol, abs_tol=tol)


def one(rows, predicate, label):
    found = [row for row in rows if predicate(row)]
    check(f"unique:{label}", len(found) == 1, {"matches": len(found)})
    if len(found) != 1:
        raise RuntimeError(f"expected one {label}, found {len(found)}")
    return found[0]


# ---------------------------------------------------------------- registry

registry_path = STUDY / "method-registry.json"
coverage_path = STUDY / "coverage.json"
registry = load(registry_path)
coverage = load(coverage_path)
registry_ids = [row["id"] for row in registry]
coverage_ids = [row["id"] for row in coverage["methods"]]
solver_ids = [row["id"] for row in coverage["solver_backends"]]
check("registry:74-top-level", len(registry_ids) == 74, len(registry_ids))
check("registry:unique-top-level", len(set(registry_ids)) == len(registry_ids), len(set(registry_ids)))
check("registry:coverage-exact", set(registry_ids) == set(coverage_ids), {
    "missing": sorted(set(registry_ids) - set(coverage_ids)),
    "extra": sorted(set(coverage_ids) - set(registry_ids)),
})
check("registry:11-nested-solvers", len(solver_ids) == 11 and len(set(solver_ids)) == 11, solver_ids)


# ---------------------------------------------------- standing generic boundary

leaderboard_path = ROOT / "docs/ic/leaderboard.json"
leaderboard = load(leaderboard_path)
generic_leaders = {
    family: {
        "slug": leaderboard["leaders"][family]["slug"],
        "ratio_rho": leaderboard["leaders"][family]["ratio_rho"],
    }
    for family in ("prime", "char2", "koblitz")
}
check(
    "generic:standing-leaders-above-rho",
    all(row["ratio_rho"] > 1 for row in generic_leaders.values()),
    generic_leaders,
)
protocol = load(STUDY / "protocol.json")
check("generic:fresh-campaign-remains-draft", protocol["freeze"]["status"] == "draft", protocol["freeze"])


# ----------------------------------------------------------- GHS-weak exception

n4_summary_path = ROOT / "research/quartic_n4_20261019/results/summary_primary.json"
n4_rows = load_jsonl(n4_summary_path)
n4_b0 = one(n4_rows, lambda x: x.get("arm") == "B0" and x.get("q") == 4051, "N4 B0 q=4051")
check(
    "ghs:N4-B0-below-plain-rho",
    n4_b0["ratio_hi"][0] < 1 and n4_b0["resolved"] == "IC lower",
    {"ratio": n4_b0["ratio"][0], "ci95": [n4_b0["ratio_lo"][0], n4_b0["ratio_hi"][0]]},
)

n4v2_summary_path = ROOT / "research/quartic_n4v2_20261020/results/summary.json"
n4v2_rows = load_jsonl(n4v2_summary_path)
n4v2_dl = one(
    n4v2_rows,
    lambda x: x.get("kind") == "ratio" and x.get("ic") == "DL" and x.get("rho") == "neg"
    and x.get("q") == 2097091,
    "N4-v2 DL/neg q=2097091",
)
check(
    "ghs:N4v2-DL-below-negation-rho",
    n4v2_dl["hi"][0] < 1 and n4v2_dl["resolved"] == "IC lower",
    {"ratio": n4v2_dl["ratio"][0], "ci95": [n4v2_dl["lo"][0], n4v2_dl["hi"][0]]},
)
ghs_raw = ROOT / "research/quartic_n4v2_20261020/results/grid-20261025/units.tar.gz"
ghs_manifest = load(ROOT / "research/quartic_n4v2_20261020/results/grid-20261025/.artifacts.json")


# ---------------------------------------------------- P20 second-host validation

p20_root = ROOT / "research/prime_base_oracle_tuning_20261029"
p20_validation_path = p20_root / "VALIDATION.json"
p20_validation = load(p20_validation_path)
weights = ("0", "0.1", "1")
p20 = {"selected_over_incumbent": {}, "selected_over_rho": {}}
for weight in weights:
    inc = load(p20_root / f"validation/second-host/compare/compare-inc__sel-w{weight}.json")
    rho = load(p20_root / f"validation/second-host/compare/compare-rho-neg__sel-w{weight}.json")
    p20["selected_over_incumbent"][weight] = {
        "ratio": inc["ops"]["ratio_b_over_a"], "ci95": inc["ops"]["ci95_bootstrap"]
    }
    p20["selected_over_rho"][weight] = {
        "ratio": rho["ops"]["ratio_b_over_a"], "ci95": rho["ops"]["ci95_bootstrap"]
    }
check(
    "p20:second-host-validation",
    p20_validation["verdict"] == "passed"
    and p20_validation["second_host_replay"]["status"] == "verified"
    and p20_validation["second_host_replay"]["records_matched_identical"] == "1008/1008",
    {
        "verdict": p20_validation["verdict"],
        "status": p20_validation["second_host_replay"]["status"],
        "matched": p20_validation["second_host_replay"]["records_matched_identical"],
    },
)
check(
    "p20:selected-beats-incumbent-across-band",
    all(p20["selected_over_incumbent"][w]["ci95"][1] < 1 for w in weights),
    p20["selected_over_incumbent"],
)
check(
    "p20:selected-remains-behind-rho-across-band",
    all(p20["selected_over_rho"][w]["ci95"][0] > 1 for w in weights),
    p20["selected_over_rho"],
)


# ---------------------------------------------------- F4/F5/F6 historical data

f456_root = ROOT / "research/f4_f5_f6_cross_family_20261011"


def measured_by_arm(cell: str):
    rows = load_jsonl(f456_root / f"sessions/{cell}/records.jsonl")
    out = {}
    for row in rows:
        if row.get("warmup"):
            continue
        out.setdefault(row["arm"], []).append(row)
    return out


def med(rows, getter):
    return statistics.median(getter(row) for row in rows)


def x_has_solver(rows):
    return bool(rows and rows[0].get("solver") and rows[0]["solver"].get("ops") is not None)


def paired_median_ratio(by, numerator, denominator, getter):
    def keyed(rows):
        return {(row["workload"]["workload_id"], row["round"]): row for row in rows}

    num = keyed(by[numerator])
    den = keyed(by[denominator])
    keys = sorted(set(num) & set(den))
    if len(keys) != len(num) or len(keys) != len(den):
        raise RuntimeError(f"unpaired {numerator}/{denominator}: {len(keys)}, {len(num)}, {len(den)}")
    return statistics.median(getter(num[key]) / getter(den[key]) for key in keys)


f456_historical = {}
for cell in ("K13", "R13", "K19", "R19"):
    by = measured_by_arm(cell)
    row = {
        "records": sum(map(len, by.values())),
        "median_s": {arm: med(rows, lambda x: x["cost"]["s"]) for arm, rows in by.items()},
        "median_solver_ops": {
            arm: med(rows, lambda x: x["solver"]["ops"])
            for arm, rows in by.items() if x_has_solver(rows)
        },
    }
    if "inherited-f4" in by and "f6-ic" in by:
        row["f6_over_inherited_solver_ops"] = paired_median_ratio(
            by, "f6-ic", "inherited-f4", lambda x: x["solver"]["ops"]
        )
        row["f6_over_inherited_e2e_s"] = paired_median_ratio(
            by, "f6-ic", "inherited-f4", lambda x: x["cost"]["s"]
        )
        row["inherited_over_rho_negation_s"] = paired_median_ratio(
            by, "inherited-f4", "rho-negation", lambda x: x["cost"]["s"]
        )
    if "matrix-f4" in by and "matrix-f5" in by:
        row["f5_over_f4_solver_ops"] = paired_median_ratio(
            by, "matrix-f5", "matrix-f4", lambda x: x["solver"]["ops"]
        )
    f456_historical[cell] = row


def historical_solver_gate(row):
    return row["f6_over_inherited_solver_ops"] < 1 and row["f6_over_inherited_e2e_s"] > 1


check(
    "f456:historical-local-saving-is-not-e2e-saving",
    all(historical_solver_gate(f456_historical[cell]) for cell in ("K13", "R13", "K19", "R19"))
    and all(f456_historical[cell]["f5_over_f4_solver_ops"] < 1 for cell in ("K13", "R13")),
    {
        cell: {k: v for k, v in row.items() if k.endswith("_ops") or k.endswith("_s")}
        for cell, row in f456_historical.items()
    },
)


# Current-binary all-replay receipts.  Failures are evidence and are expected.
receipt_dir = STUDY / "establishment/receipts"
f456_replays = {}
for cell in ("K13", "R13", "K19", "R19", "X19"):
    receipt = load(receipt_dir / f"{cell}.json")
    candidates = receipt.get("replay_candidates", 0)
    reproduced = sum(1 for row in receipt.get("replays", []) if row["reproduced"])
    f456_replays[cell] = {
        "ok": receipt["ok"],
        "records": receipt["records"],
        "verified_records": receipt["verified_records"],
        "replay_candidates": candidates,
        "reproduced": reproduced,
        "failures": candidates - reproduced,
        "receipt_sha256": sha256(receipt_dir / f"{cell}.json"),
    }
f456_totals = {
    "candidates": sum(row["replay_candidates"] for row in f456_replays.values()),
    "reproduced": sum(row["reproduced"] for row in f456_replays.values()),
    "failures": sum(row["failures"] for row in f456_replays.values()),
}
check("f456:current-binary-replay-regression-retained", f456_totals == {
    "candidates": 144, "reproduced": 72, "failures": 72
}, {"cells": f456_replays, "total": f456_totals})


# -------------------------------------------------------- rho implementation

rho_root = ROOT / "research/ecbench_rho_optimized_20261014"
rho_result = load(rho_root / "RESULT.json")
callgrind_dir = rho_root / "callgrind"
manifest_ok = True
manifest_entries = 0
for line in (callgrind_dir / "SHA256SUMS").read_text().splitlines():
    if not line.strip():
        continue
    expected, rel = line.split(None, 1)
    manifest_entries += 1
    manifest_ok &= sha256(callgrind_dir / rel.strip()) == expected
check("rho:callgrind-raw-hashes", manifest_ok and manifest_entries == 30, manifest_entries)


def callgrind_solve_ir(prefix: Path):
    total = 0
    for part in range(2, 6):
        summaries = [
            int(line.split(":", 1)[1])
            for line in (Path(f"{prefix}.{part}")).read_text().splitlines()
            if line.startswith("summary:")
        ]
        if len(summaries) != 1:
            raise RuntimeError(f"unexpected summaries in {prefix}.{part}: {summaries}")
        total += summaries[0]
    return total


legacy_ir = callgrind_solve_ir(callgrind_dir / "final-legacy/legacy")
optimized_ir = callgrind_solve_ir(callgrind_dir / "final-optimized/candidate")
rho_ratio = optimized_ir / legacy_ir
rho_speedup = legacy_ir / optimized_ir
check(
    "rho:instruction-speedup-recomputed",
    legacy_ir == rho_result["callgrind"]["same_binary_legacy"]["solve_ir"]
    and optimized_ir == rho_result["callgrind"]["optimized"]["solve_ir"]
    and close(rho_ratio, rho_result["callgrind"]["optimized_over_same_binary_legacy"])
    and close(rho_speedup, rho_result["callgrind"]["same_binary_speedup"], 1e-6),
    {"legacy_ir": legacy_ir, "optimized_ir": optimized_ir, "ratio": rho_ratio, "speedup": rho_speedup},
)
rho_receipt_path = receipt_dir / "rho.json"
rho_receipt = load(rho_receipt_path)
rho_reproduced = sum(1 for row in rho_receipt["replays"] if row["reproduced"])
check(
    "rho:current-binary-all-replay",
    rho_receipt["ok"] and rho_receipt["replay_candidates"] == 720 and rho_reproduced == 720,
    {"verified": rho_receipt["verified_records"], "candidates": rho_receipt["replay_candidates"],
     "reproduced": rho_reproduced, "receipt_sha256": sha256(rho_receipt_path)},
)
check(
    "rho:l2-wall-remains-open",
    rho_result["ecbench_session"]["native_wall"]["earned_level"] == "L0"
    and rho_result["ecbench_session"]["native_wall"]["required_level"] == "L2"
    and rho_result["frontier_effect"]["native_wall_claim_admitted"] is False,
    rho_result["ecbench_session"]["native_wall"],
)


certificate = {
    "schema": "ecbench.frontier-establishment/v1",
    "as_of": "2026-10-31",
    "scope": "versioned evidence audit; not a substitute for the draft EKS wall campaign",
    "source_identity": {
        "code_source_commit": coverage["source_commit"],
        "auditor_binary_sha256": coverage["binary_sha256"],
        "evidence_commit_at_audit_start": "66c71217bef08b414826f9c1e50773d7eeb96603",
        "registry_sha256": sha256(registry_path),
        "coverage_sha256": sha256(coverage_path),
    },
    "gates": {
        "generic_prime_random_binary_koblitz": {
            "status": "standing_admitted_frontier_established; fresh_current_source_wall_open",
            "leaders": generic_leaders,
            "fresh_protocol_status": protocol["freeze"]["status"],
        },
        "ghs_weak_exception": {
            "status": "established_in_counted_aggregate; raw_archive_unavailable_in_this_workspace",
            "scope": "GHS-weak E0/Fq^3 with #E0(K)=4*ell only",
            "n4_b0_over_plain_rho": {
                "q": 4051, "ratio": n4_b0["ratio"][0],
                "ci95": [n4_b0["ratio_lo"][0], n4_b0["ratio_hi"][0]],
            },
            "n4v2_dl_over_negation_rho": {
                "q": 2097091, "ratio": n4v2_dl["ratio"][0],
                "ci95": [n4v2_dl["lo"][0], n4v2_dl["hi"][0]],
            },
            "raw_archive_present": ghs_raw.exists(),
            "raw_archive_manifest": ghs_manifest["files"]["units.tar.gz"],
        },
        "prime_p20_factor_base": {
            "status": "established_second_host_counted_evidence; one_curve_no_scaling_claim",
            **p20,
        },
        "f4_f5_f6": {
            "status": "historical_binary_result_established; current_source_result_not_established",
            "historical": f456_historical,
            "current_binary_all_replay": {"cells": f456_replays, "total": f456_totals},
        },
        "rho_optimization": {
            "status": "instruction_and_count_stability_established; L2_wall_open",
            "legacy_ir": legacy_ir,
            "optimized_ir": optimized_ir,
            "optimized_over_legacy": rho_ratio,
            "speedup": rho_speedup,
            "current_binary_replays": {"candidates": 720, "reproduced": rho_reproduced},
            "l2_wall_status": "unknown",
        },
        "registry_coverage": {
            "status": "established",
            "top_level_methods": len(registry_ids),
            "nested_solver_backends": len(solver_ids),
        },
    },
    "checks": checks,
}
certificate["all_artifact_consistency_checks_passed"] = all(row["ok"] for row in checks)
certificate["open_obligations"] = [
    "Bind an immutable ECR image, freeze protocol.json, and execute all nine prepared EKS L2 jobs.",
    "Rerun F4/F5/F6 and all 11 solver backends from the current binary; historical IC counts do not all-replay.",
    "Restore the GHS units.tar.gz artifact to permit a raw-record recomputation of the N4-v2 aggregate.",
    "Measure optimized versus legacy signed-Frobenius rho on L2; Callgrind Ir does not establish wall speedup.",
]
OUT.write_text(json.dumps(certificate, indent=2, sort_keys=True) + "\n")
print(json.dumps({
    "out": str(OUT.relative_to(ROOT)),
    "ok": certificate["all_artifact_consistency_checks_passed"],
    "checks": len(checks),
    "f456_replays": f456_totals,
    "rho_replays": {"candidates": 720, "reproduced": rho_reproduced},
}, indent=2))
sys.exit(0 if certificate["all_artifact_consistency_checks_passed"] else 1)
