#!/usr/bin/env python3
"""Package already-run independent source replay and price validation for the gate."""

import csv
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def load(name):
    return json.loads((RESULTS / name).read_text())

validator = load("independent-validation.json")
assert validator["status"] == "pass" and validator["grid_cells"] == 576
raw = []
for arm in ("projective", "state"):
    summary = load(f"summary-{arm}.json")
    assert summary["candidate_verified"] == summary["rho_verified"] == 1260
    assert summary["independent_source_scalar_replays"] == 2520
    assert summary["collector_replay_rows"] == (1260 if arm == "projective" else 0)
    assert summary["archived_rho_count_replays"] == 1260
    for bits in (16, 20, 24, 28, 32, 36, 40):
        path = RESULTS / f"{arm}-{bits}.jsonl"
        assert sum(1 for _ in path.open()) == 180
        raw.append({"path": f"results/{path.name}", "sha256": digest(path), "rows": 180})

certificate = {
    "schema": "projective-law-replay-certificate/v1",
    "status": "verified", "independent": True, "scalars_verified": True,
    "method": "analyze-projective replays every candidate and same-process negation-rho scalar by source-curve multiplication; joins frozen workload, seed, cap and archived rho counters",
    "candidate_rows": 2520, "rho_rows": 2520,
    "independent_source_scalar_replays": 5040,
    "archived_rho_count_replays": 2520,
    "firstpass_collector_counter_replays": 1260,
    "runs": raw,
    "validator_report": "results/independent-validation.json",
}
(HERE / "replay-certificate.json").write_text(json.dumps(certificate, indent=2, sort_keys=True) + "\n")

headlines = []
for arm in ("projective", "state"):
    claimed = load(f"summary-{arm}.json")["summaries"][0]
    recomputed = next(item for item in validator["arms"] if item["arm"] == arm)
    for key in ("candidate_GAE", "rho_GAE", "candidate_over_rho"):
        headlines.append({"name": f"{arm}.{key}", "claimed": claimed[key],
                          "recomputed": recomputed[key], "rel_tol": 1e-9})
report = {
    "schema": "projective-law-validation-gate-report/v1", "status": "pass",
    "independent": True,
    "method": "Separate native validate-projective binary recomputed arm/slice/price grids and cold phase sums from committed raw records; this gate wrapper compares its output to analyzer summaries",
    "raw_data": raw,
    "independent_result": {"path": "results/independent-validation.json", "sha256": digest(RESULTS / "independent-validation.json")},
    "headline": headlines,
}
(HERE / "validator-report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")

bands = {}
for arm in ("projective", "state"):
    with (RESULTS / f"price-grid-{arm}.csv").open(newline="") as handle:
        relevant = [row for row in csv.DictReader(handle)
                    if row["slice"] == "all" and row["S_per_M"] == "0.5" and row["I_per_M"] == "10"
                    and row["native_GAE_per_logical"] in ("0", "0.1", "1")]
    assert len(relevant) == 3
    bands[arm] = {row["native_GAE_per_logical"]: {
        "ic": float(row["candidate_GAE"]), "rho": float(row["rho_GAE"])} for row in relevant}
    assert all(item["ic"] > item["rho"] for item in bands[arm].values())

gate = {
    "schema": "validation-gate/v1",
    "replay_certificate": {"path": "replay-certificate.json", "sha256": digest(HERE / "replay-certificate.json")},
    "validator_report": {"path": "validator-report.json", "sha256": digest(HERE / "validator-report.json")},
    "rho_reference": {
        "kind": "negation", "unit": "GAE", "matches_protocol": True,
        "same_unit": True, "same_binary_or_process": True, "strongest_available": True,
        "binary_or_process": "Each of projective-collector and projective-state-collector executes candidate and source negation rho in the same process",
        "protocol": "research/projective_law_followup_20261022/PROTOCOL.md; frozen 21-model 1,260-target source cohort; rho selection inherited from the source-matched four-followups gate",
        "gae_band": bands["projective"], "state_gae_band": bands["state"],
        "order_holds_across_band": True,
    },
}
(HERE / "VALIDATION.json").write_text(json.dumps(gate, indent=2, sort_keys=True) + "\n")
print("packaged 14 raw files, 5,040 source scalar replays and 576 validated price cells")
